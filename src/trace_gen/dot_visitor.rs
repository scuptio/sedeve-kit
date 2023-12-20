use std::collections::HashMap;
use std::string::String;

use scupt_util::error_type::ET;
use scupt_util::escape_string::escape_string;
use scupt_util::res::Res;
use scupt_util::res_of::res_option;
use serde_json::Value;
use tracing::info;
use tree_sitter::Node;

use crate::action::constant::{ACTION_LABEL, LABEL, LOGICAL_CONJUNCTION};
use crate::action::tal_vars_parser::TLAVarsParser;
use crate::action::tla_actions::TLAActionSeq;
use crate::action::tla_ast;
use crate::action::tla_var_list_visitor::TLAVarListVisitor;
use crate::trace_gen::context_string::node_context_string;
use crate::trace_gen::dot_ast;

pub struct DotVisitor {
    text: String,
    action_var_parser: TLAVarsParser,
    action_node_json: Vec<(i64, i64, String, String)>,
    const_word_dict: HashMap<String, Value>,
}


impl DotVisitor {
    pub fn new(text: String, map_const: HashMap<String, Value>) -> Self {
        let tla_parser = TLAVarsParser::new();
        Self {
            text,
            action_var_parser: tla_parser,
            action_node_json: Default::default(),
            const_word_dict: map_const,
        }
    }

    pub fn actions(&self) -> &Vec<(i64, i64, String, String)> {
        &self.action_node_json
    }

    pub fn visit_root(&mut self, node: Node) -> Res<dot_ast::SourceFile> {
        let opt_strict = node.child_by_field_name("strict");
        let is_strict = opt_strict.is_some();
        let opt_node_id = node.child_by_field_name("id");
        let opt_id = match opt_node_id {
            Some(n) => {
                let id = self.visit_id(n)?;
                Some(id)
            }
            None => None,
        };

        let opt_graph = node.child_by_field_name("graph_or_digraph");
        let is_digraph = match opt_graph {
            Some(n) => {
                let opt_digraph = n.child_by_field_name("digraph");
                opt_digraph.is_some()
            }
            None => {
                panic!("no such field");
            }
        };


        let opt_node_block = node.child_by_field_name("block");
        let block = match opt_node_block {
            Some(n) => {
                self.visit_block(n)?
            }
            None => {
                info!("possible read a string value like \\\"xxx\\\" cannot be parsed");
                return Err(ET::ParseError("no block node".to_string()));
            }
        };
        let source_file = dot_ast::SourceFile {
            is_strict,
            is_digraph,
            opt_id,
            block,
        };
        Ok(source_file)
    }

    fn visit_block(&mut self, node_block: Node) -> Res<dot_ast::Block> {
        let opt_node_stmt_list = node_block.child_by_field_name("stmt_list");
        let stmt_list = match opt_node_stmt_list {
            Some(n) => {
                let stmt_list = self.visit_stmt_list(n)?;
                stmt_list
            }
            None => { vec![] }
        };

        Ok(dot_ast::Block { stmt_list })
    }

    fn visit_stmt_list(&mut self, node_stmt_list: Node) -> Res<Vec<dot_ast::Stmt>> {
        let mut cursor = node_stmt_list.walk();
        let mut iter = node_stmt_list.children_by_field_name("stmt", &mut cursor);
        let mut vec = Vec::new();
        while let Some(n) = iter.next() {
            let stmt = self.visit_stmt(n)?;
            vec.push(stmt);
        }

        Ok(vec)
    }

    fn visit_stmt(&mut self, node_stmt: Node) -> Res<dot_ast::Stmt> {
        if node_stmt.child_count() != 1 {
            return Err(ET::NoneOption);
        }
        let opt = node_stmt.child(0);
        let n = res_option(opt)?;
        let kind = n.kind();
        let stmt = match kind {
            "subgraph" => {
                let subgraph = self.visit_subgraph(n)?;
                dot_ast::Stmt::SSubgraph(subgraph)
            }
            "node_stmt" => {
                let node_stmt = self.visit_node_stmt(n)?;
                self.add_node_action_json(&node_stmt)?;
                dot_ast::Stmt::SNodeStmt(node_stmt)
            }
            "edge_stmt" => {
                let edge_stmt = self.visit_edge_stmt(n)?;
                dot_ast::Stmt::SEdgeStmt(edge_stmt)
            }
            "attr_stmt" => {
                let attr_stmt = self.visit_attr_stmt(n)?;
                dot_ast::Stmt::SAttrStmt(attr_stmt)
            }
            "attribute" => {
                let attribute = self.visit_attribute(n)?;
                dot_ast::Stmt::SAttribute(attribute)
            }
            _ => { return Err(ET::ParseError(format!("unknown kind {}", kind))); }
        };
        Ok(stmt)
    }

