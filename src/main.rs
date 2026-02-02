mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod services;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = match config::Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Error cargando configuración: {}", e);
            return Err(e.into());
        }
    };

    tracing::info!("Conectando a DynamoDB...");
    let db_client = db::DynamoDBClient::new(&config).await;
    tracing::info!("✅ Conectado a DynamoDB (tabla: {})", db_client.table_name);

    let app = routes::create_router(db_client);

    let addr: SocketAddr = config.server_address().parse().map_err(|e| {
        eprintln!("❌ Dirección de servidor inválida");
        e
    })?;

    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        eprintln!("❌ No se pudo vincular al puerto {}", addr.port());
        eprintln!("   Razón: {}", e);
        eprintln!("   Sugerencia: ¿Otro proceso está usando este puerto?");
        e
    })?;

    tracing::info!("🚀 Servidor corriendo en http://{}", addr);
    tracing::info!("📚 Rutas disponibles:");
    tracing::info!("   GET  /");
    tracing::info!("   GET  /health");
    tracing::info!("   GET  /empleados");
    tracing::info!("   GET  /empleados/:id");
    tracing::info!("   POST /solicitudes");

    axum::serve(listener, app).await.map_err(|e| {
        eprintln!("❌ Error crítico en el servidor: {}", e);
        eprintln!("   Momento: {}", chrono::Local::now());
        e
    })?;
    Ok(())
}
