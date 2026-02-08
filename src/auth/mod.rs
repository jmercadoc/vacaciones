use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Redirect, Response},
    RequestPartsExt,
};
use std::future::Future;
use tower_sessions::Session;

use crate::{
    db::DynamoDBClient,
    models::empleado::Empleado,
    services::empleado::EmpleadoService,
};

/// Usuario autenticado extraído de la sesión
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub empleado: Empleado,
}

/// Usuario admin (autenticado + es_admin=true)
#[derive(Debug, Clone)]
pub struct AdminUser {
    pub empleado: Empleado,
}

/// Errores de autenticación
#[derive(Debug)]
pub enum AuthError {
    Unauthenticated,
    Forbidden,
    DatabaseError(String),
    SessionError(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::Unauthenticated => {
                // Redirigir a login para requests HTML, JSON 401 para API
                Redirect::to("/login").into_response()
            }
            AuthError::Forbidden => {
                (StatusCode::FORBIDDEN, "Forbidden: insufficient permissions").into_response()
            }
            AuthError::DatabaseError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            AuthError::SessionError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    DynamoDBClient: axum::extract::FromRef<S>,
{
    type Rejection = AuthError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // Extraer sesión
            let session = parts
                .extract::<Session>()
                .await
                .map_err(|_| AuthError::SessionError("Failed to get session".to_string()))?;

            // Obtener empleado_id de la sesión
            let empleado_id: String = session
                .get("empleado_id")
                .await
                .map_err(|e| AuthError::SessionError(format!("Failed to read session: {}", e)))?
                .ok_or(AuthError::Unauthenticated)?;

            // Extraer DynamoDB client del estado
            let State(db_client): State<DynamoDBClient> = State::from_request_parts(parts, state)
                .await
                .map_err(|_| AuthError::DatabaseError("Failed to get database client".to_string()))?;

            // Cargar empleado desde DB
            let service = EmpleadoService::new(db_client);
            let empleado = service
                .obtener_empleado(&empleado_id)
                .await
                .map_err(|_| AuthError::Unauthenticated)?;

            Ok(AuthUser { empleado })
        }
    }
}

impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
    DynamoDBClient: axum::extract::FromRef<S>,
{
    type Rejection = AuthError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // Primero verificar que esté autenticado
            let auth_user = AuthUser::from_request_parts(parts, state).await?;

            // Verificar que sea admin
            if !auth_user.empleado.es_admin {
                return Err(AuthError::Forbidden);
            }

            Ok(AdminUser {
                empleado: auth_user.empleado,
            })
        }
    }
}
