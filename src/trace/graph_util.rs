use std::collections::HashMap;
use std::hash::Hash;

fn _exclude_non_exist_vertex<T>(
    g1: &HashMap<T, Vec<T>>,
    vec1: &Vec<T>,
) -> Vec<T>
    where T: Eq + Hash + Clone + Ord {
    let mut v = vec![];
    for n1 in vec1 {
        if g1.contains_key(n1) {
            match v.binary_search(n1) {
                Ok(_) => {} // element already in vector @ `pos`
                Err(pos) => v.insert(pos, n1.clone()),
            }
        }
    }
    v
}


#[cfg(test)]
pub fn graph_equal<T>(
    g1: &HashMap<T, Vec<T>>,
    g2: &HashMap<T, Vec<T>>,
) -> bool
    where T: Eq + Hash + Clone + Ord {
    if g1.len() != g2.len() {
        return false;
    }
    for (u, n1_vec) in g1 {
        let opt = g2.get(u);
        match opt {
            Some(n2_vec) => {
                let u1_vec = _exclude_non_exist_vertex(g1, n1_vec);
                let u2_vec = _exclude_non_exist_vertex(g2, n2_vec);
                if !u1_vec.eq(&u2_vec) {
                    return false;
                }
            }
            None => { return false; }
        }
    }
    return true;
}

#[cfg(test)]
pub fn graph_list_equal<T>(
    g1: &Vec<HashMap<T, Vec<T>>>,
    g2: &Vec<HashMap<T, Vec<T>>>,
) -> bool
    where T: Eq + Hash + Clone + Ord
{
    if g1.len() != g2.len() {
        return false;
    }
    let mut gg1 = g1.clone();
    let mut gg2 = g2.clone();
    let compare_map = |a: &HashMap<T, Vec<T>>, b: &HashMap<T, Vec<T>>| {
        let aa: Vec<T> = a.keys().into_iter().cloned().collect();
        let bb: Vec<T> = b.keys().into_iter().cloned().collect();
        return aa.cmp(&bb);
    };
    gg1.sort_by(compare_map);
    gg2.sort_by(compare_map);
    for (i, g1) in gg1.iter().enumerate() {
        let g2 = &gg2[i];
        if !graph_equal(g1, g2) {
            return false;
        }
    }
    return true;
}

pub fn adj_add_new_edge<T: Eq + Hash + Clone + Ord>(
    adj: &mut HashMap<T, Vec<T>>,
    u: &T, v: &T) {
    if !adj.contains_key(v) {
        adj.insert(v.clone(), vec![]);
    }
    let opt = adj.get_mut(&u);
    match opt {
        Some(vec) => {
            match vec.binary_search(v) {
                Ok(_i) => {}
                Err(i) => { vec.insert(i, v.clone()); }
            }
        }
        None => {
            adj.insert(u.clone(), vec![v.clone()]);
        }
    }
}


pub fn adj_remove_edge<T: Eq + Hash + Clone + Ord>(
    adj: &mut HashMap<T, Vec<T>>,
    u: &T, v: &T) {

    let opt = adj.get_mut(&u);
    match opt {
        Some(vec) => {
            match vec.binary_search(v) {
                Ok(i) => {
                    vec.remove(i);
                }
                Err(_) => {}
            }
        }
        None => {}
    }
}
