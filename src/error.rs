use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Template error: {0}")]
    TemplateError(String),
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError::TemplateError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized(msg) => {
                // Para errores de autenticación, redirigir a login con mensaje
                return Redirect::to(&format!("/login?error={}", urlencoding::encode(msg)))
                    .into_response();
            }
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::TemplateError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template error: {}", msg),
            ),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
