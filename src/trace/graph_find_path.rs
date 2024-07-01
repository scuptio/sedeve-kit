use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Instant;

use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Standard};

use tracing::{info, trace};

use crate::trace::cover_walk_path::covering_walk_path;
use crate::trace::graph_find_scc::graph_find_scc;
use crate::trace::graph_util::{adj_add_new_edge, adj_remove_edge};
use crate::trace::subgraph_path::subgraph_from_g1_to_g2_path;

pub fn graph_find_path<V,  FP>(
    adj: &HashMap<V, Vec<V>>,
    fn_find_path: &FP,
)
    where
        V: Eq + Hash + Clone + Ord + Debug,

        FP: Fn(Vec<V>)
{
    if adj.is_empty() {
        return;
    }

    let roots = root_vertex(&adj);
    let mut root_vec: Vec<_> = roots.iter().cloned().collect();
    root_vec.sort();
    let start = Instant::now();

    let _find_path_start_from_root = |vec: &Vec<V>| {
        info!("path : {:?}", vec);
        fn_find_path(vec.clone());
    };

    for root in root_vec.iter() {
        let mut stack = Vec::new();
        let mut on_stack = HashSet::new();
        let mut visited = HashSet::new();
        dfs_find_path(
            adj,
            &_find_path_start_from_root,
            root,
            &mut stack,
            &mut on_stack,
            &mut visited);
    }
    let duration = start.elapsed();
    info!("Time elapsed to find path : {:?}", duration);
}


fn root_vertex<V>(
    adj: &HashMap<V, Vec<V>>) ->  HashSet<V>
    where V: Eq + Hash + Clone + Ord + Debug
{
    let start = Instant::now();
    let out_to_in = reverse_adj(adj);

    // Find the roots of graph
    let mut roots: HashSet<V> = HashSet::new();
    for (u, vec_v) in out_to_in.iter() {
        if vec_v.is_empty() {
            roots.insert(u.clone());
        } else if vec_v.len() == 1 && vec_v[0] == *u {
            roots.insert(u.clone());
        }
    }

    if roots.is_empty() { // if the roots empty
        // choose a root
        let mut set = HashSet::new();
        for  (k, v) in adj {
            set.insert(k.clone());
            for j in v.iter() {
                set.insert(j.clone());
            }
        };
        roots = set;
    }

    trace!("roots: {:?}", roots);
    let duration = start.elapsed();
    info!("Time elapsed to find root vertex : {:?}", duration);

    roots
}


fn dfs_find_path<V, FP>(
    adj: &HashMap<V, Vec<V>>,
    fn_find_path: &FP,
    v: &V,
    stack: &mut Vec<V>,
    on_stack: &mut HashSet<V>,
    visited: &mut HashSet<V>,
) where
    V: Eq + Hash + Clone + Ord + Debug,
    FP: Fn(&Vec<V>) {
    let opt_vec_u = adj.get(v);
    let vec_u = match opt_vec_u {
        None => { return; }
        Some(vec_u) => { vec_u }
    };

    if visited.contains(v) {
        if on_stack.contains(v) && !stack.is_empty() {
            // find a circle
            // fn_find_path(stack);
        }
        return;
    }
    stack.push(v.clone());
    visited.insert(v.clone());
    on_stack.insert(v.clone());
    if vec_u.is_empty() && !stack.is_empty() {
        fn_find_path(stack);
    }
    for u in vec_u {
        dfs_find_path(adj, fn_find_path, u, stack, on_stack, visited);
    }
    let opt_v = stack.pop();
    match opt_v {
        Some(v) => { on_stack.remove(&v); }
        None => {}
    }
}





