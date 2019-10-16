use crate::domain::interface;
use crate::domain::model;
use crate::infra::{DBConnector, DBConnectorError};
use crate::schema::*;
use async_trait::async_trait;
use diesel::dsl::*;
use diesel::prelude::*;
use serde::*;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct UserLoginRecord {
    pub user_id: String,
    pub password_hash: String,
    pub status: Option<String>,
}

impl UserLoginRecord {
    fn to_model(self) -> model::Login {
        model::Login {
            user_id: self.user_id,
            password_hash: self.password_hash,
            status: self
                .status
                .and_then(|r| serde_json::from_str(&r).ok())
                .unwrap_or(model::LoginUserStatus::Enabled),
        }
    }

    fn from_model(login: model::Login) -> Self {
        UserLoginRecord {
            user_id: login.user_id,
            password_hash: login.password_hash,
            status: serde_json::to_string(&login.status).ok(),
        }
    }
}

pub struct UserLoginRepository {
    db: DBConnector,
}

impl UserLoginRepository {
    pub fn new(db: DBConnector) -> UserLoginRepository {
        UserLoginRepository { db: db }
    }
}

#[async_trait]
impl interface::IUserLoginRepository for UserLoginRepository {
    async fn save(&self, user: model::Login) -> Result<(), DBConnectorError> {
        self.db
            .execute(
                insert_into(user_login_records::table)
                    .values::<UserLoginRecord>(UserLoginRecord::from_model(user)),
            )
            .await?;

        Ok(())
    }

    async fn get_by_user_name(
        &self,
        user_name: String,
    ) -> Result<(model::Login, model::User), DBConnectorError> {
        let (user, login) = self
            .db
            .first::<(super::user_repo::UserRecord, UserLoginRecord), _>(
                user_records::table
                    .inner_join(user_login_records::table)
                    .filter(user_records::name.eq(user_name)),
            )
            .await?;

        Ok((login.to_model(), user.to_model()))
    }

    async fn get_by_user_id(&self, user_id: String) -> Result<model::Login, DBConnectorError> {
        let record = self
            .db
            .first::<UserLoginRecord, _>(
                user_login_records::table.filter(user_login_records::user_id.eq(user_id)),
            )
            .await?;

        Ok(record.to_model())
    }
}
