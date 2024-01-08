#[cfg(test)]
mod test {
    use std::collections::{HashMap, VecDeque};
    use std::net::{IpAddr, SocketAddr};
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;

    use bincode::{Decode, Encode};
    use once_cell::sync::Lazy;
    use rand::Rng;
    use rand::rngs::ThreadRng;
    use scupt_net::event_sink::{ESServeOpt, ESStopOpt};
    use scupt_net::io_service::{IOService, IOServiceOpt};
    use scupt_net::message_receiver::Receiver;
    use scupt_net::notifier::Notifier;
    use scupt_net::task::spawn_local_task;
    use scupt_util::error_type::ET;
    use scupt_util::logger::logger_setup;
    use scupt_util::message::{Message, MsgTrait};
    use scupt_util::node_id::NID;
    use scupt_util::res::Res;
    use scupt_util::res_of::res_io;
    use serde::{Deserialize, Serialize};
    use tokio::runtime::Builder;
    use tokio::sync::Notify;
    use tokio::task;
    use tokio::task::LocalSet;
    use tracing::{error, info, Instrument, trace, trace_span};

    use crate::{action_begin, action_end, auto_clear, auto_init, input, internal_begin, internal_end, output};
    use crate::action::action_message::ActionMessage;
    use crate::action::action_type::ActionType;
    use crate::action::panic::set_panic_hook;
    use crate::player::action_incoming::ActionIncoming;
    use crate::player::automata;
    use crate::player::dtm_player::TestOption;
    use crate::player::dtm_server::DTMServer;

    const AUTOMATON_NAME:&str ="TEST_AUTOMATON";

    static TEST_LOCK: Lazy<Mutex<()>> = Lazy::new(Mutex::default);

    #[test]
    fn test_dtm_player_check_all() {
        set_panic_hook();
        logger_setup("info");
        run_test(3, 4, 5, 18000, true);
    }

    #[test]
    fn test_dtm_player_no_check() {
        set_panic_hook();
        logger_setup("info");
        run_test(3, 4, 5, 19000, false);
    }

    fn run_test(num_node: u64, num_tx: u64, num_ops: u64, port: u16, enable_check: bool) {
        let _l = TEST_LOCK.lock().unwrap();
        let mut node_ids = vec![];
        let dtm_node_id = (num_node + 1) as NID;
        let dtm_address = SocketAddr::new(
            IpAddr::V4("127.0.0.1".parse().unwrap()),
            port);
        let mut address = HashMap::new();
        let history = History::new();
        for i in 0..num_node {
            let node_id = (i + 1) as NID;
            node_ids.push(node_id);
            address.insert(
                node_id,
                SocketAddr::new(
                    IpAddr::V4("127.0.0.1".parse().unwrap()),
                    port + (i + 1) as u16));
        }
        let mut thd_simulator = vec![];
        let mut thd_nodes = vec![];
        let stop = Arc::new(Notify::new());
        {
            let node_addr = address.clone();
            let id = dtm_node_id.clone();
            let addr = dtm_address.clone();
            let stop_notify = stop.clone();
            let history = history.clone();
            let thd = thread::spawn(move || {
                let r = run_simulator(
                    id, addr,
                    node_addr,
                    num_tx, num_ops,
                    stop_notify,
                    history,
                    enable_check
                );
                assert!(r.is_ok());
            });
            thd_simulator.push(thd);
        }

        auto_init!(AUTOMATON_NAME,0,  dtm_node_id, dtm_address.to_string().as_str());

        for (k, v) in address.iter() {
            let id = k.clone();
            let addr = v.clone();
            let history = history.clone();
            let thd = thread::spawn(move || {
                let r = run_node(id, addr, history, enable_check);
                assert!(r.is_ok());
            });
            thd_nodes.push(thd);
        }

        for j in thd_nodes {
            let _ = j.join();
        }

        // after test node stop, stop player
        stop.notify_waiters();
        for j in thd_simulator {
            let _ = j.join();
        }

        auto_clear!(AUTOMATON_NAME);
    }

