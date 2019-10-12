use crate::async_await;
use crate::initializer;
use actix_http::Response;
use actix_web::{error, web, HttpResponse};
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
    Ok(Response::Ok().json(context.app.services.user_service.list().await))
}

async fn api_create_user(
    payload: web::Payload,
    context: web::Data<WebContext>,
) -> Result<HttpResponse, error::Error> {
    let body = Box::new(
        futures::compat::Compat01As03::new(payload.concat2())
            .await
            .map_err(error::ErrorBadRequest)?,
    );
    let input = serde_json::from_slice::<crate::domain::service::UserCreateInput>(body.as_ref())
        .map_err(error::ErrorBadRequest)?;

    context.app.services.user_service.create(input).await;

    Ok(Response::Created().finish())
}
