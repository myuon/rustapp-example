#[macro_use]
extern crate diesel;

mod async_await;
mod models;
mod schema;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use dotenv::dotenv;
use futures::prelude::*;
use schema::users::dsl::*;
use std::env;
use std::sync::Arc;

type Pool = r2d2::Pool<r2d2_mysql::MysqlConnectionManager>;
type Connection = r2d2::PooledConnection<r2d2_mysql::MysqlConnectionManager>;

fn get_conn(pool: &Pool) -> Connection {
    pool.get().unwrap()
}

async fn users_list(db: web::Data<Pool>) -> Result<HttpResponse, ()> {
    Ok(actix_web::HttpResponse::Ok().finish())
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let pool = Arc::new(
        r2d2::Pool::new(r2d2_mysql::MysqlConnectionManager::new(
            mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&database_url).unwrap()),
        ))
        .unwrap(),
    );

    HttpServer::new(move || {
        App::new().data(pool.clone()).service(
            web::resource("/users").route(web::get().to_async(async_await::wrap(users_list))),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