    #[derive(
    Clone,
    Hash,
    PartialEq,
    Eq,
    Debug,
    Serialize,
    Deserialize,
    Decode,
    Encode,
    )]
    enum AppMsg {
        TaskNew { task_id: u64, task_ops: Vec<u64> },
        // input..
        TaskOP { task_id: u64, task_op: u64 },
        // internal
        TaskEnd { task_id: u64 },
        // output
        TaskStop, // input
    }

    impl MsgTrait for AppMsg {}

    struct ActionInputStub {
        queue: Arc<Mutex<VecDeque<ActionMessage<AppMsg>>>>,
    }

    impl ActionInputStub {
        pub fn new(vec: Vec<ActionMessage<AppMsg>>) -> Self {
            Self {
                queue: Arc::new(Mutex::new(VecDeque::from(vec)))
            }
        }
    }

    impl ActionIncoming for ActionInputStub {
        fn next(&self) -> Res<String> {
            let mut queue = self.queue.lock().unwrap();
            let opt_m = queue.pop_front();
            match opt_m {
                Some(m) => { Ok(m.to_serde_json_string()?.to_string()?) }
                None => { Err(ET::EOF) }
            }
        }
    }

    struct _TextNodeInner {}

    #[derive(Clone)]
    struct History {
        history: Arc<Mutex<Vec<ActionMessage<AppMsg>>>>,
    }

    #[derive(Clone)]
    struct TestNode {
        service: Arc<IOService<AppMsg>>,
        history: History,
        stop: Arc<AtomicBool>,
        enable_check: bool,
        name:String,
    }

    impl History {
        fn new() -> Self {
            Self {
                history: Arc::new(Mutex::new(vec![])),
            }
        }

        fn check_history(&self, history: Vec<ActionMessage<AppMsg>>) {
            let guard = self.history.lock().unwrap();
            if guard.len() != history.len() {
                error!("expected length {}, but length {}", history.len(), guard.len());
                error!("expected {:?}, but  {:?}", history, guard);
                panic!("length error");
            }
            for i in 0..history.len() {
                let action = &history[i];
                let action1 = &(guard[i]);
                if action.ne(action1) {
                    error!("action:{:?}, expected {:?}", action1, action)
                }
                assert!(action.eq(action1))
            }
            info!("Check history OK");
        }

        fn add_action_to_history(&self, action: ActionMessage<AppMsg>) {
            let mut history = self.history.lock().unwrap();
            trace!("add action to history {:?} total size:{}", action, history.len() + 1);
            history.push(action);
        }
    }

    impl TestNode {
        fn new(node_id: NID,  history: History, enable_check: bool) -> Res<TestNode> {
            let name = format!("node_{}", node_id);
            let opt = IOServiceOpt {
                num_message_receiver: 1,
                testing: true,
            };
            let service = IOService::<AppMsg>::new(node_id, name.clone(), opt, Notifier::new())?;
            Ok(Self {
                service: Arc::new(service),
                history,
                stop: Arc::new(AtomicBool::new(false)),
                enable_check,
                name,
            })
        }

        async fn stop(&self) {
            let r = self.stop.compare_exchange(
                false,
                true,
                Ordering::SeqCst,
                Ordering::SeqCst);
            if r.is_ok() {
                let r_stop = self.service.default_event_sink().stop(
                    ESStopOpt::default().enable_no_wait(true)).await;
                assert!(r_stop.is_ok());
            }
        }

        fn run_test_app(&self, opt_ls: Option<LocalSet>) {
            let ls = match opt_ls {
                Some(ls) => { ls }
                None => { LocalSet::new() }
            };
            {
                let s = self.clone();
                let node_id = self.service.node_id();
                ls.spawn_local(async move {
                    spawn_local_task(
                        Notifier::new(),
                        format!("app handle message {}", node_id).as_str(),
                        async move {
                            let r = s.app_run_handle_message().await;
                            match r {
                                Ok(()) => {}
                                Err(e) => { error!("{}", e.to_string()) }
                            }
                        }).unwrap();
                });
            }
            self.service.run_local(&ls);
            let r_build = Builder::new_current_thread()
                .enable_all()
                .build();
            let runtime = res_io(r_build).unwrap();
            let name = self.name.clone();
            runtime.block_on(
                async move {
                    ls.await;
                    info!("Stop run node {}",  name);
                }
            );
        }


        async fn app_run_handle_message(
            &self,
        ) -> Res<()> {
            let receiver = {
                let receivers = self.service.message_receiver();
                if receivers.len() != 1 {
                    panic!("must only 1 receiver");
                }
                receivers[0].clone()
            };
            self.app_message_loop(receiver, self.enable_check).await?;
            Ok(())
        }

        async fn app_message_loop(
            &self,
            receiver: Arc<dyn Receiver<AppMsg>>,
            enable_check: bool
        ) -> Res<()> {
            loop {
                let r = self.app_handle_message(receiver.clone(), enable_check).await;
                match r {
                    Ok(()) => {}
                    Err(e) => {
                        match &e {
                            ET::EOF => { break; }
                            _ => { return Err(e); }
                        }
                    }
                }
            }
            trace!("app handle message done");
            Ok(())
        }

        async fn process_app_message(
            &self,
            dtm_msg: Message<AppMsg>,
            enable_check: bool,
        ) -> Res<()> {
            trace!("NODE {} receive app message {:?}", self.service.node_id(), dtm_msg);
            let from = dtm_msg.source();
            let to = dtm_msg.dest();
            match dtm_msg.payload() {
                AppMsg::TaskNew { task_id, task_ops } => {
                    let this = self.clone();
                    spawn_local_task(Notifier::default(),
                                     format!("app task :{}", task_id).as_str(),
                                     async move {
                                let r = this.app_task_run(from, to, task_id, task_ops, enable_check).await;
                                match r {
                                    Ok(()) => {}
                                    Err(e) => { error!("app task run error, {}", e.to_string()) }
                                }
                            }
                        ).unwrap();
                }
                AppMsg::TaskStop => {
                    let this = self.clone();
                    let enable_check = self.enable_check;
                    spawn_local_task(
                        Notifier::new(),
                        format!("node {} stop task", from).as_str(),
                        async move {
                            let r = this.app_task_stop(from, to, enable_check).await;
                            match r {
                                Ok(()) => {}
                                Err(e) => { error!("app task run error, {}", e.to_string()) }
                            }
                        }).unwrap();
                }
                _ => {}
            }
            Ok(())
        }

        async fn app_task_run(
            &self,
            from: NID,
            to: NID,
            task_id: u64,
            task_ops: Vec<u64>,
            enable_check: bool,
        ) -> Res<()> {

            {
                let m = Message::new(
                    AppMsg::TaskNew { task_id, task_ops: task_ops.clone() },
                    from,
                    to,
                );

                if enable_check {
                    action_begin!(AUTOMATON_NAME, ActionType::Input, m.clone());
                    self.add_action_to_history(ActionMessage::Input(m.clone()));
                    action_end!(AUTOMATON_NAME, ActionType::Input, m.clone());
                } else {
                    input!(AUTOMATON_NAME, m.clone());
                }
            }

            self.handle_task_ops(task_id, task_ops, enable_check).await?;

            {
                let m = Message::new(
                    AppMsg::TaskEnd { task_id },
                    to,
                    from,
                );

                if enable_check {
                    action_begin!(AUTOMATON_NAME, ActionType::Output, m.clone());
                    self.add_action_to_history(ActionMessage::Output(m.clone()));
                    action_end!(AUTOMATON_NAME, ActionType::Output, m.clone());
                } else {
                    output!(AUTOMATON_NAME, m.clone());
                }
            }
            Ok(())
        }

        async fn app_task_stop(
            &self,
            from: NID,
            to: NID,
            enable_check: bool,
        ) -> Res<()> {
            trace!("app task stop {} ", to);
            let msg = Message::new(
                AppMsg::TaskStop,
                from,
                to,
            );

            if enable_check {
                action_begin!(AUTOMATON_NAME, ActionType::Input, msg.clone());
                self.add_action_to_history(ActionMessage::Input(msg.clone()));
                action_end!(AUTOMATON_NAME, ActionType::Input, msg.clone());
            } else {
                input!(AUTOMATON_NAME, msg.clone());
            }
            self.stop().await;
            trace!("app task stop {} end", to);
            Ok(())
        }

        fn add_action_to_history(&self, action: ActionMessage<AppMsg>) {
            self.history.add_action_to_history(action)
        }

        async fn app_handle_message(
            &self,
            receiver: Arc<dyn Receiver<AppMsg>>,
            enable_check: bool
        ) -> Res<()> {
            let dtm_msg = receiver.receive().await?;
            let s = self.clone();
            task::spawn_local(async move {
                let _ = s.process_app_message(dtm_msg, enable_check).await;
            });
            Ok(())
        }

        async fn handle_task_ops(
            &self,

            task_id: u64,
            op_ids: Vec<u64>,
            enable_check: bool,
        ) -> Res<()> {
            trace!("handle action begin {}", task_id);
            for op_id in op_ids {
                self.handle_task_op( task_id, op_id, enable_check)
                    .instrument(trace_span!("handle task op")).await?;
            }
            Ok(())
        }

        async fn handle_task_op(
            &self,

            id: u64,
            op_id: u64,
            enable_check: bool,
        ) -> Res<()> {
            let node_id = self.service.node_id();
            let m = Message::new(
                AppMsg::TaskOP { task_id: id, task_op: op_id },
                node_id,
                node_id,
            );
            trace!("internal begin, {:?}", m);
            let action_internal = ActionMessage::Internal(m.clone());

            if enable_check {
                action_begin!(AUTOMATON_NAME, ActionType::Internal, m.clone());
                trace!("do something {:?}", m);
                self.add_action_to_history(action_internal.clone());
                action_end!(AUTOMATON_NAME, ActionType::Internal, m.clone());
            } else {
                internal_begin!(AUTOMATON_NAME, m.clone());
                trace!("do something {:?}", m);
                internal_end!(AUTOMATON_NAME, m.clone());
            }
            trace!("{} {}", id, op_id);

            Ok(())
        }
    }

    fn run_node(
        node_id: NID,
        address: SocketAddr,
        history: History,
        enable_check: bool,
    ) -> Res<()> {
        let node = Arc::new(TestNode::new(node_id,  history, enable_check)?);
        let local = LocalSet::new();
        {
            let addr = address.clone();
            let n = node.clone();
            local.spawn_local(async move {
                let r = serve_node(n, addr).await;
                assert!(r.is_ok());
            });
        }
        node.run_test_app(Some(local));
        trace!("test node run done");
        Ok(())
    }

    async fn serve_node(
        node: Arc<TestNode>,
        address: SocketAddr,
    ) -> Res<()> {
        let sender = node.service.default_event_sink();
        sender.serve(address, ESServeOpt::default().enable_no_wait(false)).await?;
        trace!("serve node {}, listen on address {}", node.service.node_id(), address.to_string());
        Ok(())
    }

    fn run_simulator(
        node_id: NID,
        address: SocketAddr,
        node_address: HashMap<NID, SocketAddr>,
        num_tx: u64,
        num_ops: u64,
        stop: Arc<Notify>,
        history: History,
        enable_check: bool
    ) -> Res<()> {
        let r_build = Builder::new_current_thread()
            .enable_all()
            .build();
        let r = res_io(r_build)?;
        let runtime = Arc::new(r);
        let mut nodes = vec![];
        for (id, _) in node_address.iter() {
            nodes.push(id.clone());
        }
        let action_list = generate_all_test_message(nodes, num_tx, num_ops);
        let action_input = Arc::new(ActionInputStub::new(action_list.clone()));
        let stop_notify = Notifier::new();
        let name = format!("{}", node_id);

        // when testing,  we enable wait_both_begin_and_end_action to check the history if check is enable.
        let mut test_option = TestOption::new();
        test_option = test_option.set_seconds_wait_message_timeout(10);
        if enable_check {
            test_option = test_option
                .set_wait_both_begin_and_end_action(true)
                .set_sequential_output_action(true);
        } else {
            test_option = test_option
                .set_wait_both_begin_and_end_action(false)
                .set_sequential_output_action(false);
        }
        let dtm_server = Arc::new(DTMServer::new(node_id, name, stop_notify, test_option)?);
        let local = LocalSet::new();
        {
            let addr = address.clone();
            let node_addr = node_address.clone();
            let s = dtm_server.clone();
            local.spawn_local(async move {
                let r = serve_simulator(s, addr, node_addr, action_input).await;
                match r {
                    Ok(()) => {}
                    Err(e) => {
                        error!("error serve player {}", e.to_string());
                    }
                }
            });
        }

        {
            let s = dtm_server.clone();
            local.spawn_local(async move {
                let _ = stop.notified().await;
                if enable_check {
                    history.check_history(action_list);
                }
                let r = s.stop().await;
                match r {
                    Ok(()) => {}
                    Err(e) => { error!("stop player error {}", e.to_string()); }
                }
            });
        }
        dtm_server.run(Some(local), runtime);
        trace!("player run done");
        Ok(())
    }

    async fn serve_simulator(
        dtm_server: Arc<DTMServer>,
        address: SocketAddr,
        node_address: HashMap<NID, SocketAddr>,
        action_input: Arc<dyn ActionIncoming>,
    ) -> Res<()> {
        dtm_server.start_network(address, node_address).await?;
        let r = dtm_server.start_dtm_test(action_input).await?;
        let _ = r.await;
        Ok(())
    }

    fn generate_all_test_message(
        nodes: Vec<NID>,
        num_tx: u64,
        num_ops: u64,
    ) -> Vec<ActionMessage<AppMsg>> {
        let mut vec = vec![];
        let mut rnd = ThreadRng::default();
        let begin_task_id = 20220000;
        let num_node = nodes.len();
        for i in 0..num_tx {
            let id = i + begin_task_id;
            let (from, to) = {
                let index = rnd.gen_range(0..num_node - 1);
                if nodes.len() >= 1 {
                    (nodes[index], nodes[index])
                } else {
                    (nodes[index], nodes[(index + 1) % num_node])
                }
            };
            let v = generate_test_message(from, to, id, num_ops);
            vec.push(VecDeque::from(v));
        }

        let mut ret = vec![];

        let size = vec.len();
        loop {
            let index = rnd.gen_range(0..size - 1);
            let mut num_end = 0;
            for i in index..index + vec.len() {
                let opt_msg = vec[i % size].pop_front();
                match opt_msg {
                    Some(msg) => {
                        trace!("Action: {:?}", msg);
                        ret.push(msg);
                        break;
                    }
                    None => { num_end += 1; }
                }
            }
            if num_end == vec.len() {
                break;
            }
        }
        for node_id in nodes.iter() {
            ret.push(ActionMessage::Input(
                Message::new(
                    AppMsg::TaskStop,
                    node_id.clone(),
                    node_id.clone(),
                )));
        }
        info!("Generate test case, total action {}", ret.len());
        for (i, action) in ret.iter().enumerate() {
            trace!("Action: {} {:?}", i, action);
        }
        ret
    }

    fn generate_test_message(from: NID, to: NID, id: u64, num_ops: u64) -> Vec<ActionMessage<AppMsg>> {
        let mut vec = vec![];
        let mut op_ids = vec![];
        for i in 0..num_ops {
            let op_id = i + 1;
            op_ids.push(op_id);
        }

        vec.push(
            ActionMessage::Input(
                Message::new(
                    AppMsg::TaskNew { task_id: id, task_ops: op_ids.clone() },
                    from,
                    to,
                ))
        );
        for op_id in op_ids {
            vec.push(ActionMessage::Internal(
                Message::new(
                    AppMsg::TaskOP { task_id: id, task_op: op_id },
                    to,
                    to))
            );
        }

        vec.push(
            ActionMessage::Output(
                Message::new(
                    AppMsg::TaskEnd { task_id: id },
                    to,
                    to,
                )));
        vec
    }
}