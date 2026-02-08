use aws_sdk_dynamodb::types::AttributeValue;
use std::io::{self, Write};
use vacaciones_app::{config::Config, db::DynamoDBClient, services::auth::AuthService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Setup de Contraseñas para Empleados ===\n");

    // Cargar configuración
    let config = Config::from_env().expect("Failed to load configuration");
    let db_client = DynamoDBClient::new(&config).await;
    let auth_service = AuthService::new(db_client.clone());

    // Listar todos los empleados
    println!("Obteniendo lista de empleados...\n");
    let result = db_client
        .client
        .scan()
        .table_name(&db_client.table_name)
        .filter_expression("tipo = :tipo")
        .expression_attribute_values(":tipo", AttributeValue::S("empleado".to_string()))
        .send()
        .await?;

    let items = result.items.unwrap_or_default();

    if items.is_empty() {
        println!("No se encontraron empleados en la base de datos.");
        return Ok(());
    }

    println!("Empleados encontrados: {}\n", items.len());
    println!("----------------------------------------");

    for item in items {
        // Extraer datos del empleado
        let id = item
            .get("id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.as_str())
            .unwrap_or("unknown");
        let nombre = item
            .get("nombre")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.as_str())
            .unwrap_or("Sin nombre");
        let email = item
            .get("email")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.as_str())
            .unwrap_or("Sin email");
        let es_admin = item
            .get("es_admin")
            .and_then(|v| v.as_bool().ok())
            .copied()
            .unwrap_or(false);

        // Verificar si ya tiene password
        let tiene_password = item
            .get("password_hash")
            .and_then(|v| v.as_s().ok())
            .is_some();

        println!("\nEmpleado: {} ({})", nombre, email);
        println!("ID: {}", id);
        println!("Admin: {}", if es_admin { "Sí" } else { "No" });
        println!(
            "Estado: {}",
            if tiene_password {
                "✓ Ya tiene contraseña"
            } else {
                "✗ Sin contraseña"
            }
        );

        // Preguntar si desea configurar/cambiar password
        if tiene_password {
            print!("¿Cambiar contraseña? (s/N): ");
        } else {
            print!("¿Configurar contraseña? (S/n): ");
        }
        io::stdout().flush()?;

        let mut respuesta = String::new();
        io::stdin().read_line(&mut respuesta)?;
        let respuesta = respuesta.trim().to_lowercase();

        let debe_configurar = if tiene_password {
            respuesta == "s" || respuesta == "si" || respuesta == "sí"
        } else {
            respuesta != "n" && respuesta != "no"
        };

        if !debe_configurar {
            continue;
        }

        // Solicitar nueva contraseña
        loop {
            print!("Nueva contraseña (mínimo 8 caracteres, mayúsculas, minúsculas y números): ");
            io::stdout().flush()?;

            let mut password = String::new();
            io::stdin().read_line(&mut password)?;
            let password = password.trim();

            // Validar fortaleza
            match auth_service.validate_password_strength(password) {
                Ok(_) => {
                    // Confirmar contraseña
                    print!("Confirmar contraseña: ");
                    io::stdout().flush()?;

                    let mut password2 = String::new();
                    io::stdin().read_line(&mut password2)?;
                    let password2 = password2.trim();

                    if password != password2 {
                        println!("✗ Las contraseñas no coinciden. Intenta de nuevo.\n");
                        continue;
                    }

                    // Configurar password
                    match auth_service.set_password(id, password).await {
                        Ok(_) => {
                            println!("✓ Contraseña configurada exitosamente");
                            println!("  Email: {}", email);
                            println!("  Password: {}", password);
                            break;
                        }
                        Err(e) => {
                            println!("✗ Error al configurar contraseña: {}", e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("✗ {}", e);
                    continue;
                }
            }
        }

        println!("----------------------------------------");
    }

    println!("\n✓ Proceso completado");
    Ok(())
}
