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
use rocket::fairing::AdHoc;
use std::{
    sync::{atomic::AtomicBool, Arc},
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
    info!(target: "compiler", "Logger Initialized");

    dotenv().ok();
    debug!(target: "compiler", "dotenv loaded");

    let queue = CompilationQueue::new();
    let compilation_queue = Arc::new(queue);
    let compilation_queue_clone = compilation_queue.clone();

    let shutdown_flag = Arc::new(AtomicBool::new(false));

    let compiler = Compiler::init(compilation_queue_clone, shutdown_flag.clone());
    let compiler_thread = thread::spawn(move || {
        compiler.start();
    });
    debug!(target: "compiler", "compiler initialized");

    let db = MongoRepo::init();
    debug!(target: "compiler", "mongo repo initialized");

    rocket::build()
        .manage(compilation_queue)
        .manage(db)
        .manage(shutdown_flag.clone())
        .mount(
            "/",
            routes![
                create_contract,
                new_deployment,
                get_contract_deployments,
                get_contract_metadata
            ],
        )
        .attach(AdHoc::on_shutdown("Shutdown Handler", |_| {
            Box::pin(async move {
                info!("Shutting down");
                shutdown_flag.store(true, std::sync::atomic::Ordering::Relaxed);
                compiler_thread.join().unwrap();
            })
        }))
}
