use actix::prelude::*;
use diesel::{mysql::MysqlConnection, r2d2};

#[derive(Clone)]
pub struct MySQLConnPool(r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>);

impl MySQLConnPool {
    pub fn new(database_url: String) -> MySQLConnPool {
        let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create pool");
        MySQLConnPool(pool)
    }

    pub fn get_connection(
        &self,
    ) -> r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>> {
        self.0.get().unwrap()
    }
}

#[derive(Clone)]
pub struct DBExecutor(MySQLConnPool);

impl DBExecutor {
    pub fn new(database_url: String) -> DBExecutor {
        DBExecutor(MySQLConnPool::new(database_url))
    }

    pub fn get_connection(
        &self,
    ) -> r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>> {
        self.0.get_connection()
    }
}

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}
