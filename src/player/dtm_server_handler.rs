use std::collections::HashMap;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::process::exit;
use std::sync::Arc;

use async_trait::async_trait;
use scupt_net::endpoint::Endpoint;
use scupt_net::handle_event::HandleEvent;
use scupt_net::message_incoming::MessageIncoming;
use scupt_net::message_sender::Sender;
use scupt_net::notifier::Notifier;
use scupt_net::opt_send::OptSend;
use scupt_net::task::spawn_local_task;
use scupt_util::error_type::ET;
use scupt_util::message::Message;
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use serde_json::Value;
use tokio::select;
use tokio::sync::{mpsc, Mutex, oneshot};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, Instrument, trace, trace_span};

use crate::action::action_serde_json_string::ActionSerdeJsonString;
use crate::action::action_serde_json_value::ActionSerdeJsonValue;
use crate::action::action_type::ActionType;
use crate::player::action_executor::ActionExecutor;
use crate::player::action_incoming::ActionIncoming;
use crate::player::action_waiter::ActionPrefixWaiter;
use crate::player::dtm_player::TestOption;
use crate::player::msg_ctrl::MessageControl;

enum DTMCmd {
    StartAction((Arc<dyn ActionIncoming>, oneshot::Sender<Res<()>>)),
}

struct Handler {
    output_action_sequential:bool,
    trace_in_one_sequence:bool,
    notify: Notifier,
    node_sender: Arc<dyn Sender<ActionSerdeJsonString>>,
    executor: Arc<ActionExecutor>,
    channel_s: Arc<mpsc::UnboundedSender<DTMCmd>>,
    channel_r: Arc<Mutex<mpsc::UnboundedReceiver<DTMCmd>>>,
}

pub struct DTMServerHandler {
    handler: Arc<Handler>,
    seconds_timeout:u64,
    node_id:NID,
}

impl  DTMServerHandler {
    pub fn new(
        node_id:NID,
        node_sender: Arc<dyn Sender<ActionSerdeJsonString>>,
        notify:Notifier,
        option:TestOption,
    ) -> DTMServerHandler {
        let (s, r) = mpsc::unbounded_channel::<
            DTMCmd>();

        let h = Handler {
            output_action_sequential: option.output_action_sequential,
            trace_in_one_sequence: option.wait_both_begin_and_end_action,
            notify,
            node_sender,
            executor: Arc::new(ActionExecutor::new(
                option.wait_both_begin_and_end_action,
                option.seconds_wait_message_timeout
            )),
            channel_s: Arc::new(s),
            channel_r: Arc::new(Mutex::new(r)),
        };
        Self {
            handler: Arc::new(h),
            seconds_timeout: option.seconds_wait_message_timeout,
            node_id,
        }
    }

    #[async_backtrace::framed]
    pub async fn async_run(&self) -> Res<()> {
        let mut ch = self.handler.channel_r.lock().await;
        loop {
            let opt_cmd = ch.recv().await;
            match opt_cmd {
                Some(cmd) => { self.run_cmd(cmd).await? }
                None => {}
            }
        }
    }

    pub async fn begin_run_test(&self, input: Arc<dyn ActionIncoming>) -> Res<oneshot::Receiver<Res<()>>> {
        let (s, r) = oneshot::channel::<Res<()>>();
        self.send_dtm_cmd(DTMCmd::StartAction((input, s)))?;
        Ok(r)
    }

    fn send_dtm_cmd(&self, c: DTMCmd) -> Res<()> {
        match self.handler.channel_s.send(c) {
            Ok(()) => { Ok(()) }
            Err(e) => { Err(ET::TokioSenderError(e.to_string())) }
        }
    }

    #[async_backtrace::framed]
    async fn run_cmd(&self, cmd: DTMCmd) -> Res<()> {
        match cmd {
            DTMCmd::StartAction((input, result_sender)) => {
                self.run_read_input(input, result_sender).await?;
            }
        }
        Ok(())
    }

