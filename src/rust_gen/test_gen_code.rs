#[cfg(test)]
mod test {
    use scupt_util::logger::logger_setup;
    use scupt_util::res::Res;

    use crate::rust_gen::action_visitor::ActionVisitor;

    #[test]
    fn test_gen_code() {
        _test_gen_code().unwrap();
    }

    fn _test_gen_code() -> Res<()> {
        logger_setup("debug");
        let action_text = r#"


automata handle_message {
    input  handle_append_request(
        message:  RaftMessage,
        from: NodeId,
        to: NodeId,
        name: unknown TODO,
        name: unknown XXX,
    )

    output append_response(message: AppendResponse, from:NodeId, to:NodeId) -> bool


    internal handle_append_response(message: AppendResponse, from:NodeId, to:NodeId)
}

struct AppendRequest {
    term            : u64,
    prev_log_index  : u64,
    prev_log_term   : u64,
    commit_index    : u64,
    log_entries     : Sequence<LogEntry>,
}

enum RaftMessage {
    RMAppendRequest(AppendRequest)
}

        "#.to_string();
        let mut visitor = ActionVisitor::new(action_text.clone());
        visitor.parse().expect("parse error");
        Ok(())
    }
}