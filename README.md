# 🌴 Sistema de Gestión de Vacaciones

Sistema web seguro para gestionar solicitudes de vacaciones de empleados, construido con Rust, Axum y DynamoDB. Incluye autenticación con sesiones, autorización basada en roles, y cálculo automático de días de vacaciones según la Ley Federal del Trabajo de México.

> 🔐 **Sistema Seguro**: Incluye autenticación basada en sesiones, bcrypt para passwords, cookies HTTP-only, y autorización por roles (Admin/Usuario).

## ⚡ Quick Start

```bash
# 1. Clonar y configurar
git clone <repo-url>
cd vacaciones
cp .env.example .env
# Editar .env con tus credenciales AWS

# 2. Crear tabla DynamoDB (ver sección Configuración)
aws dynamodb create-table ...

# 3. Compilar y configurar passwords
cargo build
cargo run --bin setup_passwords

# 4. Iniciar servidor
cargo run

# 5. Abrir en navegador
# http://localhost:3000/login
```

## 🚀 Características

- 🔐 **Autenticación segura**: Sistema de login con sesiones y bcrypt para passwords
- 👥 **Control de acceso**: Autorización basada en roles (Admin vs. Usuario regular)
- ✅ **Gestión de empleados**: Registro y consulta de empleados con cálculo automático de días de vacaciones
- 📅 **Solicitudes de vacaciones**: Creación, aprobación y rechazo de solicitudes
- 🧮 **Cálculo automático**: Días de vacaciones según antigüedad (Ley Federal del Trabajo - México)
- 📊 **Días laborables**: Solo cuenta lunes a viernes, excluyendo fines de semana
- 🎯 **Control de días**: Validación de días disponibles antes de aprobar solicitudes
- 🔍 **Filtros**: Filtrar solicitudes por estado (pendiente, aprobada, rechazada)
- 🎨 **Interfaz web**: Templates HTML con Askama
- 🔌 **API REST**: Endpoints JSON para integraciones

## 🛠️ Tecnologías

- **Rust** - Lenguaje de programación (1.89+)
- **Axum 0.8** - Framework web asíncrono
- **DynamoDB** - Base de datos NoSQL (AWS)
- **tower-sessions** - Gestión de sesiones con cookies HTTP-only
- **bcrypt** - Hashing de passwords (cost=12)
- **Askama** - Motor de templates (Jinja-like)
- **Tokio** - Runtime asíncrono
- **Chrono** - Manejo de fechas

## 📋 Requisitos previos

- Rust 1.70+ y Cargo
- AWS CLI configurado con credenciales válidas
- Tabla de DynamoDB creada
- Perfil AWS con permisos de lectura/escritura en DynamoDB

## ⚙️ Configuración

### 1. Clonar el repositorio

```bash
git clone <url-del-repo>
cd vacaciones
```

### 2. Configurar variables de entorno

Crea un archivo `.env` basado en `.env.example`:

```bash
cp .env.example .env
```

Edita `.env` con tu configuración:

```env
# AWS Configuration
AWS_PROFILE=tu-perfil
AWS_REGION=us-east-1

# DynamoDB
DYNAMODB_TABLE_NAME=vacaciones

# Server
SERVER_HOST=127.0.0.1
SERVER_PORT=3000

# Session Configuration (requerido para autenticación)
SESSION_SECRET=tu-secreto-aleatorio-de-32-caracteres-o-mas
SESSION_TTL_DAYS=7
```

> ⚠️ **Importante**: Genera un `SESSION_SECRET` fuerte y único para producción. Puedes usar:
> ```bash
> openssl rand -base64 32
> ```

### 3. Crear tabla en DynamoDB

La tabla debe tener el siguiente esquema:

- **Nombre**: `vacaciones` (o el nombre configurado en `.env`)
- **Partition Key (PK)**: `PK` (String)
- **Sort Key (SK)**: `SK` (String)

```bash
aws dynamodb create-table \
    --table-name vacaciones \
    --attribute-definitions \
        AttributeName=PK,AttributeType=S \
        AttributeName=SK,AttributeType=S \
    --key-schema \
        AttributeName=PK,KeyType=HASH \
        AttributeName=SK,KeyType=RANGE \
    --billing-mode PAY_PER_REQUEST \
    --profile tu-perfil
```

