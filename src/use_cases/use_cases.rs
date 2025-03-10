use crate::use_cases::{
    record_use_case::{RecordUseCase, RecordUseCaseImpl},
    user_use_case::{UserUseCase, UserUseCaseImpl},
    auth_use_case::{AuthUseCase, AuthUseCaseImpl},
};

pub struct UseCases {
    pub user: Box<dyn UserUseCase>,
    pub record: Box<dyn RecordUseCase>,
    pub auth: Box<dyn AuthUseCase>,
}

pub fn create_use_cases() -> UseCases {
    let user = Box::new(UserUseCaseImpl::new());
    let record = Box::new(RecordUseCaseImpl::new());
    let auth = Box::new(AuthUseCaseImpl::new());
    UseCases { user, record, auth }
}