    #[async_backtrace::framed]
    async fn run_read_input(
        &self,
        input: Arc<dyn ActionIncoming>,
        result_sender: oneshot::Sender<Res<()>>,
    ) -> Res<()> {
        trace!("run read input");
        let executor = self.handler.executor.clone();
        let sender = self.handler.node_sender.clone();
        let notify = self.handler.notify.clone();
        let seconds_timeout = self.seconds_timeout;
        let r = Self::loop_read_input_action(
            self.node_id,
            notify, input, executor, sender,
            self.handler.output_action_sequential,
            !self.handler.trace_in_one_sequence,
            seconds_timeout
        ).await;
        match &r {
            Ok(()) => {}
            Err(e) => {
                match e {
                    ET::EOF => {}
                    _ => { error!("{}", e.to_string()); }
                }
            }
        }
        let _ = result_sender.send(r);
        Ok(())
    }

    #[async_backtrace::framed]
    async fn loop_read_input_action(
        server_node_id:NID,
        notify:Notifier,
        input: Arc<dyn ActionIncoming>,
        executor: Arc<ActionExecutor>,
        sender: Arc<dyn Sender<ActionSerdeJsonString>>,
        output_action_sequential:bool,
        per_node_trace:bool,
        seconds_timeout:u64
    ) -> Res<()> {
        let waiter = ActionPrefixWaiter::new();
        if !per_node_trace {
            let mut tasks = vec![];
            let trace = Self::input_to_one_trace(input)?;
            let r = Self::handle_trace(
                server_node_id,
                None, notify, trace, executor, sender,
                output_action_sequential, seconds_timeout,
                waiter.clone(),
                &mut tasks).await;
            for t in tasks {
                let _ = t.await;
            }
            r?
        } else {
            let trace_map = Self::input_to_per_node_trace(input)?;
            let mut join = vec![];
            for (nid, trace) in trace_map {
                let n = notify.clone();
                let e = executor.clone();
                let s = sender.clone();
                let w = waiter.clone();
                let f = async move {
                    let mut tasks = vec![];
                    let r = Self::handle_trace(
                        server_node_id,
                        Some(nid), n, trace, e, s, output_action_sequential, seconds_timeout,
                        w,
                        &mut tasks).await;
                    for t in tasks {
                        let _ = t.await;
                    }
                    r
                };
                let j = spawn_local_task(notify.clone(), format!("handle_trace_node_{:?}", nid).as_str(), f);
                join.push(j);
            }
            for j in join {
                let opt_r  = j?.await;
                match opt_r {
                    Ok(_) => {},
                    Err(e) => { error!("join handle node trace {:?}", e)}
                }
            }
        }
        Ok(())
    }

    fn handle_input_action<F>(
        input: Arc<dyn ActionIncoming>,
        f:&mut F,
    ) -> Res<()>
    where  F: FnMut(ActionSerdeJsonValue, u64) -> Res<()>
    {
        let mut index = 0;
        loop {
            let res_action = input.next();
            let action:String;
            match res_action {
                Ok(a) => { action = a }
                Err(e) => {
                    if e == ET::EOF {
                        break;
                    } else {
                        return Err(e)
                    }
                }
            };
            let r  = serde_json::from_str(action.as_str());
            let value:Value = match r {
                Ok(r) => { r }
                Err(e) => { return Err(ET::SerdeError(e.to_string())); }
            };

            index += 1;
            let value = ActionSerdeJsonValue::from_json_value(value)?;
            f(value, index)?;
        }
        Ok(())
    }


    fn input_to_per_node_trace(
        input: Arc<dyn ActionIncoming>,
    ) -> Res<HashMap<NID, Vec<(ActionSerdeJsonValue, u64)>>> {
        let mut map : HashMap<NID, Vec<(ActionSerdeJsonValue, u64)>> = HashMap::new();
        let mut f = |v:ActionSerdeJsonValue, index:u64| -> Res<()> {
            let action_type = v.action_type()?;

            let nid = match action_type {
                ActionType::Input |
                ActionType::Setup |
                ActionType::Check => {
                    v.dest_node_id()?
                }
                ActionType::Internal => {
                    v.dest_node_id()?
                }
                ActionType::Output => {
                    v.source_node_id()?
                }
            };
            if !map.contains_key(&nid) {
                map.insert(nid, vec![(v, index)]);
            } else {
                if let Some(vec) = map.get_mut(&nid) {
                    vec.push((v, index));
                }
            }
            Ok(())
        };
        Self::handle_input_action(input, &mut f)?;
        Ok(map)
    }

