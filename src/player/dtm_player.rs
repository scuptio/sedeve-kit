use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use scupt_net::notifier::Notifier;
use scupt_net::task::spawn_local_task;
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use scupt_util::res_of::res_io;
use tokio::runtime::Builder;
use tokio::task::LocalSet;
use tracing::{debug, error};

use crate::player::action_incoming::ActionIncoming;
use crate::player::dtm_server::DTMServer;

/// Deterministic  Player
pub struct DTMPlayer {

}

/// Option parameters for a testing
pub struct TestOption {
    /// when wait_both_begin_and_end_action is enable, the DTM player would synchronize all
    /// begin or end Setup/Check/Input/Internal/Output actions, otherwise,
    /// only end Setup/Check/Input actions, begin Output actions, begin/end Internal actions are
    /// synchronized.
    pub wait_both_begin_and_end_action:bool,

    /// when output_action_sequential is enable, we do not concern the order of a sequence of output
    /// actions
    pub output_action_sequential:bool,

    /// after `seconds_wait_message_timeout`(default is 30) seconds, report as an inconsistency
    /// trace error
    pub seconds_wait_message_timeout:u64,
}


impl TestOption {
    pub fn new() -> Self {
        Self {
            wait_both_begin_and_end_action: false,
            output_action_sequential: false,
            seconds_wait_message_timeout: 30,
        }
    }

    pub fn set_wait_both_begin_and_end_action(self, enable:bool) -> Self {
        let mut s = self;
        s.wait_both_begin_and_end_action = enable;
        s
    }

    pub fn set_sequential_output_action(self, enable:bool) -> Self {
        let mut s = self;
        s.output_action_sequential = enable;
        s
    }

    pub fn set_seconds_wait_message_timeout(self, seconds:u64) -> Self {
        let mut s = self;
        s.seconds_wait_message_timeout = seconds;
        s
    }
}

impl Default for TestOption {
    fn default() -> Self {
        Self::new()
    }
}

impl DTMPlayer
{
    /// Create a new deterministic player
    pub fn new() -> Self {
        Self {
        }
    }


    /// Deterministic run a trace.
    ///
    /// `player_node_id` is the node id of the player server
    ///
    /// `player_addr` is the network address of the player server
    ///
    /// `peers` is a map of the tested node's node id and their network addresses
    ///
    /// `action_incoming` is a Arc pointer of ActionIncoming trait which the player can read
    ///     a sequence of actions(a trace)
    ///
    /// `notifier` is a Notifier type to notify stop the service
    ///
    /// `option` is option parameters for the testing
    ///
    /// `fn_done` invoke when testing finish
    pub fn run_trace<F>(
        player_node_id: NID,
        player_addr: SocketAddr,
        peers: HashMap<NID, SocketAddr>,
        action_incoming: Arc<dyn ActionIncoming>,
        notifier: Notifier,
        option:TestOption,
        fn_done: F,
    ) -> Res<()>
        where F: Fn() + 'static
    {
        let r_build = Builder::new_current_thread()
            .enable_all()
            .build();
        let r = res_io(r_build)?;
        let runtime = Arc::new(r);
        let name = format!("player_{}", player_node_id);
        let s =
            DTMServer::new(
                player_node_id,
                name,
                notifier.clone(),
                option
            )?;
        let server = Arc::new(s);
        let ls = LocalSet::new();
        {
            let server = server.clone();
            let future = async move {
                let r1 = server.start_network(
                    player_addr,
                    peers).await;
                match r1 {
                    Ok(()) => {}
                    Err(e) => {
                        error!("{}", e.to_string());
                        return;
                    }
                }
                let r2 = server.start_dtm_test(action_incoming).await;
                match r2 {
                    Ok(r) => {
                        let _ = r.await;
                    }
                    Err(e) => {
                        error!("dtm player error: {}", e.to_string());
                        return;
                    }
                }

                debug!("dtm player handle test done");
                fn_done();
            };
            let _n = notifier.clone();
            ls.spawn_local(async {
                spawn_local_task(_n, "dtm", future).unwrap();
            });
        }
        server.run(Some(ls), runtime);
        debug!("dtm player server stopped");
        Ok(())
    }
}