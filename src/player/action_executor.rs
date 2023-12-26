use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

use scupt_util::message::Message;
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::sleep;
use tracing::{error, trace};

use crate::action::action_serde_json_value::ActionSerdeJsonValue;
use crate::action::action_type::ActionType;
use crate::player::action_reorder::ActionReorder;
use crate::player::msg_ctrl::MessageControl;

struct ActionExecutorInner {
    seconds_timeout:u64,
    wait_both_begin_and_end_action: bool,
    trace_wait_input_begin: ActionReorder,
    trace_wait_input_end: ActionReorder,
    node_wait_input: ActionReorder,

    trace_wait_output_begin: ActionReorder,
    trace_wait_output_end: ActionReorder,
    node_wait_output: ActionReorder,

    trace_wait_internal_begin: ActionReorder,
    trace_wait_internal_end: ActionReorder,
    node_wait_internal: ActionReorder,
}

#[derive(Clone)]
pub struct ActionExecutor {
    inner: Arc<ActionExecutorInner>,
}

impl ActionExecutor {
    pub fn new(
        wait_both_begin_and_end_action:bool,
        seconds_timeout:u64,
    ) -> Self {
        Self {
            inner: Arc::new(ActionExecutorInner {
                seconds_timeout,
                wait_both_begin_and_end_action,
                trace_wait_input_begin: ActionReorder::new(seconds_timeout),
                trace_wait_input_end: ActionReorder::new(seconds_timeout),
                node_wait_input: ActionReorder::new(seconds_timeout),
                trace_wait_output_begin: ActionReorder::new(seconds_timeout),
                trace_wait_output_end: ActionReorder::new(seconds_timeout),
                node_wait_output: ActionReorder::new(seconds_timeout),
                trace_wait_internal_begin: ActionReorder::new(seconds_timeout),
                trace_wait_internal_end: ActionReorder::new(seconds_timeout),
                node_wait_internal: ActionReorder::new(seconds_timeout),
            }),
        }
    }


    pub async fn expect_node_sync(&self, action: &ActionSerdeJsonValue) -> Res<bool> {
        let action_type = action.action_type()?;
        let msg = action.action_json_value_ref();
        match action_type {
            ActionType::Input |
            ActionType::Setup |
            ActionType::Check
            => {
                trace!("INPUT: notify case file input {:?}", msg);
                self.inner.node_wait_input.add_action(action).await?;
                if self.inner.wait_both_begin_and_end_action {
                    let ok = self.inner.trace_wait_input_begin.wait_action(action).await?;
                    if !ok {
                        return Ok(false);
                    }
                }
                trace!("INPUT: wait receive from node {:?}", msg);
                let ok = self.inner.trace_wait_input_end.wait_action(action).await?;
                if !ok {
                    return Ok(false);
                }
                trace!("INPUT: wait receive from node {:?} end", msg);
            }
            ActionType::Output => {
                trace!("INPUT: notify case file input {:?}", msg);
                self.inner.node_wait_output.add_action(action).await?;
                trace!("INPUT: wait receive from node {:?}", msg);
                let ok = self.inner.trace_wait_output_begin.wait_action(action).await?;
                if !ok {
                    return Ok(false);
                }
                trace!("INPUT: notify case file input {:?}", msg);
                if self.inner.wait_both_begin_and_end_action {
                    let ok = self.inner.trace_wait_output_end.wait_action(action).await?;
                    if !ok {
                        return Ok(false);
                    }
                }
            }
            ActionType::Internal => {
                trace!("INPUT: notify case file input {:?}", msg);
                self.inner.node_wait_internal.add_action(action).await?;

                trace!("INPUT: wait receive from node begin {:?}", msg);
                let ok = self.inner.trace_wait_internal_begin.wait_action(action).await?;
                if !ok {
                    return Ok(false);
                }
                trace!("INPUT: wait receive from node end {:?}", msg);
                let ok = self.inner.trace_wait_internal_end.wait_action(action).await?;
                if !ok {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    pub async fn expect_action_in_trace(
        &self,
        source:NID,
        dest:NID,
        id:String, // request message id
        action: ActionSerdeJsonValue,
        begin: bool,
        ch_sender: UnboundedSender<Message<MessageControl>>
    ) -> Res<()> {
        let ok = self.expect_action_in_trace_gut(&action, begin).await?;
        if !ok {
            let str = if begin {
                "begin"
            } else {
                "end"
            };
            error!("message timeout, expect wait {} action {:?} in trace", str, action);
            sleep(Duration::from_secs(self.inner.seconds_timeout)).await;
            exit(-1);
        } else {
            // ACK of the requested action
            let response = Message::new(
                MessageControl::ActionACK { id },
                dest,
                source);
            trace!("send response {:?}", response);
            let r = ch_sender.send(response.clone());
            trace!("end send response {:?}", response);
            match r {
                Ok(_) => {
                    trace!("send response {:?}", response);
                }
                Err(e) => {
                    error!("response action {:?} , error : {}" , action, e.to_string())
                }
            }
        }
        Ok(())
    }

    async fn expect_action_in_trace_gut(&self, action: &ActionSerdeJsonValue, begin: bool) -> Res<bool> {
        trace!("enter, SIMULATOR: receive Action {:?}, {}", action, begin);
        let action_type = action.action_type()?;
        match action_type {
            ActionType::Input |
            ActionType::Setup |
            ActionType::Check => {
                if begin {
                    if self.inner.wait_both_begin_and_end_action {
                        trace!("RECEIVE: notify case file {:?}", action);
                        self.inner.trace_wait_input_begin.add_action(action).await?;
                    }
                } else {
                    trace!("RECEIVE: wait case file {:?}", action);
                    let ok = self.inner.node_wait_input.wait_action(action).await?;
                    if !ok {
                        return Ok(false);
                    }
                    trace!("RECEIVE: notify read case file {:?}", action);
                    self.inner.trace_wait_input_end.add_action(action).await?;
                }

           }
            ActionType::Output => {
                if begin {
                    trace!("RECEIVE: notify case file {:?}", action);
                    self.inner.trace_wait_output_begin.add_action(action).await?;
                    trace!("RECEIVE: wait case file {:?}", action);
                    let ok = self.inner.node_wait_output.wait_action(action).await?;
                    if !ok {
                        return Ok(false);
                    }
                }  else {
                    if self.inner.wait_both_begin_and_end_action {
                        trace!("RECEIVE: notify read case file {:?}", action);
                        self.inner.trace_wait_output_end.add_action(action).await?;
                    }
                }
            }
            ActionType::Internal => {
                if begin {
                    trace!("RECEIVE: notify case file {:?}", action);
                    self.inner.trace_wait_internal_begin.add_action(action).await?;
                    trace!("RECEIVE: wait case file {:?}", action);
                    let ok = self.inner.node_wait_internal.wait_action(action).await?;
                    if !ok {
                        return Ok(false);
                    }
                    trace!("RECEIVE: done {:?}", action);
                } else {
                    trace!("RECEIVE: notify case file {:?}", action);
                    self.inner.trace_wait_internal_end.add_action(action).await?;
                    trace!("RECEIVE: notify case file {:?} done", action);
                }
            }
        }
        trace!("exit, SIMULATOR: receive Action {:?}, {}", action, begin);
        Ok(true)
    }
}