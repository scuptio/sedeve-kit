#[cfg(test)]
mod test {
    use tracing::info;
    use uuid::Uuid;

    use crate::data::path::_test::_test_data_path;
    use crate::trace::gen_case::{DataInput, gen_case};
    use crate::trace::read_json::tla_constant_mapping;
    use crate::trace::trace_reader::TraceReader;

    #[test]
    fn test_db_to_trace() {
        info!("test_db_to_trace");
        let output_db_path = format!("/tmp/trace_{}.db", Uuid::new_v4().to_string());
        db_to_trace("state.db".to_string(), output_db_path, false, 701);
    }

    #[test]
    fn test_db_to_setup_init_trace() {
        info!("test_db_to_setup_init_trace");
        let output_db_path = format!("/tmp/trace_setup_init_{}.db", Uuid::new_v4().to_string());
        db_to_trace("state.db".to_string(), output_db_path, true, 1830);
    }

    fn db_to_trace(input: String, output: String, setup_initialize: bool, expected_size: usize) {
        let path_json = _test_data_path("map_const.json".to_string());
        let r_dict = tla_constant_mapping(Some(path_json));
        let dict = match r_dict {
            Ok(dict) => { dict }
            Err(e) => { panic!("read from dict json file error: {}", e.to_string()); }
        };

        let state_db_path = _test_data_path(input);
        gen_case(DataInput::StateDB(state_db_path),
                 output.clone(),
                 dict,
                 None,
                 None,
                 setup_initialize,
        ).unwrap();
        let vec = TraceReader::read_trace(output).unwrap();
        assert_eq!(vec.len(), expected_size);
    }
}