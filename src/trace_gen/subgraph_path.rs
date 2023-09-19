use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// Finds the path from a vertex in a subgraph (g1) to a vertex in another subgraph (g2).
// start: The start vertex in subgraph g1.
// g: A HashMap storing the adjacent relations of the total subgraph.
// g1: A HashMap storing the adjacent relations of the first subgraph.
// g2: A HashMap storing the adjacent relations of the second subgraph.
// Returns a Result containing a vector of vertices representing the path, or an error if no path is found.
pub fn subgraph_from_g1_to_g2_path<V: Eq + Hash + Clone>(
    start: V,
    g: &HashMap<V, Vec<V>>,
    g1: &HashMap<V, Vec<V>>,
    g2: &HashMap<V, Vec<V>>,
) -> Result<(Vec<V> /* path in g1 include start */, V /* vertex in g2 */), String> {
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    // Perform a depth-first search (DFS) to find a path from g1 to g2.
    if _subgraph_g1_to_g2_dfs(&start, g, g1, g2, &mut visited, &mut path) {
        if path.is_empty() {
            panic!("cannot happen");
        }
        let v = path[path.len() - 1].clone();
        path.resize(path.len() - 1, v.clone());
        Ok((path, v))
    } else {
        Err("No path found between the subgraphs.".to_string())
    }
}

// Performs a depth-first search (DFS) to find a path from g1 to g2.
// v: The current vertex.
// g1: A HashMap storing the adjacent relations of the first subgraph.
// g2: A HashMap storing the adjacent relations of the second subgraph.
// visited: A HashSet to keep track of visited vertices.
// path: A vector to store the path.
// Returns true if a path is found, false otherwise.
fn _subgraph_g1_to_g2_dfs<V: Eq + Hash + Clone>(
    v: &V,
    g: &HashMap<V, Vec<V>>,
    g1: &HashMap<V, Vec<V>>,
    g2: &HashMap<V, Vec<V>>,
    visited: &mut HashSet<V>,
    path: &mut Vec<V>,
) -> bool {
    visited.insert(v.clone());
    path.push(v.clone());

    if g2.contains_key(v) {
        return true;
    }
    let opt_vec_u = g.get(v);
    match opt_vec_u {
        Some(vec_u) => {
            for u in vec_u {
                if g2.contains_key(u) {
                    path.push(u.clone());
                    return true;
                }
            }
        }
        None => panic!("not in super graph")
    }

    if let Some(neighbors) = g1.get(v) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                if _subgraph_g1_to_g2_dfs(neighbor, g, g1, g2, visited, path) {
                    return true;
                }
            }
        }
    }

    visited.remove(v);
    path.pop();

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_path_of() {
        let mut g = HashMap::new();
        g.insert(1, vec![2]);
        g.insert(2, vec![3]);
        g.insert(6, vec![7]);
        g.insert(7, vec![]);
        g.insert(5, vec![]);
        g.insert(3, vec![4]);
        g.insert(4, vec![5]);

        let mut g1 = HashMap::new();
        g1.insert(1, vec![2]);
        g1.insert(2, vec![3]);
        g1.insert(6, vec![7]);
        g1.insert(7, vec![]);
        g1.insert(3, vec![]);

        let mut g2 = HashMap::new();
        g2.insert(3, vec![4]);
        g2.insert(4, vec![5]);
        g2.insert(5, vec![]);
        let g3: HashMap<i32, Vec<i32>> = HashMap::new();

        let mut g4 = HashMap::new();
        g4.insert(4, vec![]);

        let result = subgraph_from_g1_to_g2_path(1, &g, &g1, &g2);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, (vec![1, 2], 3));

        let result = subgraph_from_g1_to_g2_path(2, &g, &g1, &g2);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, (vec![2], 3));
        let result = subgraph_from_g1_to_g2_path(6, &g, &g1, &g2);
        assert!(result.is_err());

        let result = subgraph_from_g1_to_g2_path(1, &g, &g1, &g3);
        assert!(result.is_err());

        let result = subgraph_from_g1_to_g2_path(1, &g, &g1, &g4);
        let path = result.unwrap();
        assert_eq!(path, (vec![1, 2, 3], 4));
    }
}
