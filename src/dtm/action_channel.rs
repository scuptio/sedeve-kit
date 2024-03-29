use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use scupt_util::error_type::ET;
use scupt_util::res::Res;
use tokio::sync::{mpsc, Mutex};
use tracing::trace;

use scupt_util::serde_json_string::SerdeJsonString;

#[derive(Clone)]
pub struct ActionReceiver {
    sequence: Arc<AtomicU64>,
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<SerdeJsonString>>>,
}

#[derive(Clone)]
pub struct ActionSender {
    sequence: Arc<AtomicU64>,
    sender: mpsc::UnboundedSender<SerdeJsonString>,
}

impl  ActionSender {
    fn new(sender: mpsc::UnboundedSender<SerdeJsonString>) -> Self {
        Self {
            sequence: Default::default(),
            sender,
        }
    }

    pub fn send(&self, event: SerdeJsonString) -> Res<()> {
        let mut _n = 0;
        let n = self.sequence.fetch_add(1, Ordering::SeqCst);
        trace!("send event {:?} {}", event, n);


        let r = self.sender.send(event);
        match r {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(ET::TokioSenderError(e.to_string())) }
        }
    }
}

impl  ActionReceiver {
    fn new(receiver: mpsc::UnboundedReceiver<SerdeJsonString>) -> Self {
        Self {
            sequence: Default::default(),
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub async fn recv(&self) -> Res<SerdeJsonString> {
        let mut recv = self.receiver.lock().await;
        let opt_event = recv.recv().await;
        match opt_event {
            Some(e) => {
                let n = self.sequence.fetch_add(1, Ordering::SeqCst);
                trace!("receive event {:?} {}", e, n);
                Ok(e)
            }
            None => { Err(ET::EOF) }
        }
    }
}

pub fn io_event_channel() -> (ActionSender, ActionReceiver) {
    let (s, r) = mpsc::unbounded_channel::<SerdeJsonString>();
    (ActionSender::new(s), ActionReceiver::new(r))
}
