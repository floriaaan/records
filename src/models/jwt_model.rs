use std::env;

use jsonwebtoken::{decode, encode, errors::{Error, ErrorKind}, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{http::Status, request::{FromRequest, Outcome, Request}};
use serde::{Deserialize, Serialize};


use crate::utils::{NetworkResponse, Response, ResponseBody};

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaim {
    pub sub: i32, // user_id

    pub iat: usize, // issued at
    pub exp: usize, // expiration
}

pub async fn generate_jwt(user_id: i32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let jwt_claim = JwtClaim {
        sub: user_id,
        iat: chrono::Utc::now().timestamp() as usize,
        exp: (chrono::Utc::now() + chrono::Duration::hours(6)).timestamp() as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &jwt_claim, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn decode_jwt(token: String) -> Result<JwtClaim, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    println!("Token: {}", token);

    match decode::<JwtClaim>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtClaim {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<JwtClaim, Error> {
            let claim = decode_jwt(String::from(key))?;

            println!("Claim: {:?}", claim);

            Ok(claim)
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = Response{ body: ResponseBody::Message(String::from("Error validating JWT token - No token provided"))};
                Outcome::Failure((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(claims),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - Expired Token"))};
                        Outcome::Failure((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - Invalid Token"))};
                        Outcome::Failure((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
                    },
                    _ => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - {}", err))};
                        Outcome::Failure((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
                    }
                }
            },
        }
    }
}