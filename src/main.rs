#[macro_use]
extern crate diesel;

mod models;
mod schema;

use actix_web::{web, App, HttpServer, Responder};
use diesel::prelude::*;
use dotenv::dotenv;
use schema::users::dsl::*;
use std::env;

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn create_mysql_connection(database_url: &str) -> MysqlConnection {
    MysqlConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let conn = create_mysql_connection(&env::var("DATABASE_URL").expect("DATABASE_URL not found"));
    let us = users.load::<models::User>(&conn);
    println!("{:?}", us);

    HttpServer::new(|| App::new().service(web::resource("/users/{id}").to(index)))
        .bind("127.0.0.1:8080")?
        .run()
}
