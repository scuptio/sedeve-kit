use rust_format::{Formatter, RustFmt};
use scupt_util::error_type::ET;
use scupt_util::res::Res;
use scupt_util::res_of::res_option;
use tracing::trace;
use tree_sitter::Node;
use tree_sitter::Tree;

use crate::rust_gen::action_parser::ActionParser;
use crate::rust_gen::node_text::node_text;
use crate::rust_gen::tla_value_ast_node::{
    action_str_to_enum,
    ActionHandleDef, ActionType,
    AutomatonDef, DataType, DefList,
    EnumDef, EnumValue, InternalType, ParameterDef,
    ParameterType, primitive_type_str_to_enum,
    StructDef, StructField, TypeDef};

pub struct ActionVisitor {
    text: String,
}

impl ActionVisitor {
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }

    pub fn parse(&mut self) -> Res<DefList> {
        let mut parser = ActionParser::new();
        let t = parser.parse(&self.text)?;
        self.visit(t)
    }

    fn visit(&mut self, tree: Tree) -> Res<DefList> {
        let node = tree.root_node();
        self.visit_root(node)
    }

    pub fn format_code(code: &String) -> Res<String> {
        let s_c = RustFmt::default().format_str(code.as_str()).unwrap();
        Ok(s_c)
    }


    fn visit_root(&mut self, node: Node) -> Res<DefList> {
        let mut vec = vec![];
        let n = node.child_count();
        for i in 0..n {
            let opt_child = node.child(i);
            match opt_child {
                Some(c) => {
                    trace!("{} {}", c.kind() , node_text(&self.text, &c).unwrap());
                    let def = self.visit_definition_list(c)?;
                    vec.push(def);
                }
                None => {}
            }
        }
        let l = DefList {
            type_def: vec
        };
        Ok(l)
    }

    fn visit_definition_list(&mut self, node: Node) -> Res<TypeDef> {
        let opt_n = node.child_by_field_name("automaton_definition");
        match opt_n {
            Some(n) => {
                let automata = self.visit_automation_definition(n)?;
                return Ok(TypeDef::Automaton(automata));
            }
            None => {}
        }
        let opt_n = node.child_by_field_name("struct_definition");
        match opt_n {
            Some(n) => {
                let struct_def = self.visit_struct_definition(n)?;
                return Ok(TypeDef::Struct(struct_def));
            }
            None => {}
        }

        let opt_n = node.child_by_field_name("enum_definition");
        match opt_n {
            Some(n) => {
                let enum_def = self.visit_enum_definition(n)?;
                return Ok(TypeDef::Enum(enum_def));
            }
            None => {}
        }
        trace!("visit_definition_list: {} {}", node.kind() , node_text(&self.text, &node).unwrap());
        Err(ET::NoneOption)
    }

    fn visit_struct_definition(&self, node: Node) -> Res<StructDef> {
        let opt_n = node.child_by_field_name("struct_name");
        let n = res_option(opt_n)?;
        let name = self.visit_name(n)?;
        let opt_nl = node.child_by_field_name("struct_field_list");
        let nl = res_option(opt_nl)?;
        let fields = self.visit_struct_field_list(nl)?;
        let def = StructDef {
            name,
            fields,
        };
        Ok(def)
    }

    fn visit_struct_field_list(&self, node: Node) -> Res<Vec<StructField>> {
        let n = node.child_count();
        let mut vec = vec![];
        for i in 0..n {
            let c = node.child(i).unwrap();
            if c.kind() == "struct_field" {
                let f = self.visit_struct_field(c)?;
                vec.push(f);
            }
        }
        Ok(vec)
    }

    fn visit_struct_field(&self, node: Node) -> Res<StructField> {
        let opt_n = node.child_by_field_name("field_name");
        let n = res_option(opt_n)?;
        let name = self.visit_name(n)?;
        let opt_n = node.child_by_field_name("data_type");
        let n = res_option(opt_n)?;
        let data_type = self.visit_data_type(n)?;
        let field = StructField {
            name,
            data_type,
        };
        Ok(field)
    }

    fn visit_enum_definition(&self, node: Node) -> Res<EnumDef> {
        let opt_n = node.child_by_field_name("enum_name");
        let n = res_option(opt_n)?;
        let name = self.visit_name(n)?;
        let opt_n = node.child_by_field_name("enum_value_definition_list");
        let n = res_option(opt_n)?;
        let values = self.visit_enum_value_definition_list(n)?;
        let def = EnumDef {
            name,
            values,
        };
        Ok(def)
    }

    fn visit_enum_value_definition_list(&self, node: Node) -> Res<Vec<EnumValue>> {
        let n = node.child_count();
        let mut vec = vec![];
        for i in 0..n {
            let c = node.child(i).unwrap();
            if c.kind() == "enum_value_definition" {
                let v = self.visit_enum_value_definition(c)?;
                vec.push(v)
            }
        }
        Ok(vec)
    }

    fn visit_enum_value_definition(&self, node: Node) -> Res<EnumValue> {
        let opt_n = node.child_by_field_name("enum_value_name");
        let n = res_option(opt_n)?;
        let name = self.visit_name(n)?;
        let opt_n = node.child_by_field_name("enum_payload");
        let payload = match opt_n {
            Some(n) => {
                let n_dt = n.child_by_field_name("data_type").unwrap();
                let data_type = self.visit_data_type(n_dt)?;
                Some(data_type)
            }
            None => {
                None
            }
        };
        let v = EnumValue {
            name,
            payload,
        };
        Ok(v)
    }

    fn visit_automation_definition(&self, node: Node) -> Res<AutomatonDef> {
        let opt_ad = node.child_by_field_name("automation_name");
        let name = node_text(&self.text, &res_option(opt_ad)?)?;
        let opt_td = node.child_by_field_name("action_definition_list");
        let action_handle = match opt_td {
            Some(l) => { self.visit_action_definition_list(l)? }
            None => { vec![] }
        };
        let def = AutomatonDef {
            name,
            action_handle,
        };
        Ok(def)
    }

    fn visit_action_definition_list(&self, node: Node) -> Res<Vec<ActionHandleDef>> {
        let n = node.child_count();
        let mut vec = vec![];
        for i in 0..n {
            let c = node.child(i).unwrap();
            let def = self.visit_action_definition(c)?;
            vec.push(def);
        }
        Ok(vec)
    }

    fn visit_action_definition(&self, node: Node) -> Res<ActionHandleDef> {
        let opt_action = node.child_by_field_name("action_type");
        let n_action = res_option(opt_action)?;
        let action_type = self.visit_action_type(n_action)?;
        let opt_name = node.child_by_field_name("action_handle_name");
        let name = res_option(opt_name)?;
        let action_handle_name = self.visit_action_handle_name(name)?;


        let opt_list = node.child_by_field_name("parameter_list");
        let list = res_option(opt_list)?;
        let parameter_list = self.visit_parameter_list(list)?;


        let opt_return = node.child_by_field_name("return_value");
        let opt_return_value = match opt_return {
            Some(c) => {
                let return_type = self.visit_return_value(c)?;
                Some(return_type)
            }
            None => {
                None
            }
        };
        let action_handle = ActionHandleDef {
            action_type,
            name: action_handle_name,
            parameter: parameter_list,
            return_value: opt_return_value,
        };

        Ok(action_handle)
    }

    fn visit_action_type(&self, node: Node) -> Res<ActionType> {
        let n = node_text(&self.text, &node).unwrap();
        action_str_to_enum(n.as_str())
    }

    fn visit_return_value(&self, node: Node) -> Res<DataType> {
        let opt_n = node.child_by_field_name("data_type");
        let n = res_option(opt_n)?;
        self.visit_data_type(n)
    }

    fn visit_action_handle_name(&self, node: Node) -> Res<String> {
        node_text(&self.text, &node)
    }

    fn visit_parameter_list(&self, node: Node) -> Res<Vec<ParameterDef>> {
        let mut param_list = vec![];
        let opt_msg = node.child_by_field_name("message_parameter");
        let msg = res_option(opt_msg)?;
        {
            let p = self.visit_message_parameter(msg)?;
            param_list.push(p);
        }

        let opt_from = node.child_by_field_name("from_parameter");
        match opt_from {
            Some(from) => {
                let p = self.visit_node_parameter(from)?;
                param_list.push(p);
            }
            None => {}
        }
        let opt_to = node.child_by_field_name("to_parameter");
        match opt_to {
            Some(to) => {
                let p = self.visit_node_parameter(to)?;
                param_list.push(p);
            }
            None => {}
        }

        let opt_others = node.child_by_field_name("other_parameter_list");
        match opt_others {
            Some(other) => {
                let n = other.child_count();
                for i in 0..n {
                    let c = other.child(i).unwrap();
                    if c.kind() == "parameter" {
                        let p = self.visit_parameter(c)?;
                        param_list.push(p);
                    }
                }
            }
            None => {}
        }
        Ok(param_list)
    }

    fn visit_parameter(&self, node: Node) -> Res<ParameterDef> {
        let opt_node_param_name = node.child_by_field_name("parameter_name");
        let node_param_name = res_option(opt_node_param_name)?;
        let name = self.visit_name(node_param_name)?;

        let opt_node_param_type = node.child_by_field_name("parameter_type");
        let node_param_type = res_option(opt_node_param_type)?;
        let data_type = self.visit_data_type(node_param_type)?;
        let def = ParameterDef {
            parameter_type: ParameterType::Others,
            name,
            data_type,
        };
        Ok(def)
    }

    fn visit_message_parameter(&self, node: Node) -> Res<ParameterDef> {
        let opt_param_name = node.child_by_field_name("parameter_name");
        let param_name = res_option(opt_param_name)?;
        let name = self.visit_name(param_name)?;
        let opt_node_param_type = node.child_by_field_name("user_define_type");
        let node_param_type = res_option(opt_node_param_type)?;
        let data_type = self.visit_user_define_type(node_param_type)?;
        let def = ParameterDef {
            parameter_type: ParameterType::Message,
            name,
            data_type,
        };
        Ok(def)
    }

    fn visit_node_parameter(&self, node: Node) -> Res<ParameterDef> {
        let opt_param_name = node.child_by_field_name("parameter_name");
        let param_name = res_option(opt_param_name)?;
        let name = self.visit_name(param_name)?;
        let def = ParameterDef {
            parameter_type: ParameterType::NodeId,
            name,
            data_type: DataType::DTInternalType(InternalType::NodeId),
        };
        Ok(def)
    }

    fn visit_name(&self, node: Node) -> Res<String> {
        node_text(&self.text, &node)
    }

    fn visit_data_type(&self, node: Node) -> Res<DataType> {
        let mut opt = node.child_by_field_name("primitive_type");
        match opt {
            Some(c) => {
                return self.visit_primitive_type(c);
            }
            None => {}
        }

        opt = node.child_by_field_name("user_define_type");
        match opt {
            Some(c) => {
                return self.visit_user_define_type(c);
            }
            None => {}
        }

        opt = node.child_by_field_name("internal_type");
        match opt {
            Some(c) => {
                return self.visit_internal_type(c);
            }
            None => {}
        }
        Err(ET::NoneOption)
    }

    fn visit_primitive_type(&self, node: Node) -> Res<DataType> {
        let s = node_text(&self.text, &node)?;
        let primitive_type = primitive_type_str_to_enum(s.as_str())?;
        Ok(DataType::DTPrimitiveType(primitive_type))
    }

    fn visit_user_define_type(&self, node: Node) -> Res<DataType> {
        let opt_unknown = node.child_by_field_name("data_type_unknown");
        let opt_data_type_name = node.child_by_field_name("data_type_name");
        let data_type_name = res_option(opt_data_type_name)?;
        let name = self.visit_name(data_type_name)?;
        Ok(DataType::DTUserDefineType(name, opt_unknown.is_none()))
    }

    fn visit_internal_type(&self, node: Node) -> Res<DataType> {
        let opt = node.child_by_field_name("data_type_node_id");
        match opt {
            Some(_c) => {
                return Ok(DataType::DTInternalType(InternalType::NodeId));
            }
            None => {}
        }

        let opt = node.child_by_field_name("data_type_sequence");
        match opt {
            Some(c) => {
                let internal_type = self.visit_sequence(c)?;
                return Ok(DataType::DTInternalType(internal_type));
            }
            None => {}
        }

        let opt = node.child_by_field_name("data_type_set");
        match opt {
            Some(c) => {
                let internal_type = self.visit_set(c)?;
                return Ok(DataType::DTInternalType(internal_type));
            }
            None => {}
        }

        let opt = node.child_by_field_name("data_type_message");
        match opt {
            Some(c) => {
                let internal_type = self.visit_message(c)?;
                return Ok(DataType::DTInternalType(internal_type));
            }
            None => {}
        }
        Err(ET::NoneOption)
    }

    fn visit_sequence(&self, node: Node) -> Res<InternalType> {
        let opt_c = node.child_by_field_name("data_type");
        let c = res_option(opt_c)?;
        let data_type = self.visit_data_type(c)?;
        Ok(InternalType::Sequence(Box::new(data_type)))
    }

    fn visit_set(&self, node: Node) -> Res<InternalType> {
        let opt_c = node.child_by_field_name("data_type");
        let c = res_option(opt_c)?;
        let data_type = self.visit_data_type(c)?;
        Ok(InternalType::Set(Box::new(data_type)))
    }

    fn visit_message(&self, node: Node) -> Res<InternalType> {
        let opt_c = node.child_by_field_name("data_type");
        let c = res_option(opt_c)?;
        let data_type = self.visit_data_type(c)?;
        Ok(InternalType::Message(Box::new(data_type)))
    }
}