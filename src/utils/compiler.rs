use log::{debug, error, info};
use std::fs::copy;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::{env, sync::Arc, thread};

use crate::utils::contract_utils::{
    compile_contract, create_files, delete_files, get_contract_data,
};

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
        let current_dir = env::current_dir().unwrap();
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
        let res = compile_contract(&self.cargo_loc, &self.dir_path);

        if res.is_err() {
            delete_files(&self.dir_path);
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

                // TODO: Rename create_files in favor of override_lib or contract or something like that
                // TODO: Rename dir_path in favor of override result or something like that
                let dir_path = create_files(&wizard_message);

                if dir_path.is_err() {
                    error!(target: "compiler", "Error creating files");
                    request
                        .tx
                        .send(Err(String::from("Error creating files.")))
                        .unwrap();
                    continue;
                }

                let dir_path =
                    dir_path.expect("This won't panic because we already checked for error");

                // Compile contract
                let res = compile_contract(&self.cargo_loc, &dir_path);
                info!(target: "compiler",
                    "compile contract called with compiler.cargo_loc: {:?}, and dir_path{:?}",
                    &self.cargo_loc, &dir_path
                );

                // Evaluate compilation result
                if res.is_err() {
                    error!(target: "compiler", "Error compiling contract");
                    request
                        .tx
                        .send(Err(String::from("Error compiling contract.")))
                        .unwrap();
                    continue;
                }

                // Get contract data
                let contract = get_contract_data(&dir_path, &request.code_id);
                debug!(
                    "get_contract_data called with params dir_path: {:?}, code_hash_str: {:?}",
                    &dir_path, &request.code_id
                );
                if contract.is_err() {
                    error!(target: "compiler", "Error getting contract data");
                    request
                        .tx
                        .send(Err(String::from("Error getting contract data.")))
                        .unwrap();
                    continue;
                }

                request.tx.send(Ok(contract.unwrap())).unwrap();
            } else {
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }

        // Stage 3 .-
        // Shutdown gracefully
        info!(target: "compiler", "Compiler shutting down...");
        delete_files(&self.dir_path);
        info!(target: "compiler", "Compiler shutdown complete");
    }
}
