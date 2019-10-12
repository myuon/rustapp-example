use crate::domain::model;
use crate::infra::MySQLConnPool;
use crate::schema::*;
use async_trait::async_trait;
use diesel::dsl::*;
use diesel::prelude::*;
use serde::*;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub role: Option<String>,
}

impl UserRecord {
    pub fn to_model(self) -> model::User {
        model::User {
            id: self.id,
            name: self.name,
            display_name: self.display_name,
            role: self
                .role
                .map(|r| model::Role::new_from_str(&r))
                .unwrap_or(model::Role::Unknown),
        }
    }

    pub fn from_model(user: model::User) -> Self {
        UserRecord {
            id: user.id,
            name: user.name,
            display_name: user.display_name,
            role: Some(user.role.as_string()),
        }
    }
}

pub struct UserRepository {
    db: MySQLConnPool,
}

impl UserRepository {
    pub fn new(db: MySQLConnPool) -> UserRepository {
        UserRepository { db: db }
    }
}

#[async_trait]
impl crate::domain::interface::IUserRepository for UserRepository {
    async fn list(&self) -> Result<Vec<model::User>, diesel::result::Error> {
        let conn = self.db.get_connection();
        let us = user_records::table.load::<UserRecord>(&conn)?;

        Ok(us.into_iter().map(|r| r.to_model()).collect())
    }

    async fn save(&self, user: model::User) -> Result<(), diesel::result::Error> {
        let conn = self.db.get_connection();
        insert_into(user_records::table)
            .values::<UserRecord>(UserRecord::from_model(user))
            .execute(&conn)?;

        Ok(())
    }

    async fn get_by_id(&self, user_id: String) -> Result<model::User, diesel::result::Error> {
        let conn = self.db.get_connection();
        let user = user_records::table
            .filter(user_records::id.eq(user_id))
            .first::<UserRecord>(&conn)?;

        Ok(user.to_model())
    }
}
