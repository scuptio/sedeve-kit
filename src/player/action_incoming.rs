use scupt_util::res::Res;

/// ActionIncoming trait for accessing a trace
pub trait ActionIncoming: Sync + Send {
    /// Return next action by JSON string representation
    /// If a trace finish, `next` return an error with ET::EOF
    fn next(&self) -> Res<String>;
}