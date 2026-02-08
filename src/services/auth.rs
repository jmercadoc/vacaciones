use aws_sdk_dynamodb::types::AttributeValue;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::{
    db::DynamoDBClient,
    error::{AppError, AppResult},
    models::empleado::Empleado,
};

pub struct AuthService {
    db: DynamoDBClient,
}

impl AuthService {
    pub fn new(db: DynamoDBClient) -> Self {
        Self { db }
    }

    /// Buscar empleado por email
    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<Empleado>> {
        let result = self
            .db
            .client
            .scan()
            .table_name(&self.db.table_name)
            .filter_expression("email = :email AND tipo = :tipo")
            .expression_attribute_values(":email", AttributeValue::S(email.to_string()))
            .expression_attribute_values(":tipo", AttributeValue::S("empleado".to_string()))
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(items) = result.items {
            if let Some(item) = items.first() {
                return Ok(Empleado::from_item(item));
            }
        }

        Ok(None)
    }

    /// Verificar password contra hash
    pub fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        verify(password, hash).map_err(|e| AppError::InternalError(format!("Password verification failed: {}", e)))
    }

    /// Hashear password
    pub fn hash_password(&self, password: &str) -> AppResult<String> {
        hash(password, DEFAULT_COST).map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))
    }

    /// Actualizar password de un empleado
    pub async fn set_password(&self, empleado_id: &str, password: &str) -> AppResult<()> {
        let password_hash = self.hash_password(password)?;

        let pk = format!("EMPLEADO#{}", empleado_id);

        self.db
            .client
            .update_item()
            .table_name(&self.db.table_name)
            .key("PK", AttributeValue::S(pk))
            .key("SK", AttributeValue::S("METADATA".to_string()))
            .update_expression("SET password_hash = :hash")
            .expression_attribute_values(":hash", AttributeValue::S(password_hash))
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Validar complejidad de password
    pub fn validate_password_strength(&self, password: &str) -> AppResult<()> {
        if password.len() < 8 {
            return Err(AppError::BadRequest(
                "La contraseña debe tener al menos 8 caracteres".to_string(),
            ));
        }

        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());

        if !has_uppercase || !has_lowercase || !has_digit {
            return Err(AppError::BadRequest(
                "La contraseña debe contener mayúsculas, minúsculas y números".to_string(),
            ));
        }

        Ok(())
    }
}
