mod api;
mod models;
mod repository;
mod utils;

extern crate dotenv;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;

use api::contract_api::{
    create_contract, get_contract_deployments, get_contract_metadata, new_deployment,
};
use repository::mongodb_repo::MongoRepo;
use utils::compiler::Compiler;

use log::{debug, info};
use log4rs;

#[launch]
fn rocket() -> _ {
    let logger = log4rs::init_file("logging_config.yaml", Default::default());
    if logger.is_err() {
        error!("Error initializing logger");
        std::process::exit(1);
    }
    info!(target: "compiler", "Logger Initialized");

    dotenv().ok();
    debug!(target: "compiler", "dotenv loaded");

    let compiler = Compiler::init();
    debug!(target: "compiler", "compiler initialized");

    let db = MongoRepo::init();
    debug!(target: "compiler", "mongo repo initialized");

    rocket::build().manage(compiler).manage(db).mount(
        "/",
        routes![
            create_contract,
            new_deployment,
            get_contract_deployments,
            get_contract_metadata
        ],
    )
}