    fn input_to_one_trace(
        input: Arc<dyn ActionIncoming>,
    ) -> Res<Vec<(ActionSerdeJsonValue, u64)>> {
        let mut vec : Vec<(ActionSerdeJsonValue, u64)> = Vec::new();
        let mut f = |v:ActionSerdeJsonValue, index:u64| -> Res<()> {
            vec.push((v, index));
            Ok(())
        };
        Self::handle_input_action(input, &mut f)?;
        Ok(vec)
    }

    #[async_backtrace::framed]
    async fn handle_trace(
        dtm_server_node_id:NID,
        node_id : Option<NID>,
        notify:Notifier,
        trace: Vec<(ActionSerdeJsonValue, u64)>,
        executor: Arc<ActionExecutor>,
        sender: Arc<dyn Sender<ActionSerdeJsonString>>,
        output_action_sequential:bool,
        seconds_timeout:u64,
        waiter: ActionPrefixWaiter,
        tasks:&mut Vec<JoinHandle<Option<Res<()>>>>,
    ) -> Res<()> {
        let mut seq_no_node = 0;

        // store node id when finding the first input action
        let mut found_input_action : HashSet<NID> = HashSet::new();

        // get the max index in trace
        let max_index = match trace.last() {
            Some((_, index)) => {
                assert!(*index > 0);
                *index - 1
            },
            None => 0
        };
        for (value, seq_no) in trace {
            seq_no_node += 1;
            assert!(seq_no >= 1);
            // num start with 1
            let index = seq_no - 1;
            let id_s;
            if node_id.is_none() {
                id_s = format!("No.{}", seq_no);
                debug!("DTM trace, {} action: {:?}", id_s.clone(),  value);
            } else {
                id_s = format!("No.{}.{}", seq_no, seq_no_node);
                debug!("DTM trace, Node:{:?}, {} action: {:?}", node_id, id_s.clone(),  value);
            }

            let action_type = value.action_type()?;
            let need_send_message_to_node = if
                action_type == ActionType::Input
                    || action_type == ActionType::Setup
                    || action_type == ActionType::Check
            {
                if action_type == ActionType::Input {
                    let dest_nid = value.dest_node_id()?;
                    let _ = found_input_action.insert(dest_nid);
                }
                true
            } else {
                if action_type == ActionType::Internal {
                    let nid = value.source_node_id()?;
                    // send message to node when only there are no previous input actions.
                    ! found_input_action.contains(&nid)
                } else {
                    false
                }
            };

            if need_send_message_to_node {
                waiter.wait_finish_prefix(index).await;
                let  value = value.action_json_value_ref();
                let json_value = ActionSerdeJsonValue::from_json_value(value.clone())?;
                let dest_node_id = json_value.dest_node_id()?;
                let action_message = ActionSerdeJsonString::from_json_value(
                    json_value.message_payload_json_value()?);
                trace!("action message: {:?}", action_message);
                let m = Message::new(action_message, dtm_server_node_id, dest_node_id);
                sender.send(m, OptSend::default().enable_no_wait(false))
                    .instrument(trace_span!("message to node"))
                    .await?;
            }
            let w = waiter.clone();
            if action_type == ActionType::Output && !output_action_sequential{
                let e = executor.clone();
                let n = notify.clone();
                let s = value.to_action_message().to_string().unwrap();
                let ss = s.clone();

                let f = async move {
                    let r = e.expect_node_sync(&value).instrument(trace_span!("file input")).await;
                    match r {
                        Ok(ok) => {
                            w.finish_one(index).await;
                            if !ok {
                                error!("message timeout, expect tested node sync {} {:?}", id_s, ss);
                                sleep(Duration::from_secs(seconds_timeout)).await;
                                exit(-1);
                            }
                        }
                        Err(e) => {
                            error!("failed error {:?}, expect action {} {}", e, id_s, ss);
                            exit(-1);
                        }
                    }
                    Res::Ok(())
                };
                let task = spawn_local_task(n, format!("wait_output_action_{}", s).as_str(), f)?;
                tasks.push(task);
            } else {
                let ok = executor.expect_node_sync(&value).instrument(trace_span!("file input")).await?;
                if !ok {
                    let s = value.to_action_message().to_string().unwrap();
                    error!("message timeout, expect tested node sync {} {:?}", id_s, s);
                    sleep(Duration::from_secs(seconds_timeout)).await;
                    exit(-1);
                } else {
                    w.finish_one(index).await;
                }
            }
        }
        // synchronize and wait for all actions done
        waiter.wait_finish_prefix(max_index).await;
        Ok(())
    }

