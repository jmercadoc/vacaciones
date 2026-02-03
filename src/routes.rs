use crate::db::DynamoDBClient;
use crate::handlers;
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;

pub fn create_router(db_client: DynamoDBClient) -> Router {
    Router::new()
        // Rutas de salud y bienvenida
        .route("/", get(handlers::home))
        .route("/health", get(handlers::health))
        // Rutas de empleados
        .route("/empleados", get(handlers::empleado::listar_empleados))
        .route("/empleados/{id}", get(handlers::empleado::obtener_empleado))
        // Rutas de solicitudes
        .route("/solicitudes", get(handlers::solicitud::listar_solicitudes))
        .route(
            "/solicitudes/nueva",
            get(handlers::solicitud::nueva_solicitud_form),
        )
        // Archivos estáticos (CSS, JS, imágenes)
        .nest_service("/static", ServeDir::new("static"))
        // Rutas API JSON (opcional)
        // Rutas de empleados
        .route(
            "/api/empleados",
            get(handlers::empleado::listar_empleados_json),
        )
        .route(
            "/api/empleados/{id}",
            get(handlers::empleado::obtener_empleado_json),
        )
        .route(
            "/api/solicitudes",
            post(handlers::solicitud::crear_solicitud),
        )
        .route(
            "/api/solicitudes/{empleado_id}/{solicitud_id}/aprobar",
            post(handlers::solicitud::aprobar_solicitud),
        )
        .route(
            "/api/solicitudes/{empleado_id}/{solicitud_id}/rechazar",
            post(handlers::solicitud::rechazar_solicitud),
        )
        // Rutas de solicitudes
        //.route("/api/solicitudes", post(handlers::solicitud::crear_solicitud_json))
        // Compartir el cliente de DB con todos los handlers
        .with_state(db_client)
}
