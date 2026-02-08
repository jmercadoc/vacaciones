use crate::db::DynamoDBClient;
use crate::handlers;
use axum::{Router, routing::{get, post}};
use tower_http::services::ServeDir;

pub fn create_router(db_client: DynamoDBClient) -> Router {
    // Rutas públicas (sin autenticación)
    let public_routes = Router::new()
        .route("/", get(handlers::home))
        .route("/health", get(handlers::health))
        .route("/login", get(handlers::auth::login_page))
        .route("/login", post(handlers::auth::login_submit))
        .route("/empleados", get(handlers::empleado::listar_empleados))
        .route("/empleados/{id}", get(handlers::empleado::obtener_empleado))
        .route("/api/empleados", get(handlers::empleado::listar_empleados_json))
        .route("/api/empleados/{id}", get(handlers::empleado::obtener_empleado_json));

    // Rutas autenticadas (requieren AuthUser)
    let auth_routes = Router::new()
        .route("/solicitudes", get(handlers::solicitud::listar_solicitudes))
        .route("/solicitudes/nueva", get(handlers::solicitud::nueva_solicitud_form))
        .route("/api/solicitudes", post(handlers::solicitud::crear_solicitud))
        .route("/logout", post(handlers::auth::logout));

    // Rutas admin (requieren AdminUser)
    let admin_routes = Router::new()
        .route(
            "/api/solicitudes/{empleado_id}/{solicitud_id}/aprobar",
            post(handlers::solicitud::aprobar_solicitud),
        )
        .route(
            "/api/solicitudes/{empleado_id}/{solicitud_id}/rechazar",
            post(handlers::solicitud::rechazar_solicitud),
        );

    Router::new()
        .merge(public_routes)
        .merge(auth_routes)
        .merge(admin_routes)
        .nest_service("/static", ServeDir::new("static"))
        .with_state(db_client)
}
