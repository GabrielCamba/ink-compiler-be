use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contract {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub code_id: String,
    pub metadata: String,
    pub wasm: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerError {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerResponse {
    pub contract: Option<Contract>,
    pub error: Option<ServerError>,
}
impl ServerResponse {
    pub fn new_valid(contract: Contract) -> Self {
        ServerResponse {
            contract: Some(contract),
            error: None,
        }
    }

    pub fn new_error(message: String) -> Self {
        ServerResponse {
            contract: None,
            error: Some(ServerError { message }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WizardMessage {
    pub address: String,
    pub code: String,
    pub features: Vec<String>,
}
