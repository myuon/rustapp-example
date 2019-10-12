use crate::domain::interface;
use crate::domain::model;
use crate::infra::connection_pool::MySQLConnPool;
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
    db: MySQLConnPool,
}

impl UserLoginRepository {
    pub fn new(db: MySQLConnPool) -> UserLoginRepository {
        UserLoginRepository { db: db }
    }
}

#[async_trait]
impl interface::IUserLoginRepository for UserLoginRepository {
    async fn save(&self, user: model::Login) -> Result<(), diesel::result::Error> {
        let conn = self.db.get_connection();
        insert_into(user_login_records::table)
            .values::<UserLoginRecord>(UserLoginRecord::from_model(user))
            .execute(&conn)?;

        Ok(())
    }

    async fn get_by_user_name(
        &self,
        user_name: String,
    ) -> Result<model::Login, diesel::result::Error> {
        let conn = self.db.get_connection();
        let (_, record) = user_records::table
            .inner_join(user_login_records::table)
            .filter(user_records::name.eq(user_name))
            .first::<(super::user_repo::UserRecord, UserLoginRecord)>(&conn)?;

        Ok(record.to_model())
    }

    async fn get_by_user_id(&self, user_id: String) -> Result<model::Login, diesel::result::Error> {
        let conn = self.db.get_connection();
        let record = user_login_records::table
            .filter(user_login_records::user_id.eq(user_id))
            .first::<UserLoginRecord>(&conn)?;

        Ok(record.to_model())
    }
}
