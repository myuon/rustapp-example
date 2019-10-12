#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "DB Error: {}", _0)]
    DBError(#[fail(cause)] diesel::result::Error),

    #[fail(display = "Parse Error: {}", _0)]
    ParseError(#[fail(cause)] serde_json::Error),

    #[fail(display = "{}", _0)]
    GeneralError(failure::Error),

    #[fail(display = "Unauthorized: {}", _0)]
    Unauthorized(failure::Error),

    #[fail(display = "Invalid request: {}", _0)]
    InvalidRequest(Box<ServiceError>),

    #[fail(display = "Internal Server Error: {}", _0)]
    InternalServerError(Box<ServiceError>),
}

impl ServiceError {
    pub fn to_http_error(self) -> actix_web::Error {
        use ServiceError::*;

        match self {
            InvalidRequest(err) => actix_web::error::ErrorBadRequest(err),
            InternalServerError(err) => actix_web::error::ErrorInternalServerError(err),
            Unauthorized(err) => actix_web::error::ErrorUnauthorized(err),
            err => actix_web::error::ErrorInternalServerError(err),
        }
    }
}
