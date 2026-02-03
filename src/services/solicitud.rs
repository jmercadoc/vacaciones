use crate::db::DynamoDBClient;
use crate::error::{AppError, AppResult};
use crate::models::{Empleado, SolicitudVacaciones};
use aws_sdk_dynamodb::types::AttributeValue;

use chrono::Datelike;

pub struct SolicitudService {
    db: DynamoDBClient,
}

impl SolicitudService {
    pub fn new(db: DynamoDBClient) -> Self {
        Self { db }
    }

    /// Lista todas las solicitudes de vacaciones
    pub async fn listar_solicitudes(&self) -> AppResult<Vec<SolicitudVacaciones>> {
        let result = self
            .db
            .client
            .scan()
            .table_name(&self.db.table_name)
            .filter_expression("begins_with(SK, :sk)")
            .expression_attribute_values(":sk", AttributeValue::S("SOLICITUD#".to_string()))
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let solicitudes: Vec<SolicitudVacaciones> = result
            .items()
            .iter()
            .filter_map(|item| SolicitudVacaciones::from_item(item))
            .collect();

        Ok(solicitudes)
    }

    /// Actualiza el estado de una solicitud                                                                                                 
    pub async fn actualizar_estado(
        &self,
        empleado_id: &str,
        solicitud_id: &str,
        nuevo_estado: &str,
    ) -> AppResult<SolicitudVacaciones> {
        // Validar que el estado sea válido
        if !["pendiente", "aprobada", "rechazada"].contains(&nuevo_estado) {
            return Err(AppError::BadRequest(format!(
                "Estado inválido: {}",
                nuevo_estado
            )));
        }

        // Primero obtener la solicitud para verificar que existe
        let result = self
            .db
            .client
            .get_item()
            .table_name(&self.db.table_name)
            .key("PK", AttributeValue::S(format!("EMPLEADO#{}", empleado_id)))
            .key(
                "SK",
                AttributeValue::S(format!("SOLICITUD#{}", solicitud_id)),
            )
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let item = result
            .item()
            .ok_or_else(|| AppError::NotFound("Solicitud no encontrada".to_string()))?;

        let mut solicitud = SolicitudVacaciones::from_item(item)
            .ok_or_else(|| AppError::InternalError("Error al parsear solicitud".to_string()))?;

        // Actualizar el estado
        solicitud.estado = nuevo_estado.to_string();

        // Guardar en DynamoDB
        self.db
            .client
            .put_item()
            .table_name(&self.db.table_name)
            .set_item(Some(solicitud.to_item()))
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(solicitud)
    }
}
