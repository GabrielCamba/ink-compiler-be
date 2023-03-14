use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contract {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub code_hash: String,
    pub metadata: String,
    pub wasm: Vec<u8>, // TODO Review if this should be a String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WizardMessage {
    pub address: String,
    pub code: String,
    pub features: Vec<String>,
}
