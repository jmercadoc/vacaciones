use askama::Template;
use axum::{
    Json, debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
// use std::collections::HashMap;

use crate::db::DynamoDBClient;
use crate::error::{AppError, AppResult};
use crate::models::{Empleado, NuevaSolicitud, SolicitudVacaciones};
use crate::services::{EmpleadoService, SolicitudService};

use chrono::Utc;
use uuid::Uuid;

// ─── templates ──────────────────────────────────────────────
// solicitudes.html recibe:
//   - solicitudes          : Vec<SolicitudVacaciones>
//   - empleado_nombres     : HashMap<String, String>   // id → nombre
//   - estado_filtro        : Option<String>            // query param ?estado=…
#[derive(Template)]
#[template(path = "solicitudes.html")]
struct SolicitudesTemplate {
    solicitudes: Vec<SolicitudVacaciones>,
    //empleado_nombres: HashMap<String, String>,
    estado_filtro: Option<String>,
    total: usize,
    pendientes: usize,
    aprobadas: usize,
    rechazadas: usize,
}

// nueva_solicitud.html recibe:
//   - empleados                : Vec<Empleado>   (con dias_disponibles calculados)
//   - empleado_preseleccionado : Option<String>  // query param ?empleado_id=…
#[derive(Template)]
#[template(path = "nueva_solicitud.html")]
struct NuevaSolicitudTemplate {
    empleados: Vec<Empleado>,
    empleado_preseleccionado: Option<String>,
}

// ─── query params ───────────────────────────────────────────
#[derive(serde::Deserialize)]
pub struct SolicitudesQuery {
    pub estado: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct NuevaSolicitudQuery {
    pub empleado_id: Option<String>,
}

// ─── handlers ───────────────────────────────────────────────

/// GET /solicitudes
#[debug_handler]
pub async fn listar_solicitudes(
    State(db): State<DynamoDBClient>,
    Query(query): Query<SolicitudesQuery>,
) -> AppResult<impl IntoResponse> {
    // 1. traer todas las solicitudes (en tu servicio real filtrar por estado si quieres)
    let service = SolicitudService::new(db.clone());
    let solicitudes = service.listar_solicitudes().await?;

    // 2. filtrar client-side si viene ?estado=…
    let solicitudes: Vec<_> = match &query.estado {
        Some(estado) => solicitudes
            .into_iter()
            .filter(|s| s.estado == *estado)
            .collect(),
        None => solicitudes,
    };
    let total = solicitudes.len();
    let pendientes = solicitudes
        .iter()
        .filter(|s| s.estado == "pendiente")
        .count();
    let aprobadas = solicitudes
        .iter()
        .filter(|s| s.estado == "aprobada")
        .count();
    let rechazadas = solicitudes
        .iter()
        .filter(|s| s.estado == "rechazada")
        .count();

    // 3. traer nombres de empleados para mostrar en la tabla
    // let service_empleados = EmpleadoService::new(db.clone());
    // let empleados = service_empleados.listar_empleados_con_dias().await?;

    // let empleado_nombres: HashMap<String, String> = empleados
    //     .into_iter()
    //     .map(|e| (e.id.clone(), e.nombre))
    //     .collect();

    let template = SolicitudesTemplate {
        solicitudes,
        // empleado_nombres,
        estado_filtro: query.estado,
        total,
        pendientes,
        aprobadas,
        rechazadas,
    };

    let html = template.render().map_err(|e| {
        crate::error::AppError::TemplateError(format!("Error rendering template: {}", e))
    })?;

    Ok(Html(html))
}

/// GET /solicitudes/nueva
#[debug_handler]
pub async fn nueva_solicitud_form(
    State(db): State<DynamoDBClient>,
    Query(query): Query<NuevaSolicitudQuery>,
) -> AppResult<impl IntoResponse> {
    // traer empleados con días disponibles calculados

    let service_empleados = EmpleadoService::new(db.clone());
    let empleados = service_empleados.listar_empleados_con_dias().await?;
    let template = NuevaSolicitudTemplate {
        empleados,
        empleado_preseleccionado: query.empleado_id,
    };
    let html = template.render().map_err(|e| {
        crate::error::AppError::TemplateError(format!("Error rendering template: {}", e))
    })?;
    Ok(Html(html))
}

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
        empleado_nombre: solicitud.empleado_nombre,
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
      use chrono::{NaiveDate, Datelike, Duration};                                                                                         
                                                                                                                                           
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
                                                                                                                                           
      // Contar solo días laborables (lunes a viernes)                                                                                     
      let mut dias_laborables = 0;                                                                                                         
      let mut fecha_actual = fecha_inicio;                                                                                                 
                                                                                                                                           
      while fecha_actual <= fecha_fin {                                                                                                    
          // weekday(): 0=Lunes, 1=Martes, ..., 4=Viernes, 5=Sábado, 6=Domingo                                                             
          let dia_semana = fecha_actual.weekday().num_days_from_monday();                                                                  
                                                                                                                                           
          if dia_semana < 5 {  // Lunes a Viernes (0-4)                                                                                    
              dias_laborables += 1;                                                                                                        
          }                                                                                                                                
                                                                                                                                           
          fecha_actual += Duration::days(1);                                                                                               
      }                                                                                                                                    
                                                                                                                                           
      Ok(dias_laborables)                                                                                                                  
  }

/// POST /api/solicitudes/:empleado_id/:solicitud_id/aprobar                                                                             
#[debug_handler]
pub async fn aprobar_solicitud(
    State(db): State<DynamoDBClient>,
    Path((empleado_id, solicitud_id)): Path<(String, String)>,
) -> AppResult<Json<SolicitudVacaciones>> {
    use crate::services::SolicitudService;

    let service = SolicitudService::new(db);
    let solicitud = service
        .actualizar_estado(&empleado_id, &solicitud_id, "aprobada")
        .await?;
    Ok(Json(solicitud))
}

/// POST /api/solicitudes/:empleado_id/:solicitud_id/rechazar                                                                            
#[debug_handler]
pub async fn rechazar_solicitud(
    State(db): State<DynamoDBClient>,
    Path((empleado_id, solicitud_id)): Path<(String, String)>,
) -> AppResult<Json<SolicitudVacaciones>> {
    use crate::services::SolicitudService;

    let service = SolicitudService::new(db);
    let solicitud = service
        .actualizar_estado(&empleado_id, &solicitud_id, "rechazada")
        .await?;
    Ok(Json(solicitud))
}
