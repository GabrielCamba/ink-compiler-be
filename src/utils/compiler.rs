use log::debug;
use std::env;

pub struct Compiler {
    pub cargo_loc: String,
}

impl Compiler {
    pub fn init() -> Self {
        debug!("Initializing compiler");
        let cargo_loc = match env::var("CARGO") {
            Ok(v) => v.to_string(),
            Err(_) => {
                error!("CARGO environment variable not set");
                std::process::exit(1);
            }
        };

        Compiler { cargo_loc }
    }
}
