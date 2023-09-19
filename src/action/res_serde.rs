use scupt_util::error_type::ET;
use scupt_util::res::Res;

pub fn res_serde<T>(result:serde_json::error::Result<T>) -> Res<T> {
    match result {
        Ok(t) => { Ok(t) }
        Err(e) => { Err(ET::SerdeError(e.to_string())) }
    }
}