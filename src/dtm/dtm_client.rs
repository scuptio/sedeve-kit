use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::time::Duration;

use scc::HashMap;
use scupt_net::endpoint_async::EndpointAsync;
use scupt_net::es_option::ESConnectOpt;
use scupt_net::node::Node;
use scupt_net::notifier::Notifier;
use scupt_net::task::spawn_local_task;
use scupt_util::error_type::ET;
use scupt_util::message::Message;
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use tokio::runtime::Runtime;
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel};
use tokio::sync::oneshot::Sender as AsyncOneshotSender;
use tokio::task::LocalSet;
use tokio::time::sleep;
use tokio::sync::mpsc::UnboundedSender as AsyncSender;
use tokio::sync::mpsc::UnboundedReceiver as AsyncReceiver;
use std::sync::mpsc::Sender as SyncSender;
use scupt_net::handle_event::HandleEventDummy;
use scupt_net::io_service::{IOService, IOServiceOpt};
use scupt_net::io_service_sync::IOServiceSync;
use scupt_util::serde_json_string::SerdeJsonString;

use crate::dtm::async_action_driver::AsyncActionDriver;
use crate::dtm::async_action_driver_impl::AsyncActionDriverImpl;
use crate::dtm::msg_ctrl::MessageControl;
use crate::dtm::sync_action_driver::SyncActionDriver;
use crate::dtm::sync_action_driver_impl::SyncActionDriverImpl;

type PlayerNodeClient = Node<
    MessageControl,
    HandleEventDummy
>;

type TestedNodeServer = IOService<
    SerdeJsonString
>;

struct _ClientContext {
    node_id:NID,
    dtm_server_node_id: NID,
    dtm_server_addr: SocketAddr,
    player_node_client: PlayerNodeClient,
    tested_node_server:Arc<dyn IOServiceSync<SerdeJsonString>>,
    // sender/receiver would redirect send message to message loop task
    async_sender: AsyncSender<(Message<MessageControl>, AsyncOneshotSender<Message<MessageControl>>)>,
    async_receiver: StdMutex<Option<AsyncReceiver<(Message<MessageControl>, AsyncOneshotSender<Message<MessageControl>>)>>>,
    sync_sender: AsyncSender<(Message<MessageControl>, SyncSender<Message<MessageControl>>)>,
    sync_receiver: StdMutex<Option<AsyncReceiver<(Message<MessageControl>, SyncSender<Message<MessageControl>>)>>>,
}

#[derive(Clone)]
pub struct DTMClient {
    context: Arc<_ClientContext>,
}

impl _ClientContext {

}

impl DTMClient {
    pub fn new(
        task_name: String,
        client_id:NID,
        server_id: NID,
        server_addr: SocketAddr,
        stop_notify: Notifier,
    ) -> Res<Self> {
        let handle = HandleEventDummy::default();
        let player_node_client = PlayerNodeClient::new(
            client_id,
            task_name.clone(),
            handle.clone(),
            false,
            stop_notify.clone(),
        )?;
        let opt = IOServiceOpt {
            num_message_receiver: 1,
            testing: false,
            sync_service: true,
            port_debug: None,
        };
        let tested_node_server = IOService::new_sync_service(
            client_id,
            task_name.clone(),
            opt,
            stop_notify.clone()
        )?;

        let (async_sender, async_receiver) = unbounded_channel();
        let (sync_sender, sync_receiver) = unbounded_channel();

        Ok(Self {
            context: Arc::new(_ClientContext {
                node_id: client_id,
                dtm_server_node_id: server_id,
                dtm_server_addr: server_addr,
                player_node_client,
                tested_node_server,
                async_sender,
                async_receiver:StdMutex::new(Some(async_receiver)),
                sync_sender,
                sync_receiver: StdMutex::new(Some(sync_receiver)),
            }),
        })
    }


    fn async_sender(&self) -> Res<AsyncSender<(Message<MessageControl>, AsyncOneshotSender<Message<MessageControl>>)>> {
        Ok(self.context.async_sender.clone())
    }

    fn sync_sender(&self) -> Res<AsyncSender<(Message<MessageControl>, SyncSender<Message<MessageControl>>)>> {
        Ok(self.context.sync_sender.clone())
    }

    pub fn new_async_driver(&self) -> Res<Arc<dyn AsyncActionDriver>> {
        let driver = Arc::new(AsyncActionDriverImpl::new(
            self.context.node_id,
            self.context.dtm_server_node_id,
            self.async_sender()?));
        Ok(driver)
    }

    pub fn new_sync_driver(&self) -> Res<Arc<dyn SyncActionDriver>> {
        let driver = Arc::new(SyncActionDriverImpl::new(
            self.context.node_id,
            self.context.dtm_server_node_id,
            self.sync_sender()?));
        Ok(driver)
    }


    pub fn run(&self, opt_ls: Option<LocalSet>, runtime: Arc<Runtime>) {
        let local_set = match opt_ls {
            Some(ls) => { ls }
            None => { LocalSet::new() }
        };
        let c = self.context.clone();
        let f = async move {
            let n = c.player_node_client.stop_notify().clone();
            let j = spawn_local_task(
                n,
                "dtm client message loop",
                async move {
                    c.dtm_client_message_loop().await?;
                    Ok::<(), ET>(())
                });
            let _ = j.unwrap().await;
        };
        local_set.spawn_local(f);
        self.context.player_node_client.block_run(Some(local_set), runtime);
    }

