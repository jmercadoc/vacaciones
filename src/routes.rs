use crate::db::DynamoDBClient;
use crate::handlers;
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router(db_client: DynamoDBClient) -> Router {
    Router::new()
        // Rutas de salud y bienvenida
        .route("/", get(handlers::home))
        .route("/health", get(handlers::health))
        // Rutas de empleados
        .route("/empleados", get(handlers::empleado::listar_empleados))
        .route("/empleados/:id", get(handlers::empleado::obtener_empleado))
        // Rutas de solicitudes
        //.route("/solicitudes", post(handlers::solicitud::crear_solicitud))
        // Compartir el cliente de DB con todos los handlers
        .with_state(db_client)
}
