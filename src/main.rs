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
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let private_key_file =
        env::var("JWT_PRIVATE_KEY_FILE").expect("JWT_PRIVATE_KEY_FILE not found");
    let private_key = std::fs::read(private_key_file)?;

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .data(web::WebContext {
                app: initializer::new(database_url.clone(), private_key.clone()),
            })
            .configure(web::handlers)
    })
    .bind("127.0.0.1:8080")?
    .run()
}
