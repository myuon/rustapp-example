#[macro_use]
extern crate diesel;

mod async_await;
mod models;
mod schema;

use actix_http::Response;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::dsl::*;
use diesel::prelude::*;
use dotenv::dotenv;
use futures01::stream::Stream;
use schema::users::dsl::*;
use serde::*;
use std::env;

type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::mysql::MysqlConnection>>;

async fn users_list(db: web::Data<Pool>) -> Result<HttpResponse, ()> {
    let conn = db.get().unwrap();
    let us = users.load::<models::User>(&conn).unwrap();

    Ok(Response::Ok().json(us))
}

#[derive(Deserialize)]
struct UserCreateInput {
    name: String,
    display_name: String,
}

async fn user_create(payload: web::Payload, db: web::Data<Pool>) -> Result<HttpResponse, ()> {
    let conn = db.get().unwrap();
    let body = Box::new(
        futures::compat::Compat01As03::new(payload.concat2())
            .await
            .map_err(|_| ())?,
    );
    let input = serde_json::from_slice::<UserCreateInput>(body.as_ref()).map_err(|_| ())?;
    insert_into(users)
        .values::<models::User>(models::User {
            id: ulid::Ulid::new().to_string(),
            name: input.name,
            display_name: input.display_name,
        })
        .execute(&conn)
        .map_err(|_| ())?;

    Ok(Response::Created().finish())
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let manager = diesel::r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new().data(pool.clone()).service(
            web::resource("/users")
                .route(web::get().to_async(async_await::wrap(users_list)))
                .route(web::post().to_async(async_await::wrap2(user_create))),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
