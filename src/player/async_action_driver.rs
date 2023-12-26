use async_trait::async_trait;
use scupt_util::res::Res;

use crate::action::action_serde_json_string::ActionSerdeJsonString;

#[async_trait]
pub trait AsyncActionDriver: Sync + Send {
    async fn begin_action(&self, action: ActionSerdeJsonString) -> Res<()>;

    async fn end_action(&self, action: ActionSerdeJsonString) -> Res<()>;
}
