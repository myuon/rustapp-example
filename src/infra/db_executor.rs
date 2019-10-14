use crate::infra::MySQLConnPool;
use actix::prelude::*;

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

pub struct Query<T> {
    query: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Query<T> {
    pub fn new(query: impl Into<String>) -> Query<T> {
        Query {
            query: query.into(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: 'static> Message for Query<T> {
    type Result = Result<Vec<T>, diesel::result::Error>;
}

impl<T: 'static + diesel::deserialize::QueryableByName<diesel::mysql::Mysql>> Handler<Query<T>>
    for DBExecutor
{
    type Result = Result<Vec<T>, diesel::result::Error>;

    fn handle(&mut self, message: Query<T>, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        let conn = self.get_connection();

        diesel::sql_query(message.query).load(&conn)
    }
}

pub struct Execute(String);

impl Execute {
    pub fn new(query: impl Into<String>) -> Execute {
        Execute(query.into())
    }
}

impl Message for Execute {
    type Result = Result<usize, diesel::result::Error>;
}

impl Handler<Execute> for DBExecutor {
    type Result = Result<usize, diesel::result::Error>;

    fn handle(&mut self, message: Execute, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        let conn = self.get_connection();

        diesel::sql_query(message.0).execute(&conn)
    }
}
