use crate::models::api_models::WizardMessage;
use crate::models::db_models::Contract;
use crate::utils::constants::CARGO_TOML; // Maybe we can use directly from module
use log::{debug, error, info};
use sha2::{Digest, Sha256};
use std::env;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn compile_contract(
    cargo_path: &String,
    dir_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!(target: "compiler",
        "Entered utils compile_contract with params {:?} and: {:?}",
        cargo_path, dir_path
    );
    let mut binding = Command::new(cargo_path);
    let compiler_cmd = binding
        .arg("contract")
        .arg("build")
        .arg("--release")
        .arg("--quiet")
        .current_dir(dir_path);

    let status = compiler_cmd.status()?;
    if !status.success() {
        error!(target: "compiler", "Compilation failed");
        return Err("Compilation failed".into());
    }
    info!(target: "compiler", "Compilation success");

    Ok(())
}

pub fn get_contract_data(
    dir_path: &Path,
    code_id: &String,
) -> Result<Contract, Box<dyn std::error::Error>> {
    debug!(target: "compiler",
        "Entered utils get_contract_data with params {:?} and: {:?}",
        dir_path, code_id
    );
    let mut wasm_file = File::open(dir_path.join("target/ink/compiled_contract.wasm"))?;
    let mut wasm = Vec::new();
    wasm_file.read_to_end(&mut wasm)?;

    let mut metadata_file = File::open(dir_path.join("target/ink/compiled_contract.json"))?;
    let mut metadata = String::new();
    metadata_file.read_to_string(&mut metadata)?;

    let contract = Contract {
        id: None,
        code_id: code_id.to_owned(),
        metadata,
        wasm,
    };
    info!(target: "compiler", "get_contract_data success");

    Ok(contract)
}

pub fn create_files(wizard_message: &WizardMessage) -> Result<PathBuf, Box<dyn std::error::Error>> {
    debug!(target: "compiler",
        "Entered create_files with wizard_message: {:?}",
        &wizard_message
    );
    let tmp_path_string = format!("/tmp/{}", wizard_message.address);

    let current_dir = env::current_dir().unwrap();
    let tmp_dir_path = current_dir.join(tmp_path_string);
    create_dir(tmp_dir_path.clone())?;

    let res_file_1 = create_cargo_toml_file(&wizard_message.features, &tmp_dir_path);
    if res_file_1.is_err() {
        error!(target: "compiler", "Error creating Cargo.toml file: {:?}", res_file_1);
        delete_files(&tmp_dir_path);
        return Err("Error creating Cargo.toml file".into());
    }

    let res_file_2 = create_lib_rs_file(&wizard_message.code, &tmp_dir_path);
    if res_file_2.is_err() {
        error!(target: "compiler", "Error creating lib.rs file: {:?}", res_file_2);
        delete_files(&tmp_dir_path);
        return Err("Error creating lib.rs file".into());
    }
    info!(target: "compiler", "create_files success");

    Ok(tmp_dir_path)
}

pub fn delete_files(dir_path: &Path) {
    debug!(target: "compiler", "Entered delete_files with dir_path: {:?}", dir_path);
    // Delete tmp folder
    let res = std::fs::remove_dir_all(dir_path);
    if res.is_err() {
        error!(target: "compiler", "Error deleting files: {:?}", res);
    }
}

fn create_cargo_toml_file(
    features: &Vec<String>,
    dir_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!(target: "compiler",
        "Entered create_cargo_toml_file with features: {:?} and dir_path: {:?}",
        features, dir_path
    );
    // Check if sent features are allowed
    let features_list = parse_features(features)?;

    // Replace features_list in CARGO_TOML with features received
    let cargo_toml_file_contents = CARGO_TOML.replace("features_list", &features_list);

    let path = dir_path.join("Cargo.toml");
    let mut cargo_toml_file = File::create(path)?;
    cargo_toml_file.write_all(cargo_toml_file_contents.as_bytes())?;

    info!(target: "compiler", "create_cargo_toml_file success");
    Ok(())
}

fn create_lib_rs_file(code: &String, dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    debug!(target: "compiler",
        "Entered create_lib_rs_file with code: {:?} and dir_path: {:?}",
        code, dir_path
    );
    let path = dir_path.join("lib.rs");
    let mut lib_rs_file = File::create(path)?;
    lib_rs_file.write_all(code.as_bytes())?;

    info!(target: "compiler", "create_lib_rs_file success");
    Ok(())
}

fn parse_features(features: &Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    debug!(target: "compiler", "Entered parse_features with features: {:?}", features);
    let features_list = features
        .iter()
        .map(|s| format!("\"{}\"", s))
        .collect::<Vec<String>>()
        .join(", ");

    info!(target: "compiler", "parse_features success");
    Ok(features_list)
}

pub fn hash_code(code: &String) -> String {
    debug!(target: "compiler", "Entered hash_code");
    let mut hasher = Sha256::new();
    hasher.update(code);
    let code_id = hasher.finalize();
    info!(target: "compiler", "hash_code success: {:?}", code_id);
    format!("{:x}", code_id)
}

#[cfg(test)]
#[path = "../tests/utils/contract_utils_tests.rs"]
mod contract_utils_tests;
