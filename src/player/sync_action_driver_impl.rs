use std::sync::Arc;

use scupt_util::error_type::ET;
use scupt_util::message::Message;
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use std::sync::mpsc::{channel, Sender};
use tokio::sync::mpsc::UnboundedSender;

use tracing::trace;
use uuid::Uuid;
use scupt_util::serde_json_string::SerdeJsonString;

use crate::action::action_serde_json_value::ActionSerdeJsonValue;
use crate::action::action_type::ActionType;
use crate::player::msg_ctrl::MessageControl;
use crate::player::sync_action_driver::SyncActionDriver;

pub struct SyncActionDriverImplInner {
    node_id:NID,
    server_id:NID,
    sender: UnboundedSender<(Message<MessageControl>, Sender<Message<MessageControl>>)>,
}

pub struct SyncActionDriverImpl {
    inner: Arc<SyncActionDriverImplInner>,
}

impl SyncActionDriverImpl {
    pub fn new(node_id:NID, server_node_id:NID, sender:UnboundedSender<(Message<MessageControl>, Sender<Message<MessageControl>>)>) -> Self {
        Self {
            inner: Arc::new(SyncActionDriverImplInner::new(node_id, server_node_id, sender)),
        }
    }
}


impl SyncActionDriver for SyncActionDriverImpl {
    fn begin_action(&self, action_type:ActionType, source:NID, dest:NID, payload_json:String) -> Res<()> {
        let action = ActionSerdeJsonValue::from_json(action_type, source, dest, payload_json)?.to_serde_json_string();
        self.inner.async_begin_action(action)
    }

    fn end_action(&self,  action_type:ActionType, source:NID, dest:NID, payload_json:String) -> Res<()> {
        let action = ActionSerdeJsonValue::from_json(action_type, source, dest, payload_json)?.to_serde_json_string();
        self.inner.async_end_action(action)
    }
}

impl SyncActionDriverImplInner {
    fn new(node_id:NID, server_id:NID,
           sender:UnboundedSender<(Message<MessageControl>, Sender<Message<MessageControl>>)>
    ) -> Self {
        Self {
            node_id,
            server_id,
            sender,
        }
    }

    fn async_begin_action(&self, action: SerdeJsonString) -> Res<()> {
        self.async_send_action(action, true)
    }

    fn async_end_action(&self, action: SerdeJsonString) -> Res<()> {
        self.async_send_action(action, false)
    }

     fn async_send_action(&self, action: SerdeJsonString, begin_action: bool) -> Res<()> {
        let uuid = Uuid::new_v4();
        let m = MessageControl::ActionReq {
            id: uuid.to_string(),
            action,
            begin: begin_action,
        };
        let req = Message::new(m, self.node_id, self.server_id);

        // send request
        let (req_sender, resp_receiver) = channel();
        match self.sender.send((req, req_sender)) {
            Ok(()) => {}
            Err(e) => { return Err(ET::SenderError(e.to_string())); }
        };

        let response:MessageControl = match resp_receiver.recv() {
            Ok(m) => { m.payload() }
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

