#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use scupt_util::init_logger;
    use crate::data::path::test::test_data_path;
    use crate::trace_gen::graph_find_path::gen_new_vertex_id;
    use crate::trace_gen::from_parse_dot::parse_dot;

    #[test]
    fn test_parse_dot() {
        init_logger::logger_setup();
        let path = test_data_path("toolbox_dump.dot".to_string()).unwrap();
        let r = parse_dot(path, HashMap::new());
        match r {
            Ok(actions) => {
                let _r = actions.handle_action(
                    &gen_new_vertex_id,
                    &|_v|
                        {

                        },
                    &|_v, _value|
                        {

                        }
                );
            }
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }
}