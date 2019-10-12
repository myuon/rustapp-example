use diesel::{mysql::MysqlConnection, r2d2};

#[derive(Clone)]
pub struct MySQLConnPool(r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>);

impl MySQLConnPool {
    pub fn new(database_url: String) -> MySQLConnPool {
        let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
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
