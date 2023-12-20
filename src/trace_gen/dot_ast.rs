use std::string::String;

use crate::action::tla_ast::TLAVariableList;

#[derive(Debug)]
pub enum Id {
    Identifier(String),
    NumberLiteral(String),
    Html(String),
    StringList(Vec<String>),
}

pub struct Subgraph {
    pub opt_subgraph_id: Option<Id>,
    pub block: Block,
}

pub struct Port {
    pub id: Id,
    pub opt_id: Option<Id>,
}

pub struct NodeId {
    pub id: Id,
    pub opt_port: Option<Port>,
}


impl NodeId {}

pub struct NodeStmt {
    pub node_id: NodeId,
    pub attr_list: Vec<Vec<Attribute>>,
}

pub enum EdgeNode {
    ENNodeId(NodeId),
    ENSubGraph(Subgraph),
}

pub enum EdgeOp {
    Line,
    Arrow,
}

pub enum Stmt {
    SSubgraph(Subgraph),
    SNodeStmt(NodeStmt),
    SEdgeStmt(EdgeStmt),
    SAttrStmt(AttrStmt),
    SAttribute(Attribute),
}

pub struct EdgeStmt {
    pub left: EdgeNode,
    pub right_list: Vec<(EdgeOp, EdgeNode)>,
    pub attr_list: Vec<Vec<Attribute>>,
}

pub enum AttrType {
    ATGraph,
    ATNode,
    ATEdge,
}

pub struct AttrStmt {
    pub attr_type: AttrType,
    pub attr_list: Vec<Vec<Attribute>>,
}

#[derive(Debug)]
pub struct Attribute {
    pub name: Id,
    pub value: Id,
    pub opt_tla_var_list: Vec<TLAVariableList>,
}

pub struct Block {
    pub stmt_list: Vec<Stmt>,
}

pub struct SourceFile {
    pub is_strict: bool,
    pub is_digraph: bool,
    pub opt_id: Option<Id>,
    pub block: Block,
}