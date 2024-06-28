use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use lazy_static::lazy_static;
use scc::HashIndex;
use scupt_net::es_option::ESServeOpt;
use scupt_net::io_service::{IOService, IOServiceOpt};
use scupt_net::io_service_sync::IOServiceSync;
use scupt_net::message_receiver_sync::ReceiverSync;
use scupt_net::notifier::Notifier;
use scupt_util::error_type::ET;
use scupt_util::message::{Message, MsgTrait};
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use scupt_util::res_of::res_io;
use scupt_util::serde_json_string::SerdeJsonString;
use tokio::runtime::Builder;
use tracing::{debug, error};

use crate::action::action_type::{ActionBeginEnd, ActionType};
use crate::action::message_json::MessageJson;
use crate::dtm::async_action_driver::AsyncActionDriver;
use crate::dtm::dtm_client::DTMClient;
use crate::dtm::sync_action_driver::SyncActionDriver;

/// Initialize an automata setting
pub fn automata_setup(
    name: &str,
    tested_nid: NID,
    player_nid: NID,
    player_addr: &str,
) {
    action_driver_setup_gut(name, tested_nid, None, player_nid, player_addr);
}

pub fn automata_setup_with_input(
    name: &str,
    tested_nid: NID,
    tested_addr:&str,
    player_nid: NID,
    player_addr: &str,
) {
    debug!("setup with input id {} {}", tested_nid, tested_addr);
    action_driver_setup_gut(name, tested_nid, Some(tested_addr), player_nid, player_addr);
}

/// Clean an automata setting
pub fn automata_clear(name: &str) {
    action_driver_unset_gut(name)
}

/// Close input
pub fn automata_close_input(name: &str) {
    action_driver_close_input(name)
}


/// Is an automation named `name` enable
pub fn automata_enable(name: &str) -> bool {
    __DRIVERS.contains(name)
}


/// Automata read next input action
pub fn automata_next_input(
    _automata_name: &str
) -> Res<(NID, NID, String)> {
    debug!("input {}", _automata_name);
    let opt = __DRIVERS.get(&_automata_name.to_string());
    let driver = match opt {
        Some(e) => { Arc::new(e.get().clone()) }
        None => {
            debug!("cannot get automata {} node", _automata_name);
            return Err(ET::NoneOption);
        }
    };

    let receiver = driver.sync_input_action().unwrap();

    let msg = receiver.receive()?;

    let _m = msg.clone();

    let json = msg.payload().to_serde_json_value().into_serde_json_value();
    let mj = MessageJson::new(&json);
    let r_source = mj.source_nid();
    let source = match r_source {
        Ok(a) => a,
        Err(_e) => { panic!("{:?}, json error {}, no source node id", mj, _e) }
    };
    let r_dest = mj.dest_nid();
    let dest = match r_dest {
        Ok(a) => a,
        Err(_e) => { panic!("{:?}, json error {}, no dest node id", mj, _e) }
    };


    let r_message = mj.payload();
    let message = match r_message {
        Ok(a) => a.to_string(),
        Err(_e) => { panic!("{:?}, json error {}, no message payload", mj, _e) }
    };


    debug!("input {:?}", _m);

    return Ok((source, dest, message));
}

pub fn automata_send_action_to_player(
    _automata_name: &str,
    _action_type: ActionType,
    _action_begin_end: ActionBeginEnd,
    _source: NID,
    _dest: NID,
    _message: &str) {
    let opt = __DRIVERS.get(&_automata_name.to_string());
    let driver = match opt {
        Some(e) => { Arc::new(e.get().clone()) }
        None => {
            return;
        }
    };
    let driver = driver._driver_sync.clone();

    let r = driver.action(_action_type, _action_begin_end, _source, _dest, _message.to_string());
    match r {
        Ok(_) => {}
        Err(e) => {
            error!("send action, {:?} , begin or end: {:?}, error: {}.", _message, _action_begin_end, e);
        }
    }
}


/// A-synchronize begin an action
pub async fn automata_action_async<M: MsgTrait + 'static>(
    automata_name: &str,
    action_type: ActionType,
    action_begin_end: ActionBeginEnd,
    message: Message<M>) {
    async_action_gut(automata_name, action_type, action_begin_end, message).await
}

/// Initialize an automata.
/// The automation name is `automata_name`. And it connects to player whose node id(NID) is
/// `player_id`, and network address(with string representation) is `player_addr`
#[macro_export]
macro_rules! auto_init {
    (
        $automata_name:expr,
        $node_id:expr,
        $player_id:expr,
        $player_addr:expr
    ) => {
        {
            $crate::dtm::automata::automata_setup($automata_name, $node_id, $player_id, $player_addr);
        }
    };
}

