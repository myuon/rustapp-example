use crate::domain::interface::IUserRepository;
use crate::domain::model;
use serde::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn IUserRepository + Sync + Send>,
}

#[derive(Deserialize)]
pub struct UserCreateInput {
    pub name: String,
    pub display_name: String,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn IUserRepository + Sync + Send>) -> UserService {
        UserService {
            user_repository: user_repository,
        }
    }

    pub async fn create(&self, input: UserCreateInput) {
        self.user_repository
            .save(model::User {
                id: ulid::Ulid::new().to_string(),
                name: input.name,
                display_name: input.display_name,
            })
            .await
            .unwrap()
    }

    pub async fn list(&self) -> Vec<model::User> {
        self.user_repository.list().await.unwrap()
    }
}
