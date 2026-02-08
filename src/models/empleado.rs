use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Empleado {
    pub id: String,
    pub nombre: String,
    pub departamento: String,
    pub email: String,
    pub es_admin: bool,
    pub fecha_ingreso: String, // Formato: "YYYY-MM-DD"

    // Campo de autenticación (no se serializa en respuestas JSON por seguridad)
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,

    // Campos calculados (no se guardan en DB, se calculan dinámicamente)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dias_disponibles: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dias_tomados: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antiguedad_anos: Option<i32>,
}

impl Empleado {
    /// Calcula los años de antigüedad desde la fecha de ingreso
    pub fn calcular_antiguedad(&self) -> i32 {
        let fecha_ingreso = NaiveDate::parse_from_str(&self.fecha_ingreso, "%Y-%m-%d")
            .unwrap_or_else(|_| Utc::now().date_naive());

        let hoy = Utc::now().date_naive();
        let anos = (hoy.year() - fecha_ingreso.year()) as i32;

        // Ajustar si aún no ha llegado el aniversario este año
        if hoy.month() < fecha_ingreso.month()
            || (hoy.month() == fecha_ingreso.month() && hoy.day() < fecha_ingreso.day())
        {
            anos - 1
        } else {
            anos
        }
    }

    /// Calcula días de vacaciones según Ley Federal del Trabajo (México)
    ///
    /// Reglas:
    /// - 1er año: 12 días
    /// - 2do año: 14 días
    /// - 3er año: 16 días
    /// - 4to año: 18 días
    /// - 5to año: 20 días
    /// - A partir del 6to año: +2 días cada 5 años
    pub fn calcular_dias_por_ley(&self) -> i32 {
        let anos = self.calcular_antiguedad();

        match anos {
            0 => 0,  // Menos de 1 año = sin vacaciones
            1 => 12, // 1er año
            2 => 14, // 2do año
            3 => 16, // 3er año
            4 => 18, // 4to año
            5 => 20, // 5to año
            n if n >= 6 => {
                // A partir del 6to año: 20 + 2 días cada 5 años
                let anos_extra = n - 5;
                let incrementos = anos_extra / 5;
                20 + (incrementos * 2)
            }
            _ => 0,
        }
    }

    /// Calcula los días disponibles restando los días tomados
    pub fn calcular_dias_disponibles(&self, dias_tomados: i32) -> i32 {
        let dias_por_ley = self.calcular_dias_por_ley();
        (dias_por_ley - dias_tomados).max(0)
    }

    // pub fn to_item(&self) -> HashMap<String, AttributeValue> {
    //     let mut item = HashMap::new();
    //     item.insert(
    //         "PK".to_string(),
    //         AttributeValue::S(format!("EMPLEADO#{}", self.id)),
    //     );
    //     item.insert("SK".to_string(), AttributeValue::S("METADATA".to_string()));
    //     item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
    //     item.insert("nombre".to_string(), AttributeValue::S(self.nombre.clone()));
    //     item.insert(
    //         "departamento".to_string(),
    //         AttributeValue::S(self.departamento.clone()),
    //     );
    //     item.insert("email".to_string(), AttributeValue::S(self.email.clone()));

    //     item.insert("es_admin".to_string(), AttributeValue::Bool(self.es_admin));
    //     item.insert(
    //         "fecha_ingreso".to_string(),
    //         AttributeValue::S(self.fecha_ingreso.clone()),
    //     );
    //     item.insert(
    //         "tipo".to_string(),
    //         AttributeValue::S("empleado".to_string()),
    //     );
    //     item
    // }

    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert(
            "PK".to_string(),
            AttributeValue::S(format!("EMPLEADO#{}", self.id)),
        );
        item.insert("SK".to_string(), AttributeValue::S("METADATA".to_string()));
        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("nombre".to_string(), AttributeValue::S(self.nombre.clone()));
        item.insert(
            "departamento".to_string(),
            AttributeValue::S(self.departamento.clone()),
        );
        item.insert("email".to_string(), AttributeValue::S(self.email.clone()));
        item.insert("es_admin".to_string(), AttributeValue::Bool(self.es_admin));
        item.insert(
            "fecha_ingreso".to_string(),
            AttributeValue::S(self.fecha_ingreso.clone()),
        );
        item.insert(
            "tipo".to_string(),
            AttributeValue::S("empleado".to_string()),
        );

        // Incluir password_hash si existe
        if let Some(ref password_hash) = self.password_hash {
            item.insert(
                "password_hash".to_string(),
                AttributeValue::S(password_hash.clone()),
            );
        }

        item
    }

    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        Some(Empleado {
            id: item.get("id")?.as_s().ok()?.clone(),
            nombre: item.get("nombre")?.as_s().ok()?.clone(),
            email: item.get("email")?.as_s().ok()?.clone(),
            departamento: item.get("departamento")?.as_s().ok()?.clone(),
            es_admin: *item.get("es_admin")?.as_bool().ok()?,
            fecha_ingreso: item.get("fecha_ingreso")?.as_s().ok()?.clone(),
            password_hash: item
                .get("password_hash")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.clone()),
            dias_disponibles: None,
            dias_tomados: None,
            antiguedad_anos: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_dias_por_ley() {
        // Empleado con 1 año
        let emp1 = Empleado {
            id: "1".to_string(),
            nombre: "Test".to_string(),
            departamento: "IT".to_string(),
            email: "test@test.com".to_string(),
            es_admin: false,
            fecha_ingreso: "2024-01-01".to_string(),
            password_hash: None,
            dias_disponibles: None,
            dias_tomados: None,
            antiguedad_anos: None,
        };
        assert_eq!(emp1.calcular_dias_por_ley(), 14);

        // Empleado con 3 años
        let emp3 = Empleado {
            fecha_ingreso: "2022-01-01".to_string(),
            ..emp1.clone()
        };
        assert_eq!(emp3.calcular_dias_por_ley(), 18);

        // Empleado con 10 años (5to año base + 5 años extra / 5 = 1 incremento)
        let emp10 = Empleado {
            fecha_ingreso: "2015-01-01".to_string(),
            ..emp1.clone()
        };
        assert_eq!(emp10.calcular_dias_por_ley(), 22); // 20 + 2
    }
}
