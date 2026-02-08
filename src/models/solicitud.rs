use serde::{Deserialize, Serialize};
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolicitudVacaciones {
    pub id: String,
    pub empleado_id: String,
    pub empleado_nombre: String,
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub estado: String,
    pub dias_solicitados: i32,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevaSolicitud {
    pub empleado_id: String,
    pub empleado_nombre: String,
    pub fecha_inicio: String,
    pub fecha_fin: String,
}

impl SolicitudVacaciones {
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert("PK".to_string(), AttributeValue::S(format!("EMPLEADO#{}", self.empleado_id)));
        item.insert("SK".to_string(), AttributeValue::S(format!("SOLICITUD#{}", self.id)));
        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("empleado_id".to_string(), AttributeValue::S(self.empleado_id.clone()));
        item.insert("empleado_nombre".to_string(), AttributeValue::S(self.empleado_nombre.clone()));
        item.insert("fecha_inicio".to_string(), AttributeValue::S(self.fecha_inicio.clone()));
        item.insert("fecha_fin".to_string(), AttributeValue::S(self.fecha_fin.clone()));
        item.insert("estado".to_string(), AttributeValue::S(self.estado.clone()));
        item.insert("dias_solicitados".to_string(), AttributeValue::N(self.dias_solicitados.to_string()));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.clone()));
        item.insert("tipo".to_string(), AttributeValue::S("solicitud".to_string()));
        item
    }

    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        Some(SolicitudVacaciones {
            id: item.get("id")?.as_s().ok()?.clone(),
            empleado_id: item.get("empleado_id")?.as_s().ok()?.clone(),
            empleado_nombre: item.get("empleado_nombre")?.as_s().ok()?.clone(),
            fecha_inicio: item.get("fecha_inicio")?.as_s().ok()?.clone(),
            fecha_fin: item.get("fecha_fin")?.as_s().ok()?.clone(),
            estado: item.get("estado")?.as_s().ok()?.clone(),
            dias_solicitados: item.get("dias_solicitados")?.as_n().ok()?.parse().ok()?,
            created_at: item.get("created_at")?.as_s().ok()?.clone(),
        })
    }
}