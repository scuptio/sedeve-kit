use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Instant;

use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Standard};
use scupt_util::error_type::ET;
use scupt_util::res::Res;
use tracing::{info, trace};

use crate::trace_gen::cover_walk_path::covering_walk_path;
use crate::trace_gen::graph_find_scc::graph_find_scc;
use crate::trace_gen::graph_util::{adj_add_new_edge, adj_remove_edge};
use crate::trace_gen::subgraph_path::subgraph_from_g1_to_g2_path;

pub fn graph_find_path<V, NV, FC, FP>(
    adj: &HashMap<V, Vec<V>>,
    fn_new_vertex: &NV,
    find_scc: &FC,
    fn_find_path: &FP,
)
    where
        V: Eq + Hash + Clone + Ord + Debug,
        NV: Fn(&HashSet<V>) -> V,
        FC: Fn(&HashMap<V, Vec<V>>, &V),
        FP: Fn(Vec<V>)
{
    if adj.is_empty() {
        return;
    }

    let start = Instant::now();
    let out_to_in = reverse_adj(&adj);

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
        panic!("not root");
    }
    trace!("roots: {:?}", roots);
    let duration = start.elapsed();
    info!("Time elapsed to find root vertex : {:?}", duration);

    // virtual vertex -> SCC adj
    let vv_2_scc_map = RefCell::new(HashMap::new());
    // vertex of SCC  -> virtual vertex
    let vscc_2_vv_map = RefCell::new(HashMap::new());

    let fn_find_scc = |scc: &HashMap<V, Vec<V>>, vv: &V| {
        let mut map = vv_2_scc_map.borrow_mut();
        map.insert(vv.clone(), scc.clone());

        let mut map = vscc_2_vv_map.borrow_mut();
        for (v, _) in scc.iter() {
            map.insert(v.clone(), vv.clone());
        }

        find_scc(scc, vv);
    };

    let adj_non_scc = contracted_scc(
        &adj, fn_new_vertex,
        &fn_find_scc);

    let start = Instant::now();

    let _find_path_start_from_root = |vec: &Vec<V>| {
        let _scc_map = vv_2_scc_map.borrow();
        let path_vec = _replace_subgraph_scc_path(adj, vec, &roots, &_scc_map);
        assert!(path_vec.len() > 0 && roots.contains(&path_vec[0]));
        fn_find_path(path_vec);
    };

    for root in roots.iter() {
        let mut stack = Vec::new();
        let mut on_stack = HashSet::new();
        let mut visited = HashSet::new();
        let v = if adj.contains_key(root) && !adj_non_scc.contains_key(root) {
            let map = vscc_2_vv_map.borrow();
            let opt = map.get(root);
            match opt {
                Some(v) => { v.clone() }
                None => { panic!("error, not possible, vertex of SCC to virtual vertex map must contain root vertex"); }
            }
        } else {
            root.clone()
        };
        dfs_find_path(
            &adj_non_scc,
            &_find_path_start_from_root,
            &v,
            &mut stack,
            &mut on_stack,
            &mut visited);
    }
    let duration = start.elapsed();
    info!("Time elapsed to find path : {:?}", duration);
}

fn path_link_subgraph<V>(
    start: V,
    g: &HashMap<V, Vec<V>>,
    g1: &HashMap<V, Vec<V>>,
    g2: &HashMap<V, Vec<V>>,
) -> Res<(Vec<V>, Option<V>)>
    where V: Eq + Hash + Clone + Ord + Debug
{
    let result = covering_walk_path(start, g1);
    let mut path1 = match result {
        Ok(path) => { path }
        Err(_) => { return Err(ET::TODO); }
    };
    let v1 = match path1.last() {
        Some(v) => { v.clone() }
        None => { panic!("cannot happen"); }
    };
    trace!("covering_walk_path {:?}", path1);

    let r2 = subgraph_from_g1_to_g2_path(v1, g, g1, g2);
    let (path2, v2) = match r2 {
        Ok((p, v)) => {
            assert!(g2.contains_key(&v));
            assert!(!p.is_empty());
            if p.len() > 0 && path1.len() > 0 {
                if path1[path1.len() - 1] == p[0] {
                    let _ = path1.pop();
                }
            }
            (p, Some(v))
        }
        Err(_) => { (vec![], None) }
    };
    let mut path = path1;
    trace!("g2_to_g2_path {:?}", path2);
    path.extend(path2);

    Ok((path, v2))
}

