use std::env;

use crate::models::contract_model::Contract;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Contract>,
}

impl MongoRepo {
    pub fn init() -> Self {
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Contract> = db.collection("Contract");
        MongoRepo { col }
    }

    pub fn create_contract(&self, new_contract: Contract) -> Result<InsertOneResult, Error> {
        let new_doc = Contract {
            id: None,
            code_hash: new_contract.code_hash,
            metadata: new_contract.metadata,
            wasm: new_contract.wasm,
        };
        let contract = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating contract");
        Ok(contract)
    }

    pub fn get_contract(&self, id: &String) -> Result<Contract, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let contract_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting contract's detail");
        Ok(contract_detail.unwrap())
    }

    pub fn get_contract_by_hash(&self, hash: &String) -> Result<Option<Contract>, Error> {
        let filter = doc! {"code_hash": hash};
        let contract = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("There was an error fetching the contract");
        Ok(contract)
    }

}
