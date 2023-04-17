use std::env;

use crate::models::api_models::GetDeploymentsMessage;
use crate::models::db_models::{Contract, Deployment};
use mongodb::{
    bson::doc,
    results::InsertOneResult,
    sync::{Client, Collection},
};

use log::{debug, error};
pub struct MongoRepo {
    contracts: Collection<Contract>,
    deployments: Collection<Deployment>,
}

// Mongo Repo implementation
impl MongoRepo {
    // Create a new Mongo Repo
    pub fn init() -> Self {
        // Generating dabase connection
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => {
                error!(target: "compiler", "MONGOURI environment variable not set");
                std::process::exit(1);
            }
        };

        let client = match Client::with_uri_str(uri) {
            Ok(v) => v,
            Err(_) => {
                error!(target: "compiler", "Error connecting to MongoDB");
                std::process::exit(1);
            }
        };

        let db = client.database("ContractWizard");
        let contracts: Collection<Contract> = db.collection("Contracts");
        let deployments: Collection<Deployment> = db.collection("Deployments");

        // Test db is up and running
        let ping_database = client
            .database("ContractWizard")
            .run_command(doc! {"ping": 1}, None);

        // Checking the response
        match ping_database {
            Ok(_) => debug!(target: "compiler", "Connected to Database"),
            _ => {
                error!(target: "compiler", "Error connecting to database. Connection timed out");
                std::process::exit(1);
            }
        }

        MongoRepo {
            contracts,
            deployments,
        }
    }

    // Insert a new contract into the database
    pub fn create_contract(&self, new_contract: &Contract) -> Result<InsertOneResult, Box<dyn std::error::Error>> {
        let contract = self
            .contracts
            .insert_one(new_contract, None)?;        
        Ok(contract)
    }

    // Get an existing contract from the DB
    pub fn get_contract_by_hash(&self, hash: &String) -> Result<Option<Contract>,  Box<dyn std::error::Error>> {
        let filter = doc! {"code_id": hash};
        let contract = self
            .contracts
            .find_one(filter, None)?;
        Ok(contract)
    }

    // Create a deployment in the database
    pub fn create_deployment(&self, new_deployment: &Deployment) -> Result<InsertOneResult, Box<dyn std::error::Error>> {
        let deployment = self
            .deployments
            .insert_one(new_deployment, None)?;
        Ok(deployment)
    }

    // Fetch stored deployments from the db
    pub fn get_deployments(
        &self,
        deployment_message: &GetDeploymentsMessage,
    ) -> Result<Vec<Deployment>, Box<dyn std::error::Error>> {
        let filter;

        match &deployment_message.network {
            Some(network) if (network != "") => {
                filter = doc! {"user_address": &deployment_message.user_address, "network": &deployment_message.network};
            }
            _ => {
                filter = doc! {"user_address": &deployment_message.user_address};
            }
        }

        let deployments = self
            .deployments
            .find(filter, None)?;

        let deployments_vec: Vec<Deployment> = deployments
            .filter(|deployment| deployment.is_ok())
            .map(|deployment| {
                deployment.expect("This will never panic because of the filter above")
            })
            .collect();

        Ok(deployments_vec)
    }
}
