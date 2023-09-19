#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use project_root::get_project_root;
    use scupt_util::init_logger::logger_setup;

    #[test]
    fn test_to_exec() {
        logger_setup();

        let proj_path = get_project_root().unwrap();
        let parent = proj_path.parent().unwrap();
        let parent_buf = PathBuf::from(parent);
        let input = parent_buf.join("spec/2pc/dump/2pc.sqlite");
        let output = parent_buf.join("spec/2pc/dump/2pc.action.sqlite");
        if input.exists() && !output.exists() {
            //let _ = to_trace(input.to_str().unwrap().to_string(),
            //                  output.to_str().unwrap().to_string(), "two_phase".to_string()).unwrap();
        }
    }
}