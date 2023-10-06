#[cfg(test)]
mod post_deployments_test {
    use super::super::*;
    use crate::MongoRepo;
    use mongodb::bson::doc;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn post_deployments_missing_address_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/deployments")).body(r#"{ }"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert!(response.into_string().unwrap().contains(
            "The request was well-formed but was unable to be followed due to semantic errors."
        ));
        client.terminate();
    }

    #[test]
    fn post_deployments_missing_network_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/deployments"))
            .body(r#"{ "contract_address": "some_address" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert!(response.into_string().unwrap().contains(
            "The request was well-formed but was unable to be followed due to semantic errors."
        ));
        client.terminate();
    }

    #[test]
    fn post_deployments_missing_code_id_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/deployments"))
            .body(r#"{ "contract_address": "some_address", "network": "some_network" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert!(response.into_string().unwrap().contains(
            "The request was well-formed but was unable to be followed due to semantic errors."
        ));
        client.terminate();
    }

    #[test]
    fn post_deployments_missing_user_address_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/deployments")).body(r#"{ "contract_address": "some_address", "network": "some_network", "code_id": "some_id" }"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        assert!(response.into_string().unwrap().contains(
            "The request was well-formed but was unable to be followed due to semantic errors."
        ));
        client.terminate();
    }

    #[test]
    fn post_deployments_invalid_address_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/deployments")).body(r#"{ "contract_address": "some_address", "network": "some_network", "code_id": "some_id", "user_address": "some_user_address", "date":"2021-03-03T15:00:00.000Z", "contract_type":"custom" }"#).dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
        assert!(response
            .into_string()
            .unwrap()
            .contains("Invalid address length"));
        client.terminate();
    }

    #[test]
    fn post_deployments_contract_address_error() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!("/deployments")).body(r#"{ "contract_address": "some_address", "network": "some_network", "code_id": "some_id", "user_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "date":"2021-03-03T15:00:00.000Z", "contract_type": "psp22" }"#).dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
        assert!(response
            .into_string()
            .unwrap()
            .contains("Invalid address length"));
        client.terminate();
    }

    #[test]
    fn post_deployments_empty_data_is_ok() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let db = client.rocket().state::<MongoRepo>().unwrap();
        let response = client.post(uri!("/deployments")).body(r#"{ "contract_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "network": "some_network", "code_id": "some_id", "user_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "date":"2021-03-03T15:00:00.000Z", "contract_type":"custom" }"#).dispatch();
        // status ok means that the deployment was stored in the database
        assert_eq!(response.status(), Status::Ok);

        let db_res = db.deployments.delete_one(
            doc! {"contract_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","network": "some_network", "user_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"},
            None,
        );
        assert!(db_res.is_ok());
        std::mem::drop(response);
        client.terminate();
    }

    #[test]
    fn patch_deployments_update_is_ok() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let db = client.rocket().state::<MongoRepo>().unwrap();

        let response = client.post(uri!("/deployments")).body(r#"{ "contract_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "network": "some_network", "code_id": "some_id", "user_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "date":"2021-03-03T15:00:00.000Z", "contract_type":"custom" }"#).dispatch();
        // status ok means that the deployment was stored in the database
        assert_eq!(response.status(), Status::Ok);
        std::mem::drop(response);

        let response = client.patch(uri!("/deployments")).body(r#"{ "contract_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "network": "some_network",  "user_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",  "contract_name":"name", "hidden": true}"#).dispatch();
        // status ok means that the deployment was updated in the database
        assert_eq!(response.status(), Status::Ok);
        std::mem::drop(response);

        // Cleanup
        let db_res = db.deployments.delete_one(
            doc! {"contract_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","network": "some_network", "user_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"},
            None,
        );
        assert!(db_res.is_ok());
        client.terminate();
    }
}
