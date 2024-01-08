use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use scupt_net::event_sink::{ESConnectOpt, ESServeOpt, ESStopOpt};
use scupt_net::handle_event::HandleEventDummy;
use scupt_net::node::Node;
use scupt_net::notifier::Notifier;
use scupt_net::task::spawn_local_task;
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use tokio::task::LocalSet;
use tokio::time::sleep;
use tracing::{error, trace};

use crate::action::action_serde_json_string::ActionSerdeJsonString;
use crate::player::action_incoming::ActionIncoming;
use crate::player::dtm_player::TestOption;
use crate::player::dtm_server_handler::DTMServerHandler;
use crate::player::msg_ctrl::MessageControl;

type DTMNode = Node<
    MessageControl,
    DTMServerHandler,
>;

type ClientNode = Node<
    ActionSerdeJsonString,
    HandleEventDummy,
>;

pub struct DTMServer {
    handler: Arc<DTMServerHandler>,
    player_node: Arc<DTMNode>,
    client_node: Arc<ClientNode>,
}


/// Deterministic testing player server
impl DTMServer {
    pub fn new(node_id: NID,
               name: String,
               stop_notify: Notifier,
               option:TestOption,
    ) -> Res<Self> {
        let client_ch_name = format!("{}_client_channel", name);
        let client_node = ClientNode::new(
            node_id,
            client_ch_name,
            HandleEventDummy::default(),
            false,
            stop_notify.clone())?;
        let node_sender = client_node.default_message_sender();
        let h = DTMServerHandler::new(node_id, node_sender, stop_notify.clone(), option);
        let player_node: DTMNode = DTMNode::new(
            node_id,
            name,
            h.clone(),
            false,
            stop_notify.clone())?;
        Ok(Self {
            handler: Arc::new(h),
            player_node: Arc::new(player_node),
            client_node: Arc::new(client_node),
        })
    }

    pub fn run(&self, opt: Option<LocalSet>, runtime: Arc<Runtime>) {
        let ls = match opt {
            Some(l) => { l }
            None => { LocalSet::new() }
        };
        self.client_node.run_local(&ls);
        self.run_server_local(&ls);
        self.player_node.block_run(Some(ls), runtime);
    }

    pub async fn stop(&self) -> Res<()> {
        self.player_node.default_event_sink().stop(ESStopOpt::default().enable_no_wait(true)).await?;
        Ok(())
    }

    pub fn run_server_local(&self, ls: &LocalSet) {
        self.run_handle_local(ls);
        self.player_node.run_local(ls);
    }

    pub async fn start_connect_to_node(
        &self,
        node_address: HashMap<NID, SocketAddr>,
    ) -> Res<()> {
        let mut connected = HashSet::new();
        let mut n = 0;
        loop {
            for (id, addr) in node_address.iter() {
                let r = self.connect_to(id.clone(), addr.clone()).await;
                match r {
                    Ok(_) => {
                        connected.insert(id.clone());
                    }
                    Err(_e) => {
                        n += 1;
                        if n % 10 == 0 {
                            error!("dtm connection node {} {} error, {}", id, addr.to_string(), _e.to_string())
                        }
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }
            if node_address.len() == connected.len() {
                break;
            }
        }
        trace!("serve player, connect to all");
        Ok(())
    }


    pub async fn start_network(
        &self,
        simulate_bind_address: SocketAddr,
        node_address: HashMap<NID, SocketAddr>,
    ) -> Res<()> {
        self.serve_player(simulate_bind_address).await?;

        trace!("serve dtm player, listen on address {}", simulate_bind_address.to_string());

        self.start_connect_to_node(node_address).await?;
        Ok(())
    }

    pub async fn start_dtm_test(
        &self,
        input: Arc<dyn ActionIncoming>) -> Res<oneshot::Receiver<Res<()>>> {
        self.handler.begin_run_test(input).await
    }

    fn run_handle_local(&self, local_set: &LocalSet) {
        let h = self.handler.clone();

        let future = async move {
            let r = h.async_run().await;
            match r {
                Ok(()) => {}
                Err(_) => {}
            };
        };
        let notify = self.player_node.stop_notify();

        local_set.spawn_local(async move {
            spawn_local_task(notify, "dtm handler", future).unwrap();
        });
    }

    async fn serve_player(&self, address: SocketAddr) -> Res<()> {
        self.player_node.default_event_sink().serve(
            address,
            ESServeOpt::default().enable_no_wait(false)).await
    }

    async fn connect_to(&self, node_id: NID, address: SocketAddr) -> Res<()> {
        let _ = self.client_node.default_event_sink().connect(
            node_id, address,
            ESConnectOpt::default()
                .enable_no_wait(false)
                .enable_return_endpoint(false),
        ).await?;
        Ok(())
    }
}