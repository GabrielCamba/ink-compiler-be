use crate::models::api_models::{ServerResponse, WizardMessage};
use crate::models::db_models::Contract;
use crate::utils::constants::{ALLOWED_FEATURES, CONTRACTS, MAX_SIZE_ALLOWED};
use log::error;
use rocket::{http::Status, response::status::Custom, serde::json::Json};

pub fn sanity_check(
    wizard_message: &Json<WizardMessage>,
) -> Result<(), Custom<Json<ServerResponse<Contract>>>> {
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
#[path = "../tests/utils/sanity_check_tests.rs"]
mod sanity_check_tests;
