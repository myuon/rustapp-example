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
    )
    .service(
        web::resource("/auth/login")
            .route(web::post().to_async(async_await::wrap2(api_auth_login))),
    )
    .service(
        web::resource("/private/login/{user_id}")
            .route(web::put().to_async(async_await::wrap2(private_api_enable_user_with_password))),
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

async fn api_auth_login(
    payload: web::Payload,
    context: web::Data<WebContext>,
) -> Result<HttpResponse, error::Error> {
    let body = Box::new(
        futures::compat::Compat01As03::new(payload.concat2())
            .await
            .map_err(error::ErrorBadRequest)?,
    );
    let input = serde_json::from_slice::<crate::domain::service::AuthenticateInput>(body.as_ref())
        .map_err(error::ErrorBadRequest)?;

    let res = context
        .app
        .services
        .login_service
        .authenticate(input)
        .await
        .map_err(|e| e.to_http_error())?;

    Ok(Response::Ok().json(res))
}

async fn private_api_enable_user_with_password(
    payload: web::Payload,
    context: web::Data<WebContext>,
) -> Result<HttpResponse, error::Error> {
    let body = Box::new(
        futures::compat::Compat01As03::new(payload.concat2())
            .await
            .map_err(error::ErrorBadRequest)?,
    );
    let input = serde_json::from_slice::<crate::domain::service::EnableUserWithPasswordInput>(
        body.as_ref(),
    )
    .map_err(error::ErrorBadRequest)?;

    let res = context
        .app
        .services
        .login_service
        .enable_user_with_password(input)
        .await
        .map_err(|e| e.to_http_error())?;

    Ok(Response::Ok().json(res))
}
