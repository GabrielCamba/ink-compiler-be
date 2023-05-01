#[cfg(test)]
mod main_get_contract_test {
    use crate::models::api_models::ServerResponse;

    use super::super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use crate::models::db_models::Contract;

    const VALID_INK_SC :&str= r#"#![cfg_attr(not(feature = \"std\"), no_std)] #![feature(min_specialization)] #[openbrush::contract] pub mod my_psp21 { use openbrush::contracts::psp22::*; use openbrush::traits::Storage; #[ink(storage)] #[derive(Default, Storage)] pub struct Contract { #[storage_field] psp22: psp22::Data, } impl PSP22 for Contract {} impl Contract { #[ink(constructor)] pub fn new(initial_supply: Balance) -> Self { let mut _instance = Self::default(); _instance._mint_to(_instance.env().caller(), initial_supply); _instance } } }"#;

    #[test]
    fn get_contract_no_matching_routes_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/contract")).dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert!(response
            .into_string()
            .unwrap()
            .contains("The requested resource could not be found."));
    }

    #[test]
    fn get_contract_missing_code_id_returns_not_found_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/contract?code_id=")).dispatch();
        assert_eq!(response.status(), Status::NotFound);
        //println!("{}", response.into_string().unwrap());
        assert!(response
            .into_string()
            .unwrap()
            .contains("Contract not found."));
    }

    // This tests is assuming code id 1 is not used
    #[test]
    fn get_contract_not_existing_code_id_returns_not_found_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/contract?code_id=1")).dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert!(response
            .into_string()
            .unwrap()
            .contains("Contract not found."));
    }
    
    #[test]
    fn get_contract_working_as_expected() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let body = format!(
            r#"{{ "address": "4GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "code": "{}", "features": ["psp22"] }}"#,
            VALID_INK_SC
        );
        client.post(uri!("/contract")).body(body).dispatch();
        let response = client.get(uri!("/contract?code_id=d066340268269918605ad56139b35f4c2e421349b380166535f6ab17beeaf1fc")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let json: ServerResponse<Contract> = response.into_json().unwrap();
        let contract = json.data.unwrap();
        assert!(!contract.metadata.is_empty());
        assert_eq!(contract.code_id, "d066340268269918605ad56139b35f4c2e421349b380166535f6ab17beeaf1fc");
        assert!(contract.id.is_none());
        assert!(contract.wasm.is_empty());
        assert!(!contract.metadata.is_empty());
    }

}