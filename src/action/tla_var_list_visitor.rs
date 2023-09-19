use std::string::String;

use scupt_util::error_type::ET;
use scupt_util::res::Res;
use scupt_util::res_of::{res_option, res_parse};
use substring::Substring;
use tree_sitter::Node;

use crate::action::tla_ast;

pub struct TLAVarListVisitor {
    text: String,
}

impl TLAVarListVisitor {
    pub fn new(text: String) -> Self {
        Self {
            text,
        }
    }

    pub fn visit_root(&self, node: Node) -> Res<tla_ast::TLAVariableList> {
        self.visit_tla_conjunction_variable_value_list(node)
    }

    fn visit_tla_conjunction_variable_value_list(&self, node: Node) -> Res<tla_ast::TLAVariableList> {
        let mut cursor = node.walk();
        let mut ok = cursor.goto_first_child();
        let mut vec = vec![];
        while ok {
            if cursor.node().kind() == "tla_conjunction_variable_value" {
                let expr = self.visit_tla_conjunction_variable_value(cursor.node())?;
                vec.push(expr);
            }
            ok = cursor.goto_next_sibling();
        }

        Ok(tla_ast::TLAVariableList {
            vec,
        })
    }

    fn visit_tla_conjunction_variable_value(&self, node: Node) -> Res<tla_ast::TLAVarExpr> {
        let var_expr = match node.child_by_field_name("tla_variable_value") {
            Some(n) => { self.visit_tla_variable_val_expr(n) }
            None => { return Err(ET::ParseError(format!("no tla variable value field {},", self.node_context(node)?).to_string())); }
        };
        return var_expr;
    }

    fn visit_tla_variable(&self, node: Node) -> Res<String> {
        self.node_context(node)
    }

    fn visit_tla_variable_val_expr(&self, node: Node) -> Res<tla_ast::TLAVarExpr> {
        let left = node.child_by_field_name("tla_variable_value_left");
        let id = match left {
            Some(n) => { self.visit_tla_variable(n)? }
            None => { return Err(ET::ParseError("no left identifier".to_string())); }
        };
        let right = node.child_by_field_name("tla_variable_value_right");
        let tla_expr = match right {
            Some(n) => {
                self.visit_tla_value_expr(n)?
            }
            None => { return Err(ET::ParseError("no right identifier".to_string())); }
        };
        Ok(tla_ast::TLAVarExpr {
            name: id,
            value: tla_expr,
        })
    }

    fn visit_tla_value_expr(&self, node: Node) -> Res<tla_ast::TLAValue> {
        let opt_n = node.child(0);
        let n = res_option(opt_n)?;
        let value = match n.kind() {
            "tla_number" => { self.visit_tla_number(n)? }
            "tla_string" => { self.visit_tla_string(n)? }
            "tla_record" => { self.visit_tla_record(n)? }
            "tla_sequence" => { self.visit_tla_sequence(n)? }
            "tla_finite_set" => { self.visit_tla_finite_set(n)? }
            "tla_function" => { self.visit_tla_function(n)? }
            "tla_variable" => { tla_ast::TLAValue::TVStringLiteral(self.visit_tla_variable(n)?) }
            "tla_boolean" => { self.visit_tla_boolean(n)? }
            _ => { return Err(ET::ParseError(format!("unknown kind value {}", n.kind_id()).to_string())); }
        };
        Ok(value)
    }

    fn visit_tla_boolean(&self, node: Node) -> Res<tla_ast::TLAValue> {
        let n = res_option(node.child(0))?;
        let k = n.kind();
        let v = if k.eq("TRUE") {
            tla_ast::TLAValue::TVBoolean(true)
        } else if k.eq("FALSE") {
            tla_ast::TLAValue::TVBoolean(false)
        } else {
            panic!("error kind {} of boolean value", k)
        };
        Ok(v)
    }

    fn visit_tla_number(&self, node: Node) -> Res<tla_ast::TLAValue> {
        let opt_n = node.child(0);
        let n = match opt_n {
            Some(n) => { n }
            None => { return Err(ET::ParseError("no child node".to_string())); }
        };

        let value = match n.kind() {
            "tla_nat_number" => { self.visit_tla_int_number(n)? }
            "tla_real_number" => { self.visit_tla_float_number(n)? }
            "tla_binary_number" => { self.visit_tla_int_number(n)? }
            "tla_octal_number" => { self.visit_tla_int_number(n)? }
            "tla_hex_number" => { self.visit_tla_int_number(n)? }
            _ => { return Err(ET::ParseError("unknown kind value".to_string())); }
        };
        Ok(tla_ast::TLAValue::TVNumberLiteral(value))
    }

    fn visit_tla_int_number(&self, node: Node) -> Res<tla_ast::TLANumber> {
        let s = self.node_context(node)?;
        let r = s.parse::<i64>();
        let i = res_parse(r)?;
        Ok(tla_ast::TLANumber::TNInteger(i))
    }

    fn visit_tla_float_number(&self, node: Node) -> Res<tla_ast::TLANumber> {
        let s = self.node_context(node)?;
        let r = s.parse::<f64>();
        let i = res_parse(r)?;
        Ok(tla_ast::TLANumber::TNFloat(i))
    }

