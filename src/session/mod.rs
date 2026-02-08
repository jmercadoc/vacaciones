use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use tower_sessions_core::{
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};

use crate::db::DynamoDBClient;

#[derive(Clone, Debug)]
pub struct DynamoDBSessionStore {
    db: DynamoDBClient,
}

impl DynamoDBSessionStore {
    pub fn new(db: DynamoDBClient) -> Self {
        Self { db }
    }

    fn session_to_item(&self, id: &Id, record: &Record) -> HashMap<String, AttributeValue> {
        let session_id = id.to_string();
        let mut item = HashMap::new();

        item.insert(
            "PK".to_string(),
            AttributeValue::S(format!("SESSION#{}", session_id)),
        );
        item.insert("SK".to_string(), AttributeValue::S("METADATA".to_string()));
        item.insert("tipo".to_string(), AttributeValue::S("session".to_string()));
        item.insert("session_id".to_string(), AttributeValue::S(session_id));

        // Serializar el record completo
        if let Ok(data) = serde_json::to_string(record) {
            item.insert("data".to_string(), AttributeValue::S(data));
        }

        // Agregar expiry time
        item.insert(
            "expires_at".to_string(),
            AttributeValue::N(record.expiry_date.unix_timestamp().to_string()),
        );

        item.insert(
            "created_at".to_string(),
            AttributeValue::S(chrono::Utc::now().to_rfc3339()),
        );

        item
    }

    fn item_to_record(&self, item: HashMap<String, AttributeValue>) -> Option<Record> {
        let data = item.get("data")?.as_s().ok()?;
        serde_json::from_str(data).ok()
    }
}

#[async_trait::async_trait]
impl SessionStore for DynamoDBSessionStore {
    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let item = self.session_to_item(&record.id, record);

        self.db
            .client
            .put_item()
            .table_name(&self.db.table_name)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| {
                session_store::Error::Backend(format!("Failed to save session: {}", e))
            })?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let pk = format!("SESSION#{}", session_id);

        let result = self
            .db
            .client
            .get_item()
            .table_name(&self.db.table_name)
            .key("PK", AttributeValue::S(pk))
            .key("SK", AttributeValue::S("METADATA".to_string()))
            .send()
            .await
            .map_err(|e| {
                session_store::Error::Backend(format!("Failed to load session: {}", e))
            })?;

        if let Some(item) = result.item {
            // Verificar si la sesión ha expirado
            if let Some(expires_at) = item.get("expires_at") {
                if let Some(timestamp) = expires_at.as_n().ok().and_then(|n| n.parse::<i64>().ok()) {
                    let now = chrono::Utc::now().timestamp();
                    if timestamp < now {
                        // Sesión expirada, eliminarla
                        let _ = self.delete(session_id).await;
                        return Ok(None);
                    }
                }
            }

            Ok(self.item_to_record(item))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let pk = format!("SESSION#{}", session_id);

        self.db
            .client
            .delete_item()
            .table_name(&self.db.table_name)
            .key("PK", AttributeValue::S(pk))
            .key("SK", AttributeValue::S("METADATA".to_string()))
            .send()
            .await
            .map_err(|e| {
                session_store::Error::Backend(format!("Failed to delete session: {}", e))
            })?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl ExpiredDeletion for DynamoDBSessionStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        // DynamoDB TTL manejará esto automáticamente si está configurado
        // Este método es principalmente para compatibilidad con el trait
        Ok(())
    }
}
