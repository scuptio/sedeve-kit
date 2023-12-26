use std::collections::HashMap;
use std::fmt::Write;

use lazy_static::lazy_static;
use scupt_util::error_type::ET;
use scupt_util::res::Res;

use crate::define::const_str::{
    STR_INVALID_NODE_ID, STR_PARAMETER_MESSAGE,
    STR_PARAMETER_NODE_ID_FROM, STR_PARAMETER_NODE_ID_TO,
};
use crate::rust_gen::ast2rust::{A2RContext, AST2Rust};

const TEST_STR: &str = "TEST";

const PRIMITIVE_TYPE_BOOL: &str = "bool";
const PRIMITIVE_TYPE_I8: &str = "i8";
const PRIMITIVE_TYPE_I16: &str = "i16";
const PRIMITIVE_TYPE_I32: &str = "i32";
const PRIMITIVE_TYPE_I64: &str = "i64";
const PRIMITIVE_TYPE_U8: &str = "u8";
const PRIMITIVE_TYPE_U16: &str = "u16";
const PRIMITIVE_TYPE_U32: &str = "u32";
const PRIMITIVE_TYPE_U64: &str = "u64";
const PRIMITIVE_TYPE_F32: &str = "f32";
const PRIMITIVE_TYPE_F64: &str = "f64";
const PRIMITIVE_TYPE_STRING: &str = "string";


const INTERNAL_TYPE_SEQUENCE: &str = "Sequence";
const INTERNAL_TYPE_SET: &str = "Set";
const INTERNAL_TYPE_NODE_ID: &str = "NodeId";

const INPUT: &str = "input";
const INTERNAL: &str = "internal";
const OUTPUT: &str = "output";


lazy_static! {
    static ref INTERNAL_S2E: HashMap<&'static str, InternalType> = {
        let mut m = HashMap::new();
        m.insert(INTERNAL_TYPE_SEQUENCE, InternalType::Sequence(Box::new(DataType::DTPrimitiveType(PrimitiveType::I8))));
        m.insert(INTERNAL_TYPE_SET, InternalType::Set(Box::new(DataType::DTPrimitiveType(PrimitiveType::I8))));
        m.insert(INTERNAL_TYPE_NODE_ID, InternalType::NodeId);
        m
    };
}

lazy_static! {
    static ref PRIMITIVE_S2E: HashMap<&'static str, PrimitiveType> = {
        let mut m = HashMap::new();
        m.insert(PRIMITIVE_TYPE_BOOL, PrimitiveType::BOOL);
        m.insert(PRIMITIVE_TYPE_I8, PrimitiveType::I8);
        m.insert(PRIMITIVE_TYPE_I16, PrimitiveType::I16);
        m.insert(PRIMITIVE_TYPE_I32, PrimitiveType::I32);
        m.insert(PRIMITIVE_TYPE_I64, PrimitiveType::I64);
        m.insert(PRIMITIVE_TYPE_U8, PrimitiveType::U8);
        m.insert(PRIMITIVE_TYPE_U16, PrimitiveType::U16);
        m.insert(PRIMITIVE_TYPE_U32, PrimitiveType::U32);
        m.insert(PRIMITIVE_TYPE_U64, PrimitiveType::U64);
        m.insert(PRIMITIVE_TYPE_F32, PrimitiveType::F32);
        m.insert(PRIMITIVE_TYPE_F64, PrimitiveType::F64);
        m.insert(PRIMITIVE_TYPE_STRING, PrimitiveType::STRING);
        m
    };
    static ref PRIMITIVE_E2S: HashMap<PrimitiveType, &'static str> = {
        let mut m = HashMap::new();
        for (k, v) in PRIMITIVE_S2E.iter() {
            m.insert(v.clone(), *k);
        }
        m
    };
}


lazy_static! {
    static ref ACTION_S2E: HashMap<&'static str, ActionType> = {
        let mut m = HashMap::new();
        m.insert(INPUT, ActionType::Input);
        m.insert(INTERNAL, ActionType::Internal);
        m.insert(OUTPUT, ActionType::Output);
        m
    };
}

#[derive(Clone, Copy)]
pub enum ActionType {
    Input,
    Internal,
    Output,
}


#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum PrimitiveType {
    BOOL,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    STRING,
}

#[derive(Clone)]
pub enum InternalType {
    Message(Box<DataType>),
    Sequence(Box<DataType>),
    Set(Box<DataType>),
    NodeId,
}


pub struct DefList {
    pub type_def: Vec<TypeDef>,
}

