use std::path::PathBuf;

use clap::Parser;

use sedeve_kit::trace::gen_case::{DataInput, gen_case};
use sedeve_kit::trace::read_json;

#[derive(clap::ValueEnum, Clone, Debug)]
enum TestType {
    #[allow(non_camel_case_types)]
    two_phase,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GenArgs {
    /// Path of the input state/action sqlite DB file
    #[arg(short, long)]
    state_db_path: Option<String>,

    /// Output trace path
    #[arg(short, long)]
    out_trace_db_path: Option<String>,

    /// Type intermediate database path
    #[arg(short, long)]
    intermediate_db_path: Option<String>,

    /// Path of the json file stores the constant value map
    #[arg(short, long)]
    map_const_path: Option<String>,

    /// Remove the intermediate table that records TLA+ actions and trace paths after generating the trace.
    #[arg(short, long, default_value = "true")]
    remove_intermediate: bool,

    /// Generate setup initialize state trace, default value is false
    #[arg(short = 'e', long, default_value = "false")]
    setup_initialize_state: bool,

    #[arg(short = 'c', long)]
    /// SQLite cache size in MBs.
    sqlite_cache_size: Option<u64>,
}


pub fn portal(args: GenArgs) {
    let r_dict = read_json::read_from_dict_json(args.map_const_path);
    let dict = match r_dict {
        Ok(dict) => { dict }
        Err(e) => { panic!("read from dict json file error: {}", e.to_string()); }
    };

    let state_db_path: Option<String> = args.state_db_path;
    let intermediate_db_path: Option<String> = args.intermediate_db_path;
    let setup_initialize_state: bool = args.setup_initialize_state;
    let sqlite_cache_size = args.sqlite_cache_size;
    let path_input = if state_db_path.is_some() {
        DataInput::StateDB(state_db_path.unwrap())
    } else {
        panic!("no input path");
    };

    let path_buf = PathBuf::from(path_input.path());
    if !path_buf.exists() {
        panic!("no such file {:?}", path_buf);
    }
    let file_name = path_buf.file_name().unwrap().to_str().unwrap().to_string();

    let path_output = match &args.out_trace_db_path {
        Some(path) => { path.clone() }
        None => {
            let output_sqlite_path = match path_buf.parent() {
                None => {
                    panic!("error path parent");
                }
                Some(dir) => { dir.to_path_buf().to_str().unwrap().to_string() }
            };
            let path_buf_sqlite = PathBuf::from(output_sqlite_path);
            let path_buf_sqlite = path_buf_sqlite.join(format!("{}.trace.db", file_name));
            path_buf_sqlite.to_str().unwrap().to_string()
        }
    };

    gen_case(
        path_input,
        path_output,
        dict,
        intermediate_db_path,
        sqlite_cache_size,
        setup_initialize_state,
    ).unwrap();
}

#[cfg(test)]
mod test {
    use std::fs;

    use tracing::info;
    use uuid::Uuid;

    use sedeve_kit::data::path::_test::_test_data_path;

    use crate::trace_gen_portal::{GenArgs, portal};

    #[test]
    fn test_trace_portal_1() {
        info!("test_trace_portal_1");
        let output_db_path = format!("/tmp/trace_{}.db", Uuid::new_v4().to_string());
        let args = GenArgs {
            state_db_path: Some(_test_data_path("state.db".to_string())),
            out_trace_db_path: Some(output_db_path),
            intermediate_db_path: None,
            map_const_path: Some(_test_data_path("map_const.json".to_string())),
            remove_intermediate: false,
            setup_initialize_state: false,
            sqlite_cache_size: None,
        };
        portal(args);
    }

    #[test]
    fn test_trace_portal_2() {
        info!("test_trace_portal_2");
        let db_path = format!("/tmp/state_{}.db", Uuid::new_v4().to_string());
        fs::copy(_test_data_path("state.db".to_string()), db_path.clone()).unwrap();
        let args = GenArgs {
            state_db_path: Some(db_path),
            out_trace_db_path: None,
            intermediate_db_path: None,
            map_const_path: Some(_test_data_path("map_const.json".to_string())),
            remove_intermediate: false,
            setup_initialize_state: false,
            sqlite_cache_size: None,
        };
        portal(args);
    }
}