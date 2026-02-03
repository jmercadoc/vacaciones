# 🌴 Sistema de Gestión de Vacaciones

Sistema web para gestionar solicitudes de vacaciones de empleados, construido con Rust, Axum y DynamoDB.

## 🚀 Características

- ✅ **Gestión de empleados**: Registro y consulta de empleados con cálculo automático de días de vacaciones
- 📅 **Solicitudes de vacaciones**: Creación, aprobación y rechazo de solicitudes
- 🧮 **Cálculo automático**: Días de vacaciones según antigüedad (Ley Federal del Trabajo - México)
- 📊 **Días laborables**: Solo cuenta lunes a viernes, excluyendo fines de semana
- 🎯 **Control de días**: Validación de días disponibles antes de aprobar solicitudes
- 🔍 **Filtros**: Filtrar solicitudes por estado (pendiente, aprobada, rechazada)
- 🎨 **Interfaz web**: Templates HTML con Askama
- 🔌 **API REST**: Endpoints JSON para integraciones

## 🛠️ Tecnologías

- **Rust** - Lenguaje de programación
- **Axum** - Framework web asíncrono
- **DynamoDB** - Base de datos NoSQL (AWS)
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
```

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

## 🚀 Ejecución

### Modo desarrollo

```bash
cargo run
```

### Modo producción

```bash
cargo run --release
```

El servidor estará disponible en `http://127.0.0.1:3000`

## 📁 Estructura del proyecto

```
vacaciones/
├── src/
│   ├── main.rs              # Punto de entrada
│   ├── config.rs            # Configuración desde .env
│   ├── routes.rs            # Definición de rutas
│   ├── error.rs             # Manejo de errores
│   ├── db/
│   │   └── dynamodb.rs      # Cliente de DynamoDB
│   ├── models/
│   │   ├── empleado.rs      # Modelo de Empleado
│   │   └── solicitud.rs     # Modelo de SolicitudVacaciones
│   ├── services/
│   │   ├── empleado.rs      # Lógica de negocio de empleados
│   │   └── solicitud.rs     # Lógica de negocio de solicitudes
│   └── handlers/
│       ├── mod.rs           # Handler home
│       ├── empleado.rs      # Handlers de empleados
│       └── solicitud.rs     # Handlers de solicitudes
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
└── README.md
```

## 🌐 Endpoints

### Rutas HTML (Web)

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/` | Página de inicio |
| GET | `/empleados` | Lista de empleados |
| GET | `/empleados/{id}` | Detalle de empleado |
| GET | `/solicitudes` | Lista de solicitudes |
| GET | `/solicitudes/nueva` | Formulario nueva solicitud |

### API REST (JSON)

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/api/empleados` | Lista empleados (JSON) |
| GET | `/api/empleados/{id}` | Obtener empleado (JSON) |
| POST | `/api/solicitudes` | Crear solicitud |
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
  "tipo": "empleado"
}
```

### Solicitud de Vacaciones

```json
{
  "PK": "EMPLEADO#1",
  "SK": "SOLICITUD#uuid",
  "id": "uuid",
  "empleado_id": "1",
  "fecha_inicio": "2025-03-15",
  "fecha_fin": "2025-03-22",
  "dias_solicitados": 5,
  "estado": "pendiente",
  "created_at": "2025-02-01T10:30:00Z",
  "tipo": "solicitud"
}
```

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

## 📝 Ejemplo de uso con curl

### Crear solicitud

```bash
curl -X POST http://localhost:3000/api/solicitudes \
  -H "Content-Type: application/json" \
  -d '{
    "empleado_id": "1",
    "fecha_inicio": "2025-03-15",
    "fecha_fin": "2025-03-22"
  }'
```

### Aprobar solicitud

```bash
curl -X POST http://localhost:3000/api/solicitudes/1/uuid-solicitud/aprobar
```

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
