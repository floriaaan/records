use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromForm, Debug)]
pub struct UserEmail {
    pub email: String,
}


#[derive(Deserialize, Serialize, FromForm, Debug)]
pub struct UserInput {
    pub email: String,
    pub password: String,
}