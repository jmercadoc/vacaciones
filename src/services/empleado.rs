use crate::db::DynamoDBClient;
use crate::error::{AppError, AppResult};
use crate::models::{Empleado, SolicitudVacaciones};
use aws_sdk_dynamodb::types::AttributeValue;

pub struct EmpleadoService {
    db: DynamoDBClient,
}

impl EmpleadoService {
    pub fn new(db: DynamoDBClient) -> Self {
        Self { db }
    }

    /// Obtiene un empleado con sus días calculados
    pub async fn obtener_empleado_con_dias(&self, empleado_id: &str) -> AppResult<Empleado> {
        // 1. Obtener empleado de la DB
        let mut empleado = self.obtener_empleado(empleado_id).await?;

        // 2. Calcular días tomados en el año actual
        let dias_tomados = self.calcular_dias_tomados(empleado_id).await?;

        // 3. Calcular información
        let antiguedad = empleado.calcular_antiguedad();
        let dias_por_ley = empleado.calcular_dias_por_ley();
        let dias_disponibles = empleado.calcular_dias_disponibles(dias_tomados);

        // 4. Agregar información calculada
        empleado.dias_tomados = Some(dias_tomados);
        empleado.dias_disponibles = Some(dias_disponibles);
        empleado.antiguedad_anos = Some(antiguedad);

        Ok(empleado)
    }

    /// Obtiene empleado de la base de datos
    async fn obtener_empleado(&self, empleado_id: &str) -> AppResult<Empleado> {
        let result = self
            .db
            .client
            .get_item()
            .table_name(&self.db.table_name)
            .key("PK", AttributeValue::S(format!("EMPLEADO#{}", empleado_id)))
            .key("SK", AttributeValue::S("METADATA".to_string()))
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let item = result
            .item()
            .ok_or_else(|| AppError::NotFound(format!("Empleado {} no encontrado", empleado_id)))?;

        Empleado::from_item(item)
            .ok_or_else(|| AppError::InternalError("Error al parsear empleado".to_string()))
    }

    /// Calcula los días tomados en el año actual
    async fn calcular_dias_tomados(&self, empleado_id: &str) -> AppResult<i32> {
        let ano_actual = chrono::Utc::now().year();

        let result = self
            .db
            .client
            .query()
            .table_name(&self.db.table_name)
            .key_condition_expression("PK = :pk AND begins_with(SK, :sk)")
            .expression_attribute_values(
                ":pk",
                AttributeValue::S(format!("EMPLEADO#{}", empleado_id)),
            )
            .expression_attribute_values(":sk", AttributeValue::S("SOLICITUD#".to_string()))
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let dias_tomados: i32 = result
            .items()
            .iter()
            .filter_map(|item| SolicitudVacaciones::from_item(item))
            .filter(|solicitud| {
                // Solo contar solicitudes aprobadas del año actual
                solicitud.estado == "aprobada"
                    && solicitud.fecha_inicio.starts_with(&ano_actual.to_string())
            })
            .map(|solicitud| solicitud.dias_solicitados)
            .sum();

        Ok(dias_tomados)
    }

    /// Lista todos los empleados con sus días calculados
    pub async fn listar_empleados_con_dias(&self) -> AppResult<Vec<Empleado>> {
        // Obtener todos los empleados
        let result = self
            .db
            .client
            .scan()
            .table_name(&self.db.table_name)
            .filter_expression("tipo = :tipo")
            .expression_attribute_values(":tipo", AttributeValue::S("empleado".to_string()))
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut empleados: Vec<Empleado> = result
            .items()
            .iter()
            .filter_map(|item| Empleado::from_item(item))
            .collect();

        // Calcular días para cada empleado
        for empleado in &mut empleados {
            let dias_tomados = self.calcular_dias_tomados(&empleado.id).await?;
            empleado.dias_tomados = Some(dias_tomados);
            empleado.antiguedad_anos = Some(empleado.calcular_antiguedad());
            empleado.dias_disponibles = Some(empleado.calcular_dias_disponibles(dias_tomados));
        }

        Ok(empleados)
    }
}
