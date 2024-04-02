use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use lazy_static::lazy_static;
use scc::HashIndex;
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
use tracing::error;
use crate::action::action_serde_json_value::ActionSerdeJsonValue;

use crate::action::action_type::{ActionBeginEnd, ActionType};
use crate::dtm::async_action_driver::AsyncActionDriver;
use crate::dtm::dtm_client::DTMClient;
use crate::dtm::sync_action_driver::SyncActionDriver;

/// Clean an automata setting
pub fn automaton_clear(name: &str,) {
    action_driver_unset_gut(name)
}


/// Initialize an automata setting
pub fn automaton_init(
    name: &str,
    client_id:NID,
    server_id: NID,
    server_addr: &str
) {
    action_driver_setup_gut(name, client_id, server_id, server_addr, false);
}

/// Is an automation named `name` enable
pub fn automaton_enable(name:&str) -> bool {
    __DRIVERS.contains(name)
}

/// A-synchronize begin an action
pub async fn automaton_action_async<M: MsgTrait + 'static>(
    automaton_name: &str,
    action_type: ActionType,
    action_begin_end: ActionBeginEnd,
    message: Message<M>) {
    async_action_gut(automaton_name, action_type, action_begin_end, message).await
}

pub fn automaton_next_input_action(
    _automaton_name: &str
) -> Res<(NID, NID, ActionType, String)> {
    let opt = __DRIVERS.get(&_automaton_name.to_string());
    let driver = match opt {
        Some(e) => { Arc::new(e.get().clone()) }
        None => {
            return Err(ET::NoneOption);
        }
    };

    let receiver = driver.sync_input_action().unwrap();

    let msg =  receiver.receive()?;
    let json = msg.payload().to_serde_json_value().into_serde_json_value();
    let action = ActionSerdeJsonValue::from_json_value(json)?;
    let source = action.source_node_id()?;
    let dest = action.dest_node_id()?;
    let action_type = action.action_type()?;
    let message = action.message_payload_json_value()?.to_string();
    return Ok((source, dest, action_type, message));
}

pub fn automaton_action_str(
    _automaton_name: &str,
    _action_type: ActionType,
    _action_begin_end: ActionBeginEnd,
    _source:NID,
    _dest:NID,
    _message: &str) {
    let opt = __DRIVERS.get(&_automaton_name.to_string());
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

/// Initialize an automata.
/// The automation name is `automaton_name`. And it connects to player whose node id(NID) is
/// `player_id`, and network address(with string representation) is `player_addr`
#[macro_export]
macro_rules! auto_init {
    (
        $automaton_name:expr,
        $node_id:expr,
        $player_id:expr,
        $player_addr:expr
    ) => {
        {
            $crate::dtm::automata::automaton_init($automaton_name, $node_id, $player_id, $player_addr);
        }
    };
}

/// Clear an automata setting
#[macro_export]
macro_rules! auto_clear {
    (
        $automaton_name:expr
    ) => {
        {
            $crate::dtm::automata::automaton_clear($automaton_name);
        }
    };
}

/// Begin an action
#[macro_export]
macro_rules! action_begin {
    ($automaton_name:expr, $action_type:expr, $message:expr) => {
        {
            $crate::dtm::automata::automaton_action_async(
                $automaton_name,
                $action_type,
                $crate::action::action_type::ActionBeginEnd::Begin,
                $message).await;
        }
    };
}

/// End an action
#[macro_export]
macro_rules! action_end {
    ($automaton_name:expr, $action_type:expr, $message:expr) => {
        {
            $crate::dtm::automata::automaton_action_async(
                $automaton_name,
                $action_type,
                $crate::action::action_type::ActionBeginEnd::End,
                $message).await;
        }
    };
}

/// End a Setup action , alias of `setup_end`
#[macro_export]
macro_rules! setup {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automaton_name,
                $crate::action::action_type::ActionType::Setup,
                $message
            )
        }
    };
}

/// End a Check action , alias of `check_end`
#[macro_export]
macro_rules! check {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::dtm::automata::automaton_action_async(
                $automaton_name,
                false,
                $crate::action::action_type::ActionType::Check,
                $message).await;
        }
    };
}

/// End an Input action , alias of `input_end`
#[macro_export]
macro_rules! input {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automaton_name,
                $crate::action::action_type::ActionType::Input,
                $message
            )
        }
    };
}

/// End an Output action , alias of `output_end`
#[macro_export]
macro_rules! output {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_begin!(
                $automaton_name,
                $crate::action::action_type::ActionType::Output,
                $message
            )
        }
    };
}

/// Begin a Setup action
#[macro_export]
macro_rules! setup_begin {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_begin!(
                $automaton_name,
                $crate::action::action_type::ActionType::Setup,
                $message
            )
        }
    };
}

/// End a Setup action
#[macro_export]
macro_rules! setup_end {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automaton_name,
                $crate::action::action_type::ActionType::Setup,
                $message
            )
        }
    };
}

/// Begin a Check action
#[macro_export]
macro_rules! check_begin {
    ($automaton_name:expr,  $message:expr) => {
        {
             $crate::action_begin!(
                $automaton_name,
                $crate::action::action_type::ActionType::Check,
                $message
            )
        }
    };
}