fn _replace_subgraph_scc_path<V>(
    adj: &HashMap<V, Vec<V>>,
    path_vec: &Vec<V>,
    root: &HashSet<V>,
    scc_map: &HashMap<V, HashMap<V, Vec<V>>>,
) -> Vec<V>
    where V: Eq + Hash + Clone + Ord + Debug,
{
    let mut ret: Vec<V> = vec![];
    let mut opt_start: Option<V> = None;
    for i in 0..path_vec.len() {
        let v1 = path_vec[i].clone();
        let mut adj1 = HashMap::new();
        let mut adj2 = HashMap::new();
        let opt = scc_map.get(&v1);
        let (g1, v) = match opt {
            Some(m) => {
                let v = match &opt_start {
                    Some(s) => { s.clone() }
                    None => {
                        let mut opt_v = None;
                        for (k, _) in m.iter() {
                            if root.contains(k) {
                                opt_v = Some(k.clone());
                            }
                        };
                        opt_v.unwrap()
                    }
                };
                (m, v)
            }
            None => {
                adj1.insert(v1.clone(), vec![]);
                (&adj1, v1.clone())
            }
        };
        let g2 = if i + 1 < path_vec.len() {
            let v2 = path_vec[i + 1].clone();
            let opt = scc_map.get(&v2);
            let g2 = match opt {
                Some(m) => {
                    m
                }
                None => {
                    adj2.insert(v2, vec![]);
                    &adj2
                }
            };
            g2
        } else {
            &adj2
        };
        trace!("start {:?} g1:{:?} g2:{:?}",v,  g1, g2);
        let (path, opt_next_start) = path_link_subgraph(v, adj, g1, g2).unwrap();
        opt_start = opt_next_start;
        trace!("path {:?}, next {:?}" , path, opt_start);
        ret.extend(path);
    }
    trace!("after replace scc {:?}", ret);
    ret
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
        None => { return }
        Some(vec_u) => { vec_u }
    };

    if visited.contains(v) {
        if on_stack.contains(v) && !stack.is_empty() {
            // find a circle
            fn_find_path(stack);
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


struct SCCContext<V> {
    v: V,
    adj:HashMap<V, Vec<V>>,
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
    for (_i, scc) in scc_vec.iter().enumerate() {
        if scc.len() > 1 {
            let v_c = fn_new_vertex(&v_set);
            v_set.insert(v_c.clone());
            let _c = SCCContext {
                        v: v_c.clone(),
                        adj: scc.clone(),
                    };
            let scc_context = Arc::new(_c);
            for (u,  vec) in scc.iter() {
                for v in vec.iter() {
                    if !all_scc_adj.contains_key(&(u.clone(), v.clone())) {
                        all_scc_adj.insert((u.clone(), v.clone()), scc_context.clone());
                    } else {
                        panic!("error");
                    }
                }
            }

            fn_find_non_trivial_scc(scc, &v_c);
        }
    }
    let find_in_scc = |u : V, v : V| {
        let opt = all_scc_adj.get(&(u.clone(), v.clone()));
        match opt {
            Some(c) => {
                Some((c.v.clone(), c.adj.contains_key(&u), c.adj.contains_key(&v)))
            }
            None => { None }
        }
    };
    contracted_one_scc( &mut adj1, find_in_scc);
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
    contains_in_scc: F
) where F: Fn(T, T) -> Option<(T, bool, bool)> {
    let mut to_add = vec![];
    let mut to_remove = vec![];
    for (u, neighbours) in adj.iter() {
        for v in neighbours {
            let opt = contains_in_scc(u.clone(), v.clone());
            match opt {
                Some((v_c, u_in_c, v_in_c)) => {
                    if !u_in_c && v_in_c {
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
                    } else {
                        to_remove.push((u.clone(), v.clone()));
                    }
                }
                None => {}
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



fn _find_path_in_scc_begin_to_end<V>(
    scc: &HashMap<V, Vec<V>>,
    opt_begin: &Option<V>,
    opt_end: &Option<V>,
) -> Vec<V>
    where V: Eq + Hash + Clone + Ord + Debug
{
    trace!("_find_path_in_scc_begin_to_end : begin:{:?} end:{:?}, scc:{:?}", opt_begin, opt_end, scc);
    if scc.is_empty() {
        return vec![];
    }
    let start_v = match opt_begin {
        Some(b) => { b.clone() }
        None => {
            scc.iter().next().unwrap().0.clone()
        }
    };
    if !scc.contains_key(&start_v) {
        panic!("not contains key {:?} in map, {:?}", start_v, scc)
    }
    let mut stack = Vec::new();
    let mut on_stack = HashSet::new();
    let mut visited = HashSet::new();
    let path = RefCell::new(Vec::new());
    let find_path = |vec: &Vec<V>| {
        trace!("path in scc:{:?}", vec);
        let mut p = path.borrow_mut();
        p.push(vec.clone());
    };
    dfs_find_path(scc, &find_path, &start_v, &mut stack, &mut on_stack, &mut visited);
    {
        let p = path.borrow();
        let end_v = match opt_end {
            None => {
                let mut opt: Option<Vec<V>> = None;
                for _p in p.iter() {
                    match &opt {
                        Some(vec) => {
                            if _p.len() > vec.len() {
                                opt = Some(_p.clone())
                            }
                        }
                        _ => {
                            opt = Some((*_p).clone())
                        }
                    }
                }
                return opt.unwrap();
            }
            Some(end_v) => {
                end_v.clone()
            }
        };

        for _path in p.iter() {
            if _path.is_empty() {
                continue;
            }
            if !scc.contains_key(&end_v) {
                panic!("not contains key {:?} in map {:?}", end_v, scc)
            }
            for (_i, _v) in _path.iter().enumerate() {
                if *_v == end_v {
                    let _vec = _path[0..=_i].to_vec().clone();
                    return _vec;
                }
            }
        }
    }
    panic!("cannot find path");
}


pub fn gen_new_vertex_id<T:Hash + Clone + Eq>(s:&HashSet<T>) -> T
    where Standard: Distribution<T>
{
    let mut rng = thread_rng();
    loop {
        let i: T = rng.gen();
        if !s.contains(&i) {
            return i.clone()
        }
    }
}

fn _find_path(adj: &HashMap<i32, Vec<i32>>) {
    let new_vertex = |s: &HashSet<i32>| -> i32 {
        gen_new_vertex_id(s)
    };
    let fn_find_scc = |m: &HashMap<i32, Vec<i32>>, v_c: &i32| {
        trace!("find scc: {} {:?}", v_c, m);
    };

    let fn_find_path = |v: Vec<i32>| {
        trace!("find path: {:?}", v);
    };

    graph_find_path(adj,
                    &new_vertex,
                    &fn_find_scc,
                    &fn_find_path,
    );
}







#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::sync::atomic::{AtomicI64, Ordering};

    use scupt_util::init_logger::logger_setup;

    use crate::data::path::test::test_data_path;

    use super::*;

    fn std_map_to_serde_json_string(map: &HashMap<i64, Vec<i64>>) -> String {
        let json = serde_json::to_string_pretty(map).unwrap();
        return json
    }

    fn serde_json_string_to_std_map(json:&String) -> HashMap<i64, Vec<i64>> {
        let json: HashMap<i64, Vec<i64>> = serde_json::from_str(json.as_str()).unwrap();
        return json
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
        let test_case_path = test_data_path("graph_adj.json".to_string()).unwrap();
        let json_string = read_to_string(test_case_path).unwrap();
        let adj_map = serde_json_string_to_std_map(&json_string);
        let _ = std_map_to_serde_json_string(&adj_map);

        let fn_new_vertex = |s: &HashSet<i64>| -> i64 {
            gen_new_vertex_id(s)
        };
        let n = AtomicI64::from(0i64);

        let fn_find_scc = |_s: &HashMap<i64, Vec<i64>>, _i: &i64|   {
            n.fetch_add(1, Ordering::SeqCst);
        };
        contracted_scc(&adj_map, &fn_new_vertex, &fn_find_scc);
        let inst = Instant::now();
        let scc_count = n.load(Ordering::SeqCst);
        info!("Time elapsed to find {} non-trivial scc, {:?},", scc_count, inst.elapsed());
        assert_eq!(scc_count, 32749);
    }
}
