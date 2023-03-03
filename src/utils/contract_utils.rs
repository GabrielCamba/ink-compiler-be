use crate::utils::constants::CARGO_TOML; // Maybe we can use directly from module
use std::fs::File;
use std::io::prelude::*;

pub fn create_cargo_toml_file(features: &Vec<String>) {
    // TODO Define a tmp path for this file
    let mut cargo_toml_file = File::create("./src/Cargo.toml").unwrap();

    //TODO Check if features exist
    let features_list = features
        .iter()
        .map(|s| format!("\"{}\"", s))
        .collect::<Vec<String>>()
        .join(", ");

    println!("features_list: {}", features_list);

    let cargo_toml_file_contents = CARGO_TOML.replace("features_list", &features_list);

    cargo_toml_file
        .write_all(cargo_toml_file_contents.as_bytes())
        .unwrap();
}

pub fn create_lib_rs_file(code: String) {
    // let mut lib_rs_file = File::create("src/lib.rs").unwrap();
    // let mut lib_rs_file_contents = format!(LIB_RS, code = code);
    // lib_rs_file
    //     .write_all(lib_rs_file_contents.as_bytes())
    //     .unwrap();
}
