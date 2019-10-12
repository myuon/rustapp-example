use crate::async_await;
use crate::infra::connection_pool::MySQLConnPool;
use crate::initializer;
use actix_http::Response;
use actix_web::{web, HttpResponse};
use futures01::stream::Stream;

#[derive(Clone)]
pub struct WebContext {
    pub app: initializer::AppContext,
}

pub fn handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to_async(async_await::wrap(api_list_users)))
            .route(web::post().to_async(async_await::wrap2(api_create_user))),
    );
}

async fn api_list_users(context: web::Data<WebContext>) -> Result<HttpResponse, ()> {
    Ok(Response::Ok().json(context.app.services.userService.list().await))
}

async fn api_create_user(
    payload: web::Payload,
    context: web::Data<WebContext>,
) -> Result<HttpResponse, ()> {
    let body = Box::new(
        futures::compat::Compat01As03::new(payload.concat2())
            .await
            .map_err(|_| ())?,
    );
    let input = serde_json::from_slice::<crate::domain::service::UserCreateInput>(body.as_ref())
        .map_err(|_| ())?;

    context.app.services.userService.create(input).await;

    Ok(Response::Created().finish())
}
