use crate::domain::model;
use crate::infra::connection_pool::MySQLConnPool;
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
}

impl UserRecord {
    fn to_model(self) -> model::User {
        model::User {
            id: self.id,
            name: self.name,
            display_name: self.display_name,
        }
    }

    fn from_model(user: model::User) -> Self {
        UserRecord {
            id: user.id,
            name: user.name,
            display_name: user.display_name,
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
