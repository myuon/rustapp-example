use crate::infra::MySQLConnPool;
use actix::prelude::*;
use futures::compat::*;

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

#[derive(Clone)]
pub struct DBConnector(Addr<DBExecutor>);

#[derive(Fail, Debug)]
pub enum DBConnectorError {
    #[fail(display = "DB Error: {}", _0)]
    DBError(#[fail(cause)] diesel::result::Error),

    #[fail(display = "Actor Error: {}", _0)]
    MailboxError(#[fail(cause)] actix::MailboxError),
}

impl DBConnector {
    pub fn new(conn: Addr<DBExecutor>) -> DBConnector {
        DBConnector(conn)
    }

    pub async fn execute<
        Q: diesel::query_builder::QueryFragment<diesel::mysql::Mysql> + Sync + Send + 'static,
    >(
        &self,
        query: Q,
    ) -> Result<usize, DBConnectorError> {
        let rows = self
            .0
            .send(Execute::new(query))
            .compat()
            .await
            .map_err(DBConnectorError::MailboxError)?
            .map_err(DBConnectorError::DBError)?;

        Ok(rows)
    }

    pub async fn first<T: 'static + Send, Q: 'static + Send>(
        &self,
        query: Q,
    ) -> Result<T, DBConnectorError>
    where
        Q: diesel::query_dsl::limit_dsl::LimitDsl,
        Q: diesel::RunQueryDsl<diesel::MysqlConnection>,
        diesel::helper_types::Limit<Q>: diesel::query_dsl::LoadQuery<diesel::MysqlConnection, T>,
    {
        let result = self
            .0
            .send(First::new(query))
            .compat()
            .await
            .map_err(DBConnectorError::MailboxError)?
            .map_err(DBConnectorError::DBError)?;

        Ok(result)
    }

    pub async fn load<T: 'static + Send, Q: 'static + Send>(
        &self,
        query: Q,
    ) -> Result<Vec<T>, DBConnectorError>
    where
        Q: diesel::RunQueryDsl<diesel::MysqlConnection>,
        Q: diesel::query_dsl::LoadQuery<diesel::MysqlConnection, T>,
    {
        let result = self
            .0
            .send(Load::new(query))
            .compat()
            .await
            .map_err(DBConnectorError::MailboxError)?
            .map_err(DBConnectorError::DBError)?;

        Ok(result)
    }

    pub async fn sql_query(&self, query: impl Into<String>) -> Result<usize, DBConnectorError> {
        let result = self
            .0
            .send(SqlQuery::new(query))
            .compat()
            .await
            .map_err(DBConnectorError::MailboxError)?
            .map_err(DBConnectorError::DBError)?;

        Ok(result)
    }
}

// -------------------
// Message/Handler
// -------------------

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

impl<Q> Execute<Q> {
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

// Run query and return the all rows
// T represents the return type
pub struct Load<T, Q>(Q, std::marker::PhantomData<T>);

impl<T, Q> Load<T, Q> {
    pub fn new(query: Q) -> Load<T, Q> {
        Load(query, std::marker::PhantomData)
    }
}

impl<T: 'static, Q> Message for Load<T, Q> {
    type Result = Result<Vec<T>, diesel::result::Error>;
}

impl<T: 'static, Q> Handler<Load<T, Q>> for DBExecutor
where
    Q: diesel::RunQueryDsl<diesel::MysqlConnection>,
    Q: diesel::query_dsl::LoadQuery<diesel::MysqlConnection, T>,
{
    type Result = Result<Vec<T>, diesel::result::Error>;

    fn handle(&mut self, message: Load<T, Q>, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();
        message.0.load(&conn)
    }
}

// Run query and return the first row
// T represents the return type
pub struct First<T, Q>(Q, std::marker::PhantomData<T>);

impl<T, Q> First<T, Q> {
    pub fn new(query: Q) -> First<T, Q> {
        First(query, std::marker::PhantomData)
    }
}

impl<T: 'static, Q> Message for First<T, Q> {
    type Result = Result<T, diesel::result::Error>;
}

impl<T: 'static, Q> Handler<First<T, Q>> for DBExecutor
where
    Q: diesel::RunQueryDsl<diesel::MysqlConnection>,
    Q: diesel::query_dsl::limit_dsl::LimitDsl,
    diesel::helper_types::Limit<Q>: diesel::query_dsl::LoadQuery<diesel::MysqlConnection, T>,
{
    type Result = Result<T, diesel::result::Error>;

    fn handle(&mut self, message: First<T, Q>, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();
        message.0.first(&conn)
    }
}
