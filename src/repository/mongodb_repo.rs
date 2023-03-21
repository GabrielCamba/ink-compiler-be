use std::env;

use crate::models::contract_model::Contract;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    sync::{Client, Collection},
};

use log::{debug, error};
pub struct MongoRepo {
    col: Collection<Contract>,
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
        let db = client.database("rustDB");
        debug!("Connected to Database");
        let col: Collection<Contract> = db.collection("Contract");
        debug!("Connected to Collection");
        MongoRepo { col }
    }

    pub fn create_contract(&self, new_contract: Contract) -> Result<InsertOneResult, Error> {
        debug!("Entered MongoRepo::create_contract()");
        let contract = self
            .col
            .insert_one(new_contract, None)
            .ok()
            .expect("Error creating contract");
        Ok(contract)
    }

    pub fn get_contract_by_hash(&self, hash: &String) -> Result<Option<Contract>, Error> {
        debug!("Entered MongoRepo::get_contract_by_hash()");
        let filter = doc! {"code_id": hash};
        let contract = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("There was an error fetching the contract");
        Ok(contract)
    }
}
