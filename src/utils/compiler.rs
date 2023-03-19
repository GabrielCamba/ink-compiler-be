use std::env;
use log::debug;

pub struct Compiler {
    pub cargo_loc: String,
}

impl Compiler {
    pub fn init() -> Self {
        debug!("Initializing compiler");
        let cargo_loc = match env::var("CARGO") {
            Ok(v) => v.to_string(),
            Err(_) => {
                debug!("Could not find CARGO env variable");
                format!("Error loading env variable")
            },
        }; //TODO Handle error better

        Compiler { cargo_loc }
    }
}
