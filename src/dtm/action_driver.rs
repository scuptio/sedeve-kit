use scupt_util::res::Res;
use scupt_util::serde_json_string::SerdeJsonString;


pub trait ActionDriver: Sync + Send {
    fn begin_action(&self, action: SerdeJsonString) -> Res<()>;

    fn end_action(&self, action: SerdeJsonString) -> Res<()>;
}