pub enum TypeDef {
    Struct(StructDef),
    Enum(EnumDef),
    Automaton(AutomatonDef),
}

pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

pub struct StructField {
    pub name: String,
    pub data_type: DataType,
}

pub struct EnumDef {
    pub name: String,
    pub values: Vec<EnumValue>,
}

pub struct EnumValue {
    pub name: String,
    pub payload: Option<DataType>,
}

pub enum ParameterType {
    Message,
    NodeId,
    Others,
}

#[derive(Clone)]
pub enum DataType {
    DTPrimitiveType(PrimitiveType),
    DTInternalType(InternalType),
    DTUserDefineType(
        //* data type name
        String,
        //* data type unknown
        bool,
    ),
}

pub struct AutomatonDef {
    pub name: String,
    pub action_handle: Vec<ActionHandleDef>,
}


pub struct ActionHandleDef {
    pub action_type: ActionType,
    pub name: String,
    pub parameter: Vec<ParameterDef>,
    pub return_value: Option<DataType>,
}

pub struct ParameterDef {
    pub parameter_type: ParameterType,
    pub name: String,
    pub data_type: DataType,
}

pub fn action_str_to_enum(s: &str) -> Res<ActionType> {
    let opt = ACTION_S2E.get(s);
    match opt {
        Some(t) => Ok(t.clone()),
        None => { Err(ET::NoneOption) }
    }
}


impl DataType {
    fn to_rust(&self, ctx: &mut A2RContext) -> Res<String> {
        return match self {
            DataType::DTPrimitiveType(t) => {
                let s = match t {
                    PrimitiveType::STRING => { "String".to_string() }
                    _ => {
                        primitive_type_enum_2_str(t)?
                    }
                };
                Ok(s)
            }
            DataType::DTInternalType(t) => {
                let string = match t {
                    InternalType::Sequence(t) => {
                        ctx.use_vec = true;
                        let s = t.to_rust(ctx)?;
                        format!("Vec<{}>", s).to_string()
                    }
                    InternalType::Set(t) => {
                        ctx.use_set = true;
                        let s = t.to_rust(ctx)?;
                        format!("Vec<{}>", s).to_string()
                    }
                    InternalType::NodeId => {
                        ctx.use_node_id = true;
                        "NodeId".to_string()
                    },
                    InternalType::Message(t) => {
                        let s = t.to_rust(ctx)?;
                        format!("Message<{}>", s).to_string()
                    }
                };
                Ok(string)
            }
            DataType::DTUserDefineType(t, _) => {
                Ok(t.clone())
            }
        }
    }
}

impl EnumValue {
    fn to_rust(&self, ctx: &mut A2RContext) -> Res<String> {
        let opt_s_payload = match &self.payload {
            None => {
                "".to_string()
            }
            Some(t) => {
                let s = t.to_rust(ctx)?;
                format!("({})", s).to_string()
            }
        };

        let mut s = String::new();
        write!(&mut s, "{} {}\n", self.name, opt_s_payload).unwrap();

        Ok(s)
    }
}

impl EnumDef {
    fn to_rust(&self, ctx: &mut A2RContext) -> Res<String> {
        let mut string = derive_str().to_string();
        write!(&mut string, "/// enum {} definition\n", self.name).unwrap();
        write!(&mut string, "enum {} {{\n", self.name).unwrap();
        for x in self.values.iter() {
            let s = x.to_rust(ctx)?;
            write!(&mut string, "{}, \n", s).unwrap();
        }
        write!(&mut string, "}}// enum {} definition end\n\n", self.name).unwrap();

        write!(&mut string, "impl MsgTrait for {} {{}} \n\n ", self.name).unwrap();

        Ok(string)
    }
}


impl StructField {
    fn to_rust(&self, ctx: &mut A2RContext) -> Res<String> {
        let s_data_type = self.data_type.to_rust(ctx)?;
        let s = format!("{}: {}", self.name, s_data_type).to_string();
        Ok(s)
    }
}

impl StructDef {
    fn to_rust(&self, ctx: &mut A2RContext) -> Res<String> {
        let mut string = String::new();

        write!(&mut string, "/// struct {} definition\n", self.name).unwrap();
        write!(&mut string, "{}\n", derive_str().to_string()).unwrap();
        write!(&mut string, "struct {} {{\n", self.name).unwrap();
        for x in self.fields.iter() {
            let s = x.to_rust(ctx)?;
            write!(&mut string, "{},\n", s).unwrap();
        }
        write!(&mut string, "}}// struct {} definition end\n\n", self.name).unwrap();

        write!(&mut string, "impl MsgTrait for {} {{}} \n\n ", self.name).unwrap();
        Ok(string)
    }
}

