use std::env;

use crate::models::api_models::GetDeploymentsMessage;
use crate::models::db_models::{Contract, Deployment};
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    sync::{Client, Collection},
};

use log::{debug, error};
pub struct MongoRepo {
    contracts: Collection<Contract>,
    deployments: Collection<Deployment>,
}

impl MongoRepo {
    pub fn init() -> Self {
        debug!("Entered MongoRepo::init()");
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => {
                error!("MONGOURI environment variable not set");
                format!("Error loading env variable")
            }
        };

        let client = Client::with_uri_str(uri).unwrap();
        debug!("Connected to MongoDB");
        let db = client.database("ContractWizard");
        debug!("Connected to Database");
        let contracts: Collection<Contract> = db.collection("Contracts");
        let deployments: Collection<Deployment> = db.collection("Deployments");

        MongoRepo {
            contracts,
            deployments,
        }
    }

    pub fn create_contract(&self, new_contract: &Contract) -> Result<InsertOneResult, Error> {
        debug!("Entered MongoRepo::create_contract()");
        let contract = self
            .contracts
            .insert_one(new_contract, None)
            .ok()
            .expect("Error creating contract");
        Ok(contract)
    }

    pub fn get_contract_by_hash(&self, hash: &String) -> Result<Option<Contract>, Error> {
        debug!("Entered MongoRepo::get_contract_by_hash()");
        let filter = doc! {"code_id": hash};
        let contract = self
            .contracts
            .find_one(filter, None)
            .ok()
            .expect("There was an error fetching the contract");
        Ok(contract)
    }

    pub fn create_deployment(&self, new_deployment: &Deployment) -> Result<InsertOneResult, Error> {
        debug!("Entered MongoRepo::create_deployment()");
        let deployment = self
            .deployments
            .insert_one(new_deployment, None)
            .ok()
            .expect("Error creating deployment");
        Ok(deployment)
    }

    pub fn get_deployments(
        &self,
        deployment_message: &GetDeploymentsMessage,
    ) -> Result<Vec<Deployment>, Error> {
        let filter;

        match &deployment_message.network {
            Some(network) if (network != "") => {
                filter = doc! {"address": &deployment_message.address, "network": &deployment_message.network};
            }
            _ => {
                filter = doc! {"address": &deployment_message.address};
            }
        }

        let deployments = self
            .deployments
            .find(filter, None)
            .ok()
            .expect("Error getting deployments"); //TODO it should return an error instead of panicking

        let deployments_vec: Vec<Deployment> = deployments
            .filter(|deployment| deployment.is_ok())
            .map(|deployment| deployment.expect("Error getting deployment"))
            .collect();

        Ok(deployments_vec)
    }
}
