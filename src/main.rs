mod api;
mod models;
mod repository;
mod utils;

#[macro_use]
extern crate rocket;

use api::contract_api::{create_contract, get_contract};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_contract])
        .mount("/", routes![get_contract])
}
