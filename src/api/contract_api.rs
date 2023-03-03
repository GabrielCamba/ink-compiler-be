use crate::{
    models::contract_model::{Contract, WizardMessage},
    repository::mongodb_repo::MongoRepo,
    utils::contract_utils::{create_cargo_toml_file, create_lib_rs_file},
};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/contract", data = "<wizard_message>")]
pub fn create_contract(
    db: &State<MongoRepo>,
    wizard_message: Json<WizardMessage>,
) -> Result<Json<InsertOneResult>, Status> {
    // TODO Check if contract already exists in DB

    // TODO If contract already exists, return data

    // TODO If contract does not exist, compile contract and return data
    create_cargo_toml_file(&wizard_message.features); //TODO Add error handling

    //let contract_detail = db.create_contract(data);
    // match contract_detail {
    //     Ok(contract) => Ok(Json(contract)),
    //     Err(_) => Err(Status::InternalServerError),
    // }
    Err(Status::InternalServerError)
}

#[get("/contract/<path>")]
pub fn get_contract(db: &State<MongoRepo>, path: String) -> Result<Json<Contract>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let contract_detail = db.get_contract(&id);
    match contract_detail {
        Ok(contract) => Ok(Json(contract)),
        Err(_) => Err(Status::InternalServerError),
    }
}
