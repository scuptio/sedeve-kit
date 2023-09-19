use std::path::{Path, PathBuf};

use sedeve_kit::rust_gen::action2rust::action_to_rust;
use clap::Parser;

/// action definition to Rust code template
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// .action or .tla+ file path
    #[arg(short, long)]
    action_path: String,

    /// output rust code file path
    #[arg(short, long)]
    output_path: Option<String>,
}


fn main() {
    let args = Args::parse();
    let action_path: String = args.action_path.clone();
    let path = Path::new(action_path.as_str());
    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
    let output_rs_path = match args.output_path {
        Some(t) => { t.clone() }
        None => {
            let mut pb = PathBuf::from("./");
            pb.push(format!("{}.rs", name));
            pb.to_str().unwrap().to_string()
        }
    };
    action_to_rust(action_path, output_rs_path).unwrap();
}