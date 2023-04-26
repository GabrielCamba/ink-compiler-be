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
use log4rs;

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
// TODO: Check the database is up and running before starting running these tests
#[cfg(test)]
mod test {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    const VALID_INK_SC :&str= r#"#![cfg_attr(not(feature = \"std\"), no_std)] #![feature(min_specialization)] #[openbrush::contract] pub mod my_psp22 { use openbrush::contracts::psp22::*; use openbrush::traits::Storage; #[ink(storage)] #[derive(Default, Storage)] pub struct Contract { #[storage_field] psp22: psp22::Data, } impl PSP22 for Contract {} impl Contract { #[ink(constructor)] pub fn new(initial_supply: Balance) -> Self { let mut _instance = Self::default(); _instance._mint_to(_instance.env().caller(), initial_supply); _instance } } }"#;

    #[test]
    fn post_contract_missing_address_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/contract")).body(r#"{ }"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn post_contract_missing_no_code_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/contract"))
            .body(r#"{ "address": "asdf" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn post_contract_missing_no_features_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/contract"))
            .body(r#"{ "address": "address", "code": "something" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn post_contract_expected_features_to_be_an_array() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/contract"))
            .body(r#"{ "address": "address", "code": "something", "features": "asdf" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn post_contract_expects_a_valid_len_address() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/contract"))
            .body(r#"{ "address": "address", "code": "something", "features": ["asdf"] }"#)
            .dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
        assert!(response
            .into_string()
            .unwrap()
            .contains("Address is not valid."));
    }

    #[test]
    fn post_contract_expects_a_valid_feature() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/contract"))
            .body(r#"{ "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "code": "something", "features": ["asdf"] }"#)
            .dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
        assert!(response
            .into_string()
            .unwrap()
            .contains("Feature not allowed"));
    }

    #[test]
    fn post_contract_expects_code_not_to_be_too_long() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let one_mb_string = "a".repeat(1000000);
        let body = format!(
            r#"{{ "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "code": "{}", "features": ["psp22"] }}"#,
            one_mb_string
        );
        let response = client.post(uri!("/contract")).body(body).dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
        let res_str = response.into_string().unwrap();
        assert!(res_str.contains("Code size too big."));
    }

    #[test]
    fn post_contract_expects_code_is_ok() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let body = format!(
            r#"{{ "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "code": "{}", "features": ["psp22"] }}"#,
            VALID_INK_SC
        );
        let response = client.post(uri!("/contract")).body(body).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

}
