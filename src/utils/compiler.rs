use log::debug;
use std::{env, sync::Arc, thread};

use crate::utils::contract_utils::{
    compile_contract, create_files, delete_files, get_contract_data,
};

use super::super::models::api_models::WizardMessage;
use super::compilation_queue::CompilationQueue;

pub struct Compiler {
    pub cargo_loc: String,
    pub compilation_queue: Arc<CompilationQueue>,
}

impl Compiler {
    pub fn init(compilation_queue: Arc<CompilationQueue>) -> Self {
        debug!("Initializing compiler");
        let cargo_loc = match env::var("CARGO") {
            Ok(v) => v.to_string(),
            Err(_) => {
                error!("CARGO environment variable not set");
                std::process::exit(1);
            }
        };

        Compiler {
            cargo_loc,
            compilation_queue,
        }
    }

    pub fn start(&self) {
        debug!("Starting compiler");
        loop {
            let request = {
                let mut queue = self.compilation_queue.queue.lock().unwrap();
                if queue.is_empty() {
                    None
                } else {
                    Some(queue.remove(0))
                }
            };
            if let Some(request) = request {
                // Perform the compilation for the request here
                println!(
                    "Compiling code for user: {}",
                    request.wizard_message.address
                );

                let wizard_message = request.wizard_message;

                // If it doesn't exist, create files and compile
                let dir_path = create_files(&wizard_message);
                debug!("create_files called");

                if dir_path.is_err() {
                    error!("Error creating files");
                    request
                        .tx
                        .send(Err(String::from("Error creating files.")))
                        .unwrap();
                    continue;
                }

                let dir_path =
                    dir_path.expect("This won't panic because we already checked for error");
                info!("dir_path created: {:?}", &dir_path);

                // Compile contract
                let res = compile_contract(&self.cargo_loc, &dir_path);
                info!(
                    "compile contract called with compiler.cargo_loc: {:?}, and dir_path{:?}",
                    &self.cargo_loc, &dir_path
                );

                if res.is_err() {
                    delete_files(&dir_path);
                    error!("Error compiling contract");
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
                    delete_files(&dir_path);
                    error!("Error getting contract data");
                    request
                        .tx
                        .send(Err(String::from("Error getting contract data.")))
                        .unwrap();
                    continue;
                }

                request.tx.send(Ok(contract.unwrap())).unwrap();
                delete_files(&dir_path);
            } else {
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