### 4. Instalar dependencias y compilar

```bash
cargo build --release
```

### 5. Configurar passwords de empleados

Antes de poder usar el sistema, necesitas configurar passwords para los empleados existentes:

```bash
cargo run --bin setup_passwords
```

Este comando:
- Lista todos los empleados en la base de datos
- Te permite configurar passwords de forma interactiva
- Valida la complejidad de passwords (8+ caracteres, mayúsculas, minúsculas, números)
- Muestra el estado de cada empleado (con/sin password)

**Ejemplo de ejecución:**

```
=== Setup de Contraseñas para Empleados ===

Obteniendo lista de empleados...

Empleados encontrados: 2
----------------------------------------

Empleado: Juan Pérez (juan@ejemplo.com)
ID: 1
Admin: Sí
Estado: ✗ Sin contraseña
¿Configurar contraseña? (S/n): s
Nueva contraseña (mínimo 8 caracteres, mayúsculas, minúsculas y números): ********
Confirmar contraseña: ********
✓ Contraseña configurada exitosamente
```

## 🚀 Ejecución

### Modo desarrollo

```bash
cargo run
```

### Modo producción

```bash
cargo run --release
```

El servidor estará disponible en:
- **Aplicación web**: `http://127.0.0.1:3000`
- **Login**: `http://127.0.0.1:3000/login`
- **API**: `http://127.0.0.1:3000/api/`

> 💡 **Primera vez**: Accede a `/login` con el email y password configurados en el paso 5.

## 📁 Estructura del proyecto

```
vacaciones/
├── src/
│   ├── main.rs              # Punto de entrada
│   ├── lib.rs               # Biblioteca para binarios
│   ├── config.rs            # Configuración desde .env
│   ├── routes.rs            # Definición de rutas (público/autenticado/admin)
│   ├── error.rs             # Manejo de errores (con thiserror)
│   ├── auth/
│   │   └── mod.rs           # Extractores AuthUser y AdminUser
│   ├── session/
│   │   └── mod.rs           # Session store en DynamoDB
│   ├── db/
│   │   └── dynamodb.rs      # Cliente de DynamoDB
│   ├── models/
│   │   ├── empleado.rs      # Modelo de Empleado (con password_hash)
│   │   └── solicitud.rs     # Modelo de SolicitudVacaciones
│   ├── services/
│   │   ├── auth.rs          # Servicio de autenticación
│   │   ├── empleado.rs      # Lógica de negocio de empleados
│   │   └── solicitud.rs     # Lógica de negocio de solicitudes
│   ├── handlers/
│   │   ├── mod.rs           # Handler home
│   │   ├── auth.rs          # Handlers de login/logout
│   │   ├── empleado.rs      # Handlers de empleados
│   │   └── solicitud.rs     # Handlers de solicitudes
│   └── bin/
│       └── setup_passwords.rs  # CLI para configurar passwords
├── templates/               # Templates HTML (Askama)
│   ├── base.html
│   ├── home.html
│   ├── empleados.html
│   ├── empleado_detalle.html
│   ├── solicitudes.html
│   └── nueva_solicitud.html
├── static/                  # Archivos estáticos (CSS, JS)
├── Cargo.toml
├── .env
├── CLAUDE.md               # Documentación técnica detallada
└── README.md
```

## 🌐 Endpoints

### 🔓 Rutas Públicas (sin autenticación)

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/` | Página de inicio |
| GET | `/health` | Health check |
| GET | `/login` | Página de login |
| POST | `/login` | Autenticación |
| GET | `/empleados` | Lista de empleados |
| GET | `/empleados/{id}` | Detalle de empleado |
| GET | `/api/empleados` | Lista empleados (JSON) |
| GET | `/api/empleados/{id}` | Obtener empleado (JSON) |

### 🔐 Rutas Autenticadas (requieren login)

| Método | Ruta | Descripción | Permisos |
|--------|------|-------------|----------|
| GET | `/solicitudes` | Lista de solicitudes | Usuario: solo propias<br>Admin: todas |
| GET | `/solicitudes/nueva` | Formulario nueva solicitud | Todos |
| POST | `/api/solicitudes` | Crear solicitud | Usuario: solo propia<br>Admin: cualquiera |
| POST | `/logout` | Cerrar sesión | Todos |

### 👑 Rutas de Administrador (requieren `es_admin=true`)

| Método | Ruta | Descripción |
|--------|------|-------------|
| POST | `/api/solicitudes/{empleado_id}/{solicitud_id}/aprobar` | Aprobar solicitud |
| POST | `/api/solicitudes/{empleado_id}/{solicitud_id}/rechazar` | Rechazar solicitud |

## 📊 Modelo de datos (DynamoDB)

### Empleado

```json
{
  "PK": "EMPLEADO#1",
  "SK": "METADATA",
  "id": "1",
  "nombre": "Juan Pérez",
  "email": "juan@ejemplo.com",
  "departamento": "Tecnología",
  "es_admin": true,
  "fecha_ingreso": "2024-01-15",
  "password_hash": "$2b$12$...",
  "tipo": "empleado"
}
```

> 🔒 **Seguridad**: El campo `password_hash` contiene el hash bcrypt del password y nunca se expone en respuestas API (marcado con `#[serde(skip_serializing)]`).

