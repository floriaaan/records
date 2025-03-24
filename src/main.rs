#[macro_use]
extern crate rocket;

pub mod app;
pub mod config;
pub mod db;
pub mod utils;

mod error {
    pub mod app_error;
    pub mod logging;
}

mod controllers {
    pub mod record_controller;
    pub mod auth_controller;
    pub mod user_controller;
}
mod use_cases {
    pub mod record_use_case;
    pub mod user_use_case;
    pub mod auth_use_case;
    pub mod use_cases;
}
mod repositories {
    pub mod error;
    pub mod record_repo;
    pub mod repositories;
    pub mod user_repo;
}

mod models {
    pub mod record_model;
    pub mod user_model;
    pub mod jwt_model;
}

mod dto {
    pub mod record_dto;
    pub mod user_dto;
    pub mod discogs_dto;
    pub mod spotify_dto;
}

#[cfg(test)]
mod test {
    pub mod app;
    pub mod db;
    pub mod fixture {
        pub mod record;
        pub mod user;
    }

    pub mod repositories {
        pub mod prepare {
            pub mod record;
            pub mod user;

        }
    }
}

use crate::app::create_app;
use crate::config::Config;
use crate::controllers::{record_controller, user_controller, auth_controller};
use crate::db::Db;
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::config::<Config>())
        .manage(create_app())
        .mount("/users", user_controller::routes())
        .mount("/records", record_controller::routes())
        .mount("/auth", auth_controller::routes())
        .mount("/health-check", routes![health_check])
}

#[get("/")]
async fn health_check() -> &'static str {
    "OK"
}