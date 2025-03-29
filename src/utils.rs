use rocket::Responder;
use rocket::serde::Serialize;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

// Define a custom Either type to handle different response types
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

// Implement Responder for Either
impl<'r, L: rocket::response::Responder<'r, 'static>, R: rocket::response::Responder<'r, 'static>> rocket::response::Responder<'r, 'static> for Either<L, R> {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            Either::Left(left) => left.respond_to(req),
            Either::Right(right) => right.respond_to(req),
        }
    }
}