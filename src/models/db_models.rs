use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contract {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub code_id: String,
    pub metadata: String,
    pub wasm: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Deployment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub contract_name: Option<String>,
    pub address: String,
    pub network: String,
    pub code_id: String,
}
