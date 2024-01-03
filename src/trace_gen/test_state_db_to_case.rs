
#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::data::path::test::test_data_path;
    use crate::trace_gen::gen_case::{DataInput, gen_case};
    use crate::trace_gen::read_json::read_from_dict_json;
    use crate::trace_gen::trace_reader::TraceReader;

    #[test]
    fn test_db_to_trace() {
        let output_db_path = format!("/tmp/trace_{}.db", Uuid::new_v4().to_string());
        db_to_trace("state.db".to_string(), output_db_path, false, 701);
    }

    #[test]
    fn test_db_to_setup_init_trace() {
        let output_db_path = format!("/tmp/trace_setup_init_{}.db", Uuid::new_v4().to_string());
        db_to_trace("state.db".to_string(), output_db_path, true, 1830);
    }

    fn db_to_trace(input:String, output:String, setup_initialize:bool, expected_size:usize) {
        let path_json = test_data_path("map_const.json".to_string()).unwrap();
        let r_dict = read_from_dict_json(Some(path_json));
        let dict = match r_dict {
            Ok(dict) => { dict }
            Err(e) => { panic!("read from dict json file error: {}", e.to_string()); }
        };

        let state_db_path = test_data_path(input).unwrap();
        gen_case(DataInput::StateDB(state_db_path),
                 output.clone(),
                 dict,
                 None,
                 setup_initialize
        ).unwrap();
        let vec = TraceReader::read_trace(output).unwrap();
        assert_eq!(vec.len(), expected_size);
    }
}