    pub fn close(&self) {
        self.context.close();
    }
}

impl _ClientContext {
    async fn connect_to_dtm_player(&self) -> Res<Arc<dyn EndpointAsync<MessageControl>>> {
        loop {
            let r_connect = self.player_node_client.default_event_sink().connect(
                self.dtm_server_node_id,
                self.dtm_server_addr,
                ESConnectOpt::default().enable_return_endpoint(true),
            ).await;
            match r_connect {
                Ok(opt_ep) => {
                    return Ok(opt_ep.unwrap());
                }
                Err(_e) => {
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    async fn dtm_client_message_loop(&self) -> Res<()> {
        let mut receiver1 = {
            let mut opt_r = None;
            let mut g = self.async_receiver.lock().unwrap();
            std::mem::swap(&mut (*g), &mut opt_r);
            opt_r.unwrap()
        };

        let mut receiver2 = {
            let mut opt_r = None;
            let mut g = self.sync_receiver.lock().unwrap();
            std::mem::swap(&mut (*g), &mut opt_r);
            opt_r.unwrap()
        };
        let endpoint1 = self.connect_to_dtm_player().await?;
        let endpoint2 = self.connect_to_dtm_player().await?;
        let resp_senders1 = HashMap::new();
        let resp_senders2 = HashMap::new();

        loop {
            self.handle_message(
                & *endpoint1, &mut receiver1, &resp_senders1,
                & *endpoint2, &mut receiver2, &resp_senders2).await?;
        }
    }


    async fn handle_message(
        &self,
        endpoint1:&dyn EndpointAsync<MessageControl>,
        incoming1:&mut AsyncReceiver<(Message<MessageControl>, AsyncOneshotSender<Message<MessageControl>>)>,
        resp_senders1: &HashMap<String, AsyncOneshotSender<Message<MessageControl>>>,
        endpoint2:&dyn EndpointAsync<MessageControl>,
        incoming2:&mut AsyncReceiver<(Message<MessageControl>, SyncSender<Message<MessageControl>>)>,
        resp_senders2 : &HashMap<String, SyncSender<Message<MessageControl>>>,
    ) -> Res<()> {
        select! {
            r1 = self.handle_recv_response_async(endpoint1, resp_senders1) => {
                r1
            },
            r2 = self.handle_incoming_request(endpoint1, incoming1, resp_senders1) => {
                r2
            }
            r3 = self.handle_recv_response_sync(endpoint2, resp_senders2) => {
                r3
            },
            r4 = self.handle_incoming_request(endpoint2, incoming2, resp_senders2) => {
                r4
            }
        }
    }

    async fn handle_recv_response<S>(
        &self,
        endpoint:&dyn EndpointAsync<MessageControl>,
        resp_senders : &HashMap<String, S>,
    ) -> Res<(NID, NID, MessageControl, S)> {
        let r_m = endpoint.recv().await;
        let (from, to, m) = match r_m {
            Ok(m) => {
                (m.source(), m.dest(), m.payload())
            }
            Err(e) => { return Err(e); }
        };

        let id = m.id().unwrap();
        let opt = resp_senders.remove(&id);
        let sender = match opt {
            Some((_id, s)) => {
                s
            }
            None => { panic!("error, no such message, id:{}", id); }
        };

        Ok((from, to, m, sender))
    }

    async fn handle_recv_response_async(
        &self,
        endpoint:&dyn EndpointAsync<MessageControl>,
        resp_senders : &HashMap<String, AsyncOneshotSender<Message<MessageControl>>>,
    ) -> Res<()> {
        let (from, to, m, sender) =
            self.handle_recv_response(endpoint, resp_senders).await?;
        let mm = Message::new(m, to, from);
        let _ = sender.send(mm);
        Ok(())
    }


    async fn handle_recv_response_sync(
        &self,
        endpoint:&dyn EndpointAsync<MessageControl>,
        resp_senders : &HashMap<String, SyncSender<Message<MessageControl>>>,
    ) -> Res<()> {
        let (from, to, m, sender) =
            self.handle_recv_response(endpoint, resp_senders).await?;
        let mm = Message::new(m, to, from);
        let _ = sender.send(mm);
        Ok(())
    }

    async fn handle_incoming_request<S:Debug>(
        &self,
        endpoint:&dyn EndpointAsync<MessageControl>,
        incoming:&mut AsyncReceiver<(Message<MessageControl>, S)>,
        resp_senders : &HashMap<String, S>,
    ) -> Res<()> {
        let opt_in = incoming.recv().await;
        match opt_in {
            Some((m, s)) => {
                let id = m.payload_ref().id().unwrap();
                let _ = resp_senders.insert(id, s).unwrap();
                endpoint.send(m).await?;
            }
            None => {}
        }
        Ok(())
    }

    pub fn close(&self) {
        self.player_node_client.stop_notify().notify_all();
    }
}