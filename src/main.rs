#![feature(async_closure)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

mod domain;
mod infra;
mod initializer;
mod schema;
mod serviceclient;
mod web;

mod async_await;
mod error;

use actix::prelude::*;
use dotenv::dotenv;
use std::env;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "warn,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    let private_key_file = env::var("JWT_PRIVATE_KEY_FILE").unwrap();

    System::new("rustapp");

    let addr = {
        let database_url = database_url.clone();
        SyncArbiter::start(5, move || infra::DBExecutor::new(database_url.clone()))
    };

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .data(web::WebContext {
                app: initializer::new(database_url.clone(), &private_key_file),
                dbexecutor: addr.clone(),
            })
            .configure(web::handlers)
    })
    .bind("127.0.0.1:8080")?
    .workers(1) // for local development
    .run()
}
