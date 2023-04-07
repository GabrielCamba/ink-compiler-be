use crate::{
    models::{
        api_models::{
            ContractMetadata, DeployMessage, GetDeploymentsMessage, ServerResponse, WizardMessage,
        },
        db_models::{Contract, Deployment},
    },
    repository::mongodb_repo::MongoRepo,
    utils::compiler::Compiler,
    utils::contract_utils::{
        compile_contract, create_files, delete_files, get_contract_data, hash_code,
    },
    utils::sanity_check::sanity_check,
};

use log::{debug, error, info};
use rocket::response::status::Custom;
use rocket::{http::Status, serde::json::Json, State};

#[post("/contract", data = "<wizard_message>")]
pub fn create_contract(
    compiler: &State<Compiler>,
    db: &State<MongoRepo>,
    wizard_message: Json<WizardMessage>,
) -> Result<Json<ServerResponse<Contract>>, Custom<Json<ServerResponse<Contract>>>> {
    sanity_check(&wizard_message)?;

    let code_hash_str = hash_code(&wizard_message.code);
    debug!(target: "compiler", "hash_code completed");

    // Check if contract already exists in DB
    let contract_on_db = db.get_contract_by_hash(&code_hash_str);
    debug!(target: "compiler", "get_contract_by_hash completed");

    match contract_on_db {
        Ok(contract) => match contract {
            Some(mut contract) => {
                info!(target: "compiler", "Contract existing in the db with id: {:?}", &contract.id);
                contract.id = None;
                return Ok(Json(ServerResponse::new_valid(contract)));
            }
            None => (),
        },
        Err(_) => {
            error!(target: "compiler", "Error getting contract from db");
        }
    }

    // If it doesn't exist, create files and compile
    let dir_path = create_files(&wizard_message);
    debug!(target: "compiler", "create_files called");

    if dir_path.is_err() {
        error!(target: "compiler", "Error creating files");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Error creating files.",
            ))),
        ));
    }

    let dir_path = dir_path.expect("This won't panic because we already checked for error");
    info!(target: "compiler", "dir_path created: {:?}", &dir_path);

    // Compile contract
    let res = compile_contract(&compiler.cargo_loc, &dir_path);
    info!(target: "compiler",
        "compile contract called with compiler.cargo_loc: {:?}, and dir_path{:?}",
        &compiler.cargo_loc, &dir_path
    );

    if res.is_err() {
        delete_files(&dir_path);
        error!(target: "compiler", "Error compiling contract");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Error compiling contract.",
            ))),
        ));
    }

    // Get contract data
    let contract = get_contract_data(&dir_path, &code_hash_str);
    debug!(target: "compiler",
        "get_contract_data called with params dir_path: {:?}, code_hash_str: {:?}",
        &dir_path, &code_hash_str
    );

    match contract {
        Ok(contract_unwrapped) => {
            let contract_save_result = db.create_contract(&contract_unwrapped);
            info!(target: "compiler",
                "create_contract called with contract: {:?}",
                &contract_unwrapped
            );
            match contract_save_result {
                Ok(insert_one_result) => {
                    info!(target: "compiler", "insert_one_result: {:?}", &insert_one_result);
                }
                Err(_) => {
                    error!(target: "compiler", "something bad happened");
                }
            };
            delete_files(&dir_path);
            debug!(target: "compiler", "delete_files called with arg dir_path: {:?}", &dir_path);

            return Ok(Json(ServerResponse::new_valid(contract_unwrapped)));
        }
        Err(_) => {
            error!(target: "compiler", "something bad happened");
            delete_files(&dir_path);

            return Err(Custom(
                Status::InternalServerError,
                Json(ServerResponse::new_error(String::from(
                    "Error getting contract data.",
                ))),
            ));
        }
    };
}

#[post("/deploy", data = "<deploy_message>")]
pub fn new_deployment(
    db: &State<MongoRepo>,
    deploy_message: Json<DeployMessage>,
) -> Result<Json<ServerResponse<String>>, Custom<Json<ServerResponse<String>>>> {
    // TODO Check input

    let deployment = Deployment::new(&deploy_message);
    let deployment_save_result = db.create_deployment(&deployment);

    info!(target: "compiler", "create_deployment called with: {:?}", &deployment);
    match deployment_save_result {
        Ok(insert_one_result) => {
            info!(target: "compiler", "insert_one_result: {:?}", &insert_one_result);
            Ok(Json(ServerResponse::new_valid(String::from("ok"))))
        }

        Err(_) => {
            error!(target: "compiler", "something bad happened");
            Err(Custom(
                Status::InternalServerError,
                Json(ServerResponse::new_error(String::from(
                    "Error storing deployment.",
                ))),
            ))
        }
    }
}

#[get("/deployments?<user_address>&<network>")]
pub fn get_contract_deployments(
    db: &State<MongoRepo>,
    user_address: String,
    network: Option<String>,
) -> Result<Json<ServerResponse<Vec<Deployment>>>, Custom<Json<ServerResponse<Vec<Deployment>>>>> {
    let get_deployments = GetDeploymentsMessage {
        user_address,
        network,
    };
    let deployments = db.get_deployments(&get_deployments);

    match deployments {
        Ok(deployments_unwrapped) => Ok(Json(ServerResponse::new_valid(deployments_unwrapped))),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Error getting deployments.",
            ))),
        )),
    }
}

#[get("/contract-metadata?<code_id>")]
pub fn get_contract_metadata(
    db: &State<MongoRepo>,
    code_id: String,
) -> Result<Json<ServerResponse<ContractMetadata>>, Custom<Json<ServerResponse<ContractMetadata>>>>
{
    let contract = db.get_contract_by_hash(&code_id);

    match contract {
        Ok(contract_unwrapped) => match contract_unwrapped {
            Some(contract) => {
                let contract_metadata = ContractMetadata {
                    metadata: contract.metadata,
                };
                Ok(Json(ServerResponse::new_valid(contract_metadata)))
            }
            None => Err(Custom(
                Status::NotFound,
                Json(ServerResponse::new_error(String::from(
                    "Contract not found.",
                ))),
            )),
        },
        Err(_) => Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Error getting contract.",
            ))),
        )),
    }
}
