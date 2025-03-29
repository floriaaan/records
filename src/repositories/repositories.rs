use crate::repositories::{
    record_repo::{RecordRepo, RecordRepoImpl},
    user_repo::{UserRepo, UserRepoImpl},
    tag_repo::{TagRepo, TagRepoImpl},
};

pub struct Repos {
    pub user: Box<dyn UserRepo>,
    pub record: Box<dyn RecordRepo>,
    pub tag: Box<dyn TagRepo>,
}

pub fn create_repos() -> Repos {
    let user = Box::new(UserRepoImpl::new());
    let record = Box::new(RecordRepoImpl::new());
    let tag = Box::new(TagRepoImpl::new());
    Repos { user, record, tag }
}