impl ParameterDef {
    fn to_rust(&self, ctx: &mut A2RContext) -> Res<String> {
        let s_data_type = self.data_type.to_rust(ctx)?;
        let s = format!("{} :{}", self.name, s_data_type).to_string();
        Ok(s)
    }
}

impl ActionHandleDef {
    fn to_rust(&self, ctx: &mut A2RContext) -> Res<String> {
        let automaton_name = ctx.current_action.clone();
        let is_local = self.is_local();
        let mut s = String::new();
        write!(&mut s, "async fn {} (\n", self.name).unwrap();
        write!(&mut s, " &self, \n").unwrap();
        for x in self.parameter.iter() {
            let s_p = x.to_rust(ctx)?;
            s.push_str(format!("{},\n ", s_p).as_str());
        }
        write!(&mut s, ")\n").unwrap();
        match &self.return_value {
            None => {}
            Some(v) => {
                let s_r = v.to_rust(ctx)?;
                write!(&mut s, " -> {}", s_r).unwrap();
            }
        }
        write!(&mut s, "{{\n").unwrap();
        let testing = automaton_name.contains(TEST_STR);
        if !testing {
            match self.action_type {
                ActionType::Input => {
                    let input = Self::gen_action_input(automaton_name.as_str(), is_local)?;
                    write!(&mut s, "{}\n", input).unwrap();
                }
                ActionType::Internal => {
                    let begin = Self::gen_action_internal_begin(automaton_name.as_str(), is_local)?;
                    let end = Self::gen_action_internal_end(automaton_name.as_str(), is_local)?;
                    write!(&mut s, "{}\n todo!();\n {}\n", begin, end).unwrap();
                }
                ActionType::Output => {
                    let output = Self::gen_action_output(automaton_name.as_str(), is_local)?;
                    write!(&mut s, "todo!();\n {}\n", output).unwrap();
                }
            }
        } else {
            let code = r###"
match message {
    _ => { todo!(); }
}"###;
            write!(&mut s, "{}", code).unwrap();
        }
        write!(&mut s, "todo!();\n").unwrap();
        write!(&mut s, "}}\n").unwrap();
        Ok(s)
    }

    fn gen_action_input(automaton_name: &str, is_local: bool) -> Res<String> {
        Self::gen_action("action_input", automaton_name, is_local)
    }

    fn gen_action_output(automaton_name: &str, is_local: bool) -> Res<String> {
        Self::gen_action("action_output", automaton_name, is_local)
    }

    fn gen_action_internal_begin(automaton_name: &str, is_local: bool) -> Res<String> {
        Self::gen_action("action_internal_begin", automaton_name, is_local)
    }

    fn gen_action_internal_end(automaton_name: &str, is_local: bool) -> Res<String> {
        Self::gen_action("action_internal_end", automaton_name, is_local)
    }

    fn gen_action(action_type_name: &str, automaton_name: &str, is_local: bool) -> Res<String> {
        let (from, to) = if is_local {
            (STR_INVALID_NODE_ID, STR_INVALID_NODE_ID)
        } else {
            (STR_PARAMETER_NODE_ID_FROM,
             STR_PARAMETER_NODE_ID_TO)
        };
        let s = format!(r#"{}!(
             {},
             {},
             {},
             {}
        );
        "#, action_type_name, automaton_name,
                        STR_PARAMETER_MESSAGE,
                        from,
                        to
        );
        Ok(s)
    }

    fn is_local(&self) -> bool {
        let mut has_from = false;
        let mut has_to = false;
        for p in self.parameter.iter() {
            if p.name == STR_PARAMETER_NODE_ID_FROM {
                has_from = true;
            } else if p.name == STR_PARAMETER_NODE_ID_TO {
                has_to = true;
            }
        }
        !(has_from && has_to)
    }
}


