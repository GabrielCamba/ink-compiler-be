use crate::utils::constants::CONTRACTS;
use log::error;
use rocket::{http::Status, serde::json::Json, response::status::Custom};
use crate::models::contract_model::{ServerResponse, WizardMessage};

pub fn sanity_check(wizard_message: &Json<WizardMessage>) -> Result<(), Custom<Json<ServerResponse>>> {
    // Sanity check WizardMessage data codesize , check address, y freatures not empty ( must include psp22, psp34 or psp37)
    if wizard_message.code.len() > 49999 {
        error!("Code size is too big");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Code size too big.",
            ))),
        ));
    }

    if wizard_message.address.len() != 41 {
        error!("Address is not valid");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from("Invalid address."))),
        ));
    }

    if wizard_message.features.is_empty() {
        error!("Features are empty");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Features must not be empty.",
            ))),
        ));
    }

    let mut found = false;

    for feature in &wizard_message.features {
        if CONTRACTS.contains(&feature.as_str()) {
            if !found {
                found = true;
            } else {
                error!("Feature contains ambiguous contract standard");
                return Err(Custom(
                    Status::InternalServerError,
                    Json(ServerResponse::new_error(String::from(
                        "Features must contain at least one contract standard",
                    )))
                ));
            }
        }
    }
    if !found {
        error!("Features must contain at least one contract standard");
        return Err(Custom(
            Status::InternalServerError,
            Json(ServerResponse::new_error(String::from(
                "Features must contain at least one contract standard",
            )))
        ));
    }
    Ok(())
}
