use crate::repositories::error::DbRepoError;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::Request;
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database Error")]
    DbError(#[from] DbRepoError),
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Validation error")]
    ValidationError { errors: ValidationErrors },

    #[error("{message}")]
    CustomError { status_code: u16, message: String },
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    details: Option<Vec<String>>,
    code: u16,
}

impl AppError {
    pub fn new(status_code: u16, message: &str) -> Self {
        AppError::CustomError {
            status_code,
            message: message.to_string(),
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            AppError::DbError { .. } => 500,
            AppError::BadRequest => 400,
            AppError::Unauthorized => 401,
            AppError::Forbidden => 403,
            AppError::NotFound => 404,
            AppError::InternalServerError => 500,
            AppError::ValidationError { .. } => 400,
            AppError::CustomError { status_code, .. } => *status_code,
        }
    }
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status_code = self.status_code();
        let status = Status::from_code(status_code).unwrap_or(Status::InternalServerError);

        let body = serde_json::to_string(&ErrorResponse {
            error: self.to_string(),
            details: match self {
                AppError::DbError(err) => Some(vec![err.to_string()]),
                AppError::ValidationError { errors } => Some(
                    errors
                        .field_errors()
                        .into_iter()
                        .flat_map(|(field, errs)| {
                            errs.into_iter().map(move |error| {
                                error
                                    .message
                                    .as_ref()
                                    .map(|msg| msg.to_string())
                                    .unwrap_or_else(|| field.to_string())
                            })
                        })
                        .collect(),
                ),
                _ => None,
            },
            code: status_code,
        })
        .unwrap();

        Response::build_from(body.respond_to(req)?)
            .status(status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[allow(dead_code)]
pub trait AppErr<T, E> {
    fn app_err(self, status_code: u16, message: &str) -> Result<T, AppError>;
}

impl<T, E> AppErr<T, E> for Result<T, E>
where
    E: core::fmt::Display,
{
    fn app_err(self, status_code: u16, message: &str) -> Result<T, AppError> {
        self.map_err(|_| AppError::new(status_code, message))
    }
}

impl<T> AppErr<T, ()> for Option<T> {
    fn app_err(self, status_code: u16, message: &str) -> Result<T, AppError> {
        self.map_or_else(|| Err(AppError::new(status_code, message)), Ok)
    }
}

#[macro_export]
macro_rules! app_err {
    ($status_code:expr, $message:expr) => {
        Err($crate::error::app_error::AppError::new(
            $status_code,
            $message,
        ))
    };
}

#[macro_export]
macro_rules! app_err_bail {
    ($status_code:expr, $message:expr) => {
        return Err($crate::error::app_error::AppError::new(
            $status_code,
            $message,
        ));
    };
}

#[macro_export]
macro_rules! app_err_ensure {
    ($condition:expr, $status_code:expr, $message:expr) => {
        if !$condition {
            return Err($crate::error::app_error::AppError::new(
                $status_code,
                $message,
            ));
        }
    };
}
