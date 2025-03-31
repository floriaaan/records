use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use zxcvbn::{zxcvbn, Score};

use regex::Regex;
use std::sync::LazyLock;

static VALID_USERNAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9_]+$").unwrap()
});

#[derive(Deserialize, Serialize, FromForm, Debug, Validate)]
pub struct UserUpdateInput {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    
    #[validate(
        length(min = 3, message = "Username must be at least 3 characters long"),
        regex(path = "VALID_USERNAME_REGEX", message = "Username can only contain alphanumeric characters and underscores")
    )]
    pub username: String,
}

#[derive(Deserialize, Serialize, FromForm, Debug, Validate)]
pub struct UserLoginInput {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Deserialize, Serialize, FromForm, Debug, Validate)]
pub struct UserRegisterInput {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(
        length(min = 3, message = "Username must be at least 3 characters long"),
        regex(path = "VALID_USERNAME_REGEX", message = "Username can only contain alphanumeric characters and underscores")
    )]
    pub username: String,
    
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters long"),
        custom(function = "validate_password")
    )]
    pub password: String,

    #[validate(
        must_match(other = "password", message = "Passwords do not match")
    )]
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
