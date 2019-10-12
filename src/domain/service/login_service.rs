use crate::domain::interface::{Hash, IHashManager, IJWTHandler, IUserLoginRepository};
use crate::domain::model;
use crate::error::ServiceError;
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

#[derive(Deserialize)]
pub struct EnableUserWithPasswordInput {
    user_id: String,
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

    async fn authenticate_user(
        &self,
        input: AuthenticateInput,
    ) -> Result<model::User, ServiceError> {
        let (login, user) = self
            .login_repository
            .get_by_user_name(input.user_name)
            .await
            .map_err(ServiceError::DBError)?;

        if !self
            .hash_manager
            .verify(Hash::from_string(login.password_hash), input.password)
        {
            return Err(ServiceError::InvalidRequest(Box::new(
                ServiceError::GeneralError(failure::err_msg("invalid password")),
            )));
        }

        Ok(user)
    }

    pub async fn authenticate(&self, input: AuthenticateInput) -> Result<String, ServiceError> {
        let user = self.authenticate_user(input).await?;

        self.jwt_handler.sign(user).map_err(|err| {
            ServiceError::InvalidRequest(Box::new(ServiceError::GeneralError(err.into())))
        })
    }

    pub async fn enable_user_with_password(
        &self,
        input: EnableUserWithPasswordInput,
    ) -> Result<(), ServiceError> {
        let login = model::Login {
            user_id: input.user_id,
            password_hash: self.hash_manager.hash(input.password).to_string(),
            status: model::LoginUserStatus::Enabled,
        };

        self.login_repository
            .save(login)
            .await
            .map_err(ServiceError::DBError)?;

        Ok(())
    }
}
