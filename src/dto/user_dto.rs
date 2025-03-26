use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use zxcvbn::{zxcvbn, Score};

#[derive(Deserialize, Serialize, FromForm, Debug, Validate, JsonSchema)]
pub struct UserUpdateInput {
    #[validate(email(message = "Invalid email address"))]
    // #[schemars(example = "user@mail.com")]
    pub email: String,
}

#[derive(Deserialize, Serialize, FromForm, Debug, Validate, JsonSchema)]
pub struct UserLoginInput {
    #[validate(email(message = "Invalid email address"))]
    #[schemars(example = "user@mail.com")]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    #[schemars(example = "This;Is,a@Str0ngPassword==")]
    pub password: String,
}

#[derive(Deserialize, Serialize, FromForm, Debug, Validate, JsonSchema)]
pub struct UserRegisterInput {
    #[validate(email(message = "Invalid email address"))]
    #[schemars(example = "user@mail.com")]
    pub email: String,

    #[validate(
        length(min = 8, message = "Password must be at least 8 characters long"),
        custom(function = "validate_password")
    )]
    #[schemars(example = "This;Is,a@Str0ngPassword==")]
    pub password: String,

    #[validate(
        must_match(other = "password", message = "Passwords do not match")
    )]
    #[schemars(example = "This;Is,a@Str0ngPassword==")]
    pub password_confirmation: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let estimation = zxcvbn(password, &[]).score();

    if estimation == Score::Three || estimation == Score::Four {
        return Ok(());
    }

    Err(ValidationError::new("weak_password")
        .with_message(Cow::Borrowed(
            "Password is too weak, please use a stronger password (including numbers, special characters, and capital letters)",
        ))
        .into())
}
