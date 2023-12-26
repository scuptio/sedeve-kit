#[cfg(test)]
mod tests {
    use scupt_util::init_logger;

    use crate::data::path::test::test_data_path;
    use crate::trace_gen::action_from_parse_dot::parse_dot;
    use crate::trace_gen::read_json::read_from_dict_json;

    #[test]
    fn test_parse_dot() {
        init_logger::logger_setup();
        let path_json = test_data_path("map_const.json".to_string()).unwrap();
        let dict = read_from_dict_json(Some(path_json)).unwrap();
        let path = test_data_path("toolbox_dump.dot".to_string()).unwrap();
        let r = parse_dot(path, dict);
        match r {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }
}