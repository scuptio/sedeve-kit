use std::sync::Arc;
use async_trait::async_trait;
use scupt_util::error_type::ET;
use scupt_util::res::Res;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot::{channel, Sender};
use tracing::trace;
use uuid::Uuid;

use crate::action::action_serde_json_string::ActionSerdeJsonString;
use crate::action_player::async_action_driver::AsyncActionDriver;
use crate::action_player::msg_ctrl::MessageControl;

pub struct AsyncActionDriverImplInner {
    sender: UnboundedSender<(MessageControl, Sender<MessageControl>)>,
}

pub struct AsyncActionDriverImpl {
    inner: Arc<AsyncActionDriverImplInner>,
}


impl AsyncActionDriverImpl {
    pub fn new(sender:UnboundedSender<(MessageControl, Sender<MessageControl>)>) -> Self {
        Self {
            inner: Arc::new(AsyncActionDriverImplInner::new(sender)),
        }
    }
}

#[async_trait]
impl AsyncActionDriver for AsyncActionDriverImpl {
    async fn begin_action(&self, action: ActionSerdeJsonString) -> Res<()> {
        self.inner.async_begin_action(action).await
    }

    async fn end_action(&self, action: ActionSerdeJsonString) -> Res<()> {
        self.inner.async_end_action(action).await
    }
}

impl AsyncActionDriverImplInner {
    pub fn new(sender:UnboundedSender<(MessageControl, Sender<MessageControl>)>) -> Self {
        Self {
            sender,
        }
    }

    async fn async_begin_action(&self, action: ActionSerdeJsonString) -> Res<()> {
        self.async_send_action(action, true).await
    }

    async fn async_end_action(&self, action: ActionSerdeJsonString) -> Res<()> {
        self.async_send_action(action, false).await
    }

    async fn async_send_action(&self, action: ActionSerdeJsonString, begin_action: bool) -> Res<()> {
        let uuid = Uuid::new_v4();
        let request = MessageControl::ActionReq {
            id: uuid.to_string(),
            action,
            begin: begin_action,
        };

        // send request
        let (req_sender, resp_receiver) = channel();
        match self.sender.send((request, req_sender)) {
            Ok(()) => {}
            Err(e) => { return Err(ET::SerdeError(e.to_string())); }
        };

        // receive response
        let response:MessageControl = match resp_receiver.await {
            Ok(m) => { m }
            Err(e) => { return Err(ET::RecvError(e.to_string())); }
        };

        trace!("receive response {:?}", response);
        match response {
            MessageControl::ActionACK { .. } => { }
            _ => { panic!("not possible") }
        }
        Ok(())
    }
}

