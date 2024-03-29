extern crate cbindgen;

use std::env;
use std::path::PathBuf;
use cbindgen::Config;


fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(&crate_dir)
        .join("out/include/_sedeve_kit.h".to_string())
        .display()
        .to_string();

    let config:Config = Config::default();
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(&output_file);
}


