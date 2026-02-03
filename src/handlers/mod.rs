pub mod empleado;
pub mod solicitud;

use crate::error::AppResult;
use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

pub async fn home() -> AppResult<impl IntoResponse> {
    let template = HomeTemplate;
    let html = template.render().map_err(|e| {
        crate::error::AppError::TemplateError(format!("Error rendering template: {}", e))
    })?;
    Ok(Html(html))
}

pub async fn health() -> &'static str {
    "OK"
}
