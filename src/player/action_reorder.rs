use std::collections::HashMap;
use std::sync::Arc;

use scupt_util::res::Res;
use tokio::select;
use tokio::sync::Mutex;
use tokio::time::Duration;
use tokio::time::sleep;
use tracing::{Instrument, trace, trace_span};

use crate::action::action_serde_json_value::ActionSerdeJsonValue;
use crate::player::action_channel::{ActionReceiver, ActionSender, io_event_channel};

type ActionCh = (ActionSender, ActionReceiver, Arc<Mutex<u64>>);

#[derive(Clone)]
pub struct ActionReorder {
    seconds_timeout : u64,
    hash_map: Arc<Mutex<HashMap<ActionSerdeJsonValue,
        ActionCh>>
    >,
}


impl  ActionReorder {
    pub fn new(seconds_timeout:u64) -> Self {
        Self {
            seconds_timeout,
            hash_map: Default::default(),
        }
    }

    pub async fn wait_action(&self, action: &ActionSerdeJsonValue) -> Res<bool> {
        let (_, r) = self.get_channel(action).await;
        let ok = select! {
            recv = r.recv() => {
                self.dec_ref_num(action).await;
                let a = recv?;
                let j = a.to_action_serde_json_value()?;
                assert!(j.eq(action));
                true
            }
            _ = sleep(Duration::from_secs(self.seconds_timeout)) => {
                false
            }
        };
        Ok(ok)
    }

    pub async fn add_action(&self, action: &ActionSerdeJsonValue) -> Res<()> {
        let (s, _) = self.get_channel(action).await;
        s.send(action.to_action_message())?;
        self.dec_ref_num(action).await;
        Ok(())
    }

    async fn get_channel(&self, action: &ActionSerdeJsonValue) -> (ActionSender, ActionReceiver) {
        let mut map = self.hash_map.lock().await;
        let opt_recv = map.get(&action);
        match opt_recv {
            None => {
                trace!("could not find {}", action.action_json_value_ref().to_string());
                let (s, r) = io_event_channel();
                map.insert(action.clone(), (s.clone(), r.clone(), Arc::new(Mutex::new(1))));
                (s, r)
            }
            Some((s, r, num)) => {
                trace!("find existing {}", action.action_json_value_ref().to_string());
                let mut n = num.lock()
                    .instrument(trace_span!("num lock"))
                    .await;
                *n += 1;
                (s.clone(), r.clone())
            }
        }
    }

    async fn dec_ref_num(&self, action: &ActionSerdeJsonValue) {
        let map = self.hash_map.lock().await;
        let opt_recv = map.get(&action);
        match opt_recv {
            None => {}
            Some((_, _, num)) => {
                let nn = num.clone();
                let mut n = nn.lock().await;
                if *n >= 1 {
                    *n -= 1;
                }
            }
        }
    }
}