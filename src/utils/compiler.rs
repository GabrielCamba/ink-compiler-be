use log::debug;
use std::{env, sync::Arc, thread};

use super::compilation_queue::{CompilationQueue, CompileRequest};

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
                println!("Compiling code: {}", request.code);
                thread::sleep(std::time::Duration::from_millis(1000));
                request
                    .tx
                    .send(format!("Compiled! {}", request.code))
                    .unwrap();
            } else {
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
