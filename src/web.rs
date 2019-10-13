use crate::async_await;
use crate::domain::model;
use crate::error::ServiceError;
use crate::initializer;
use actix_http::Response;
use actix_web::{error, web, HttpResponse};
use futures01::stream::Stream;

#[derive(Clone)]
pub struct WebContext {
    pub app: initializer::AppContext,
}

impl WebContext {
    fn auth_token(req: web::HttpRequest) -> Option<String> {
        req.headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_owned())
    }

    async fn authorize(
        &self,
        req: web::HttpRequest,
        validate_user_role: Option<model::Role>,
    ) -> Result<model::User, error::Error> {
        let token = WebContext::auth_token(req)
            .ok_or(ServiceError::Unauthorized(failure::err_msg("Empty token")).to_http_error())?;
        let stoken = token.split("Bearer ").collect::<Vec<&str>>();
        if stoken.len() != 2 {
            return Err(
                ServiceError::Unauthorized(failure::err_msg("Invalid token")).to_http_error(),
            );
        }

        let user = self
            .app
            .services
            .login_service
            .authorize(stoken[1].to_owned())
            .await
            .map_err(|e| e.to_http_error())?;

        if let Some(check_role) = validate_user_role {
            if user.role < check_role {
                return Err(
                    ServiceError::Unauthorized(failure::err_msg("Role is not enough"))
                        .to_http_error(),
                );
            }
        }

        Ok(user)
    }
}

pub fn handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/admin/users")
            .route(web::get().to_async(async_await::wrap2(api_list_users)))
            .route(web::post().to_async(async_await::wrap3(api_create_user))),
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

async fn api_list_users(
    context: web::Data<WebContext>,
    req: web::HttpRequest,
) -> Result<HttpResponse, error::Error> {
    context
        .get_ref()
        .authorize(req, Some(model::Role::PowerUser))
        .await?;

    let res = context
        .app
        .services
        .user_service
        .list()
        .await
        .map_err(|e| e.to_http_error())?;

    Ok(Response::Ok().json(res))
}

async fn api_create_user(
    payload: web::Payload,
    context: web::Data<WebContext>,
    req: web::HttpRequest,
) -> Result<HttpResponse, error::Error> {
    context
        .get_ref()
        .authorize(req, Some(model::Role::Admin))
        .await?;

    let body = Box::new(
        futures::compat::Compat01As03::new(payload.concat2())
            .await
            .map_err(error::ErrorBadRequest)?,
    );
    let input = serde_json::from_slice::<crate::domain::service::UserCreateInput>(body.as_ref())
        .map_err(error::ErrorBadRequest)?;

    context
        .app
        .services
        .user_service
        .create(input)
        .await
        .map_err(|e| e.to_http_error())?;

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
