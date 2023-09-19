

#[cfg(test)]
pub mod test {
    use scupt_util::res::Res;
    use scupt_util::res_of::res_io;
    use project_root::get_project_root;
    use log::error;
    use scupt_util::error_type::ET;

    pub fn test_data_path(file_name: String) -> Res<String> {
        let mut path_buf = res_io(get_project_root())?;
        path_buf = path_buf
            .join("src/data")
            .join(file_name);
        let s = match path_buf.as_path().to_str() {
            Some(s) => s.to_string(),
            None => {
                error!("build path error");
                return Err(ET::NoneOption);
            }
        };
        Ok(s)
    }
}