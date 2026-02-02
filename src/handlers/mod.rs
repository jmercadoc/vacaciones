pub mod empleado;
pub mod solicitud;

use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

pub async fn home() -> Json<Value> {
    Json(json!({
        "mensaje": "API de Gestión de Vacaciones",
        "version": "1.0.0"
    }))
}

pub async fn health() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"status": "ok"})))
}
