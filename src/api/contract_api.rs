use crate::{
    models::contract_model::{ServerResponse, WizardMessage},
    repository::mongodb_repo::MongoRepo,
    utils::compiler::Compiler,
    utils::contract_utils::{
        compile_contract, create_files, delete_files, get_contract_data, hash_code,
    },
};

use rocket::response::status::Custom;
use rocket::{http::Status, serde::json::Json, State};

#[post("/contract", data = "<wizard_message>")]
pub fn create_contract(
    compiler: &State<Compiler>,
    db: &State<MongoRepo>,
    wizard_message: Json<WizardMessage>,
) -> Result<Json<ServerResponse>, Custom<Json<ServerResponse>>> {
    // TODO Sanity check WizardMessage data
    // tamaño del código, chequear la address, y freatures not empty (include psp22, psp34 or psp 37)

    let code_hash_str = hash_code(&wizard_message.code);

    // Check if contract already exists in DB
    let contract_on_db = db.get_contract_by_hash(&code_hash_str);

    match contract_on_db {
        Ok(contract) => match contract {
            Some(contract) => {
                println!("contract: {:?}", &contract);
                return Ok(Json(ServerResponse::new_valid(contract)));
            }
            None => (),
        },
        Err(_) => {
            println!("something bad happened");
            return Err(Status::InternalServerError);
        }
    }

    // If it doesn't exist, create files and compile
    let dir_path = create_files(&wizard_message);

    if dir_path.is_err() {
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Error creating files.",
            ))),
        ));
    }

    let dir_path = dir_path.expect("This won't panic because we already checked for error");
    print!("dir_path: {:?}", dir_path);

    // Compile contract
    let res = compile_contract(&compiler.cargo_loc, &dir_path);
    if res.is_err() {
        let res = delete_files(&dir_path);
        if res.is_err() {
            println!("Error deleting files");
        }
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Error compiling contract.",
            ))),
        ));
    }

    // Get contract data
    let contract = get_contract_data(&dir_path, &code_hash_str);
    match contract {
        Ok(contract_unwrapped) => {
            let contract_save_result = db.create_contract(contract_unwrapped.clone());
            match contract_save_result {
                Ok(insert_one_result) => {
                    println!("insert_one_result: {:?}", &insert_one_result);
                }
                Err(_) => {
                    println!("something bad happened");
                }
            };
            let res = delete_files(&dir_path);
            if res.is_err() {
                println!("Error deleting files");
            }
            return Ok(Json(ServerResponse::new_valid(contract_unwrapped)));
        }
        Err(_) => {
            println!("something bad happened");
            let res = delete_files(&dir_path);
            if res.is_err() {
                println!("Error deleting files");
            }
            return Err(Custom(
                Status::InternalServerError,
                Json(ServerResponse::new_error(String::from(
                    "Error getting contract data.",
                ))),
            ));
        }
    };
}
