use crate::{
    models::contract_model::{Contract, WizardMessage},
    repository::mongodb_repo::MongoRepo,
    utils::compiler::Compiler,
    utils::contract_utils::{compile_contract, create_files, delete_files},
};
use mongodb::{results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/contract", data = "<wizard_message>")]
pub fn create_contract(
    compiler: &State<Compiler>,
    db: &State<MongoRepo>,
    wizard_message: Json<WizardMessage>,
) -> Result<Json<InsertOneResult>, Status> {
    // TODO Sanity check WizardMessage data
    // tamaño del código, chequear la address, y freatures not empty (include psp22, psp34 or psp 37)

    // TODO Check if contract already exists in DB

    // TODO If contract already exists, return data

    // TODO If contract does not exist, compile contract and return data
    let dir_path = create_files(&wizard_message);

    //println!("dir_path: {:?}", dir_path);
    if dir_path.is_err() {
        return Err(Status::InternalServerError);
    }

    let dir_path = dir_path.expect("This won't fail because we already checked for error");
    print!("dir_path: {:?}", dir_path);
    compile_contract(&compiler.cargo_loc, &dir_path); //TODO Handle error

    //LUCA TODO Compile contract

    //TODO Save contract in DB

    //TODO Return contract data

    //TODO Delete tmp folder
    delete_files(&dir_path); //TODO Handle error

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
