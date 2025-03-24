use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use zxcvbn::{zxcvbn, Score};

#[derive(Deserialize, Serialize, FromForm, Debug, Validate)]
pub struct UserEmail {
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Serialize, FromForm, Debug, Validate)]
pub struct UserInput {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8), custom(function = "validate_password"))]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let estimation = zxcvbn(password, &[]).score();

    if estimation == Score::Three || estimation == Score::Four {
        return Ok(());
    }

    Err(ValidationError::new("weak_password")
        .with_message(Cow::Borrowed("Password is too weak, please use a stronger password"))
        .into())
}
