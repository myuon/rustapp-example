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

// This is for running "raw" sql query
pub struct SqlQuery(String);

impl SqlQuery {
    pub fn new(query: impl Into<String>) -> SqlQuery {
        SqlQuery(query.into())
    }
}

impl Message for SqlQuery {
    type Result = Result<usize, diesel::result::Error>;
}

impl Handler<SqlQuery> for DBExecutor {
    type Result = Result<usize, diesel::result::Error>;

    fn handle(&mut self, message: SqlQuery, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        let conn = self.get_connection();

        diesel::sql_query(message.0).execute(&conn)
    }
}

// Do execution and return usize (affected rows)
pub struct Execute<Q>(Q);

impl<Q: diesel::query_builder::QueryFragment<diesel::mysql::Mysql>> Execute<Q> {
    pub fn new(query: Q) -> Execute<Q> {
        Execute(query)
    }
}

impl<Q> Message for Execute<Q> {
    type Result = Result<usize, diesel::result::Error>;
}

impl<Q: diesel::query_builder::QueryFragment<diesel::mysql::Mysql>> Handler<Execute<Q>>
    for DBExecutor
{
    type Result = Result<usize, diesel::result::Error>;

    fn handle(&mut self, message: Execute<Q>, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use diesel::query_builder::QueryBuilder;
        let conn = self.get_connection();

        let mut builder = diesel::mysql::MysqlQueryBuilder::new();
        message.0.to_sql(&mut builder)?;
        diesel::sql_query(builder.finish()).execute(&conn)
    }
}

// Run query and return the first row
// T represents the return type
pub struct First<T, Q>(Q, std::marker::PhantomData<T>);

impl<T, Q: diesel::query_builder::QueryFragment<diesel::mysql::Mysql>> First<T, Q> {
    pub fn new(query: Q) -> First<T, Q> {
        First(query, std::marker::PhantomData)
    }
}

impl<T: 'static, Q> Message for First<T, Q> {
    type Result = Result<T, diesel::result::Error>;
}

impl<T, Q> Handler<First<T, Q>> for DBExecutor where
    T: 'static,
    T: diesel::Queryable<(), diesel::mysql::Mysql>,
    Q: diesel::query_builder::QueryFragment<diesel::mysql::Mysql>,
    Q: diesel::RunQueryDsl<diesel::MysqlConnection>,
    Q: diesel::query_dsl::limit_dsl::LimitDsl,
    <Q as diesel::query_dsl::limit_dsl::LimitDsl>::Output: diesel::Table,
    <Q as diesel::query_dsl::limit_dsl::LimitDsl>::Output: diesel::query_builder::Query,
    <<Q as diesel::query_dsl::limit_dsl::LimitDsl>::Output as diesel::query_builder::AsQuery>::Query: diesel::query_builder::QueryId,
    <<Q as diesel::query_dsl::limit_dsl::LimitDsl>::Output as diesel::query_builder::AsQuery>::Query: diesel::query_builder::QueryFragment<diesel::mysql::Mysql>,
    diesel::mysql::Mysql: diesel::sql_types::HasSqlType<<<Q as diesel::query_dsl::limit_dsl::LimitDsl>::Output as diesel::query_builder::Query>::SqlType>,
    diesel::mysql::Mysql: diesel::sql_types::HasSqlType<<<Q as diesel::query_dsl::limit_dsl::LimitDsl>::Output as diesel::query_builder::AsQuery>::SqlType>,
    T: diesel::Queryable<<<Q as diesel::query_dsl::limit_dsl::LimitDsl>::Output as diesel::query_builder::AsQuery>::SqlType, diesel::mysql::Mysql>,
{
    type Result = Result<T, diesel::result::Error>;

    fn handle(&mut self, message: First<T, Q>, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();
        message.0.first(&conn)
    }
}