/// Clear an automata setting
#[macro_export]
macro_rules! auto_clear {
    (
        $automata_name:expr
    ) => {
        {
            $crate::dtm::automata::automata_clear($automata_name);
        }
    };
}

/// Begin an action
#[macro_export]
macro_rules! action_begin {
    ($automata_name:expr, $action_type:expr, $message:expr) => {
        {
            $crate::dtm::automata::automata_action_async(
                $automata_name,
                $action_type,
                $crate::action::action_type::ActionBeginEnd::Begin,
                $message).await;
        }
    };
}

/// End an action
#[macro_export]
macro_rules! action_end {
    ($automata_name:expr, $action_type:expr, $message:expr) => {
        {
            $crate::dtm::automata::automata_action_async(
                $automata_name,
                $action_type,
                $crate::action::action_type::ActionBeginEnd::End,
                $message).await;
        }
    };
}

/// End an Input action , alias of `input_end`
#[macro_export]
macro_rules! input {
    ($automata_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automata_name,
                $crate::action::action_type::ActionType::Input,
                $message
            )
        }
    };
}

/// End an Output action , alias of `output_end`
#[macro_export]
macro_rules! output {
    ($automata_name:expr,  $message:expr) => {
        {
            $crate::action_begin!(
                $automata_name,
                $crate::action::action_type::ActionType::Output,
                $message
            )
        }
    };
}

/// Begin a Setup action
#[macro_export]
macro_rules! setup_begin {
    ($automata_name:expr,  $message:expr) => {
        {
            $crate::action_begin!(
                $automata_name,
                $crate::action::action_type::ActionType::Setup,
                $message
            )
        }
    };
}

/// End a Setup action
#[macro_export]
macro_rules! setup_end {
    ($automata_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automata_name,
                $crate::action::action_type::ActionType::Setup,
                $message
            )
        }
    };
}

/// Begin an Input action
#[macro_export]
macro_rules! input_begin {
    ($automata_name:expr,  $message:expr) => {
        {
            $crate::action_begin!(
                $automata_name,
                $crate::action::action_type::ActionType::Input,
                $message
            )
        }
    };
}

/// End an Input action
#[macro_export]
macro_rules! input_end {
    ($automata_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automata_name,
                $crate::action::action_type::ActionType::Input,
                $message
            )
        }
    };
}

/// Begin an Output action
#[macro_export]
macro_rules! output_begin {
    ($automata_name:expr,  $message:expr) => {
        {
            $crate::action_begin!(
                $automata_name,
                $crate::action::action_type::ActionType::Output,
                $message
            )
        }
    };
}

/// End an Output action
#[macro_export]
macro_rules! output_end {
    ($automata_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automata_name,
                $crate::action::action_type::ActionType::Output,
                $message
            )
        }
    };
}

/// Begin an Internal action
#[macro_export]
macro_rules! internal_begin {
    ($automata_name:expr,  $message:expr) => {
        {
            $crate::action_begin!(
                $automata_name,
                $crate::action::action_type::ActionType::Internal,
                $message
            )
        }
    };
}


/// End an Internal action
#[macro_export]
macro_rules! internal_end {
    ($automata_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automata_name,
                $crate::action::action_type::ActionType::Internal,
                $message
            )
        }
    };
}

/// Is an automation enable
#[macro_export]
macro_rules! auto_enable {
    ($automata_name:expr) => {
        {
            $crate::dtm::automata::automata_enable($automata_name)
        }
    };
}



fn action_driver_unset_gut(name: &str) {
    let opt = __DRIVERS.get(&name.to_string());
    match opt {
        Some(t) => {
            t.get().close();
        }
        None => {}
    }
    let _ = __DRIVERS.remove(&name.to_string());
}


fn action_driver_close_input(name: &str) {
    let opt = __DRIVERS.get(&name.to_string());
    match opt {
        Some(t) => {
            t.get().close_incoming();
        }
        None => {}
    }
}

fn action_driver_setup_gut(
    name: &str,
    tested_nid: NID,
    tested_addr:Option<&str>,
    player_nid: NID,
    player_addr: &str
) {
    let addr: SocketAddr =
        player_addr.parse()
            .expect("Unable to resolve domain");

    let tested_addr = tested_addr.map(|s|{
        s.parse()
            .expect("Unable to resolve domain")
    });
    let _tested_addr = tested_addr.clone();
    if !__DRIVERS.contains(name) {
        let driver = __ActionDriver::new(tested_nid, tested_addr, player_nid, addr).unwrap();
        let opt_d = __DRIVERS.insert(name.to_string(), driver);
        debug!("create driver tested node:{} addr {:?}, player node:{} addr:{}", tested_nid, _tested_addr, player_nid, addr);
        match opt_d {
            Ok(_) => {}
            Err((k, _)) => {
                panic!("existing key {}", k);
            }
        };
    }
}

