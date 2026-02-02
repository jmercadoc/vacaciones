use crate::db::DynamoDBClient;
use crate::error::{AppError, AppResult};
use crate::models::{NuevaSolicitud, SolicitudVacaciones};
use axum::{Json, extract::State, http::StatusCode};
use chrono::Utc;
use uuid::Uuid;

/// POST /solicitudes - Crea una nueva solicitud de vacaciones
pub async fn crear_solicitud(
    State(db): State<DynamoDBClient>,
    Json(solicitud): Json<NuevaSolicitud>,
) -> AppResult<(StatusCode, Json<SolicitudVacaciones>)> {
    // Calcular días solicitados (simplificado, asume formato YYYY-MM-DD)
    let dias = calcular_dias_entre_fechas(&solicitud.fecha_inicio, &solicitud.fecha_fin)?;

    let nueva_solicitud = SolicitudVacaciones {
        id: Uuid::new_v4().to_string(),
        empleado_id: solicitud.empleado_id,
        fecha_inicio: solicitud.fecha_inicio,
        fecha_fin: solicitud.fecha_fin,
        estado: "pendiente".to_string(),
        dias_solicitados: dias,
        created_at: Utc::now().to_rfc3339(),
    };

    // Guardar en DynamoDB
    db.client
        .put_item()
        .table_name(&db.table_name)
        .set_item(Some(nueva_solicitud.to_item()))
        .send()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(nueva_solicitud)))
}

fn calcular_dias_entre_fechas(inicio: &str, fin: &str) -> AppResult<i32> {
    use chrono::NaiveDate;

    let fecha_inicio = NaiveDate::parse_from_str(inicio, "%Y-%m-%d").map_err(|_| {
        AppError::BadRequest("Formato de fecha_inicio inválido. Use YYYY-MM-DD".to_string())
    })?;

    let fecha_fin = NaiveDate::parse_from_str(fin, "%Y-%m-%d").map_err(|_| {
        AppError::BadRequest("Formato de fecha_fin inválido. Use YYYY-MM-DD".to_string())
    })?;

    if fecha_fin < fecha_inicio {
        return Err(AppError::BadRequest(
            "La fecha_fin debe ser posterior a fecha_inicio".to_string(),
        ));
    }

    let dias = (fecha_fin - fecha_inicio).num_days() + 1;
    Ok(dias as i32)
}
