#[cfg(test)]
mod main_contract_test {
    use super::super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    const VALID_INK_SC :&str= r#"#![cfg_attr(not(feature = \"std\"), no_std)] #![feature(min_specialization)] #[openbrush::contract] pub mod my_psp21 { use openbrush::contracts::psp22::*; use openbrush::traits::Storage; #[ink(storage)] #[derive(Default, Storage)] pub struct Contract { #[storage_field] psp22: psp22::Data, } impl PSP22 for Contract {} impl Contract { #[ink(constructor)] pub fn new(initial_supply: Balance) -> Self { let mut _instance = Self::default(); _instance._mint_to(_instance.env().caller(), initial_supply); _instance } } }"#;

    #[test]
    fn post_contract_missing_address_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/contract")).body(r#"{ }"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn post_contract_missing_no_code_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/contract"))
            .body(r#"{ "address": "asdf" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn post_contract_missing_no_features_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/contract"))
            .body(r#"{ "address": "address", "code": "something" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn post_contract_expected_features_to_be_an_array() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/contract"))
            .body(r#"{ "address": "address", "code": "something", "features": "asdf" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn post_contract_expects_a_valid_len_address() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/contract"))
            .body(r#"{ "address": "address", "code": "something", "features": ["asdf"] }"#)
            .dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
        assert!(response
            .into_string()
            .unwrap()
            .contains("Address is not valid."));
    }

    #[test]
    fn post_contract_expects_a_valid_feature() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/contract"))
            .body(r#"{ "address": "4GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "code": "something", "features": ["asdf"] }"#)
            .dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
        assert!(response
            .into_string()
            .unwrap()
            .contains("Feature not allowed"));
    }

    #[test]
    fn post_contract_expects_code_not_to_be_too_long() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let one_mb_string = "a".repeat(999999);
        let body = format!(
            r#"{{ "address": "4GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "code": "{}", "features": ["psp22"] }}"#,
            one_mb_string
        );
        let response = client.post(uri!("/contract")).body(body).dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
        let res_str = response.into_string().unwrap();
        assert!(res_str.contains("Code size too big."));
    }

    #[test]
    fn post_contract_expects_code_is_ok() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let body = format!(
            r#"{{ "address": "4GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "code": "{}", "features": ["psp22"] }}"#,
            VALID_INK_SC
        );
        let response = client.post(uri!("/contract")).body(body).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}