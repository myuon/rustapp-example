use crate::domain::interface::{Hash, IHashManager, IJWTHandler, IUserLoginRepository};
use crate::domain::model;
use serde::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct LoginService {
    login_repository: Arc<dyn IUserLoginRepository + Sync + Send>,
    hash_manager: Arc<dyn IHashManager + Sync + Send>,
    jwt_handler: Arc<dyn IJWTHandler<model::User> + Sync + Send>,
}

#[derive(Deserialize)]
pub struct AuthenticateInput {
    user_name: String,
    password: String,
}

impl LoginService {
    pub fn new(
        login_repository: Arc<dyn IUserLoginRepository + Sync + Send>,
        hash_manager: Arc<dyn IHashManager + Sync + Send>,
        jwt_handler: Arc<dyn IJWTHandler<model::User> + Sync + Send>,
    ) -> LoginService {
        LoginService {
            login_repository: login_repository,
            hash_manager: hash_manager,
            jwt_handler: jwt_handler,
        }
    }

    async fn authenticateUser(
        &self,
        input: AuthenticateInput,
    ) -> Result<model::User, diesel::result::Error> {
        let (login, user) = self
            .login_repository
            .get_by_user_name(input.user_name)
            .await?;

        if !self
            .hash_manager
            .verify(Hash::from_string(login.password_hash), input.password)
        {
            // return Err();
            unimplemented!()
        }

        Ok(user)
    }

    pub async fn authenticate(
        &self,
        input: AuthenticateInput,
    ) -> Result<String, diesel::result::Error> {
        let user = self.authenticateUser(input).await?;

        // self.jwt_handler.sign(user)?
        unimplemented!()
    }
}
