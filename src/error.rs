use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("LOGIN_FAILED")]
    LoginFailed,
    #[error("SQLX_ERROR")]
    DbError(#[from] sqlx::Error),
    #[error("USER_NOT_FOUND")]
    UserNotFound,
    #[error("PASSWORD_DONT_MATCH")]
    PasswordsDontMatch,
    #[error("FAILED_TO_CREATE")]
    FailedToCreate,
    #[error("TOKEN_NOT_FOUND")]
    TokenNotFound,
    #[error("FAILED_TO_AUTHENTICATE")]
    FailedToAuthenticate,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
