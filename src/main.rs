#![feature(async_closure)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;

mod domain;
mod infra;
mod initializer;
mod schema;
mod serviceclient;
mod web;

mod async_await;
mod error;

use dotenv::dotenv;
use std::env;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let private_key_file =
        env::var("JWT_PRIVATE_KEY_FILE").expect("JWT_PRIVATE_KEY_FILE not found");
    let private_key = std::fs::read_to_string(private_key_file)?;

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(web::WebContext {
                app: initializer::new(database_url.clone(), private_key.clone()),
            })
            .configure(web::handlers)
    })
    .bind("127.0.0.1:8080")?
    .run()
}
