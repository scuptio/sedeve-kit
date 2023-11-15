use std::collections::HashMap;
use std::fs::read_to_string;
use std::string::String;

use log::error;
use scupt_util::error_type::ET;
use scupt_util::res::Res;
use scupt_util::res_of::res_io;
use serde_json::Value;

use crate::trace_gen::action_graph::ActionGraph;
use crate::trace_gen::action_node::ActionNode;
use crate::trace_gen::dot_visitor::DotVisitor;
use crate::trace_gen::dot_parser::DotParser;

pub fn parse_dot(path: String, dict: HashMap<String, Value>) -> Res<ActionGraph<i64, ActionNode>> {
    let read_result = read_to_string(path);
    let dot = res_io(read_result)?;
    let action_graph = parse_from_dot_text(dot, dict)?;
    Ok(action_graph)
}

fn parse_from_dot_text(text: String, dict: HashMap<String, Value>) -> Res<ActionGraph<i64, ActionNode>> {
    let mut dot_parser = DotParser::new();

    let tree = dot_parser.parse(&text)?;
    if tree.root_node().is_error() {
        error!("parse tree : {:?}", tree);
        return Err(ET::ParseError("parse dot error!".to_string()));
    }

    let mut visitor = DotVisitor::new(text, dict);
    let _ = visitor.visit_root(tree.root_node())?;
    let action_graph = visitor.action_graph();
    Ok(action_graph)
}