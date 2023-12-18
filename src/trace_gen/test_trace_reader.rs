
#[cfg(test)]
mod tests {
    use scupt_util::init_logger::logger_setup;

    use crate::data::path::test::test_data_path;
    use crate::trace_gen::trace_reader::TraceReader;

    #[test]
    fn test_trace_reader() {
        logger_setup();
        let sqlite_path = test_data_path("trace.db".to_string()).unwrap();
        let vec = TraceReader::read_trace(sqlite_path).unwrap();
        let vec_len = vec.len();
        let mut n = 0;
        for a in vec.iter() {
            while let Ok(_s) = a.next() {
                n+=1;
            }
        }
        assert_eq!(vec_len, 701);
        assert_eq!(n, 26012);
    }
}