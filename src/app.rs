use crate::repositories::repositories::Repositories;
use crate::use_cases::use_cases::UseCases;
use rocket::State;
use std::sync::Arc;

pub type AppState = State<Arc<App>>;

pub struct App {
    pub use_cases: UseCases,
    pub repos: Repositories,
}

impl App {
    pub fn new(use_cases: UseCases, repos: Repositories) -> Self {
        Self { use_cases, repos }
    }
}

pub fn create_app() -> Arc<App> {
    let repos = Repositories::new();
    let use_cases = UseCases::new();
    Arc::new(App::new(use_cases, repos))
}
