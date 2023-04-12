use std::sync::mpsc::channel;
use std::sync::Arc;

use crate::utils::compilation_queue::CompilationRequest;
use crate::utils::contract_utils::hash_code;
use crate::{
    models::{
        api_models::{
            ContractMetadata, DeployMessage, GetDeploymentsMessage, ServerResponse, WizardMessage,
        },
        db_models::{Contract, Deployment},
    },
    repository::mongodb_repo::MongoRepo,
    utils::compilation_queue::CompilationQueue,
    utils::sanity_check::sanity_check,
};
use log::{debug, error, info};
use rocket::response::status::Custom;
use rocket::{http::Status, serde::json::Json, State};

// /contract endpoint for obtaining a new contract compilation
#[post("/contract", data = "<wizard_message>")]
pub fn fetch_or_compile_contract(
    compilation_queue: &State<Arc<CompilationQueue>>,
    db: &State<MongoRepo>,
    wizard_message: Json<WizardMessage>,
) -> Result<Json<ServerResponse<Contract>>, Custom<Json<ServerResponse<Contract>>>> {
    // Checking input data
    sanity_check(&wizard_message)?;

    // Hashing the contract code to create an unique identifier
    let code_hash_str = hash_code(&wizard_message.code);
    debug!(target: "compiler", "hash_code completed");

    // Check if contract already exists in DB
    let contract_on_db = db.get_contract_by_hash(&code_hash_str);
    debug!(target: "compiler", "get_contract_by_hash completed");

    // If contract already exists in DB, return it
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

    // If contract does not exist in DB, create it

    // Creating a channel to communicate with the compilation thread
    let (tx, rx) = channel::<Result<Contract, String>>();

    // Sending the compilation request to the compilation thread
    let compilation_request = CompilationRequest {
        wizard_message: wizard_message.into_inner(),
        code_id: code_hash_str.clone(),
        tx: tx.clone(),
    };
    compilation_queue.add_request(compilation_request);

    // Waiting for the compilation thread to finish
    let contract = rx.recv().unwrap();

    // Checking if compilation was successful
    match contract {
        Ok(contract_unwrapped) => {
            let contract_save_result = db.create_contract(&contract_unwrapped);
            info!(target: "compiler",
                "Contract {} successfully compiled", &contract_unwrapped.code_id
            );

            // Store contract compiled
            match contract_save_result {
                Ok(_) => {
                    info!(target: "compiler", "Contract {} saved in the database", &contract_unwrapped.code_id);
                }
                Err(_) => {
                    error!(target: "compiler", "There was an error saving the contract {} in the database", &contract_unwrapped.code_id);
                }
            };

            return Ok(Json(ServerResponse::new_valid(contract_unwrapped)));
        }
        // If compilation failed, return the error
        Err(error_msg) => {
            error!(target: "compiler", "There was an error saving the contract {} in the database", &code_hash_str);

            return Err(Custom(
                Status::InternalServerError,
                Json(ServerResponse::new_error(error_msg)),
            ));
        }
    };
}

// /deployments endpoint for storing a new deployment
#[post("/deployments", data = "<deploy_message>")]
pub fn store_deployment(
    db: &State<MongoRepo>,
    deploy_message: Json<DeployMessage>,
) -> Result<Json<ServerResponse<String>>, Custom<Json<ServerResponse<String>>>> {
    // TODO Check input

    // Generating a new deployment structure and storing in db
    let deployment = Deployment::new(&deploy_message);
    let deployment_save_result = db.create_deployment(&deployment);

    info!(target: "compiler", "Storing new deployment for user {} for contract {} in network {}", &deployment.user_address , &deployment.code_id, &deployment.network);

    // Evaluate the result of the save operation
    match deployment_save_result {
        Ok(_) => {
            info!(target: "compiler", "Deployment {} saved in the database", &deployment.contract_address);
            Ok(Json(ServerResponse::new_valid(String::from("ok"))))
        }

        Err(_) => {
            error!(target: "compiler", "There was an error saving the deployment {}", &deployment.contract_address);
            Err(Custom(
                Status::InternalServerError,
                Json(ServerResponse::new_error(String::from(
                    "Error storing deployment.",
                ))),
            ))
        }
    }
}

// /deployments endpoint for fetching a deployment
#[get("/deployments?<user_address>&<network>")]
pub fn get_contract_deployments(
    db: &State<MongoRepo>,
    user_address: String,
    network: Option<String>,
) -> Result<Json<ServerResponse<Vec<Deployment>>>, Custom<Json<ServerResponse<Vec<Deployment>>>>> {
    // Creating structure and fetching the deployments from db
    let get_deployments = GetDeploymentsMessage {
        user_address: user_address.clone(),
        network,
    };
    let deployments = db.get_deployments(&get_deployments);

    // Evaluate the result of the fetch operation
    match deployments {
        Ok(deployments_unwrapped) => {
            info!(target: "compiler", "Deployments fetched from the database for user {}", &user_address);
            Ok(Json(ServerResponse::new_valid(deployments_unwrapped)))
        }
        Err(_) => {
            error!(target: "compiler", "There was an error fetching the deployment for user {}", &user_address);
            Err(Custom(
                Status::InternalServerError,
                Json(ServerResponse::new_error(String::from(
                    "Error getting deployments.",
                ))),
            ))
        }
    }
}

// /contract-metadata endpoint for fetching a contract's metadata
#[get("/contract-metadata?<code_id>")]
pub fn get_contract_metadata(
    db: &State<MongoRepo>,
    code_id: String,
) -> Result<Json<ServerResponse<ContractMetadata>>, Custom<Json<ServerResponse<ContractMetadata>>>>
{
    // Fetching metadata from code_id
    let contract = db.get_contract_by_hash(&code_id);

    // Evaluate the result of the fetch operation and building the response on each case
    match contract {
        Ok(contract_unwrapped) => match contract_unwrapped {
            Some(contract) => {
                info!(target: "compiler", "Contract metadata found for {}", &contract.code_id);
                let contract_metadata = ContractMetadata {
                    metadata: contract.metadata,
                };
                Ok(Json(ServerResponse::new_valid(contract_metadata)))
            }
            None => {
                info!(target: "compiler", "Contract metadata was not found for {}", &code_id);
                Err(Custom(
                    Status::NotFound,
                    Json(ServerResponse::new_error(String::from(
                        "Contract not found.",
                    ))),
                ))
            }
        },
        Err(_) => {
            error!(target: "compiler", "There was DB error fetching metadata for {}", &code_id);
            Err(Custom(
                Status::InternalServerError,
                Json(ServerResponse::new_error(String::from(
                    "Error getting contract.",
                ))),
            ))
        }
    }
}
