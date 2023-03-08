use crate::models::contract_model::{WizardMessage};
use crate::utils::constants::{ALLOWED_FEATURES, CARGO_TOML}; // Maybe we can use directly from module
use std::env;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn create_files(wizard_message: &WizardMessage) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let tmp_path_string = format!("/tmp/{}", wizard_message.address);

    let current_dir = env::current_dir().unwrap();
    let tmp_dir_path = current_dir.join(tmp_path_string);
    create_dir(tmp_dir_path.clone())?;

    create_cargo_toml_file(&wizard_message.features, &tmp_dir_path)?;
    create_lib_rs_file(&wizard_message.code, &tmp_dir_path)?;
    Ok(tmp_dir_path)
}

pub fn delete_files(dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Delete tmp folder
    std::fs::remove_dir_all(dir_path)?;
    Ok(())
}

fn create_cargo_toml_file(
    features: &Vec<String>,
    dir_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if sent features are allowed
    let features_list = check_features(features)?;

    // Replace features_list in CARGO_TOML with features received
    let cargo_toml_file_contents = CARGO_TOML.replace("features_list", &features_list);

    let path = dir_path.join("Cargo.toml");
    let mut cargo_toml_file = File::create(path)?;
    cargo_toml_file.write_all(cargo_toml_file_contents.as_bytes())?;

    Ok(())
}

fn create_lib_rs_file(code: &String, dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path = dir_path.join("lib.rs");
    let mut lib_rs_file = File::create(path)?;
    lib_rs_file.write_all(code.as_bytes())?;
    Ok(())
}

fn check_features(features: &Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
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

    Ok(features_list)
}
