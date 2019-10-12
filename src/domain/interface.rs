use crate::domain::model;
use async_trait::async_trait;

#[async_trait]
pub trait IUserRepository {
    async fn list(&self) -> Result<Vec<model::User>, ()>;
    async fn put(&self, user: model::User) -> Result<(), ()>;
}
