use crate::domain::interface::IUserRepository;
use crate::domain::model;
use crate::error::ServiceError;
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

    pub async fn create(&self, input: UserCreateInput) -> Result<(), ServiceError> {
        self.user_repository
            .save(model::User {
                id: ulid::Ulid::new().to_string(),
                name: input.name,
                display_name: input.display_name,
                role: model::Role::Unknown,
            })
            .await
            .map_err(ServiceError::DBError)
    }

    pub async fn list(&self) -> Result<Vec<model::User>, ServiceError> {
        self.user_repository
            .list()
            .await
            .map_err(ServiceError::DBError)
    }
}
