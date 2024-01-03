
#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::data::path::test::test_data_path;
    use crate::trace_gen::gen_case::{DataInput, gen_case};
    use crate::trace_gen::read_json::read_from_dict_json;
    use crate::trace_gen::trace_reader::TraceReader;

    #[test]
    fn test_dot_to_case() {
        let path_json = test_data_path("map_const.json".to_string()).unwrap();
        let r_dict = read_from_dict_json(Some(path_json));
        let dict = match r_dict {
            Ok(dict) => { dict }
            Err(e) => { panic!("read from dict json file error: {}", e.to_string()); }
        };
        let output_db_path = format!("/tmp/trace_from_dot_{}.db", Uuid::new_v4().to_string());

        let path_dot = test_data_path("toolbox_dump.dot".to_string()).unwrap();
        gen_case(DataInput::DotFile(path_dot),
                 output_db_path.clone(),
                 dict,
            None,
            false,
        ).unwrap();
        let vec = TraceReader::read_trace(output_db_path).unwrap();
        assert_eq!(vec.len(), 643);
    }
}