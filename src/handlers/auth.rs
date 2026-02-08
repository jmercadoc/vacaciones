use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{
    db::DynamoDBClient,
    error::{AppError, AppResult},
    services::auth::AuthService,
};

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

/// GET /login - Mostrar formulario de login
pub async fn login_page(session: Session) -> AppResult<impl IntoResponse> {
    // Si ya está autenticado, redirigir a solicitudes
    if session
        .get::<String>("empleado_id")
        .await
        .unwrap_or(None)
        .is_some()
    {
        return Ok(Redirect::to("/solicitudes").into_response());
    }

    let html = r#"
<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Iniciar Sesión - Sistema de Vacaciones</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }

        .login-container {
            background: white;
            border-radius: 12px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
            padding: 40px;
            width: 100%;
            max-width: 400px;
        }

        .login-header {
            text-align: center;
            margin-bottom: 30px;
        }

        .login-header h1 {
            color: #333;
            font-size: 28px;
            margin-bottom: 8px;
        }

        .login-header p {
            color: #666;
            font-size: 14px;
        }

        .form-group {
            margin-bottom: 20px;
        }

        .form-group label {
            display: block;
            color: #333;
            font-weight: 500;
            margin-bottom: 8px;
            font-size: 14px;
        }

        .form-group input {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e0e0e0;
            border-radius: 8px;
            font-size: 14px;
            transition: border-color 0.3s;
        }

        .form-group input:focus {
            outline: none;
            border-color: #667eea;
        }

        .submit-btn {
            width: 100%;
            padding: 14px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            border-radius: 8px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: transform 0.2s, box-shadow 0.2s;
        }

        .submit-btn:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
        }

        .submit-btn:active {
            transform: translateY(0);
        }

        .error-message {
            background: #fee;
            color: #c33;
            padding: 12px;
            border-radius: 8px;
            margin-bottom: 20px;
            font-size: 14px;
            text-align: center;
        }

        .back-link {
            text-align: center;
            margin-top: 20px;
        }

        .back-link a {
            color: #667eea;
            text-decoration: none;
            font-size: 14px;
        }

        .back-link a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="login-container">
        <div class="login-header">
            <h1>Iniciar Sesión</h1>
            <p>Sistema de Gestión de Vacaciones</p>
        </div>

        <form method="POST" action="/login">
            <div class="form-group">
                <label for="email">Correo Electrónico</label>
                <input
                    type="email"
                    id="email"
                    name="email"
                    required
                    autocomplete="email"
                    placeholder="tu@ejemplo.com"
                >
            </div>

            <div class="form-group">
                <label for="password">Contraseña</label>
                <input
                    type="password"
                    id="password"
                    name="password"
                    required
                    autocomplete="current-password"
                    placeholder="Tu contraseña"
                >
            </div>

            <button type="submit" class="submit-btn">Iniciar Sesión</button>
        </form>

        <div class="back-link">
            <a href="/">← Volver al inicio</a>
        </div>
    </div>
</body>
</html>
    "#;

    Ok(Html(html).into_response())
}

/// POST /login - Procesar login
pub async fn login_submit(
    State(db): State<DynamoDBClient>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> AppResult<impl IntoResponse> {
    let auth_service = AuthService::new(db);

    // Buscar empleado por email
    let empleado = auth_service
        .find_by_email(&form.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Email o contraseña incorrectos".to_string()))?;

    // Verificar que tenga password_hash configurado
    let password_hash = empleado
        .password_hash
        .as_ref()
        .ok_or_else(|| AppError::Unauthorized("Email o contraseña incorrectos".to_string()))?;

    // Verificar password
    let is_valid = auth_service.verify_password(&form.password, password_hash)?;

    if !is_valid {
        return Err(AppError::Unauthorized(
            "Email o contraseña incorrectos".to_string(),
        ));
    }

    // Crear sesión
    session
        .insert("empleado_id", empleado.id.clone())
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to create session: {}", e)))?;

    // Redirigir a solicitudes
    Ok(Redirect::to("/solicitudes"))
}

/// POST /logout - Cerrar sesión
pub async fn logout(session: Session) -> AppResult<impl IntoResponse> {
    session
        .flush()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to destroy session: {}", e)))?;

    Ok(Redirect::to("/login"))
}
