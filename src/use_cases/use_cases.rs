use crate::use_cases::auth_use_case::{AuthUseCase, AuthUseCaseImpl};
use crate::use_cases::record_use_case::{RecordUseCase, RecordUseCaseImpl};
use crate::use_cases::user_use_case::{UserUseCase, UserUseCaseImpl};
use crate::use_cases::collection_use_case::{CollectionUseCase, CollectionUseCaseImpl};

pub struct UseCases {
    pub record: Box<dyn RecordUseCase>,
    pub user: Box<dyn UserUseCase>,
    pub auth: Box<dyn AuthUseCase>,
    pub collection: Box<dyn CollectionUseCase>,
}

impl UseCases {
    pub fn new() -> Self {
        Self {
            record: Box::new(RecordUseCaseImpl::new()),
            user: Box::new(UserUseCaseImpl::new()),
            auth: Box::new(AuthUseCaseImpl::new()),
            collection: Box::new(CollectionUseCaseImpl::new()),
        }
    }
}
