use std::path::PathBuf;

use clap::Parser;
use scupt_util::init_logger::logger_setup;

use sedeve_kit::trace_gen::gen_case::{DataInput, gen_case};
use sedeve_kit::trace_gen::read_json;

#[derive(clap::ValueEnum, Clone, Debug)]
enum TestType {
    #[allow(non_camel_case_types)]
    two_phase,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the TLA+ output .dot file
    #[arg(short, long)]
    dot_path: Option<String>,

    /// Path of the state sqlite DB file
    #[arg(short, long)]
    state_db_path: Option<String>,

    /// Type name of the actions
    #[arg(short, long)]
    out_db_path: Option<String>,

    /// Type intermediate database path
    #[arg(short, long)]
    intermediate_db_path: Option<String>,

    /// Path of the json file stores the constant value map
    #[arg(short, long)]
    map_const_path: Option<String>,

    /// Remove the intermediate table that records TLA+ actions and trace paths after generating the trace.
    #[arg(short, long, default_value = "true")]
    remove_intermediate:bool,

    /// Generate setup initialize state trace, default value is false
    #[arg(short='e', long, default_value = "false")]
    setup_initialize_state:bool
}

fn main() {
    logger_setup();
    let args = Args::parse();

    let r_dict = read_json::read_from_dict_json(args.map_const_path);
    let dict = match r_dict {
        Ok(dict) => { dict }
        Err(e) => { panic!("read from dict json file error: {}", e.to_string()); }
    };

    let dump_path: Option<String> = args.dot_path;
    let state_db_path : Option<String>  = args.state_db_path;
    let intermediate_db_path:Option<String> = args.intermediate_db_path;
    let setup_initialize_state:bool = args.setup_initialize_state;
    let path_input = if dump_path.is_some() {
        DataInput::DotFile(dump_path.unwrap())
    } else if state_db_path.is_some() {
        DataInput::StateDB(state_db_path.unwrap())
    } else {
        panic!("no input path");
    };

    let path_buf = PathBuf::from(path_input.path());
    if !path_buf.exists() {
        panic!("no such file {:?}", path_buf);
    }
    let file_name = path_buf.file_name().unwrap().to_str().unwrap().to_string();

    let path_output = match &args.out_db_path {
        Some(path) => { path.clone() }
        None => {
            let output_sqlite_path = match path_buf.parent() {
                None => {
                    panic!("error path parent");
                }
                Some(dir) => { dir.to_path_buf().to_str().unwrap().to_string() }
            };
            let path_buf_sqlite = PathBuf::from(output_sqlite_path);
            let path_buf_sqlite = path_buf_sqlite.join(format!("{}.sqlite", file_name));
            path_buf_sqlite.to_str().unwrap().to_string()
        }
    };

    gen_case(path_input, path_output, dict,
             intermediate_db_path, setup_initialize_state).unwrap();
}