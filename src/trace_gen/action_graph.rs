use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use scupt_util::res::Res;
use tracing::{error, trace};

use crate::trace_gen::graph_find_path::graph_find_path;

pub struct ActionGraph<
    K: Eq + Hash + Clone + Ord + Debug + ToString
> {
    adj: HashMap<K, Vec<K>>,
}

impl <
    K:Eq + Hash + Clone + Ord + Debug + ToString,
>
ActionGraph<K> {
    pub fn new(
        adj: HashMap<K, Vec<K>>,
    ) -> Self {
        Self {
            adj,
        }
    }

    pub fn handle_action<NV, FP>(
        &self,
        fn_new_vertex: &NV,
        fn_handle_path: &FP,
    ) -> Res<()>
        where
            // create a new vertex ID not in set
            NV: Fn(&HashSet<K>) -> K,
            // on finding a path
            FP: Fn(Vec<K>)

    {
        let fn_find_scc = |s: &HashMap<K, Vec<K>>, i: &K| {
            trace!("find scc {:?}, {:?}", i, s);
        };
        let adj = self.adj.clone();
        let fn_find_path = |v: Vec<K>| {
            trace!("find path {:?}", v);
            for i in 0..v.len() {
                if i + 1 < v.len() {
                    let id1 = v[i].clone();
                    let id2 = v[i + 1].clone();
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
                                error!("not adjacent {:?} {:?}", id1, id2);
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

        Ok(())
    }
}
