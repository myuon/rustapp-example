use crate::domain::model;
use async_trait::async_trait;

#[async_trait]
pub trait IUserRepository {
    async fn list(&self) -> Result<Vec<model::User>, ()>;
    async fn save(&self, user: model::User) -> Result<(), ()>;
}

#[async_trait]
pub trait IUserLoginRepository {
    async fn get_by_user_name(
        &self,
        user_name: String,
    ) -> Result<model::Login, diesel::result::Error>;
    async fn get_by_user_id(&self, user_id: String) -> Result<model::Login, diesel::result::Error>;
    async fn save(&self, login: model::Login) -> Result<(), diesel::result::Error>;
}
