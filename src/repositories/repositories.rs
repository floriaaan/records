use crate::repositories::{
    record_repo::{RecordRepo, RecordRepoImpl},
    user_repo::{UserRepo, UserRepoImpl},
};

pub struct Repos {
    pub user: Box<dyn UserRepo>,
    pub record: Box<dyn RecordRepo>,
}

pub fn create_repos() -> Repos {
    let user = Box::new(UserRepoImpl::new());
    let record = Box::new(RecordRepoImpl::new());
    Repos { user, record }
}
