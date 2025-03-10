use crate::app::App;
use crate::repositories::{
    record_repo::MockRecordRepo, repositories::Repos, user_repo::MockUserRepo,
};
use crate::use_cases::{
    record_use_case::MockRecordUseCase, use_cases::UseCases, user_use_case::MockUserUseCase,
};

pub fn create_app_for_test() -> App {
    let repos = create_repos_for_test();
    let use_cases = create_use_cases_for_test();
    App::new(use_cases, repos)
}

pub fn create_repos_for_test() -> Repos {
    let user = Box::new(MockUserRepo::new());
    let record = Box::new(MockRecordRepo::new());
    Repos { user, record }
}

pub fn create_use_cases_for_test() -> UseCases {
    let user = Box::new(MockUserUseCase::new());
    let record = Box::new(MockRecordUseCase::new());
    UseCases { user, record }
}
