use crate::app::App;
use crate::repositories::collection_token_repo::{self, MockCollectionTokenRepo};
use crate::repositories::{
    record_repo::MockRecordRepo, repositories::Repositories, user_repo::MockUserRepo, tag_repo::MockTagRepo,
};
use crate::use_cases::collection_use_case::MockCollectionUseCase;
use crate::use_cases::{
    record_use_case::MockRecordUseCase, use_cases::UseCases, user_use_case::MockUserUseCase,
    auth_use_case::MockAuthUseCase,
};

pub fn create_app_for_test() -> App {
    let repos = create_repos_for_test();
    let use_cases = create_use_cases_for_test();
    App::new(use_cases, repos)
}

pub fn create_repos_for_test() -> Repositories {
    let user_repo = Box::new(MockUserRepo::new());
    let record_repo = Box::new(MockRecordRepo::new());
    let tag_repo = Box::new(MockTagRepo::new());
    let collection_token_repo = Box::new(MockCollectionTokenRepo::new());
    Repositories {
        user: user_repo,
        record: record_repo,
        tag: tag_repo,
        collection_token: collection_token_repo,
    }
}

pub fn create_use_cases_for_test() -> UseCases {
    let user = Box::new(MockUserUseCase::new());
    let record = Box::new(MockRecordUseCase::new());
    let auth = Box::new(MockAuthUseCase::new());
    let collection = Box::new(MockCollectionUseCase::new());
    UseCases { user, record, auth, collection }
}
