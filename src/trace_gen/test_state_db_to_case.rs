
#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::data::path::test::test_data_path;
    use crate::trace_gen::gen_case::{DataInput, gen_case};
    use crate::trace_gen::read_json::read_from_dict_json;
    use crate::trace_gen::trace_reader::TraceReader;

    #[test]
    fn test_state_db_to_case() {
        let path_json = test_data_path("raft_map_const.json".to_string()).unwrap();
        let r_dict = read_from_dict_json(Some(path_json));
        let dict = match r_dict {
            Ok(dict) => { dict }
            Err(e) => { panic!("read from dict json file error: {}", e.to_string()); }
        };
        let output_db_path = format!("/tmp/trace_from_state_{}.db", Uuid::new_v4().to_string());

        let state_db_path = test_data_path("state.db".to_string()).unwrap();
        gen_case(DataInput::StateDB(state_db_path),
                 output_db_path.clone(),
                 dict,
            None
        ).unwrap();
        let vec = TraceReader::read_trace(output_db_path).unwrap();
        assert_eq!(vec.len(), 701);
    }
}