use crate::{
    models::contract_model::{ServerResponse, WizardMessage},
    repository::mongodb_repo::MongoRepo,
    utils::compiler::Compiler,
    utils::contract_utils::{
        compile_contract, create_files, delete_files, get_contract_data, hash_code,
    },
    utils::sanity_check::sanity_check,
};

use log::{debug, error, info, warn};
use rocket::response::status::Custom;
use rocket::{http::Status, serde::json::Json, State};

#[post("/contract", data = "<wizard_message>")]
pub fn create_contract(
    compiler: &State<Compiler>,
    db: &State<MongoRepo>,
    wizard_message: Json<WizardMessage>,
) -> Result<Json<ServerResponse>, Custom<Json<ServerResponse>>> {
    // TODO Sanity check WizardMessage data codesize , check address, y freatures not empty ( must include psp22, psp34 or psp37)
    let check = sanity_check(&wizard_message);

    if check.is_err() {
        let e = check.unwrap_err();
        return Err(e);
    }

    let code_hash_str = hash_code(&wizard_message.code);
    debug!("hash_code completed");

    // Check if contract already exists in DB
    let contract_on_db = db.get_contract_by_hash(&code_hash_str);
    debug!("get_contract_by_hash completed");

    match contract_on_db {
        Ok(contract) => match contract {
            Some(contract) => {
                info!("Contract existing in the db: {:?}", &contract);
                return Ok(Json(ServerResponse::new_valid(contract)));
            }
            None => (),
        },
        Err(_) => {
            error!("Error getting contract from db");
        }
    }

    // If it doesn't exist, create files and compile
    let dir_path = create_files(&wizard_message);
    debug!("create_files called");

    if dir_path.is_err() {
        error!("Error creating files");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Error creating files.",
            ))),
        ));
    }

    let dir_path = dir_path.expect("This won't panic because we already checked for error");
    info!("dir_path created: {:?}", &dir_path);

    // Compile contract
    let res = compile_contract(&compiler.cargo_loc, &dir_path);
    info!(
        "compile contract called with compiler.cargo_loc: {:?}, and dir_path{:?}",
        &compiler.cargo_loc, &dir_path
    );

    if res.is_err() {
        let res = delete_files(&dir_path);
        if res.is_err() {
            warn!("Error deleting files");
        }
        error!("Error compiling contract");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Error compiling contract.",
            ))),
        ));
    }

    // Get contract data
    let contract = get_contract_data(&dir_path, &code_hash_str);
    debug!(
        "get_contract_data called with params dir_path: {:?}, code_hash_str: {:?}",
        &dir_path, &code_hash_str
    );

    match contract {
        Ok(contract_unwrapped) => {
            let contract_save_result = db.create_contract(contract_unwrapped.clone());
            info!(
                "create_contract called with contract: {:?}",
                &contract_unwrapped
            );
            match contract_save_result {
                Ok(insert_one_result) => {
                    info!("insert_one_result: {:?}", &insert_one_result);
                }
                Err(_) => {
                    error!("something bad happened");
                }
            };
            let res = delete_files(&dir_path);
            debug!("delete_files called with arg dir_path: {:?}", &dir_path);
            if res.is_err() {
                warn!("Error deleting files");
            }
            return Ok(Json(ServerResponse::new_valid(contract_unwrapped)));
        }
        Err(_) => {
            error!("something bad happened");
            let res = delete_files(&dir_path);
            if res.is_err() {
                warn!("Error deleting files");
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
