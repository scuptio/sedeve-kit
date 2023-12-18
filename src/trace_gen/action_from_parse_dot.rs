use std::collections::HashMap;
use std::fs::read_to_string;
use std::string::String;

use log::error;
use scupt_util::error_type::ET;
use scupt_util::res::Res;
use scupt_util::res_of::res_io;
use serde_json::Value;

use crate::trace_gen::action_graph::ActionGraph;
use crate::trace_gen::dot_parser::DotParser;
use crate::trace_gen::dot_visitor::DotVisitor;
use crate::trace_gen::trace_db_interm::TraceDBInterm;

#[cfg(test)]
pub fn parse_dot(path: String, dict: HashMap<String, Value>) -> Res<()> {
    let _ = parse_dot_gut(path, dict)?;
    Ok(())
}

// read action from dot file, and output actions to sqlite database
pub fn dot_action_to_db(dot_path: String, dict: HashMap<String, Value>, output_db: String) -> Res<ActionGraph<i64>> {
    let visitor = parse_dot_gut(dot_path, dict)?;
    let trace_db = TraceDBInterm::new(output_db, None, None)?;
    trace_db.write_action(visitor.actions().clone())?;
    let graph = trace_db.gen_graph()?;
    Ok(graph)
}

fn parse_dot_gut(path: String, dict: HashMap<String, Value>) -> Res<DotVisitor> {
    let text = res_io(read_to_string(path))?;
    let mut dot_parser = DotParser::new();

    let tree = dot_parser.parse(&text)?;
    if tree.root_node().is_error() {
        error!("parse tree : {:?}", tree);
        return Err(ET::ParseError("parse dot error!".to_string()));
    }

    let mut visitor = DotVisitor::new(text, dict);
    let _ = visitor.visit_root(tree.root_node())?;
    Ok(visitor)
}
