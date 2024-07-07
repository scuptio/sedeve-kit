pub mod _test {
    use std::fs;
    use std::fs::read_to_string;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    use project_root::get_project_root;
    use scupt_util::res_of::res_io;
    use serde::Serialize;
    use md5;
    use md5::Digest;
    use serde::de::DeserializeOwned;

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


    pub fn test_data_to_json<Input: Serialize, Result: Serialize>(
        path: &str,
        input: &Input,
        result: &Result,
        enable: bool,
    ) {
        if enable {
            let mut path_buf = PathBuf::from_str(path).unwrap();
            let _s = (input, result);
            let json = serde_json::to_string_pretty(&_s).unwrap();
            let mut md5 = md5::Md5::default();

            md5.update(json.as_bytes());
            let result = md5.finalize().to_vec();

            if !path_buf.exists() {
                fs::create_dir_all(path_buf.clone()).unwrap();
            }
            path_buf.push(format!("{}.json", hex::encode(result)));
            fs::write(path_buf, json).unwrap();
        }
    }

    pub fn test_data_from_json<
        Input: DeserializeOwned,
        Result: DeserializeOwned>(path: String) -> Vec<(Input, Result, String)> {
        let path = _test_data_path(path);
        let mut vec = vec![];
        let paths = fs::read_dir(path).unwrap();
        for entry in paths {
            let path = entry.unwrap().path();
            let json = read_to_string(path.clone()).unwrap();
            let r: serde_json::Result<(Input, Result)> = serde_json::from_str(json.as_str());
            let (input, result) = match r {
                Err(_t) => {
                    panic!("error serde, may be need to update the data json {:?}, file path {}",
                           json, path.to_str().unwrap());
                }
                Ok(r) => { r }
            };
            vec.push((input, result, path.to_str().unwrap().to_string()));
        }
        vec
    }
}
