mod api;
mod models;
mod repository;
mod utils;

extern crate dotenv;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;

use api::contract_api::{create_contract, get_contract_deployments, new_deployment};
use repository::mongodb_repo::MongoRepo;
use utils::compiler::Compiler;

use log::{debug, info};
use log4rs;

#[launch]
fn rocket() -> _ {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    info!("Logger Initialized");

    dotenv().ok();
    debug!("dotenv loaded");

    let compiler = Compiler::init();
    debug!("compiler initialized");

    let db = MongoRepo::init();
    debug!("mongo repo initialized");

    rocket::build().manage(compiler).manage(db).mount(
        "/",
        routes![create_contract, new_deployment, get_contract_deployments],
    )
}
