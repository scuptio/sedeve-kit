use std::cell::RefCell;
use std::collections::HashMap;
use scupt_util::res::Res;
use crate::trace::action_graph::ActionGraph;


#[allow(dead_code)]
fn build_graph(adj:&HashMap<i64, Vec<i64>>) -> Res<Vec<Vec<i64>>> {
    let adj = adj.clone();
    let graph = ActionGraph::new(adj.clone());
    let path = RefCell::from(vec![]);
    let handle_path = |v| {
        let mut r = path.borrow_mut();
        r.push(v);
    };
    graph.build_path(&handle_path)?;
    Ok(path.into_inner())
}

#[cfg(feature = "for-fuzz")]
pub mod fuzz {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use crate::data::path::_test::test_data_to_json;
    use crate::trace::fuzz_graph::build_graph;

    pub fn fuzz_graph(adj : &HashMap<i64, HashSet<i64>>) {
        let mut adj_1 = adj.iter().map(|(k, v)|
            { (k.clone(), v.iter().cloned().collect() ) }).collect();
        let r = build_graph(&adj_1);
        assert!(r.is_ok());
        test_data_to_json("/tmp/graph_find_path/", &adj_1, &r.unwrap(), true);
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use scupt_util::logger::logger_setup_with_console;
    use crate::data::path::_test::test_data_from_json;
    use crate::trace::fuzz_graph::build_graph;

    fn test_graph_find(path:&str) {
        let test_data = test_data_from_json::<HashMap<i64, Vec<i64>>, Vec<Vec<i64>>>(
            path.to_string());
        for (input, result, _p) in test_data.iter() {
            let _adj = input.clone();

            let mut path = build_graph(&_adj).unwrap();
            let mut _expected = result.clone();
            path.sort();
            _expected.sort();
            assert_eq!(path, _expected);
        }
    }

    #[test]
    fn test_graph_find_path_fuzz() {
        //test_graph_find("graph_find_path_fuzz");
    }

    #[test]
    fn test_graph_find_path_1() {
        logger_setup_with_console("debug");
        test_graph_find("graph_find_path_1");
    }
}