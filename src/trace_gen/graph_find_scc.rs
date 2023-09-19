use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

use crate::trace_gen::graph_util::adj_add_new_edge;

// Find the strongly connected components in a directed graph using Tarjan's algorithm
pub fn graph_find_scc<
    T>
(adj: &HashMap<T, Vec<T>>)
 -> Vec<HashMap<T, Vec<T>>>
    where T: Eq + Hash + Clone + Ord + Debug
{
    // Index counter for each vertex
    let mut index = 0;

    // Stack to keep track of visited vertex
    let mut stack = VecDeque::new();

    // Set to keep track of vertex on the stack
    let mut on_stack = HashSet::new();

    // Map to store the index of each vertex
    let mut indices = HashMap::new();

    // Map to store the low link value of each vertex
    let mut low_link = HashMap::new();

    // Vector to store the strongly connected components
    let mut sccs = Vec::new();

    // Iterate over each vertex in the graph
    for v in adj.keys() {
        // If the vertex has not been visited yet,
        // call the strong_connect function to find its strongly connected component
        if !indices.contains_key(v) {
            strong_connect(
                v, adj,
                &mut index, &mut stack,
                &mut on_stack, &mut indices,
                &mut low_link, &mut sccs);
        }
    }

    sccs
}

// Recursive function to find the strongly connected component of a vertex using Tarjan's algorithm
fn strong_connect<T: Eq + Hash + Clone + Ord + Debug>(
    v: &T,
    adj: &HashMap<T, Vec<T>>,
    index: &mut usize,
    stack: &mut VecDeque<T>,
    on_stack: &mut HashSet<T>,
    indices: &mut HashMap<T, usize>,
    low_link: &mut HashMap<T, usize>,
    sccs: &mut Vec<HashMap<T, Vec<T>>>,
) {
    indices.insert(v.clone(), *index);
    low_link.insert(v.clone(), *index);
    *index += 1;
    stack.push_front(v.clone());
    on_stack.insert(v.clone());

    if let Some(neighbors) = adj.get(v) {
        // Iterate over each neighbor of the vertex v
        for neighbor in neighbors {
            // If the neighbor has not been visited yet, call the strong_connect recursively
            if !indices.contains_key(neighbor) {
                strong_connect(neighbor, adj, index, stack, on_stack, indices, low_link, sccs);
                let neighbor_low_link = low_link.get(neighbor).unwrap().clone();
                let node_low_link = low_link.get_mut(v).unwrap();
                *node_low_link = (*node_low_link).min(neighbor_low_link);
            } else if on_stack.contains(neighbor) {
                // If the neighbor is on the stack, update the low link value of the node
                let neighbor_index = indices.get(neighbor).unwrap();
                let node_low_link = low_link.get_mut(v).unwrap();
                *node_low_link = (*node_low_link).min(*neighbor_index);
            }
        }
    }
    // If the low link value of the vertex is equal to its index,
    // it is the root of a strongly connected component
    if low_link[v] == indices[v] {
        // Create a new HashMap to store the strongly connected component
        let mut scc = HashMap::new();
        // Clone a copy of on_stack set.
        let v_set = on_stack.clone();
        loop {
            // Pop nodes off the stack until the current vertex is reached
            let u = stack.pop_front().unwrap();
            // Remove the vertex from the set on the stack
            on_stack.remove(&u);

            // Get the neighbors of the vertex
            let w_neighbors = adj.get(&u).unwrap().clone();
            for n in &w_neighbors {
                if v_set.contains(n) {
                    // Add the node and its neighbors to the strongly connected component
                    adj_add_new_edge(&mut scc, &u, n);
                }
            }
            if !scc.contains_key(&u) {
                scc.insert(u.clone(), vec![]);
            }

            if &u == v {
                break;
            }
        }
        // Add the strongly connected component to the vector of components
        sccs.push(scc);
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::trace_gen::graph_find_scc::graph_find_scc;
    use crate::trace_gen::graph_util::graph_list_equal;

    #[test]
    fn test() {
        // Test case 1
        let mut graph1 = HashMap::new();
        graph1.insert(1, vec![2]);
        graph1.insert(2, vec![3]);
        graph1.insert(3, vec![1]);
        graph1.insert(4, vec![3, 5]);
        graph1.insert(5, vec![4]);

        let sccs1 = graph_find_scc(&graph1);
        println!("Test case 1: {:?}", sccs1);
        let expect1 = {
            let mut s1 = HashMap::new();
            s1.insert(1, vec![2]);
            s1.insert(2, vec![3]);
            s1.insert(3, vec![1]);
            let mut s2 = HashMap::new();
            s2.insert(5, vec![4]);
            s2.insert(4, vec![5]);
            vec![s1, s2]
        };
        assert!(graph_list_equal(&sccs1, &expect1));
        // Test case 2
        let mut graph2 = HashMap::new();
        graph2.insert('A', vec!['B']);
        graph2.insert('B', vec!['C', 'D']);
        graph2.insert('C', vec!['A']);
        graph2.insert('D', vec!['E']);
        graph2.insert('E', vec!['F']);
        graph2.insert('F', vec!['D']);

        let sccs2 = graph_find_scc(&graph2);
        println!("Test case 2: {:?}", sccs2);

        let expect2 = {
            let mut t1 = HashMap::new();
            t1.insert('E', vec!['F']);
            t1.insert('D', vec!['E']);
            t1.insert('F', vec!['D']);
            let mut t2 = HashMap::new();
            t2.insert('A', vec!['B']);
            t2.insert('C', vec!['A']);
            t2.insert('B', vec!['C']);
            vec![t1, t2]
        };
        assert!(graph_list_equal(&sccs2, &expect2));

        let mut graph3 = HashMap::new();

        graph3.insert(4, vec![5, 10]);
        graph3.insert(9, vec![]);
        graph3.insert(12, vec![13]);
        graph3.insert(1, vec![2]);
        graph3.insert(7, vec![8]);
        graph3.insert(11, vec![12]);
        graph3.insert(8, vec![9]);
        graph3.insert(3, vec![4, 7]);
        graph3.insert(6, vec![2]);
        graph3.insert(10, vec![]);
        graph3.insert(13, vec![]);
        graph3.insert(5, vec![3, 6]);
        graph3.insert(2, vec![3]);
        let sccs3 = graph_find_scc(&graph3);
        println!("Test case 3: {:?}", sccs3);

        let expect3 = {
            let t1 = HashMap::from([(1, vec![])]);

            let mut t2 = HashMap::new();
            t2.insert(2, vec![3]);
            t2.insert(5, vec![3, 6]);
            t2.insert(6, vec![2]);
            t2.insert(3, vec![4]);
            t2.insert(4, vec![5]);

            let t7 = HashMap::from([(7, vec![])]);
            let t8 = HashMap::from([(8, vec![])]);
            let t9 = HashMap::from([(9, vec![])]);
            let t10 = HashMap::from([(10, vec![])]);
            let t11 = HashMap::from([(11, vec![])]);
            let t12 = HashMap::from([(12, vec![])]);
            let t13 = HashMap::from([(13, vec![])]);
            vec![t1, t2, t7, t8, t9, t10, t11, t12, t13]
        };
        assert!(graph_list_equal(&sccs3, &expect3));
    }
}