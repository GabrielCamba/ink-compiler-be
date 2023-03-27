use crate::models::contract_model::{ServerResponse, WizardMessage};
use crate::utils::constants::{ALLOWED_FEATURES, CONTRACTS, MAX_SIZE_ALLOWED};
use log::error;
use rocket::{http::Status, response::status::Custom, serde::json::Json};

pub fn sanity_check(
    wizard_message: &Json<WizardMessage>,
) -> Result<(), Custom<Json<ServerResponse>>> {
    // Checks length of the code not passing the max allowed
    if wizard_message.code.len() > MAX_SIZE_ALLOWED {
        error!("Code size is too big");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Code size too big.",
            ))),
        ));
    }

    // Checks the address len is valid
    if wizard_message.address.len() != 48 {
        error!("Address is not valid");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from("Invalid address."))),
        ));
    }

    // Checks features not to be empty
    if wizard_message.features.is_empty() {
        error!("Features are empty");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Features must not be empty.",
            ))),
        ));
    }

    // Checks all the features passed are allowed
    for feature in &wizard_message.features {
        if !ALLOWED_FEATURES.contains(&feature.as_str()) {
            error!("Feature not allowed: {:?}", feature);
            return Err(Custom(
                Status::InternalServerError,
                Json(ServerResponse::new_error(String::from(
                    "Feature not allowed",
                ))),
            ));
        }
    }

    // sets the found flag
    let mut found = false;

    // found flag is used to check the contract has a single and allowed standard
    for feature in &wizard_message.features {
        if CONTRACTS.contains(&feature.as_str()) {
            if !found {
                found = true;
            } else {
                error!("Feature contains ambiguous contract standard");
                return Err(Custom(
                    Status::InternalServerError,
                    Json(ServerResponse::new_error(String::from(
                        "Feature contains ambiguous contract standard",
                    ))),
                ));
            }
        }
    }
    // here it checks at least one standard was found
    if !found {
        error!("Features must contain at least one contract standard");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Features must contain at least one contract standard",
            ))),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod sanity_check_tests {

    use super::*;

    const BOB: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

    #[test]
    fn test_sanity_check_on_code_size_greater_than_maximum_allowed_size() {
        // Test case when the code size is greater than the maximum allowed size
        let wizard_message = WizardMessage {
            code: String::from_utf8(vec![b'1'; MAX_SIZE_ALLOWED + 1]).unwrap(),
            address: String::from(BOB),
            features: vec![String::from("psp22"), String::from("pausable")],
        };

        let expected_error = Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Code size too big.",
            ))),
        ));

        let result = sanity_check(&Json(wizard_message));
        assert_eq!(result, expected_error);
        assert_eq!(result.is_err(), true);
        let error = &result.err().unwrap();
        assert_eq!(error.0, Status::InternalServerError);
    }

    #[test]
    fn test_sanity_check_on_wrong_address() {
        let wizard_message = WizardMessage {
            code: String::from_utf8(vec![b'1'; MAX_SIZE_ALLOWED - 1]).unwrap(),
            address: String::from("0x00e329d1fb7166f9cdf6a9e6cb62b6e5dfdd67ea"),
            features: vec![String::from("psp22"), String::from("pausable")],
        };

        let expected_error = Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from("Invalid address."))),
        ));

        let result = sanity_check(&Json(wizard_message));
        assert_eq!(result, expected_error);
        assert_eq!(result.is_err(), true);
        let error = &result.err().unwrap();
        assert_eq!(error.0, Status::InternalServerError);
    }

    #[test]
    fn test_sanity_check_on_empty_features() {
        let wizard_message = WizardMessage {
            code: String::from_utf8(vec![b'1'; MAX_SIZE_ALLOWED - 1]).unwrap(),
            address: String::from(BOB),
            features: vec![],
        };

        let expected_error = Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Features must not be empty.",
            ))),
        ));

        let result = sanity_check(&Json(wizard_message));
        assert_eq!(result, expected_error);
        assert_eq!(result.is_err(), true);
        let error = &result.err().unwrap();
        assert_eq!(error.0, Status::InternalServerError);
    }

    #[test]
    fn test_sanity_check_on_not_valid_features() {
        let wizard_message = WizardMessage {
            code: String::from_utf8(vec![b'1'; MAX_SIZE_ALLOWED - 1]).unwrap(),
            address: String::from(BOB),
            features: vec![
                String::from("psp22"),
                String::from("pausable"),
                String::from("recoverable"),
            ],
        };

        let expected_error = Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Feature not allowed",
            ))),
        ));
        let result = sanity_check(&Json(wizard_message));
        assert_eq!(result, expected_error);
        assert_eq!(result.is_err(), true);
        let error = &result.err().unwrap();
        assert_eq!(error.0, Status::InternalServerError);
    }

    #[test]
    fn test_sanity_check_on_ambiguous_standard() {
        let wizard_message = WizardMessage {
            code: String::from_utf8(vec![b'1'; MAX_SIZE_ALLOWED - 1]).unwrap(),
            address: String::from(BOB),
            features: vec![
                String::from("psp22"),
                String::from("pausable"),
                String::from("psp34"),
            ],
        };

        let expected_error = Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Feature contains ambiguous contract standard",
            ))),
        ));
        let result = sanity_check(&Json(wizard_message));
        assert_eq!(result, expected_error);
        assert_eq!(result.is_err(), true);
        let error = &result.err().unwrap();
        assert_eq!(error.0, Status::InternalServerError);
    }

    #[test]
    fn test_sanity_check_on_no_standard() {
        let wizard_message = WizardMessage {
            code: String::from_utf8(vec![b'1'; MAX_SIZE_ALLOWED - 1]).unwrap(),
            address: String::from(BOB),
            features: vec![String::from("pausable")],
        };

        let expected_error = Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Features must contain at least one contract standard",
            ))),
        ));
        let result = sanity_check(&Json(wizard_message));
        assert_eq!(result, expected_error);
        assert_eq!(result.is_err(), true);
        let error = &result.err().unwrap();
        assert_eq!(error.0, Status::InternalServerError);
    }

    #[test]
    fn test_sanity_check_on_success() {
        // Test case when the code size is greater than the maximum allowed size
        let wizard_message = WizardMessage {
            code: String::from_utf8(vec![b'1'; MAX_SIZE_ALLOWED - 1]).unwrap(),
            address: String::from(BOB),
            features: vec![String::from("psp22"), String::from("pausable")],
        };

        let expected_result = Ok(());

        let result = sanity_check(&Json(wizard_message));
        assert_eq!(result, expected_result);
        assert_eq!(result.is_err(), false);
    }
}