### Solicitud de Vacaciones

```json
{
  "PK": "EMPLEADO#1",
  "SK": "SOLICITUD#uuid",
  "id": "uuid",
  "empleado_id": "1",
  "empleado_nombre": "Juan Pérez",
  "fecha_inicio": "2025-03-15",
  "fecha_fin": "2025-03-22",
  "dias_solicitados": 5,
  "estado": "pendiente",
  "created_at": "2025-02-01T10:30:00Z",
  "tipo": "solicitud"
}
```

### Sesión (gestionada automáticamente)

```json
{
  "PK": "SESSION#uuid-v4",
  "SK": "METADATA",
  "session_id": "uuid-v4",
  "data": "{\"empleado_id\":\"1\"}",
  "expires_at": 1738765432,
  "created_at": "2025-02-01T10:30:00Z",
  "tipo": "session"
}
```

> ℹ️ Las sesiones se almacenan automáticamente en DynamoDB con TTL de 7 días (configurable).

## 🧮 Reglas de negocio

### Días de vacaciones por antigüedad (LFT México)

| Antigüedad | Días |
|-----------|------|
| 1 año | 12 días |
| 2 años | 14 días |
| 3 años | 16 días |
| 4 años | 18 días |
| 5 años | 20 días |
| 6+ años | +2 días cada 5 años |

### Cálculo de días laborables

- Solo se cuentan días de lunes a viernes
- Se excluyen sábados y domingos
- Ejemplo: Viernes a Lunes = 2 días (excluye sábado y domingo)

### Estados de solicitud

- **pendiente**: Recién creada, esperando aprobación
- **aprobada**: Aprobada por administrador, días descontados
- **rechazada**: Rechazada por administrador

## 🔧 Desarrollo

### Ejecutar tests

```bash
cargo test
```

### Verificar código

```bash
cargo clippy
cargo fmt --check
```

### Compilar para producción

```bash
cargo build --release
```

### Binarios disponibles

```bash
cargo run                              # Servidor principal
cargo run --bin setup_passwords        # Configuración de passwords
```

## 🐛 Troubleshooting

### No puedo hacer login

**Síntoma**: La página de login no acepta mis credenciales

**Soluciones:**
1. Verifica que el empleado tenga `password_hash` configurado:
   ```bash
   cargo run --bin setup_passwords
   ```
2. Confirma que el email es exacto (case-sensitive)
3. Verifica que `SESSION_SECRET` esté configurado en `.env`

### Redirección constante a /login

**Síntoma**: Después de login exitoso, soy redirigido de vuelta a login

**Soluciones:**
1. Verifica que las cookies estén habilitadas en el navegador
2. Si usas HTTPS, asegúrate que `with_secure(true)` esté configurado
3. Revisa los logs del servidor para errores de sesión
4. Confirma que DynamoDB tiene permisos de escritura

### Error 403 en rutas de admin

**Síntoma**: Usuario autenticado recibe 403 en `/api/solicitudes/.../aprobar`

**Solución:**
- Solo usuarios con `es_admin: true` pueden aprobar/rechazar
- Verifica en DynamoDB:
  ```bash
  aws dynamodb get-item \
    --table-name vacaciones \
    --key '{"PK":{"S":"EMPLEADO#1"},"SK":{"S":"METADATA"}}'
  ```
