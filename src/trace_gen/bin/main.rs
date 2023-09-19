use std::path::PathBuf;
use clap::Parser;
use scupt_util::init_logger::logger_setup;
use sedeve_kit::trace_gen::dot2case::dot_to_case;
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
    dot_path: String,

    /// Type name of the actions
    #[arg(short, long)]
    out_db_path: Option<String>,

    /// Path of the json file stores the constant value map
    #[arg(short, long)]
    map_const_path: Option<String>,

    /// Remove the intermediate table that records TLA+ actions and trace paths after generating the trace.
    #[arg(short, long, default_value = "true")]
    remove_intermediate:bool
}

fn main() {
    logger_setup();
    let args = Args::parse();

    let r_dict = read_json::read_from_dict_json(args.map_const_path);
    let dict = match r_dict {
        Ok(dict) => { dict }
        Err(e) => { panic!("read from dict json file error: {}", e.to_string()); }
    };

    let dump_path: String = args.dot_path;


    let path_buf_dot = PathBuf::from(dump_path.clone());
    if !path_buf_dot.exists() {
        panic!("no such dot file {:?}", path_buf_dot);
    }
    let file_name = path_buf_dot.file_name().unwrap().to_str().unwrap().to_string();

    let path = match &args.out_db_path {
        Some(path) => { path.clone() }
        None => {
            let output_sqlite_path = match path_buf_dot.parent() {
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

    dot_to_case(dump_path, path, args.remove_intermediate, dict).unwrap();
}