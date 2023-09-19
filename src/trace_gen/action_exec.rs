use std::collections::{HashMap, HashSet};
use std::time::Instant;

use scupt_util::res::Res;
use serde_json::Value;
use tracing::{error, info, trace};

use crate::trace_gen::graph_find_path::{gen_new_vertex_id, graph_find_path};

pub struct ActionNode {
    json_value: Value,
    sub: HashSet<i64>,
}

impl ActionNode {
    pub fn new(value: Value) -> Self {
        Self {
            json_value: value,
            sub: Default::default(),
        }
    }

    pub fn add_sub_link(&mut self, id: i64) {
        let _ = self.sub.insert(id);
    }


    pub fn json_value(&self) -> &Value {
        &self.json_value
    }

}

pub struct ActionGraph {
    node: HashMap<i64, ActionNode>,
    adj: HashMap<i64, Vec<i64>>,
}

impl ActionGraph {
    pub fn new(
        node: HashMap<i64, ActionNode>,
        adj: HashMap<i64, Vec<i64>>,
    ) -> Self {
        Self {
            node,
            adj,
        }
    }

    pub fn handle_action<FP, FV>(
        &self,
        fn_handle_path: &FP,
        fn_handle_json_value: &FV,
    ) -> Res<()>
        where
            // on finding a path
            FP: Fn(Vec<i64>),
            // on finding an action
            FV: Fn(i64, Value),
    {
        let fn_new_vertex = |s: &HashSet<i64>| -> i64 {
            gen_new_vertex_id(s)
        };

        let fn_find_scc = |s: &HashMap<i64, Vec<i64>>, i: &i64| {
            trace!("find scc {}, {:?}", i, s);
        };
        let adj = self.adj.clone();
        let fn_find_path = |v: Vec<i64>| {
            trace!("find path {:?}", v);
            for i in 0..v.len() {
                if i + 1 < v.len() {
                    let id1 = v[i];
                    let id2 = v[i + 1];
                    let opt_v = adj.get(&id1);
                    match opt_v {
                        Some(_v) => {
                            let mut ok = false;
                            for id3 in _v.iter() {
                                if *id3 == id2 {
                                    ok = true;
                                    break;
                                }
                            }
                            if !ok {
                                error!("not adjacent {} {}", id1, id2);
                                panic!("todo");
                            }
                        }
                        None => {
                            panic!("todo");
                        }
                    }
                }
            }
            fn_handle_path(v);
        };
        graph_find_path(&self.adj, &fn_new_vertex, &fn_find_scc, &fn_find_path);

        let start = Instant::now();

        for (id, action) in self.node.iter() {
            fn_handle_json_value(id.clone(), action.json_value().clone());
        }
        let duration = start.elapsed();
        info!("Time elapsed write json value : {:?}", duration);
        Ok(())
    }
}