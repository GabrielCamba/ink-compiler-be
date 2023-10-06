use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::api_models::DeployMessage;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contract {
    //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing)]
    pub id: Option<ObjectId>,
    pub code_id: String,
    pub metadata: String,
    pub wasm: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Deployment {
    //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing)]
    pub id: Option<ObjectId>,
    pub contract_name: Option<String>,
    pub contract_address: String,
    pub network: String,
    pub code_id: String,
    pub user_address: String,
    pub tx_hash: Option<String>,
    pub date: String,
    pub contract_type: String,
    pub external_abi: Option<String>,
    pub hidden: bool,
}

impl Deployment {
    pub fn new(deploy_message: &DeployMessage) -> Self {
        Deployment {
            id: None,
            contract_name: deploy_message.contract_name.clone(),
            contract_address: deploy_message.contract_address.clone(),
            network: deploy_message.network.clone(),
            code_id: deploy_message.code_id.clone(),
            user_address: deploy_message.user_address.clone(),
            tx_hash: deploy_message.tx_hash.clone(),
            date: deploy_message.date.clone(),
            contract_type: deploy_message.contract_type.clone(),
            external_abi: deploy_message.external_abi.clone(),
            hidden: false,
        }
    }
}