fn contracted_scc<V, NV, FC>(
    adj: &HashMap<V, Vec<V>>,
    fn_new_vertex: &NV,
    fn_find_non_trivial_scc: &FC,
) -> HashMap<V, Vec<V>>
    where
        V: Eq + Hash + Clone + Ord + Debug,
        NV: Fn(&HashSet<V>) -> V,
        FC: Fn(&HashMap<V, Vec<V>>, &V),
{
    let mut v_set: HashSet<V> = adj.keys().cloned().collect();

    let start = Instant::now();
    let scc_vec = graph_find_scc(adj);
    let duration = start.elapsed();

    info!("Time elapsed to find {} SCC: {:?}", scc_vec.len(), duration);

    let start = Instant::now();
    let mut adj1 = adj.clone();
    let mut all_scc_adj = HashMap::new();
    let mut vertex_scc:HashMap<_, HashSet<_>> = HashMap::new();
    for (_i, scc) in scc_vec.iter().enumerate() {
        if scc.len() > 1 {
            let v_c = fn_new_vertex(&v_set);
            v_set.insert(v_c.clone());
            for (u, vec) in scc.iter() {
                for v in vec.iter() {
                    if !all_scc_adj.contains_key(&(u.clone(), v.clone())) {
                        for e in vec![u.clone(), v.clone()] {
                            let opt_v = vertex_scc.get_mut(&e);
                            match opt_v {
                                Some(_set) => {
                                    _set.insert(v_c.clone());
                                },
                                None => {
                                    vertex_scc.insert(e, HashSet::from_iter(vec![v_c.clone()].into_iter()));
                                }
                            }
                        }
                        all_scc_adj.insert((u.clone(), v.clone()), v_c.clone());
                    } else {
                        panic!("error");
                    }
                }
            }

            fn_find_non_trivial_scc(scc, &v_c);
        }
    }
    let find_in_scc = |u: V, v: V| {
        let opt = all_scc_adj.get(&(u.clone(), v.clone()));
        match opt {
            Some(c) => {
                (vec![c.clone()], true, true)
            }
            None => {
                if vertex_scc.contains_key(&u) && adj.contains_key(&v) {
                    let _vec:Vec<_> = vertex_scc.get(&u).unwrap().iter().cloned().collect();
                    (_vec, true, false)
                } else if adj.contains_key(&u) && vertex_scc.contains_key(&v) {
                    let _vec:Vec<_> = vertex_scc.get(&v).unwrap().iter().cloned().collect();
                    (_vec, false, true)
                } else {
                    (vec![], false, false)
                }
            }
        }
    };
    contracted_one_scc(&mut adj1, find_in_scc);
    let duration = start.elapsed();
    info!("Time elapsed to contract SCC {:?}",  duration);
    adj1
}


fn reverse_adj<V>(
    adj: &HashMap<V, Vec<V>>,
) -> HashMap<V, Vec<V>>
    where
        V: Eq + Hash + Clone + Ord + Debug,
{
    // Initialize the set of vertices with incoming edges
    let mut out_to_in: HashMap<V, Vec<V>> = HashMap::new();
    for (u, edges) in adj.iter() {
        if !out_to_in.contains_key(u) {
            out_to_in.insert(u.clone(), vec![]);
        }
        for v in edges.iter() {
            adj_add_new_edge(&mut out_to_in, v, u);
        }
    }
    out_to_in
}


/// We now define a new weighted directed graph *D'= <V', E'>*
/// in which the cycle C is "contracted" into one node as follows:
fn contracted_one_scc<T: Eq + Hash + Clone + Ord + Debug, F>(
    adj: &mut HashMap<T, Vec<T>>,
    contains_in_scc: F,
) where F: Fn(T, T) -> (Vec<T>, bool, bool) {
    let mut to_add = vec![];
    let mut to_remove = vec![];
    for (u, neighbours) in adj.iter() {
        for v in neighbours {
            let (vec, u_in_c, v_in_c) = contains_in_scc(u.clone(), v.clone());
            for v_c in vec {
                if u_in_c && v_in_c {
                    to_remove.push((u.clone(), v.clone()));
                } else if !u_in_c && v_in_c {
                    // If (u,v) is an edge in E with:
                    //      u \notin C /\ v \in C
                    // (an edge coming into the cycle), then include in E' a new edge:
                    //     e = ( u , v_C),
                    to_add.push((u.clone(), v_c.clone()));
                    //adj_add_new_edge(adj_non_scc, u, v_c);
                } else if u_in_c && !v_in_c {
                    // If (u,v) is an edge in E with:
                    //     u \in C /\ v \notin C
                    // (an edge going away from the cycle), then include in E' a new edge:
                    //     e = (v_C, v),
                    to_add.push((v_c.clone(), v.clone()));
                    //adj_add_new_edge(adj_non_scc, v_c, v);
                } else if !u_in_c && !v_in_c {
                    // If (u,v) is an edge in E with:
                    //     v \notin C
                    // (an edge unrelated to the cycle), then include in E ′ a new edge:
                    //     e=(u,v),
                    // adj_add_new_edge(adj_non_scc, u, v);
                }
            }
        }
    }

    for (u, v) in to_add {
        adj_add_new_edge(adj, &u, &v);
    }

    for (u, v) in to_remove {
        adj_remove_edge(adj, &u, &v);
    }
}


