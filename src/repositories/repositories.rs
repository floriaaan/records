use crate::repositories::record_repo::{RecordRepo, RecordRepoImpl};
use crate::repositories::tag_repo::{TagRepo, TagRepoImpl};
use crate::repositories::user_repo::{UserRepo, UserRepoImpl};
use crate::repositories::collection_token_repo::{CollectionTokenRepo, CollectionTokenRepoImpl};

pub struct Repositories {
    pub record: Box<dyn RecordRepo>,
    pub user: Box<dyn UserRepo>,
    pub tag: Box<dyn TagRepo>,
    pub collection_token: Box<dyn CollectionTokenRepo>,
}

impl Repositories {
    pub fn new() -> Self {
        Self {
            record: Box::new(RecordRepoImpl::new()),
            user: Box::new(UserRepoImpl::new()),
            tag: Box::new(TagRepoImpl::new()),
            collection_token: Box::new(CollectionTokenRepoImpl::new()),
        }
    }
}
