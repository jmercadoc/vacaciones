use axum::{Router, routing::{get}};
use std::net::SocketAddr;


#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>>{

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));


    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e|{
            eprint!("❌ No se pudo vincular el puerto: {}", addr.port());
            eprint!(" Razón: {}", e);
            eprint!(" Sugerencia: ¿Otro proceso podría estar usando este puerto?");
            e
        })?;

    println!("✅ Escuchando en {}", addr);

    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }));


    axum::serve(listener, app)
        .await
        .map_err(|e|{
            eprint!("❌ Error critico en el servidor: {}", e);
            eprint!(" Momento: {}", chrono::Local::now());
            e
        })?;

    Ok(())
}

