use crate::db::DynamoDBClient;
use crate::error::AppResult;
use crate::models::Empleado;
use crate::services::EmpleadoService;

use askama::Template;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "empleados.html")]
struct EmpleadosTemplate {
    empleados: Vec<Empleado>,
}

#[derive(Template)]
#[template(path = "empleado_detalle.html")]
struct EmpleadoDetalleTemplate {
    empleado: Empleado,
}

/// GET /empleados - Lista todos los empleados con sus días calculados
#[debug_handler]
pub async fn listar_empleados(State(db): State<DynamoDBClient>) -> AppResult<impl IntoResponse> {
    let service = EmpleadoService::new(db);
    let empleados = service.listar_empleados_con_dias().await?;
    let template = EmpleadosTemplate { empleados };
    let html = template.render().map_err(|e| {
        crate::error::AppError::TemplateError(format!("Error rendering template: {}", e))
    })?;
    Ok(Html(html))
}

/// GET /empleados/:id - Obtiene un empleado específico
#[debug_handler]
pub async fn obtener_empleado(
    Path(id): Path<String>,
    State(db): State<DynamoDBClient>,
) -> AppResult<impl IntoResponse> {
    let service = EmpleadoService::new(db);
    let empleado = service.obtener_empleado_con_dias(&id).await?;
    let template = EmpleadoDetalleTemplate {
        empleado: empleado.clone(),
    };
    let html = template.render().map_err(|e| {
        crate::error::AppError::TemplateError(format!("Error rendering template: {}", e))
    })?;
    Ok(Html(html))
}

// ============ HANDLERS JSON (API) ============

/// GET /api/empleados - API JSON de empleados
#[debug_handler]
pub async fn listar_empleados_json(
    State(db): State<DynamoDBClient>,
) -> AppResult<Json<Vec<Empleado>>> {
    let service = EmpleadoService::new(db);
    let empleados = service.listar_empleados_con_dias().await?;
    Ok(Json(empleados))
}

/// GET /api/empleados/:id - API JSON de empleado específico
#[debug_handler]
pub async fn obtener_empleado_json(
    Path(id): Path<String>,
    State(db): State<DynamoDBClient>,
) -> AppResult<Json<Empleado>> {
    let service = EmpleadoService::new(db);
    let empleado = service.obtener_empleado_con_dias(&id).await?;
    Ok(Json(empleado))
}
