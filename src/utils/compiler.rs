use log::{error, info};
use std::fs::{copy, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::{env, sync::Arc, thread};

use crate::models::api_models::WizardMessage;
use crate::models::db_models::Contract;

use super::compilation_queue::CompilationQueue;

// Compiler is a singleton that handles the compilation of contracts
pub struct Compiler {
    pub cargo_loc: String,
    pub compilation_queue: Arc<CompilationQueue>,
    pub shutdown_flag: Arc<AtomicBool>,
    pub dir_path: PathBuf,
}

// Compiler implementation
impl Compiler {
    // Initializes the compiler
    pub fn init(compilation_queue: Arc<CompilationQueue>, shutdown_flag: Arc<AtomicBool>) -> Self {
        info!(target: "compiler", "Initializing compiler");
        let cargo_loc = match env::var("CARGO") {
            Ok(v) => v.to_string(),
            Err(_) => {
                error!(target: "compiler", "CARGO environment variable not set");
                std::process::exit(1);
            }
        };

        // Create the directory for the compiler
        let current_dir = env::current_dir();
        if current_dir.is_err() {
            error!(target: "compiler", "Error getting current directory");
            std::process::exit(1);
        }

        let current_dir =
            current_dir.expect("This will never panic because we checked for errors before");

        let dir_path = current_dir.join("compilation_target");

        Compiler {
            cargo_loc,
            compilation_queue,
            shutdown_flag,
            dir_path,
        }
    }

    // Main compiler function
    // It has 3 stages:
    // Stage 1.- Initialize compiler and compile template contract
    // Stage 2.- Loop that takes compilation requests from the queue
    // Stage 3.- Shutdown
    pub fn start(&self) {
        // Stage 1
        // Create the directory for the compiler
        let source_file_path = &self.dir_path.join("template-lib.rs");
        let destination_file_path = &self.dir_path.join("lib.rs");

        // Copy the file and rename it
        let copy_res = copy(source_file_path, destination_file_path);
        if copy_res.is_err() {
            error!(target: "compiler", "Error copying template-lib.rs to lib.rs");
        }

        // Compile init contract
        let res = self.compile_contract();

        if res.is_err() {
            self.delete_compilation_files();
            error!(target: "compiler", "Error compiling init contract");
        }

        // Stage 2.-
        // Loop and compile requests until shutdown flag is set
        while !self
            .shutdown_flag
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            // Take a request from the queue
            let request = self.compilation_queue.take_request();

            // Checking if there's something to do
            if let Some(request) = request {
                // Perform the compilation for the request here
                info!(target: "compiler",
                    "Compiling code for user: {}",
                    request.wizard_message.address
                );

                let wizard_message = request.wizard_message;

                let compile_res = self.create_contract_files(&wizard_message);

                if compile_res.is_err() {
                    self.delete_compilation_files();
                    error!(target: "compiler", "Error creating files");
                    let msg_res = request.tx.send(Err(String::from("Error creating files.")));

                    if msg_res.is_err() {
                        error!(target: "compiler", "Error sending message");
                    }

                    continue;
                }

                // Compile contract
                let res = self.compile_contract();

                // Evaluate compilation result
                if res.is_err() {
                    error!(target: "compiler", "Error compiling contract");
                    let msg_res = request
                        .tx
                        .send(Err(String::from("Error compiling contract.")));

                    if msg_res.is_err() {
                        error!(target: "compiler", "Error sending message");
                    }

                    continue;
                }

                // Get contract data
                let contract = self.get_contract_data(&request.code_id);

                if contract.is_err() {
                    error!(target: "compiler", "Error getting contract data");
                    let msg_res = request
                        .tx
                        .send(Err(String::from("Error getting contract data.")));

                    if msg_res.is_err() {
                        error!(target: "compiler", "Error sending message");
                    }

                    continue;
                }

                let msg_res =
                    request.tx.send(Ok(contract
                        .expect("This will not panic because we already checked for errors")));
                if msg_res.is_err() {
                    error!(target: "compiler", "Error sending message");
                }
            } else {
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }

        // Stage 3 .-
        // Shutdown gracefully
        info!(target: "compiler", "Compiler shutting down...");
        self.delete_compilation_files();
        info!(target: "compiler", "Compiler shutdown complete");
    }

    // Function called by the compiler to generate the contract wasm and metadata
    fn compile_contract(&self) -> Result<(), Box<dyn std::error::Error>> {
        // This is the command used to compile the contract
        let mut binding = Command::new(self.cargo_loc.clone());
        let compiler_cmd = binding
            .arg("contract")
            .arg("build")
            .arg("--release")
            .arg("--quiet")
            .current_dir(self.dir_path.clone());

        // Check the status of the command execution
        let status = compiler_cmd.status()?;
        if !status.success() {
            error!(target: "compiler", "Compilation failed");
            return Err("Compilation failed".into());
        }
        info!(target: "compiler", "Compilation success");

        Ok(())
    }

    // This function is used to create the contract files in the filesystem
    fn create_contract_files(
        &self,
        wizard_message: &WizardMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.dir_path.join("lib.rs");
        let mut lib_rs_file = File::create(path)?;
        lib_rs_file.write_all(wizard_message.code.as_bytes())?;

        info!(target: "compiler", "lib.rs successfully created");

        Ok(())
    }

    // This function is used to delete the compiled contract files in the filesystem
    fn delete_compilation_files(&self) {
        let res = std::fs::remove_dir_all(self.dir_path.join("target"));
        if res.is_err() {
            error!(target: "compiler", "Error deleting files: {:?}", res);
        }

        let res = std::fs::remove_dir_all(self.dir_path.join("__openbrush_metadata_folder"));
        if res.is_err() {
            error!(target: "compiler", "Error deleting files: {:?}", res);
        }
    }

    // this function is used to read from the file system the wasm and metadata files generated by the compiler
    pub fn get_contract_data(
        &self,
        code_id: &String,
    ) -> Result<Contract, Box<dyn std::error::Error>> {
        // Read compiled contract
        let mut wasm_file = File::open(self.dir_path.join("target/ink/compiled_contract.wasm"))?;
        let mut wasm = Vec::new();
        wasm_file.read_to_end(&mut wasm)?;

        // Read contract metadata
        let mut metadata_file =
            File::open(self.dir_path.join("target/ink/compiled_contract.json"))?;
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
}

#[cfg(test)]
#[path = "../tests/utils/compiler_tests.rs"]
mod compiler_tests;
