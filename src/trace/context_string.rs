use scupt_util::error_type::ET;
use scupt_util::res::Res;
use tree_sitter::Node;

pub fn node_context_string(text: &String, node: Node) -> Res<String> {
    let range = node.range();
    let r = String::from_utf8(
        text.as_bytes()[range.start_byte..range.end_byte].to_vec()
    );
    match r {
        Ok(s) => Ok(s),
        Err(e) => Err(ET::ParseError(e.to_string()))
    }
}
