mod api;
mod models;
mod repository;
mod utils;

extern crate dotenv;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;

use api::contract_api::{create_contract, get_contract};
use repository::mongodb_repo::MongoRepo;
use utils::compiler::Compiler;

// TODO Implement logger

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let compiler = Compiler::init();
    let db = MongoRepo::init();
    rocket::build()
        .manage(compiler)
        .manage(db)
        .mount("/", routes![create_contract])
        .mount("/", routes![get_contract])
}
