#![feature(async_closure)]

#[macro_use]
extern crate diesel;

mod domain;
mod infra;
mod initializer;
mod schema;
mod serviceclient;
mod web;

mod async_await;

use dotenv::dotenv;
use std::env;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(web::WebContext {
                app: initializer::new(database_url.clone()),
            })
            .configure(web::handlers)
    })
    .bind("127.0.0.1:8080")?
    .run()
}
