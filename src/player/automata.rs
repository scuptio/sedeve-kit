use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use lazy_static::lazy_static;
use scc::HashIndex;
use scupt_net::notifier::Notifier;
use scupt_util::message::{Message, MsgTrait};
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use scupt_util::res_of::res_io;
use tokio::runtime::Builder;
use tracing::error;

use crate::action::action_message::ActionMessage;
use crate::action::action_type::ActionType;
use crate::player::async_action_driver::AsyncActionDriver;
use crate::player::dtm_client::DTMClient;

pub type AT = ActionType;

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
    action_driver_setup_gut(name, client_id, server_id, server_addr);
}

/// Is an automation named `name` enable
pub fn automaton_enable(name:&str) -> bool {
    __DRIVERS.contains(name)
}

/// A-synchronize begin an action
pub async fn automaton_async_begin_action<M: MsgTrait + 'static>(
    automaton_name: &str,
    action_type: ActionType,
    message: Message<M>) {
    async_action_gut(automaton_name, action_type, message, true).await
}

/// A-synchronize end an action
pub async fn automaton_async_end_action<M: MsgTrait + 'static>(
    automaton_name: &str,
    action_type: ActionType,
    message: Message<M>) {
    async_action_gut(automaton_name, action_type, message, false).await
}

/// Initialize an automata.
/// The automation name is `automaton_name`. And it connect to player whose node id(NID) is
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
            automata::automaton_init($automaton_name, $node_id, $player_id, $player_addr);
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
            automata::automaton_clear($automaton_name);
        }
    };
}

/// Begin an action
#[macro_export]
macro_rules! action_begin {
    ($automaton_name:expr, $action_type:expr, $message:expr) => {
        {
            automata::automaton_async_begin_action($automaton_name, $action_type, $message).await;
        }
    };
}

/// End an action
#[macro_export]
macro_rules! action_end {
    ($automaton_name:expr, $action_type:expr, $message:expr) => {
        {
            automata::automaton_async_end_action($automaton_name, $action_type, $message).await;
        }
    };
}

/// End a Setup action , alias of `setup_end`
#[macro_export]
macro_rules! setup {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_end_action($automaton_name, automata::AT::Setup, $message).await;
        }
    };
}

/// End a Check action , alias of `check_end`
#[macro_export]
macro_rules! check {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_end_action($automaton_name, automata::AT::Check, $message).await;
        }
    };
}

/// End an Input action , alias of `input_end`
#[macro_export]
macro_rules! input {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_end_action($automaton_name, automata::AT::Input, $message).await;
        }
    };
}

/// End an Output action , alias of `output_end`
#[macro_export]
macro_rules! output {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_begin_action($automaton_name, automata::AT::Output, $message).await;
        }
    };
}

/// Begin a Setup action
#[macro_export]
macro_rules! setup_begin {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_begin_action($automaton_name, automata::AT::Setup, $message).await;
        }
    };
}

/// End a Setup action
#[macro_export]
macro_rules! setup_end {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_end_action($automaton_name, automata::AT::Setup, $message).await;
        }
    };
}

/// Begin a Check action
#[macro_export]
macro_rules! check_begin {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_begin_action($automaton_name, automata::AT::Check, $message).await;
        }
    };
}


/// End a Check action
#[macro_export]
macro_rules! check_end {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_end_action($automaton_name, automata::AT::Check, $message).await;
        }
    };
}

/// Begin an Input action
#[macro_export]
macro_rules! input_begin {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_begin_action($automaton_name, automata::AT::Input, $message).await;
        }
    };
}

/// End an Input action
#[macro_export]
macro_rules! input_end {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_end_action($automaton_name, automata::AT::Input, $message).await;
        }
    };
}

/// Begin an Output action
#[macro_export]
macro_rules! output_begin {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_begin_action($automaton_name, automata::AT::Output, $message).await;
        }
    };
}

/// End an Output action
#[macro_export]
macro_rules! output_end {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_end_action($automaton_name, automata::AT::Output, $message).await;
        }
    };
}

/// Begin an Internal action
#[macro_export]
macro_rules! internal_begin {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_begin_action($automaton_name, automata::AT::Internal, $message).await;
        }
    };
}


/// End an Internal action
#[macro_export]
macro_rules! internal_end {
    ($automaton_name:expr,  $message:expr) => {
        {
            automata::automaton_async_end_action($automaton_name, automata::AT::Internal, $message).await;
        }
    };
}

/// Is an automation enable
#[macro_export]
macro_rules! auto_enable {
    ($automaton_name:expr) => {
        {
            automata::automaton_enable($automaton_name)
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
) {
    let addr: SocketAddr =
        server_addr.parse()
            .expect("Unable to resolve domain");
    if !__DRIVERS.contains(name) {
        let driver = __ActionDriver::new(client_id, server_id, addr).unwrap();
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
    message: Message<M>,
    begin: bool
) {
    let opt = __DRIVERS.get_async(&automaton_name.to_string()).await;
    let driver = match opt {
        Some(e) => { Arc::new(e.get().clone()) }
        None => {
            return;
        }
    };
    let driver = driver._input.clone();
    let action = ActionMessage::from_message(action_type, message);
    let action_s = action.to_serde_json_string().unwrap();
    let r = if begin {
        driver.begin_action(action_s).await
    } else {
        driver.end_action(action_s).await
    };
    match r {
        Ok(_) => {}
        Err(e) => {
            error!("send action {:?} , is begin: {}, error: {}.",
                action.to_serde_json_string().unwrap(), begin, e);
        }
    }
}

unsafe impl Sync for __ActionDriver {}

unsafe impl Send for __ActionDriver {}

#[derive(Clone)]
struct __ActionDriver {
    _thd: Arc<Mutex<Option<JoinHandle<()>>>>,
    _dtm_client: Arc<DTMClient>,
    _input: Arc<dyn AsyncActionDriver>,
    _notify: Notifier,
}


impl __ActionDriver {
    fn new(
        client_id:NID,
        server_id: NID, server_addr: SocketAddr,
    ) -> Res<Self> {
        let r_build = Builder::new_current_thread()
            .enable_all()
            .build();
        let r = res_io(r_build)?;
        let runtime = Arc::new(r);
        let node_name = format!("driver_{}->{}", client_id, server_id);
        let notifier = Notifier::new_with_name(node_name.clone());
        let cli = DTMClient::new(
            node_name, client_id, server_id,
            server_addr,
            notifier.clone())?;
        let thd = {
            let c = cli.clone();
            let f = move || {
                c.run(None, runtime);
            };
            let thd = thread::Builder::new()
                .name(format!("dtm-client").to_string())
                .spawn(f);
            thd.unwrap()
        };
        let async_driver = cli.new_async_driver()?;
        let s = Self {
            _thd: Arc::new(Mutex::new(Some(thd))),
            _dtm_client: Arc::new(cli),
            _input: async_driver,
            _notify: notifier,
        };
        Ok(s)
    }

    fn close(&self) {
        self._dtm_client.close();
    }
}

lazy_static! {
    static  ref __DRIVERS : HashIndex<String, __ActionDriver> = HashIndex::new();
}