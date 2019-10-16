use crate::domain::model;
use crate::infra::DBConnectorError;
use async_trait::async_trait;

#[async_trait]
pub trait IUserRepository {
    async fn get_by_id(&self, user_id: String) -> Result<model::User, DBConnectorError>;
    async fn list(&self) -> Result<Vec<model::User>, DBConnectorError>;
    async fn save(&self, user: model::User) -> Result<(), DBConnectorError>;
}

#[async_trait]
pub trait IUserLoginRepository {
    async fn get_by_user_name(
        &self,
        user_name: String,
    ) -> Result<(model::Login, model::User), DBConnectorError>;
    async fn get_by_user_id(&self, user_id: String) -> Result<model::Login, DBConnectorError>;
    async fn save(&self, login: model::Login) -> Result<(), DBConnectorError>;
}

pub struct Hash(String);

impl Hash {
    pub fn from_string(s: String) -> Hash {
        Hash(s)
    }

    pub fn to_string(self) -> String {
        self.0
    }
}

pub trait IHashManager {
    fn hash(&self, raw: String) -> Hash;
    fn verify(&self, hash: Hash, raw: String) -> bool;
}

pub trait IJWTHandler<Payload> {
    fn sign(&self, payload: Payload) -> Result<String, biscuit::errors::Error>;
    fn verify(&self, jwt: &str) -> Result<Payload, biscuit::errors::Error>;
}