/// End a Check action
#[macro_export]
macro_rules! check_end {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automaton_name,
                $crate::action::action_type::ActionType::Check,
                $message
            )
        }
    };
}

/// Begin an Input action
#[macro_export]
macro_rules! input_begin {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_begin!(
                $automaton_name,
                $crate::action::action_type::ActionType::Input,
                $message
            )
        }
    };
}

/// End an Input action
#[macro_export]
macro_rules! input_end {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automaton_name,
                $crate::action::action_type::ActionType::Input,
                $message
            )
        }
    };
}

/// Begin an Output action
#[macro_export]
macro_rules! output_begin {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_begin!(
                $automaton_name,
                $crate::action::action_type::ActionType::Output,
                $message
            )
        }
    };
}

/// End an Output action
#[macro_export]
macro_rules! output_end {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automaton_name,
                $crate::action::action_type::ActionType::Output,
                $message
            )
        }
    };
}

/// Begin an Internal action
#[macro_export]
macro_rules! internal_begin {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_begin!(
                $automaton_name,
                $crate::action::action_type::ActionType::Internal,
                $message
            )
        }
    };
}


/// End an Internal action
#[macro_export]
macro_rules! internal_end {
    ($automaton_name:expr,  $message:expr) => {
        {
            $crate::action_end!(
                $automaton_name,
                $crate::action::action_type::ActionType::Internal,
                $message
            )
        }
    };
}

/// Is an automation enable
#[macro_export]
macro_rules! auto_enable {
    ($automaton_name:expr) => {
        {
            $crate::dtm::automata::automaton_enable($automaton_name)
        }
    };
}



fn action_driver_unset_gut(name: &str) {
    let opt = __DRIVERS.get(&name.to_string());
    match opt {
        Some(t) => {
            t.get().close();
        }
        None => {

        }
    }
    let _ =  __DRIVERS.remove(&name.to_string());
}

fn action_driver_setup_gut(
    name: &str,
    client_id:NID,
    server_id: NID,
    server_addr: &str,
    enable_input_channel:bool,
) {
    let addr: SocketAddr =
        server_addr.parse()
            .expect("Unable to resolve domain");
    if !__DRIVERS.contains(name) {
        let driver = __ActionDriver::new(client_id, server_id, addr, enable_input_channel).unwrap();
        let opt_d = __DRIVERS.insert(name.to_string(), driver);
        match opt_d {
            Ok(_) => {}
            Err((k, _)) => {
                panic!("existing key {}", k);
            }
        };
    }
}

async fn async_action_gut<M: MsgTrait + 'static>(
    automaton_name: &str,
    action_type: ActionType,
    action_begin_end: ActionBeginEnd,
    message: Message<M>
) {
    let opt = __DRIVERS.get_async(&automaton_name.to_string()).await;
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
    _driver_sync:Arc<dyn SyncActionDriver>,
    _server:Arc<Mutex<Option<Arc<dyn IOServiceSync<SerdeJsonString>>>>>
}


///
/// `enable_input_channel`
/// receive input action from incoming channel
/// When the a node build with scupt-net library, the library can automatically translate the
/// SerdeJsonString action message to the expected message, and we need no extra receiver channel of
/// input action, this value must set by `false`
impl __ActionDriver {
    fn new(
        client_id:NID,
        server_id: NID,
        server_addr: SocketAddr,
        enable_input_channel:bool,
    ) -> Res<Self> {
        let r_build = Builder::new_current_thread()
            .enable_all()
            .build();
        let r = res_io(r_build)?;
        let runtime = Arc::new(r);
        let node_name = format!("driver_{}->{}", client_id, server_id);
        let notifier = Notifier::new_with_name(node_name.clone());
        let cli = DTMClient::new(
            node_name.clone(), client_id, server_id,
            server_addr,
            notifier.clone())?;
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
        let opt_server = if !enable_input_channel {
            let opt = IOServiceOpt {
                num_message_receiver: 1,
                testing: false,
                sync_service: true,
                port_debug: None,
            };
            let io_service = IOService::new_sync_service(
                client_id, node_name.clone(), opt, notifier.clone())?;
            {
                let r = runtime.clone();
                let service = io_service.clone();
                let f = move || {
                    service.block_run(None, r);
                };
                let r_thd = thread::Builder::new()
                    .name("dtm-incoming-action".to_string())
                    .spawn(f);
                let thd = r_thd.unwrap();
                vec_thd.push(thd);
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
            _server: Arc::new(Mutex::new(opt_server)),
        };
        Ok(s)
    }

    fn close(&self) {
        self._dtm_client.close();
    }

    fn sync_input_action(&self) -> Option<Arc<dyn ReceiverSync<SerdeJsonString>>> {
        let opt_server = self._server.lock().unwrap();
        return match &*opt_server {
            Some(s) => {
                let receivers = s.receiver();
                if receivers.len() == 1 {
                    Some(receivers[0].clone())
                } else {
                    None
                }
            }
            None => { None }
        }
    }
}

lazy_static! {
    static  ref __DRIVERS : HashIndex<String, __ActionDriver> = HashIndex::new();
}