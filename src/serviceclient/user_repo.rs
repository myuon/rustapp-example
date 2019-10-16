use crate::domain::interface::IUserRepository;
use crate::domain::model;
use crate::infra::{DBConnector, DBConnectorError};
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
    db: DBConnector,
}

impl UserRepository {
    pub fn new(db: DBConnector) -> UserRepository {
        UserRepository { db: db }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn list(&self) -> Result<Vec<model::User>, DBConnectorError> {
        let us = self.db.load::<UserRecord, _>(user_records::table).await?;
        Ok(us.into_iter().map(|r| r.to_model()).collect())
    }

    async fn save(&self, user: model::User) -> Result<(), DBConnectorError> {
        self.db
            .execute(
                insert_into(user_records::table).values::<UserRecord>(UserRecord::from_model(user)),
            )
            .await?;

        Ok(())
    }

    async fn get_by_id(&self, user_id: String) -> Result<model::User, DBConnectorError> {
        let user = self
            .db
            .first::<UserRecord, _>(user_records::table.filter(user_records::id.eq(user_id)))
            .await?;

        Ok(user.to_model())
    }
}
