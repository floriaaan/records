#[macro_use]
extern crate rocket;

pub mod app;
pub mod config;
pub mod db;
pub mod utils;
pub mod templating;

mod error {
    pub mod app_error;
    pub mod logging;
}

mod controllers;
mod use_cases;
mod repositories;
mod models;

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
use crate::controllers::{record_controller, user_controller, auth_controller, collection_controller};
use crate::db::Db;
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    
    // Initialize templates
    if let Err(err) = templating::init_templates() {
        tracing::error!("Failed to initialize templates: {}", err);
    }

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::config::<Config>())
        .manage(create_app())
        .mount("/users", user_controller::routes())
        .mount("/records", record_controller::routes())
        .mount("/auth", auth_controller::routes())
        .mount("/records/collection", collection_controller::routes())
        .mount("/health-check", routes![health_check])
}

#[get("/")]
async fn health_check() -> &'static str {
    "OK"
}