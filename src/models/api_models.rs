use serde::{Deserialize, Serialize};

// Generic server response
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ServerResponse<T> {
    pub data: Option<T>,
    pub error: Option<ServerError>,
}
impl<T> ServerResponse<T> {
    pub fn new_valid(data: T) -> Self {
        ServerResponse {
            data: Some(data),
            error: None,
        }
    }

    pub fn new_error(message: String) -> Self {
        ServerResponse {
            data: None,
            error: Some(ServerError { message }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ServerError {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WizardMessage {
    pub address: String,
    pub code: String,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployMessage {
    pub contract_name: Option<String>,
    pub contract_address: String,
    pub network: String,
    pub code_id: String,
    pub user_address: String,
    pub tx_hash: Option<String>,
    pub date: String,
    pub contract_type: String,
    pub external_abi: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeploymentsMessage {
    pub user_address: String,
    pub network: Option<String>,
    pub contract_address: Option<String>,
}
