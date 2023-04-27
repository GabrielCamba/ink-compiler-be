#[cfg(test)]
mod main_deployments_test {
    use super::super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn post_deployments_missing_address_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/deployments")).body(r#"{ }"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert!(response
            .into_string()
            .unwrap()
            .contains("The request was well-formed but was unable to be followed due to semantic errors."));
    }

    #[test]
    fn post_deployments_missing_network_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/deployments")).body(r#"{ "contract_address": "some_address" }"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert!(response
            .into_string()
            .unwrap()
            .contains("The request was well-formed but was unable to be followed due to semantic errors."));
    }

    #[test]
    fn post_deployments_missing_code_id_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/deployments")).body(r#"{ "contract_address": "some_address", "network": "some_network" }"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert!(response
            .into_string()
            .unwrap()
            .contains("The request was well-formed but was unable to be followed due to semantic errors."));
    }

    #[test]
    fn post_deployments_missing_user_address_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/deployments")).body(r#"{ "contract_address": "some_address", "network": "some_network", "code_id": "some_id" }"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert!(response
            .into_string()
            .unwrap()
            .contains("The request was well-formed but was unable to be followed due to semantic errors."));
    }

    #[test]
    fn post_deployments_invalid_address_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/deployments")).body(r#"{ "contract_address": "some_address", "network": "some_network", "code_id": "some_id", "user_address": "some_user_address" }"#).dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
        assert!(response
            .into_string()
            .unwrap()
            .contains("Invalid address length"));
    }

    // TODO CONTRACT ADDRESS LEN and test it
    // TODO Descriptive strings
    // TODO Check Empty data is ok
    #[test]
    fn post_deployments_empty_data_is_ok() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/deployments")).body(r#"{ "contract_address": "some_address", "network": "some_network", "code_id": "some_id", "user_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" }"#).dispatch();
        assert_eq!(response.status(), Status::Ok);
        println!("{:?}", response.into_string());
    }

}