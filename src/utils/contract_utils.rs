use crate::utils::constants::{ALLOWED_FEATURES, CARGO_TOML}; // Maybe we can use directly from module
use std::fs::File;
use std::io::prelude::*;

pub fn create_cargo_toml_file(features: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Check if sent features are allowed
    for feature in features {
        if !ALLOWED_FEATURES.contains(&feature.as_str()) {
            return Err("Feature not allowed".into());
        }
    }

    let features_list = features
        .iter()
        .map(|s| format!("\"{}\"", s))
        .collect::<Vec<String>>()
        .join(", ");

    let cargo_toml_file_contents = CARGO_TOML.replace("features_list", &features_list);

    // TODO Define a tmp path for this file
    let mut cargo_toml_file = File::create("./src/Cargo.toml")?;
    cargo_toml_file.write_all(cargo_toml_file_contents.as_bytes())?;

    Ok(())
}

pub fn create_lib_rs_file(code: &String) {
    let mut lib_rs_file = File::create("src/lib.rs").unwrap();
    lib_rs_file.write_all(code.as_bytes()).unwrap();
}
