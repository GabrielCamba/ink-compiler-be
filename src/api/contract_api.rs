use serde::{Deserialize, Serialize};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

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

#[post("/contract", data = "<wizard_message>")]
pub fn create_contract(
    compilation_queue: &State<Arc<CompilationQueue>>,
    db: &State<MongoRepo>,
    wizard_message: Json<WizardMessage>,
) -> Result<Json<ServerResponse<Contract>>, Custom<Json<ServerResponse<Contract>>>> {
    sanity_check(&wizard_message)?;

    let code_hash_str = hash_code(&wizard_message.code);
    debug!("hash_code completed");

    // Check if contract already exists in DB
    let contract_on_db = db.get_contract_by_hash(&code_hash_str);
    debug!("get_contract_by_hash completed");

    match contract_on_db {
        Ok(contract) => match contract {
            Some(mut contract) => {
                info!("Contract existing in the db with id: {:?}", &contract.id);
                contract.id = None;
                return Ok(Json(ServerResponse::new_valid(contract)));
            }
            None => (),
        },
        Err(_) => {
            error!("Error getting contract from db");
        }
    }

    let (tx, rx) = channel::<Result<Contract, String>>();
    let compilation_request = CompilationRequest {
        wizard_message: wizard_message.into_inner(),
        code_id: code_hash_str,
        tx: tx.clone(),
    };

    compilation_queue.add_request(compilation_request);
    let contract = rx.recv().unwrap();

    match contract {
        Ok(contract_unwrapped) => {
            let contract_save_result = db.create_contract(&contract_unwrapped);
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

            return Ok(Json(ServerResponse::new_valid(contract_unwrapped)));
        }
        Err(error_msg) => {
            error!("something bad happened");

            return Err(Custom(
                Status::InternalServerError,
                Json(ServerResponse::new_error(error_msg)),
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

    info!("create_deployment called with: {:?}", &deployment);
    match deployment_save_result {
        Ok(insert_one_result) => {
            info!("insert_one_result: {:?}", &insert_one_result);
            Ok(Json(ServerResponse::new_valid(String::from("ok"))))
        }

        Err(_) => {
            error!("something bad happened");
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

// #[post("/test_queue", data = "<message>")]
// pub fn test_queue(
//     compilation_queue: &State<Arc<CompilationQueue>>,
//     message: String,
// ) -> Result<Json<ServerResponse<String>>, Custom<Json<ServerResponse<String>>>> {
//     let (tx, rx) = channel();
//     let message = CompileRequest {
//         code: message,
//         id: 1.to_string(),
//         tx: tx.clone(),
//     };

//     compilation_queue.add_request(message);
//     let data = rx.recv().unwrap();
//     println!("data: {:?}", data);

//     Ok(Json(ServerResponse::new_valid(String::from("ok"))))
// }
