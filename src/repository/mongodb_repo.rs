use std::env;

use crate::models::api_models::{GetDeploymentsMessage, UpdateDeployMessage};
use crate::models::db_models::{Contract, Deployment};
use mongodb::results::UpdateResult;
use mongodb::{
    bson::doc,
    results::InsertOneResult,
    sync::{Client, Collection},
};

use log::{debug, error};
pub struct MongoRepo {
    pub contracts: Collection<Contract>,
    pub deployments: Collection<Deployment>,
}

// Mongo Repo implementation
impl MongoRepo {
    // Create a new Mongo Repo
    pub fn init() -> Self {
        // Generating dabase connection
        let uri = match env::var("MONGOURI") {
            Ok(v) => v,
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
    pub fn create_contract(
        &self,
        new_contract: &Contract,
    ) -> Result<InsertOneResult, Box<dyn std::error::Error>> {
        let contract = self.contracts.insert_one(new_contract, None)?;
        Ok(contract)
    }

    // Get an existing contract from the DB
    pub fn get_contract_by_hash(
        &self,
        hash: &String,
    ) -> Result<Option<Contract>, Box<dyn std::error::Error>> {
        let filter = doc! {"code_id": hash};
        let contract = self.contracts.find_one(filter, None)?;
        Ok(contract)
    }

    // Create a deployment in the database
    pub fn create_deployment(
        &self,
        new_deployment: &Deployment,
    ) -> Result<InsertOneResult, Box<dyn std::error::Error>> {
        let deployment = self.deployments.insert_one(new_deployment, None)?;
        Ok(deployment)
    }

    // Update a deployment in the database
    pub fn update_deployment(
        &self,
        update_deployment: &UpdateDeployMessage,
    ) -> Result<UpdateResult, Box<dyn std::error::Error>> {
        let filter = doc! {"contract_address": &update_deployment.contract_address, "network": &update_deployment.network, "user_address": &update_deployment.user_address};
        let deployment = self
            .deployments.update_one(filter, doc! {"$set": {"contract_name": &update_deployment.contract_name, "hidden": &update_deployment.hidden}}, None)?;
        Ok(deployment)
    }

    // Fetch stored deployments from the db
    pub fn get_deployments(
        &self,
        deployment_message: &GetDeploymentsMessage,
    ) -> Result<Vec<Deployment>, Box<dyn std::error::Error>> {
        let mut filter = doc! {"user_address": &deployment_message.user_address};

        if let Some(network) = &deployment_message.network {
            if !network.is_empty() {
                filter.insert("network", network);
            }
        }

        if let Some(contract_address) = &deployment_message.contract_address {
            if !contract_address.is_empty() {
                filter.insert("contract_address", contract_address);
            }
        }

        let deployments = self.deployments.find(filter, None)?;

        let deployments_vec: Vec<Deployment> = deployments
            .filter(|deployment| deployment.is_ok())
            .map(|deployment| {
                deployment.expect("This will never panic because of the filter above")
            })
            .collect();

        Ok(deployments_vec)
    }
}