    fn visit_tla_string(&self, node: Node) -> Res<tla_ast::TLAValue> {
        let string_with_quota = self.node_context(node.clone())?;
        let s = if string_with_quota.starts_with('"') && string_with_quota.ends_with('"') {
            string_with_quota.substring(1, string_with_quota.len() - 1).to_string()
        } else {
            panic!("not possible");
        };
        Ok(tla_ast::TLAValue::TVStringLiteral(s))
    }

    fn visit_tla_record(&self, node: Node) -> Res<tla_ast::TLAValue> {
        let opt_n = node.child_by_field_name("tla_record_item_comma_list");
        let n_c = match opt_n {
            Some(n) => { n }
            None => { return Ok(tla_ast::TLAValue::TVSequence(vec![])); }
        };
        let vec = self.visit_tla_record_field_list(n_c)?;
        Ok(tla_ast::TLAValue::TVRecord(vec))
    }

    fn visit_tla_value_list(&self, node: Node) -> Res<Vec<tla_ast::TLAValue>> {
        let size = node.child_count();
        let mut vec = vec![];
        for i in 0..size {
            let c = node.child(i).unwrap();
            if c.kind() == "tla_value_expr" {
                let v = self.visit_tla_value_expr(c)?;
                vec.push(v);
            }
        }
        Ok(vec)
    }

    fn visit_tla_record_field_list(&self, node: Node) -> Res<Vec<(String, tla_ast::TLAValue)>> {
        let size = node.child_count();
        let mut vec = vec![];
        for i in 0..size {
            let c = node.child(i).unwrap();
            if c.kind() == "tla_record_field" {
                let v = self.visit_tla_record_field(c)?;
                vec.push(v);
            }
        }
        Ok(vec)
    }

    fn visit_tla_mapping_list(&self, node: Node) -> Res<Vec<(tla_ast::TLAValue, tla_ast::TLAValue)>> {
        let size = node.child_count();
        let mut vec = vec![];
        for i in 0..size {
            let c = node.child(i).unwrap();
            if c.kind() == "tla_mapping" {
                let v = self.visit_tla_mapping(c)?;
                vec.push(v);
            }
        }
        Ok(vec)
    }

    fn visit_tla_sequence(&self, node: Node) -> Res<tla_ast::TLAValue> {
        let opt_n = node.child_by_field_name("tla_sequence_item_comma_list");
        let n_c = match opt_n {
            Some(n) => { n }
            None => { return Ok(tla_ast::TLAValue::TVSequence(vec![])); }
        };
        let vec = self.visit_tla_value_list(n_c)?;
        return Ok(tla_ast::TLAValue::TVSequence(vec));
    }

    fn visit_tla_finite_set(&self, node: Node) -> Res<tla_ast::TLAValue> {
        let opt_n = node.child_by_field_name("tla_finite_set_item_comma_list");
        let n_c = match opt_n {
            Some(n) => { n }
            None => { return Ok(tla_ast::TLAValue::TVSet(vec![])); }
        };
        let vec = self.visit_tla_value_list(n_c)?;
        Ok(tla_ast::TLAValue::TVSet(vec))
    }

    fn visit_tla_function(&self, node: Node) -> Res<tla_ast::TLAValue> {
        let opt_n = node.child_by_field_name("tla_function_comma_list");
        let n_c = match opt_n {
            Some(n) => { n }
            None => { return Ok(tla_ast::TLAValue::TVMap(vec![])); }
        };
        let vec = self.visit_tla_mapping_list(n_c)?;
        Ok(tla_ast::TLAValue::TVMap(vec))
    }

    fn visit_tla_record_field(&self, node: Node) -> Res<(String, tla_ast::TLAValue)> {
        let opt_l = node.child_by_field_name("tla_record_field_left");
        let l = match opt_l {
            None => { return Err(ET::ParseError("no such field".to_string())); }
            Some(n) => { n }
        };

        let l_expr = match l.kind() {
            "tla_variable" => { self.visit_tla_variable(l)? }
            _ => { panic!("error kind"); }
        };

        let opt_r = node.child_by_field_name("tla_record_field_right");
        let r = match opt_r {
            None => { return Err(ET::ParseError("no such field".to_string())); }
            Some(n) => { n }
        };
        let r_expr = self.visit_tla_value_expr(r)?;
        Ok((l_expr, r_expr))
    }


    fn visit_tla_mapping(&self, node: Node) -> Res<(tla_ast::TLAValue, tla_ast::TLAValue)> {
        let opt_l = node.child_by_field_name("tla_record_field_left");
        let l = match opt_l {
            None => { return Err(ET::ParseError("no such field".to_string())); }
            Some(n) => { n }
        };

        let l_expr = match l.kind() {
            "tla_value_expr" => { self.visit_tla_value_expr(l)? }
            _ => { panic!("error kind"); }
        };

        let opt_r = node.child_by_field_name("tla_record_field_right");
        let r = match opt_r {
            None => { return Err(ET::ParseError("no such field".to_string())); }
            Some(n) => { n }
        };
        let r_expr = self.visit_tla_value_expr(r)?;
        Ok((l_expr, r_expr))
    }

    fn node_context(&self, node: Node) -> Res<String> {
        let range = node.range();
        let r = String::from_utf8(
            self.text.as_bytes()[range.start_byte..range.end_byte].to_vec()
        );
        match r {
            Ok(s) => Ok(s),
            Err(e) => Err(ET::TSParseError(e.to_string()))
        }
    }
}