async fn async_action_gut<M: MsgTrait + 'static>(
    automata_name: &str,
    action_type: ActionType,
    action_begin_end: ActionBeginEnd,
    message: Message<M>,
) {
    let opt = __DRIVERS.get_async(&automata_name.to_string()).await;
    let driver = match opt {
        Some(e) => { Arc::new(e.get().clone()) }
        None => {
            return;
        }
    };
    let driver = driver._driver_async.clone();
    let s = serde_json::to_string(message.payload_ref()).unwrap();
    let r = driver.action(action_type, action_begin_end, message.source(), message.dest(), s).await;
    match r {
        Ok(_) => {}
        Err(e) => {
            error!("send action, {:?} , begin or end: {:?}, error: {}.", message, action_begin_end, e);
        }
    }
}

unsafe impl Sync for __ActionDriver {}

unsafe impl Send for __ActionDriver {}

#[derive(Clone)]
struct __ActionDriver {
    _thd: Arc<Mutex<Vec<JoinHandle<()>>>>,
    _dtm_client: Arc<DTMClient>,
    _driver_async: Arc<dyn AsyncActionDriver>,
    _driver_sync: Arc<dyn SyncActionDriver>,
    _client_stop: Notifier,
    _server_stop: Notifier,
    _server: Arc<Mutex<Option<Arc<dyn IOServiceSync<SerdeJsonString>>>>>,
}


///
/// `enable_input_channel`
/// receive input action from incoming channel
/// When the node build with scupt-net library, the library can automatically translate the
/// SerdeJsonString action message to the expected message, and we need no extra receiver channel of
/// input action, this value must set by `false`
impl __ActionDriver {
    fn new(
        tested_nid: NID,
        opt_tested_addr:Option<SocketAddr>,
        player_nid: NID,
        player_addr: SocketAddr
    ) -> Res<Self> {
        let r_build = Builder::new_current_thread()
            .enable_all()
            .build();
        let r = res_io(r_build)?;
        let runtime = Arc::new(r);
        let node_name = format!("driver_{}->{}", tested_nid, player_nid);
        let client_stop_notifier = Notifier::new_with_name(node_name.clone() + "_client");
        let cli = DTMClient::new(
            node_name.clone(), tested_nid, player_nid,
            player_addr,
            client_stop_notifier.clone())?;


        let mut vec_thd = vec![];
        {
            let r = runtime.clone();
            let c = cli.clone();
            let f = move || {
                c.run(None, r);
            };
            let r_thd = thread::Builder::new()
                .name("dtm-client".to_string())
                .spawn(f);
            let thd = r_thd.unwrap();
            vec_thd.push(thd);
        };
        let async_driver = cli.new_async_driver()?;
        let sync_driver = cli.new_sync_driver()?;

        let server_stop_notifier = Notifier::new_with_name(node_name.clone() + "_server");
        let opt_server = if let Some(addr) = opt_tested_addr  {
            let opt = IOServiceOpt {
                num_message_receiver: 1,
                testing: false,
                sync_service: true,
                port_debug: None,
            };
            let io_service = IOService::new_sync_service(
                tested_nid, node_name.clone(), opt, server_stop_notifier.clone())?;
            {
                let r = runtime.clone();
                let service = io_service.clone();
                let sink = service.default_sink();
                let f = move || {
                    service.block_run(None, r);
                };


                let r_thd = thread::Builder::new()
                    .name("dtm-incoming-action".to_string())
                    .spawn(f);
                let thd = r_thd.unwrap();
                vec_thd.push(thd);

                sink.serve(addr, ESServeOpt::default().enable_no_wait(false))?;
            }
            Some(io_service)
        } else {
            None
        };
        let s = Self {
            _thd: Arc::new(Mutex::new(vec_thd)),
            _dtm_client: Arc::new(cli),
            _driver_async: async_driver,
            _driver_sync: sync_driver,
            _client_stop: client_stop_notifier,
            _server_stop: server_stop_notifier,
            _server: Arc::new(Mutex::new(opt_server)),
        };
        Ok(s)
    }

    fn close(&self) {
        self._client_stop.notify_all();
        self._server_stop.notify_all();
        self._dtm_client.close();
    }

    fn close_incoming(&self) {
        self._server_stop.notify_all();
    }

    fn sync_input_action(&self) -> Option<Arc<dyn ReceiverSync<SerdeJsonString>>> {
        let opt_server = self._server.lock().unwrap();
        return match &*opt_server {
            Some(s) => {
                let receivers = s.receiver();
                if receivers.len() == 1 {
                    Some(receivers[0].clone())
                } else {
                    debug!("sync receiver len {}", receivers.len());
                    None
                }
            }
            None => {
                debug!("sync server None");
                None
            }
        };
    }
}

lazy_static! {
    static  ref __DRIVERS : HashIndex<String, __ActionDriver> = HashIndex::new();
}