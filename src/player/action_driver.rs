use scupt_util::res::Res;

use crate::action::action_serde_json_string::ActionSerdeJsonString;

pub trait ActionDriver: Sync + Send {
    fn begin_action(&self, action: ActionSerdeJsonString) -> Res<()>;

    fn end_action(&self, action: ActionSerdeJsonString) -> Res<()>;
}
