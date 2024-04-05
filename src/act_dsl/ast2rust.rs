use std::collections::HashMap;

use scupt_util::res::Res;

pub struct A2RContext {
    pub use_vec: bool,
    pub use_set: bool,
    pub use_node_id: bool,
    pub current_action: String,
    pub const_variable: HashMap<String, String>,
}


impl A2RContext {
    pub fn new() -> Self {
        A2RContext {
            use_vec: false,
            use_set: false,
            use_node_id: false,
            current_action: "".to_string(),
            const_variable: Default::default(),
        }
    }
}

pub trait AST2Rust {
    fn to_rust_code(&self, ctx: &mut A2RContext) -> Res<String>;
}

