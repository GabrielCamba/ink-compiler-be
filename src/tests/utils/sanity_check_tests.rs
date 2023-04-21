#[cfg(test)]
mod sanity_check_tests {

    use crate::models::db_models::Contract;

    use super::super::*;

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
            Json(ServerResponse::<Contract>::new_error(String::from(
                "Code size too big.",
            ))),
        ));

        let result = sanity_check_wizard_message(&Json(wizard_message));
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
            Json(ServerResponse::<Contract>::new_error(String::from(
                "Address is not valid.",
            ))),
        ));

        let result = sanity_check_wizard_message(&Json(wizard_message));
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
            Json(ServerResponse::<Contract>::new_error(String::from(
                "Features must not be empty.",
            ))),
        ));

        let result = sanity_check_wizard_message(&Json(wizard_message));
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
            Json(ServerResponse::<Contract>::new_error(String::from(
                "Feature not allowed",
            ))),
        ));
        let result = sanity_check_wizard_message(&Json(wizard_message));
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
            Json(ServerResponse::<Contract>::new_error(String::from(
                "Feature contains ambiguous contract standard",
            ))),
        ));
        let result = sanity_check_wizard_message(&Json(wizard_message));
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
            Json(ServerResponse::<Contract>::new_error(String::from(
                "Features must contain at least one contract standard",
            ))),
        ));
        let result = sanity_check_wizard_message(&Json(wizard_message));
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

        let result = sanity_check_wizard_message(&Json(wizard_message));
        assert_eq!(result, expected_result);
        assert_eq!(result.is_err(), false);
    }
}
