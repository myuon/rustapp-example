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

impl UserRecord {
    fn to_model(self) -> model::User {
        model::User {
            id: self.id,
            name: self.name,
            display_name: self.display_name,
            role: model::Role::Unknown,
        }
    }

    fn from_model(user: model::User) -> Self {
        UserRecord {
            id: user.id,
            name: user.name,
            display_name: user.display_name,
            role: None,
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
impl crate::domain::interface::IUserRepository for UserRepository {
    async fn list(&self) -> Result<Vec<model::User>, ()> {
        let conn = self.db.get_connection();
        let us = user_records::table.load::<UserRecord>(&conn).unwrap();

        Ok(us.into_iter().map(|r| r.to_model()).collect())
    }

    async fn save(&self, user: model::User) -> Result<(), ()> {
        let conn = self.db.get_connection();
        insert_into(user_records::table)
            .values::<UserRecord>(UserRecord::from_model(user))
            .execute(&conn)
            .map_err(|_| ())?;

        Ok(())
    }
}
