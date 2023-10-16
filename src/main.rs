mod api;
mod models;
mod repository;
mod utils;

extern crate dotenv;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;

use api::contract_api::{
    fetch_or_compile_contract, get_contract, get_contract_deployments, store_deployment,
    update_deployment,
};
use repository::mongodb_repo::MongoRepo;
use rocket::fairing::AdHoc;
use std::{
    sync::{atomic::AtomicBool, Arc},
    thread,
};
use utils::compilation_queue::CompilationQueue;
use utils::compiler::Compiler;

use log::{debug, error, info};

use utils::cors::CORS;

// Rocket launching the server in rocket function
#[launch]
fn rocket() -> _ {
    // Creating the logger and checking it's ok.
    let logger = log4rs::init_file("logging_config.yaml", Default::default());
    if logger.is_err() {
        error!("Error initializing logger");
    } else {
        info!(target: "compiler", "Logger Initialized");
    }

    // Loading env variables
    dotenv().ok();
    debug!(target: "compiler", "dotenv loaded");

    // Creating compilation queue
    let queue = CompilationQueue::new();
    let compilation_queue = Arc::new(queue);
    let compilation_queue_clone = compilation_queue.clone();

    // Setting shutdown flag to perform operations when the server is shutting down
    let shutdown_flag = Arc::new(AtomicBool::new(false));

    // Creating compiler instance and running a new thread
    let compiler = Compiler::init(compilation_queue_clone, shutdown_flag.clone());
    let compiler_thread = thread::spawn(move || {
        compiler.start();
    });
    debug!(target: "compiler", "compiler initialized");

    // Initializing mongo
    let db = MongoRepo::init();
    debug!(target: "compiler", "mongo repo initialized");

    // Initializing the server
    rocket::build()
        .manage(compilation_queue)
        .manage(db)
        .manage(shutdown_flag.clone())
        .mount(
            "/",
            routes![
                fetch_or_compile_contract,
                store_deployment,
                update_deployment,
                get_contract_deployments,
                get_contract
            ],
        )
        .attach(AdHoc::on_shutdown("Shutdown Handler", |_| {
            Box::pin(async move {
                info!(target: "compiler", "Shutting down");
                shutdown_flag.store(true, std::sync::atomic::Ordering::Relaxed);
                let join_res = compiler_thread.join();

                if join_res.is_err() {
                    error!(target: "compiler", "Error joining compiler thread");
                }

                info!(target: "compiler", "Shutdown complete");
            })
        }))
        .attach(CORS)
}
#[cfg(test)]
#[path = "./tests/main_post_contract_tests.rs"]
mod main_post_contract_test;

#[cfg(test)]
#[path = "./tests/main_get_contract_tests.rs"]
mod main_get_contract_test;

#[cfg(test)]
#[path = "./tests/main_post_deployments_tests.rs"]
mod main_post_deployments_test;

#[cfg(test)]
#[path = "./tests/main_get_deployments_tests.rs"]
mod main_get_deployments_test;
