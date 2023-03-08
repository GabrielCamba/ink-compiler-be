use std::env;
use std::path::Path;
use std::process::Command;

pub struct Compiler {
    pub cargo_loc: String,
}

impl Compiler {
    pub fn init() -> Self {
        let cargo_loc = match env::var("CARGO") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        }; //TODO Handle error better

        Compiler { cargo_loc }
    }
}
