use crate::db::DynamoDBClient;
use crate::error::AppResult;
use crate::models::Empleado;
use crate::services::EmpleadoService;

use axum::{
    Json,
    extract::{Path, State},
    debug_handler,  
};

/// GET /empleados - Lista todos los empleados con sus días calculados
#[debug_handler]
pub async fn listar_empleados(State(db): State<DynamoDBClient>) -> AppResult<Json<Vec<Empleado>>> {
    let service = EmpleadoService::new(db);
    let empleados = service.listar_empleados_con_dias().await?;
    Ok(Json(empleados))
}

/// GET /empleados/:id - Obtiene un empleado específico
#[debug_handler]
pub async fn obtener_empleado(
    Path(id): Path<String>,
    State(db): State<DynamoDBClient>,
) -> AppResult<Json<Empleado>> {
    let service = EmpleadoService::new(db);
    let empleado = service.obtener_empleado_con_dias(&id).await?;
    Ok(Json(empleado))
}
