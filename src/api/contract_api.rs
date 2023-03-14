use crate::{
    models::contract_model::{Contract, WizardMessage},
    repository::mongodb_repo::MongoRepo,
    utils::compiler::Compiler,
    utils::contract_utils::{compile_contract, create_files, delete_files, get_contract_data},
};
use mongodb::{results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};
use sha2::{Digest, Sha256};

#[post("/contract", data = "<wizard_message>")]
pub fn create_contract(
    compiler: &State<Compiler>,
    db: &State<MongoRepo>,
    wizard_message: Json<WizardMessage>,
) -> Result<Json<Contract>, Status> {
    // TODO Sanity check WizardMessage data
    // tamaño del código, chequear la address, y freatures not empty (include psp22, psp34 or psp 37)

    // TODO Check if contract already exists in DB
    let mut hasher = Sha256::new();
    hasher.update(&wizard_message.code);
    let code_hash = hasher.finalize();
    let code_hash_str = format!("{:x}", code_hash);
    println!("code_hash_str: {:?}", &code_hash_str);

    let contract_on_db = db.get_contract_by_hash(&code_hash_str);

    match contract_on_db {
        Ok(contract) => {
            match contract {
                Some(contract) => {
                    println!("contract: {:?}", &contract);
                    return Ok(Json(contract))
                },
                None => ()
            }
        },
        Err(_) => {
            println!("something bad happened");
            return Err(Status::InternalServerError);
        }
    }


    let dir_path = create_files(&wizard_message);

    if dir_path.is_err() {
        return Err(Status::InternalServerError);
    }

    let dir_path = dir_path.expect("This won't fail because we already checked for error");
    print!("dir_path: {:?}", dir_path);

    // Compile contract
    compile_contract(&compiler.cargo_loc, &dir_path); //TODO Handle error

    //TODO Get contract data
    let contract = get_contract_data(&dir_path, &code_hash_str);

    match contract {
        Ok(contract_unwrapped) => {
            let contract_save_result = db.create_contract(contract_unwrapped.clone());
            match contract_save_result {
                Ok(insert_one_result) => {
                    println!("insert_one_result: {:?}", &insert_one_result);
                },
                Err(_) => {
                    println!("something bad happened");
                }
            };
            delete_files(&dir_path); //TODO Handle error
            return Ok(Json(contract_unwrapped))
        },
        Err(_) => {
            println!("something bad happened");
            return Err(Status::InternalServerError);
        }
    };
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