impl AutomatonDef {
    fn to_rust(&self, ctx: &mut A2RContext) -> Res<String> {
        let mut s = String::new();
        ctx.current_action = camel_to_underline_uppercase(self.name.as_str());
        ctx.const_variable.insert(ctx.current_action.clone(), self.name.clone());
        write!(&mut s, "/// struct {} definition\n", self.name).unwrap();

        write!(&mut s, "struct {} {{\n", self.name).unwrap();
        write!(&mut s, "}}// struct {} definition end\n\n", self.name).unwrap();

        write!(&mut s, "/// {} implement\n", self.name).unwrap();
        write!(&mut s, "impl \n").unwrap();
        write!(&mut s, "{} \n", self.name).unwrap();
        write!(&mut s, "{{\n").unwrap();
        for x in self.action_handle.iter() {
            let s_a = x.to_rust(ctx)?;
            write!(&mut s, "{}\n", s_a).unwrap();
        }
        write!(&mut s, "}}// impl {} end\n\n", self.name).unwrap();

        ctx.current_action.clear();
        Ok(s)
    }
}

pub fn primitive_type_str_to_enum(s: &str) -> Res<PrimitiveType> {
    let opt = PRIMITIVE_S2E.get(s);
    match opt {
        Some(t) => Ok(t.clone()),
        None => { Err(ET::NoneOption) }
    }
}

pub fn primitive_type_enum_2_str(s: &PrimitiveType) -> Res<String> {
    let opt = PRIMITIVE_E2S.get(s);
    match opt {
        Some(t) => Ok(t.to_string()),
        None => { Err(ET::NoneOption) }
    }
}

impl AST2Rust for StructDef {
    fn to_rust_code(&self, ctx: &mut A2RContext) -> Res<String> {
        self.to_rust(ctx)
    }
}

impl AST2Rust for EnumDef {
    fn to_rust_code(&self, ctx: &mut A2RContext) -> Res<String> {
        self.to_rust(ctx)
    }
}

impl AST2Rust for AutomatonDef {
    fn to_rust_code(&self, ctx: &mut A2RContext) -> Res<String> {
        self.to_rust(ctx)
    }
}

impl AST2Rust for TypeDef {
    fn to_rust_code(&self, ctx: &mut A2RContext) -> Res<String> {
        match self {
            TypeDef::Struct(s) => { s.to_rust_code(ctx) }
            TypeDef::Enum(s) => { s.to_rust_code(ctx) }
            TypeDef::Automaton(s) => { s.to_rust_code(ctx) }
        }
    }
}

impl AST2Rust for DefList {
    fn to_rust_code(&self, ctx: &mut A2RContext) -> Res<String> {
        let mut type_def_list = "".to_string();
        for x in self.type_def.iter() {
            let s_ = x.to_rust_code(ctx)?;
            type_def_list.push_str(s_.as_str());
        }
        let use_import = self.gen_import(ctx)?;
        let const_vars = self.gen_const_string_variable(ctx)?;
        let mut ret = String::new();
        write!(&mut ret, "{}\n", use_import).unwrap();
        write!(&mut ret, "{}\n", const_vars).unwrap();
        write!(&mut ret, "{}\n", type_def_list).unwrap();
        Ok(ret)
    }
}

fn derive_str() -> &'static str {
    r#"#[derive(
    Clone,
    Hash,
    PartialEq,
    Eq,
    Debug,
    Serialize,
    Deserialize,
    Decode,
    Encode,
)]"#
}

impl DefList {
    fn gen_import(&self, ctx: &A2RContext) -> Res<String> {
        let mut s = "".to_string();
        s.push_str(r#"
use std::fmt::Debug;
use std::hash::Hash;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use scupt_util::id::XID;
use scupt_util::node_id::{NodeId, INVALID_NODE_ID};
use scupt_util::message::{MsgTrait, Message};
use crate::player::action_internal_begin;
use crate::player::action_internal_end;
use crate::player::action_output;
use crate::player::action_input;
use crate::player::async_begin_action;

"#);

        if ctx.use_set {
            s.push_str("use std::collections::HashSet;\n");
        }
        Ok(s)
    }

    fn gen_const_string_variable(&self, ctx: &A2RContext) -> Res<String> {
        let mut ret = String::new();
        for (v, s) in ctx.const_variable.iter() {
            write!(&mut ret, "const {}:&str = \"{}\";\n", v, s).unwrap();
        }
        Ok(ret)
    }
}


fn camel_to_underline_uppercase(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(c.to_ascii_uppercase());
        } else {
            result.push(c.to_ascii_uppercase());
        }
    }
    result
}

#[test]
fn test_camel_to_under_line() {
    let camel_case = ["thisIsCamelCase", "IsCamelCase", "IsC"];
    for s in camel_case.iter() {
        let underline_uppercase = camel_to_underline_uppercase(s);
        println!("{}", underline_uppercase); // Output: THIS_IS_CAMEL_CASE
    }
}
