mod api;
mod models;
mod repository;
mod utils;

extern crate dotenv;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;

use api::contract_api::{
    get_contract_deployments, get_contract_metadata, new_deployment, test_queue,
};
use repository::mongodb_repo::MongoRepo;
use std::{
    sync::{Arc, Mutex},
    thread,
};
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
    let queue_ref = Arc::new(queue);
    let queue_ref_clone = queue_ref.clone();

    let compiler = Compiler::init(queue_ref_clone);
    thread::spawn(move || {
        compiler.start();
    });
    debug!("compiler initialized");

    let db = MongoRepo::init();
    debug!("mongo repo initialized");

    rocket::build().manage(queue_ref).manage(db).mount(
        "/",
        routes![
            test_queue,
            new_deployment,
            get_contract_deployments,
            get_contract_metadata
        ],
    )
}