    fn visit_subgraph(&mut self, node_sub_graph: Node) -> Res<dot_ast::Subgraph> {
        let opt_n_subgraph_id = node_sub_graph.child_by_field_name("subgraph_id");
        let opt_subgraph_id = match opt_n_subgraph_id {
            Some(n) => {
                let opt_id = n.child_by_field_name("id");
                match opt_id {
                    Some(n_id) => { Some(self.visit_id(n_id)?) }
                    None => { None }
                }
            }
            None => { None }
        };

        let opt_block = node_sub_graph.child_by_field_name("block");
        let block = match opt_block {
            Some(n) => { self.visit_block(n)? }
            None => { return Err(ET::ParseError("no block in subgraph".to_string())); }
        };

        Ok(dot_ast::Subgraph {
            opt_subgraph_id,
            block,
        })
    }


    fn visit_edge_stmt(&mut self, node_edge_stmt: Node) -> Res<dot_ast::EdgeStmt> {
        let mut cursor = node_edge_stmt.walk();
        let mut count = node_edge_stmt.child_count();
        let mut ok = cursor.goto_first_child();
        count -= 1;
        let left = if ok {
            let field_name = cursor.field_name().unwrap_or("");
            match field_name {
                "left_node_id" => { dot_ast::EdgeNode::ENNodeId(self.visit_node_id(cursor.node())?) }
                "left_subgraph" => { dot_ast::EdgeNode::ENSubGraph(self.visit_subgraph(cursor.node())?) }
                _ => { return Err(ET::ParseError("unknown field name".to_string())); }
            }
        } else {
            return Err(ET::ParseError("no left node".to_string()));
        };
        let mut right_list = vec![];
        while count >= 2 {
            ok = cursor.goto_next_sibling();
            if !ok {
                break;
            } else {
                count -= 1;
            }

            let field_name = cursor.field_name().unwrap_or("");
            let edgeop = match field_name {
                "edgeop" => { self.visit_edge_op(cursor.node())? }
                "attr_list" => { break; }
                _ => { return Err(ET::ParseError("unknown field name".to_string())); }
            };
            ok = cursor.goto_next_sibling();
            if !ok {
                break;
            } else {
                count -= 1;
            }
            let field_name = cursor.field_name().unwrap_or("");
            let right_node = match field_name {
                "right_subgraph" => { dot_ast::EdgeNode::ENSubGraph(self.visit_subgraph(cursor.node())?) }
                "right_node_id" => { dot_ast::EdgeNode::ENNodeId(self.visit_node_id(cursor.node())?) }
                _ => { return Err(ET::ParseError("unknown field name".to_string())); }
            };
            right_list.push((edgeop, right_node));
        }
        let attr_list = if ok && count == 1 {
            ok = cursor.goto_next_sibling();
            count -= 1;
            assert_eq!(count, 0);
            if ok {
                self.visit_attr_list(cursor.node(), false)?
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        Ok(dot_ast::EdgeStmt {
            left,
            right_list,
            attr_list,
        })
    }

    fn visit_edge_op(&mut self, node: Node) -> Res<dot_ast::EdgeOp> {
        let op = if node.child_by_field_name("line").is_some() {
            dot_ast::EdgeOp::Line
        } else if node.child_by_field_name("arrow").is_some() {
            dot_ast::EdgeOp::Arrow
        } else {
            return Err(ET::ParseError("must be -- or ->".to_string()));
        };
        Ok(op)
    }

    fn visit_node_stmt(&mut self, node_node_stmt: Node) -> Res<dot_ast::NodeStmt> {
        let opt_node_node_id = node_node_stmt.child_by_field_name("node_id");
        let node_id = match opt_node_node_id {
            Some(n) => {
                let node_id = self.visit_node_id(n)?;
                node_id
            }
            None => { return Err(ET::ParseError("no node_id".to_string())); }
        };
        let opt_node_attr_list = node_node_stmt.child_by_field_name("attr_list");
        let attr_list = match opt_node_attr_list {
            Some(n) => {
                let attr_list = self.visit_attr_list(n, true)?;
                attr_list
            }
            None => { vec![] }
        };
        Ok(dot_ast::NodeStmt {
            node_id,
            attr_list,
        })
    }

    fn visit_attr_stmt(&mut self, node_attr_stmt: Node) -> Res<dot_ast::AttrStmt> {
        let opt_n_as = node_attr_stmt.child_by_field_name("attr_type");
        let attr_type = match opt_n_as {
            Some(n) => { self.visit_attr_type(n)? }
            None => { return Err(ET::ParseError(format!("no attr_type {}", self.node_context(node_attr_stmt)?).to_string())); }
        };

        let opt_n_al = node_attr_stmt.child_by_field_name("attr_list");
        let attr_list = match opt_n_al {
            Some(n) => { self.visit_attr_list(n, false)? }
            None => { vec![] }
        };
        Ok(dot_ast::AttrStmt {
            attr_type,
            attr_list,
        })
    }

    fn visit_attr_type(&mut self, node: Node) -> Res<dot_ast::AttrType> {
        let kind = self.node_context(node)?.to_lowercase();
        let op = match kind.as_str() {
            "" => { dot_ast::AttrType::ATGraph }
            "node" => { dot_ast::AttrType::ATNode }
            "edge" => { dot_ast::AttrType::ATEdge }
            _ => {
                return Err(
                    ET::ParseError(format!("unknown attr type {}",
                                           self.node_context(node)?).to_string()));
            }
        };
        return Ok(op);
    }
    fn visit_node_id(&mut self, node: Node) -> Res<dot_ast::NodeId> {
        let opt_id = node.child_by_field_name("id");
        let id = match opt_id {
            Some(n) => { self.visit_id(n)? }
            None => { return Err(ET::ParseError("no id in node_id".to_string())); }
        };

        let opt_id = node.child_by_field_name("port");
        let opt_port = match opt_id {
            Some(n) => {
                Some(self.visit_port(n)?)
            }
            None => { None }
        };
        Ok(dot_ast::NodeId {
            id,
            opt_port,
        })
    }

    fn visit_attr_list(&mut self, node: Node, is_node_attr: bool) -> Res<Vec<Vec<dot_ast::Attribute>>> {
        let mut cursor = node.walk();
        let mut ok = cursor.goto_first_child();
        let mut attr_list = Vec::new();
        let mut a_attr_list = Vec::new();
        while ok {
            match cursor.field_name().unwrap_or("") {
                "left_bracket" => {}
                "right_bracket" => {
                    if !a_attr_list.is_empty() {
                        let mut vec = vec![];
                        std::mem::swap(&mut vec, &mut a_attr_list);
                        attr_list.push(vec);
                    }
                }
                "attribute" => {
                    let attribute = self.visit_attribute(cursor.node())?;
                    a_attr_list.push(attribute);
                }
                "semiconlon" => {}
                _ => {}
            }
            ok = cursor.goto_next_sibling();
        }
        if is_node_attr {
            let mut vec_label_value = vec![];
            for i in 0..attr_list.len() {
                let vec = match attr_list.get(i) {
                    Some(v) => { v }
                    None => { panic!("error"); }
                };
                for j in 0..vec.len() {
                    let attr = match vec.get(j) {
                        Some(v) => { v }
                        None => { panic!("error"); }
                    };
                    match &attr.name {
                        dot_ast::Id::Identifier(l) => {
                            if l != LABEL {
                                break;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                    match &attr.value {
                        dot_ast::Id::StringList(str_vec) => {
                            self.visit_string_list(i, j, str_vec, &mut vec_label_value)?;
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }

            for (i, j, var) in vec_label_value {
                let v = match attr_list.get_mut(i) {
                    Some(v) => { v }
                    None => { panic!("no such element"); }
                };
                let e = match v.get_mut(j) {
                    Some(a) => { a }
                    None => { panic!("no such element"); }
                };
                e.opt_tla_var_list.push(var);
            }
        }
        Ok(attr_list)
    }

    fn visit_string_list(
        &mut self,
        i: usize,
        j: usize,
        str_vec: &Vec<String>,
        vec_label_value: &mut Vec<(usize, usize, tla_ast::TLAVariableList)>) -> Res<()> {
        let mut opt_string: Option<String> = None;
        for s in str_vec {
            if s.len() > 2 &&
                s.contains(ACTION_LABEL) &&
                s.starts_with('"') &&
                s.ends_with('"') {
                let mut ss = s.clone();
                ss.remove(0);
                ss.remove(ss.len() - 1);
                let s = escape_string(&ss);
                for assignment in s.split(LOGICAL_CONJUNCTION) {
                    if assignment.contains(ACTION_LABEL) {
                        let mut action_assignment = LOGICAL_CONJUNCTION.to_string();
                        action_assignment.push_str(assignment);
                        opt_string = Some(action_assignment);
                        break;
                    }
                }
                if opt_string.is_some() {
                    break;
                }
            }
        }
        match opt_string {
            None => {}
            Some(s) => {
                let tla_variable = self.parse_label(&s)?;
                vec_label_value.push((i, j, tla_variable));
            }
        }
        Ok(())
    }

    fn visit_attribute(&mut self, node_attribute: Node) -> Res<dot_ast::Attribute> {
        let opt_node_name = node_attribute.child_by_field_name("name");
        let name = match opt_node_name {
            Some(n) => { self.visit_id(n)? }
            None => { return Err(ET::ParseError("no attribute name".to_string())); }
        };
        let opt_node_value = node_attribute.child_by_field_name("value");
        let value = match opt_node_value {
            Some(n) => { self.visit_id(n)? }
            None => { return Err(ET::ParseError("no attribute value".to_string())); }
        };
        Ok(dot_ast::Attribute {
            name,
            value,
            opt_tla_var_list: vec![],
        })
    }

    fn visit_port(&mut self, node: Node) -> Res<dot_ast::Port> {
        let opt_node_id = node.child_by_field_name("id");
        let id = match opt_node_id {
            Some(n) => { self.visit_id(n)? }
            None => { return Err(ET::ParseError("no id in port".to_string())); }
        };

        let opt_optional = node.child_by_field_name("optional");
        let opt_id = match opt_optional {
            Some(n) => {
                let opt_n = n.child_by_field_name("opt_id");
                match opt_n {
                    Some(n) => { Some(self.visit_id(n)?) }
                    None => { None }
                }
            }
            None => { None }
        };
        Ok(dot_ast::Port {
            id,
            opt_id,
        })
    }

    fn visit_id(&mut self, id_node: Node) -> Res<dot_ast::Id> {
        let mut cursor = id_node.walk();
        let mut ok = cursor.goto_first_child();
        let mut vec_string = Vec::new();
        while ok {
            match cursor.field_name().unwrap_or("") {
                "identifier" => {
                    let s = self.visit_identifier(cursor.node())?;
                    return Ok(dot_ast::Id::Identifier(s));
                }
                "number_literal" => { return self.visit_number_literal(cursor.node()); }
                "html_string" => { return self.visit_html(cursor.node()); }
                "string_literal" => {
                    let s = self.visit_string_literal(cursor.node())?;
                    vec_string.push(s);
                }
                _ => { return Err(ET::ParseError("unknown field name".to_string())); }
            }
            ok = cursor.goto_next_sibling();
        }
        return Ok(dot_ast::Id::StringList(vec_string));
    }

    fn visit_string_literal(&mut self, identifier: Node) -> Res<String> {
        let s = self.node_context(identifier)?;
        Ok(s)
    }

    fn visit_identifier(&mut self, identifier: Node) -> Res<String> {
        let id = self.node_context(identifier)?;
        Ok(id)
    }

    fn visit_number_literal(&mut self, _number_literal: Node) -> Res<dot_ast::Id> {
        let number_str = self.node_context(_number_literal)?;
        Ok(dot_ast::Id::NumberLiteral(number_str))
    }

    fn visit_html(&mut self, _html: Node) -> Res<dot_ast::Id> {
        let s = self.node_context(_html)?;
        Ok(dot_ast::Id::Html(s))
    }

    fn parse_label(&mut self, text: &String) -> Res<tla_ast::TLAVariableList> {
        let tree = {
            let tree = self.action_var_parser.parse(text)?;
            tree
        };
        let visitor = TLAVarListVisitor::new(text.clone());
        visitor.visit_root(tree.root_node())
    }

    fn node_context(&self, node: Node) -> Res<String> {
        node_context_string(&self.text, node)
    }


    fn add_node_action_json(&mut self, node_stmt: &dot_ast::NodeStmt) -> Res<()> {
        let mut list = tla_ast::TLAVariableList {
            vec: vec![]
        };
        for a_list in &node_stmt.attr_list {
            for a in a_list {
                for var in &a.opt_tla_var_list {
                    list.merge_list(&mut var.clone());
                }
            }
        }
        let opt_json = list.to_json(&self.const_word_dict);
        match opt_json {
            Some(json) => {
                let seq = TLAActionSeq::from(json)?;
                self.action_node_json.push(seq.to_tuple()?);
            }
            None => {}
        }

        Ok(())
    }
}