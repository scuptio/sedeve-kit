
#[cfg(test)]
mod test {
    use crate::trace_gen::dot2case::dot_to_case;
    use crate::data::path::test::test_data_path;
    use crate::trace_gen::read_json::read_from_dict_json;

    #[test]
    fn test_dot_to_case() {
        let path_json = test_data_path("raft_map_const.json".to_string()).unwrap();
        let r_dict = read_from_dict_json(Some(path_json));
        let dict = match r_dict {
            Ok(dict) => { dict }
            Err(e) => { panic!("read from dict json file error: {}", e.to_string()); }
        };
        let path_dot = test_data_path("toolbox_dump.dot".to_string()).unwrap();
        dot_to_case(path_dot, "/tmp/trace.db".to_string(), false, dict).unwrap();
    }
}