    async fn handle_message_request_response(
        &self, ep:&Endpoint,
        channel:&mut UnboundedReceiver<Message<MessageControl>>,
        sender:&UnboundedSender<Message<MessageControl>>,
        tasks:&mut Vec<JoinHandle<Option<Res<()>>>>,
    ) -> Res<()> {
        select! {
            r_req = ep.recv::<MessageControl>()  => {
                match r_req {
                    Ok(m) => {
                        let s = sender.clone();
                        let n = self.handler.notify.clone();
                        let _self = self.clone();
                        let task = spawn_local_task(
                            n,
                            format!("dtm handle message {:?}", m).as_str(),
                            async move {
                                    _self.handle_one_message(m.source(), m.dest(), m.payload(), s).await
                            }
                        )?;
                        tasks.push(task);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            opt_resp = channel.recv() => {
                match opt_resp {
                    Some(m) => {
                        ep.send(m).await?
                    }
                    None => {
                        return Err(ET::EOF);
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_one_message(&self,
                                source:NID,
                                dest:NID,
                                message:MessageControl,
                                channel:UnboundedSender<Message<MessageControl>>) -> Res<()> {
        match message {
            MessageControl::ActionReq { id, action, begin } => {
                let action_json = action.to_action_serde_json_value()?;
                self.handler.executor.expect_action_in_trace(source, dest, id, action_json, begin, channel).await?;
            }
            MessageControl::ActionACK { id : _ }=> {
                panic!("error message")
            }
        }
        Ok(())
    }

    async fn message_loop(&self, ep:Endpoint) -> Res<()> {
        let mut tasks = vec![];
        let (s, mut r) = mpsc::unbounded_channel();
        let ret = loop {
            let r = self.handle_message_request_response(&ep, & mut r, &s, &mut tasks).await;
            match r {
                Ok(()) => {

                }
                Err(e) => {
                    break Err(e);
                }
            };
        };
        for task in tasks {
            let _ = task.await;
        }
        ret
    }
}

impl Clone for DTMServerHandler {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            seconds_timeout: self.seconds_timeout,
            node_id: self.node_id,
        }
    }
}


#[async_trait]
impl MessageIncoming<MessageControl> for DTMServerHandler {
    async fn incoming(&self, _: MessageControl) -> Res<()> {
        panic!("not possible");
    }
}

#[async_trait]
impl HandleEvent for DTMServerHandler {
    async fn on_accepted(&self, endpoint: Endpoint) -> Res<()> {
        let r = self.message_loop(endpoint).await;
        match r {
            Ok(()) => { Err(ET::EOF) }
            Err(e) => { Err(e) }
        }
    }

    async fn on_connected(
        &self,
        _: SocketAddr,
        _: Res<Endpoint>,
    ) -> Res<()> {
        Ok(())
    }
    async fn on_error(&self, error: ET) {
        error!("error {:?}", error);
    }

    async fn on_stop(&self) {}
}

unsafe impl Sync for DTMServerHandler {}

unsafe impl Send for DTMServerHandler {}