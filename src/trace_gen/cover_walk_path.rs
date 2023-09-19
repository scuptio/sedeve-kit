use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use tracing::trace;

// Finds a covering walk path that visits each vertex at least once from a start vertex in an SCC.
// start: The start vertex.
// scc: A HashMap storing the adjacent relations of the strongly connected component.
// Returns a Result containing a vector of vertices representing the covering walk path.
pub fn covering_walk_path<V: Eq + Hash + Clone + Debug>(start: V, scc: &HashMap<V, Vec<V>>) -> Result<Vec<V>, String> {
    let mut visited = HashSet::new();
    let mut visited_adj = HashSet::new();
    let mut path = Vec::new();

    // Perform a depth-first search (DFS) to find a Hamiltonian path.
    if _covering_walk_path_dfs(&start, scc, &mut visited, &mut visited_adj, &mut path) {
        Ok(path)
    } else {
        Err("No Hamiltonian path found.".to_string())
    }
}

// Performs a depth-first search (DFS) to find a Hamiltonian path.
// v: The current vertex.
// scc: A HashMap storing the adjacent relations.
// visited: A HashSet to keep track of visited vertices.
// path: A vector to store the path.
// Returns true if a Hamiltonian path is found, false otherwise.
fn _covering_walk_path_dfs<V: Eq + Hash + Clone + Debug>(
    v: &V,
    scc: &HashMap<V, Vec<V>>,
    visited: &mut HashSet<V>,
    visited_adj: &mut HashSet<V>,
    path: &mut Vec<V>,
) -> bool {
    if visited_adj.contains(v) {
        return false;
    }
    visited.insert(v.clone());
    path.push(v.clone());
    trace!("path {:?}", path);
    if path.len() == scc.len() {
        return true;
    }

    if let Some(neighbors) = scc.get(v) {
        for neighbor in neighbors {
            if _covering_walk_path_dfs(neighbor, scc, visited, visited_adj, path) {
                return true;
            }
        }
    }
    visited_adj.insert(v.clone());
    visited.remove(v);
    path.pop();

    false
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use scupt_util::error_type::ET;
    use scupt_util::init_logger::logger_setup;
    use serde_json::Value;

    use super::*;

    type MyResult<T> = scupt_util::res::Res<T>;

    #[test]
    fn test_cover_walk_path() {
        let mut scc = HashMap::new();
        scc.insert(1, vec![2, 3]);
        scc.insert(2, vec![3]);
        scc.insert(3, vec![1]);

        let result = covering_walk_path(1, &scc);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, vec![1, 2, 3]);

        let result = covering_walk_path(2, &scc);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, vec![2, 3, 1]);

        let result = covering_walk_path(3, &scc);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, vec![3, 1, 2]);

        let mut scc_no_path = HashMap::new();
        scc_no_path.insert(1, vec![2]);
        scc_no_path.insert(2, vec![]);
        scc_no_path.insert(3, vec![]);

        let result = covering_walk_path(1, &scc_no_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_cover_walk_path_json_input() {
        let s = r#"
{
   "4799361366534118608":[
      3782300127789364440,
      4799361366534118608
   ],
   "-7237697213297562615":[
      3782300127789364440,
      4799361366534118608
   ],
   "-731630448160727056":[
      -8208137455316722219,
      -3875420861519025207,
      -1315588189585315661,
      -731630448160727056,
      7783565777940814038
   ],
   "-3482636559016195851":[
      -8208137455316722219,
      -1045819237843365156,
      -731630448160727056,
      3996082869560272881
   ],
   "359014574433204938":[
      -3482636559016195851,
      359014574433204938
   ],
   "3782300127789364440":[
      -8548781290055098934,
      -7237697213297562615,
      -1315588189585315661,
      -731630448160727056
   ],
   "-4871777566429119067":[
      -8208137455316722219,
      -3875420861519025207,
      -1315588189585315661,
      -731630448160727056,
      7783565777940814038
   ],
   "-3875420861519025207":[
      -4987242560181999762,
      -4871777566429119067
   ],
   "-1045819237843365156":[
      -8208137455316722219,
      -1045819237843365156,
      -731630448160727056,
      3996082869560272881
   ],
   "3996082869560272881":[
      -3482636559016195851,
      359014574433204938
   ],
   "-4987242560181999762":[
      -4987242560181999762,
      -4871777566429119067
   ],
   "-1315588189585315661":[
      -8548781290055098934,
      -7237697213297562615,
      -1315588189585315661,
      -731630448160727056
   ],
   "-8208137455316722219":[
      -8208137455316722219,
      -1045819237843365156,
      -731630448160727056,
      3996082869560272881
   ],
   "7783565777940814038":[
      -8208137455316722219,
      -3875420861519025207,
      -1315588189585315661,
      -731630448160727056,
      7783565777940814038
   ],
   "-8548781290055098934":[
      -8548781290055098934,
      -7237697213297562615,
      -1315588189585315661,
      -731630448160727056
   ]
}"#;
        logger_setup();
        let hash_map = str_to_hash_map(s).unwrap();
        let start = -731630448160727056;
        covering_walk_path(start, &hash_map).unwrap();
    }

    fn str_to_hash_map(s: &str) -> MyResult<HashMap<i64, Vec<i64>>> {
        let json: Value = serde_json::from_str::<_>(s).unwrap();
        let mut hash_map: HashMap<i64, Vec<i64>> = Default::default();
        match json {
            Value::Object(map) => {
                for (id, v) in map {
                    let i = i64::from_str(id.as_str()).unwrap();
                    let vec: Vec<i64> = serde_json::from_value(v).unwrap();
                    hash_map.insert(i, vec);
                }
            }
            _ => { return Err(ET::TODO) }
        }

        Ok(hash_map)
    }
}