pub fn gen_new_vertex_id<T: Hash + Clone + Eq>(s: &HashSet<T>) -> T
    where Standard: Distribution<T>
{
    let mut rng = thread_rng();
    loop {
        let i: T = rng.gen();
        if !s.contains(&i) {
            return i.clone();
        }
    }
}

fn _find_path(adj: &HashMap<i32, Vec<i32>>) {
    let fn_find_path = |v: Vec<i32>| {
        trace!("find path: {:?}", v);
    };

    graph_find_path(adj, &fn_find_path);
}


#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::sync::atomic::{AtomicI64, Ordering};

    use scupt_util::init_logger::logger_setup;

    use crate::data::path::_test::_test_data_path;

    use super::*;

    fn std_map_to_serde_json_string(map: &HashMap<i64, Vec<i64>>) -> String {
        let json = serde_json::to_string_pretty(map).unwrap();
        return json;
    }

    fn serde_json_string_to_std_map(json: &String) -> HashMap<i64, Vec<i64>> {
        let json: HashMap<i64, Vec<i64>> = serde_json::from_str(json.as_str()).unwrap();
        return json;
    }

    fn gen_adj_map<V>(vec: &Vec<(V, V)>) -> HashMap<V, Vec<V>>
        where V: Eq + Hash + Clone + Ord + Debug,
    {
        let mut m = HashMap::new();
        for (u, v) in vec.iter() {
            adj_add_new_edge(&mut m, u, v);
        }
        m
    }

    // Test code for the edmonds_msa function
    #[test]
    fn test_find_path() {
        logger_setup();

        // Create a directed graph represented by an adjacency list
        let adj_vec: Vec<(i32, i32)> = vec![
            (2, 3), (3, 4), (4, 5), (5, 6), (6, 2), // * circle 2, 3, 4, 5, 6
            (5, 3), // * circle 3, 4, 5
            (1, 2), (3, 7),
            (7, 8), (8, 9),
            (4, 10),
            (11, 12), (12, 13),
        ];
        // Call the edmonds_msa function and print the result
        let adj_map = gen_adj_map(&adj_vec);
        _find_path(&adj_map);
    }

    #[test]
    fn test_contracted_scc() {
        logger_setup();
        let test_case_path = _test_data_path("graph_adj.json".to_string());
        let json_string = read_to_string(test_case_path).unwrap();
        let adj_map = serde_json_string_to_std_map(&json_string);
        let _ = std_map_to_serde_json_string(&adj_map);

        let fn_new_vertex = |s: &HashSet<i64>| -> i64 {
            gen_new_vertex_id(s)
        };
        let n = AtomicI64::from(0i64);

        let fn_find_scc = |_s: &HashMap<i64, Vec<i64>>, _i: &i64| {
            n.fetch_add(1, Ordering::SeqCst);
        };
        contracted_scc(&adj_map, &fn_new_vertex, &fn_find_scc);
        let inst = Instant::now();
        let scc_count = n.load(Ordering::SeqCst);
        info!("Time elapsed to find {} non-trivial scc, {:?},", scc_count, inst.elapsed());
        assert_eq!(scc_count, 32749);
    }
}
