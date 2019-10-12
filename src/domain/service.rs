use crate::domain::interface::IUserRepository;
use crate::domain::model;
use serde::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    userRepository: Arc<dyn IUserRepository + Sync + Send>,
}

#[derive(Deserialize)]
pub struct UserCreateInput {
    pub name: String,
    pub display_name: String,
}

impl UserService {
    pub async fn create(&self, input: UserCreateInput) {}

    pub async fn list(&self) -> Vec<model::User> {
        vec![]
    }
}
