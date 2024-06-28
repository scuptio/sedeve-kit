pub mod _test {
    use std::path::Path;

    use project_root::get_project_root;
    use scupt_util::res_of::res_io;

    pub fn _test_data_path(file_name: String) -> String {
        let mut path_buf = res_io(get_project_root()).unwrap();
        path_buf = path_buf
            .join("src/data")
            .join(file_name);
        let s = path_buf.as_path().to_str().unwrap().to_string();
        if !Path::new(&s).exists() {
            panic!("no exist such file {}", s);
        }
        s
    }

    #[test]
    #[should_panic]
    fn _test_non_exist_data_file() {
        let _s = _test_data_path("file.non_exist_data_file_name".to_string());
    }
}