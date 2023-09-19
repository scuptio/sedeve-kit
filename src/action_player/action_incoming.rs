
use scupt_util::res::Res;
pub trait ActionIncoming: Sync + Send {
    // return next action by JSON string representation
    fn next(&self) -> Res<String>;
}