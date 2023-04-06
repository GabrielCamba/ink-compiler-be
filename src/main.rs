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
use std::{sync::Arc, thread};
use utils::compilation_queue::CompilationQueue;
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
    info!("Logger Initialized");

    dotenv().ok();
    debug!("dotenv loaded");

    let queue = CompilationQueue::new();
    let compilation_queue = Arc::new(queue);
    let compilation_queue_clone = compilation_queue.clone();

    let compiler = Compiler::init(compilation_queue_clone);
    thread::spawn(move || {
        compiler.start();
    });
    debug!("compiler initialized");

    let db = MongoRepo::init();
    debug!("mongo repo initialized");

    rocket::build().manage(compilation_queue).manage(db).mount(
        "/",
        routes![
            create_contract,
            new_deployment,
            get_contract_deployments,
            get_contract_metadata
        ],
    )
}