- El campo `es_admin` debe ser tipo Boolean, no String

### Sesiones expiran muy rápido

**Síntoma**: Tengo que hacer login constantemente

**Solución:**
- Ajusta `SESSION_TTL_DAYS` en `.env` (default: 7 días)
- Verifica que el servidor no se esté reiniciando
- Confirma que DynamoDB TTL no esté configurado demasiado bajo

## 📝 Ejemplo de uso con curl

### Autenticación

#### Login (obtener cookie de sesión)

```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "email=juan@ejemplo.com&password=MiPassword123" \
  -c cookies.txt \
  -L
```

> El flag `-c cookies.txt` guarda las cookies de sesión, `-L` sigue redirecciones.

#### Logout

```bash
curl -X POST http://localhost:3000/logout \
  -b cookies.txt \
  -L
```

### Solicitudes (requieren autenticación)

#### Crear solicitud

```bash
curl -X POST http://localhost:3000/api/solicitudes \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  -d '{
    "empleado_id": "1",
    "empleado_nombre": "Juan Pérez",
    "fecha_inicio": "2025-03-15",
    "fecha_fin": "2025-03-22"
  }'
```

#### Listar solicitudes

```bash
# Usuario regular: solo ve sus propias solicitudes
# Admin: ve todas las solicitudes
curl http://localhost:3000/solicitudes \
  -b cookies.txt
```

### Administración (requiere `es_admin=true`)

#### Aprobar solicitud

```bash
curl -X POST http://localhost:3000/api/solicitudes/1/uuid-solicitud/aprobar \
  -b cookies.txt
```

#### Rechazar solicitud

```bash
curl -X POST http://localhost:3000/api/solicitudes/1/uuid-solicitud/rechazar \
  -b cookies.txt
```

> 💡 **Tip**: Si recibes un redirect a `/login`, significa que la sesión expiró o no tienes permisos.

## 🔒 Seguridad

### Características de seguridad implementadas

- ✅ **Autenticación basada en sesiones** con cookies HTTP-only
- ✅ **Bcrypt** para hashing de passwords (cost factor: 12)
- ✅ **Autorización por roles** (Admin vs. Usuario regular)
- ✅ **CSRF protection** mediante SameSite cookie policy
- ✅ **Validación de ownership** (usuarios solo pueden modificar sus propios recursos)
- ✅ **Passwords nunca expuestos** en logs ni respuestas API
- ✅ **Sesiones con TTL** (expiración automática)

### Configuración para producción

**Antes de desplegar:**

1. **Generar SESSION_SECRET fuerte:**
   ```bash
   openssl rand -base64 32
   ```
   Agregar al `.env` de producción

2. **Habilitar cookies seguras** en `src/main.rs`:
   ```rust
   .with_secure(true)  // Solo envía cookies sobre HTTPS
   ```

3. **Usar HTTPS:**
   - Requerido para `secure` cookies
   - Usar reverse proxy (nginx, ALB) con TLS

4. **Configurar IAM roles:**
   - Permisos mínimos de DynamoDB (GetItem, PutItem, Query, Scan, DeleteItem)
   - No usar access keys en producción, usar IAM roles

5. **Opcional - DynamoDB TTL:**
   - Configurar TTL en campo `expires_at` para auto-limpieza de sesiones:
   ```bash
   aws dynamodb update-time-to-live \
     --table-name vacaciones \
     --time-to-live-specification "Enabled=true, AttributeName=expires_at"
   ```

### Recomendaciones adicionales

- 🔐 Implementar rate limiting en `/login` (5 intentos / 15 min)
- 📧 Agregar funcionalidad de password reset
- 🔑 Considerar MFA para cuentas admin
- 📝 Logging de eventos de autenticación
- 🔍 Auditoría de acciones de admin

> 📖 Para más detalles técnicos, consulta [CLAUDE.md](CLAUDE.md)

## 🤝 Contribuir

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/nueva-funcionalidad`)
3. Commit tus cambios (`git commit -am 'Agrega nueva funcionalidad'`)
4. Push a la rama (`git push origin feature/nueva-funcionalidad`)
5. Abre un Pull Request

## 📄 Licencia

[MIT License](LICENSE)

## 👥 Autor

**Antonio Mercado** - [antonio.mercado@kodevox.com](mailto:antonio.mercado@kodevox.com)
