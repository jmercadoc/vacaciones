# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Sistema de Gestión de Vacaciones (Vacation Management System) - A secure web application for managing employee vacation requests, built with Rust, Axum, and AWS DynamoDB. Implements Mexican Federal Labor Law (Ley Federal del Trabajo) vacation day calculations based on employee seniority.

**Key Features:**
- ✅ Session-based authentication with bcrypt password hashing
- ✅ Role-based authorization (Admin vs. Regular User)
- ✅ Mexican Federal Labor Law vacation day calculations
- ✅ Working day calculations (excludes weekends)
- ✅ DynamoDB single-table design for scalability
- ✅ Dual interface: HTML templates + JSON REST API
- ✅ Interactive password setup tool

**Tech Stack:**
- **Language:** Rust 1.89+
- **Web Framework:** Axum 0.8
- **Database:** AWS DynamoDB (single table design)
- **Authentication:** tower-sessions + bcrypt
- **Templates:** Askama
- **Runtime:** Tokio (async/await)

## Build & Development Commands

### Development
```bash
cargo run                              # Run server in development mode
cargo run --bin setup_passwords        # Configure employee passwords
cargo test                             # Run all tests
cargo clippy                           # Lint code
cargo fmt --check                      # Check code formatting
cargo fmt                              # Auto-format code
```

### Production
```bash
cargo build --release        # Build optimized binary
cargo run --release          # Run in production mode
```

### Docker
```bash
docker build -t vacaciones-app .                    # Build Docker image
docker-compose up -d                                # Start with DynamoDB Local
docker-compose logs -f app                          # View logs
docker-compose down                                 # Stop containers
./build-windows-release.sh                          # Cross-compile for Windows (from WSL/Linux)
```

### Testing
The application is accessible at `http://127.0.0.1:3000` by default. Tests are located in model files using `#[cfg(test)]` blocks.

## Architecture Overview

### Layered Architecture
The codebase follows a clean layered architecture pattern:

1. **Handlers** (`src/handlers/`) - HTTP request/response layer using Axum
   - Receive requests, extract parameters, call services, return responses
   - Support both HTML (Askama templates) and JSON (REST API) responses
   - Use `#[debug_handler]` attribute for better error messages during development
   - `auth.rs`: Login/logout handlers with embedded HTML login page

2. **Services** (`src/services/`) - Business logic layer
   - `EmpleadoService`: Employee operations, vacation day calculations
   - `SolicitudService`: Vacation request operations, state management
   - `AuthService`: Authentication operations (find by email, password hashing/verification)
   - Services encapsulate DynamoDB operations and business rules

3. **Models** (`src/models/`) - Domain entities with serialization/deserialization
   - `Empleado`: Employee entity with vacation calculation methods, includes `password_hash` field
   - `SolicitudVacaciones`: Vacation request entity with DynamoDB mapping
   - Models include `to_item()` and `from_item()` methods for DynamoDB conversion

4. **Auth** (`src/auth/`) - Authentication and authorization layer
   - `AuthUser`: Extractor for authenticated users, validates session and loads employee
   - `AdminUser`: Extractor for admin users, validates `es_admin=true`
   - Used as parameters in handlers to enforce authentication requirements

5. **Session** (`src/session/`) - Session management
   - `DynamoDBSessionStore`: Implements tower-sessions `SessionStore` trait
   - Stores sessions in DynamoDB with automatic expiration
   - HTTP-only cookies with configurable TTL

6. **Database** (`src/db/`) - Data access layer
   - `DynamoDBClient`: Wrapper around AWS SDK DynamoDB client
   - Configured via environment variables (supports AWS profiles or access keys)

### DynamoDB Single-Table Design

The application uses a single DynamoDB table with a composite key pattern:

**Key Structure:**
- `PK` (Partition Key): Entity type + ID (e.g., `EMPLEADO#1`, `SESSION#uuid`)
- `SK` (Sort Key): Metadata or relationship (e.g., `METADATA`, `SOLICITUD#uuid`)
- `tipo` attribute: Discriminator for queries (`empleado`, `solicitud`, `session`)

**Access Patterns:**
1. **Get Employee**: `PK=EMPLEADO#{id}`, `SK=METADATA`
2. **List Employees**: Scan with filter `tipo = empleado`
3. **Get Employee's Requests**: Query `PK=EMPLEADO#{id}`, `SK` begins with `SOLICITUD#`
4. **List All Requests**: Scan with filter `SK` begins with `SOLICITUD#`
5. **Get Session**: `PK=SESSION#{session_id}`, `SK=METADATA`
6. **Find Employee by Email**: Scan with filter `email = :email AND tipo = empleado` (used for login)

**Session Storage:**
- Sessions are stored with `PK=SESSION#{uuid}`, `SK=METADATA`
- Attributes: `session_id`, `data` (serialized Record), `expires_at`, `created_at`
- Expired sessions are automatically cleaned up on access

This design allows efficient queries for employee-specific data while maintaining the ability to list all entities of a type.

### Authentication & Authorization

**Authentication System:**
- Session-based authentication using HTTP-only cookies
- Sessions stored in DynamoDB with 7-day TTL (configurable)
- Passwords hashed with bcrypt (cost=12)
- Email-based login (email + password)

**Authorization Levels:**

1. **Public Routes** (no authentication required):
   - `/`, `/health` - Home and health check
   - `/login` (GET/POST) - Login page and authentication
   - `/empleados`, `/empleados/{id}` - Employee listings (HTML)
   - `/api/empleados`, `/api/empleados/{id}` - Employee API
   - `/static/*` - Static assets

2. **Authenticated Routes** (require `AuthUser` extractor):
   - `/solicitudes` - List vacation requests
     - Regular users: see only their own requests
     - Admins: see all requests
   - `/solicitudes/nueva` - New request form
   - `POST /api/solicitudes` - Create request
     - Users can only create for their own `empleado_id`
     - Admins can create for any employee
   - `POST /logout` - End session

3. **Admin Routes** (require `AdminUser` extractor):
   - `POST /api/solicitudes/{empleado_id}/{solicitud_id}/aprobar` - Approve request
   - `POST /api/solicitudes/{empleado_id}/{solicitud_id}/rechazar` - Reject request

**Extractors:**
- `AuthUser`: Validates session exists and loads employee from DB
  - Returns `AuthError::Unauthenticated` if no valid session
  - Automatically redirects to `/login` on auth failure
- `AdminUser`: Validates session AND checks `empleado.es_admin == true`
  - Returns `AuthError::Forbidden` if user is not admin
  - Returns 403 status code

**Password Management:**
- Initial setup: Use `cargo run --bin setup_passwords` tool
- Password requirements: 8+ characters, uppercase, lowercase, digits
- Passwords stored as bcrypt hashes in `empleado.password_hash` field
- Field is marked `#[serde(skip_serializing)]` to never expose in API responses

**Session Security:**
- Cookies: HTTP-only, SameSite=Lax
- Production: Set `with_secure(true)` for HTTPS-only cookies
- Session data includes only `empleado_id`, employee loaded on each request
- Expired sessions automatically deleted on access

### State Management & Routing

**State Sharing:**
- `DynamoDBClient` is passed to all handlers via Axum's `.with_state()`
- Handlers create service instances per-request: `EmpleadoService::new(db_client.clone())`
- This pattern enables easy testing and avoids global state

**Dual Interface:**
- HTML routes: `/empleados`, `/solicitudes` (return Askama templates)
- API routes: `/api/empleados`, `/api/solicitudes` (return JSON)
- Both interfaces share the same underlying service layer

### Business Logic: Vacation Day Calculations

**Mexican Federal Labor Law (LFT) Rules:**
- Year 1: 12 days
- Year 2: 14 days
- Year 3: 16 days
- Year 4: 18 days
- Year 5: 20 days
- Year 6+: 20 + (2 days per additional 5 years)

**Working Day Calculation:**
- Only Monday-Friday count as working days
- Weekends (Saturday-Sunday) are automatically excluded
- Example: Friday to Monday = 2 days (excludes Sat/Sun)

**Request States:**
- `pendiente` - Newly created, awaiting approval
- `aprobada` - Approved by admin, days deducted from available balance
- `rechazada` - Rejected by admin

**Annual Reset:**
- Vacation days are calculated per calendar year
- `EmpleadoService::calcular_dias_tomados()` filters by current year only
- Days from previous years don't carry over

## Configuration

### Environment Variables
Required variables in `.env` (see `.env.example`):

```env
# AWS credentials (use EITHER profile OR access keys)
AWS_PROFILE=devops           # For AWS profile authentication
AWS_REGION=us-east-1         # AWS region

# OR use access keys (not recommended for production)
# AWS_ACCESS_KEY_ID=...
# AWS_SECRET_ACCESS_KEY=...

# DynamoDB
DYNAMODB_TABLE_NAME=vacaciones

# Server
SERVER_HOST=127.0.0.1        # Use 0.0.0.0 in Docker
SERVER_PORT=3000

# Session Configuration
SESSION_SECRET=change-this-to-a-random-32-character-string-in-production
SESSION_TTL_DAYS=7

# Optional: for local DynamoDB
# AWS_ENDPOINT_URL=http://localhost:8000
```

### DynamoDB Table Setup
The table must be created manually with:
- Table name: Value from `DYNAMODB_TABLE_NAME` env var
- Partition Key: `PK` (String)
- Sort Key: `SK` (String)
- Billing mode: PAY_PER_REQUEST (recommended)

Use the AWS CLI command in README.md to create the table.

## Code Conventions

### Error Handling
- Use `AppError` enum defined with `thiserror` derive macro
- Service methods return `AppResult<T>`
- Errors auto-convert to HTTP responses via `IntoResponse` implementation
- Error types:
  - `NotFound` - 404 resource not found
  - `BadRequest` - 400 invalid input
  - `Unauthorized` - 401 authentication required (redirects to `/login`)
  - `Forbidden` - 403 insufficient permissions
  - `InternalError` - 500 generic internal error
  - `DatabaseError` - 500 database operation failed
  - `TemplateError` - 500 template rendering failed

### Async/Await
- Runtime: Tokio with `features = ["full"]`
- All database operations are async
- Handlers are async functions

### Templates (Askama)
- Templates are in `templates/` directory
- Use `#[derive(Template)]` with `#[template(path = "...")]`
- Templates automatically compile at build time
- Return `Html<String>` from handlers using `.render()?`

### Date Handling
- Use `chrono` crate with `NaiveDate` for date-only values
- Date format: `YYYY-MM-DD` (ISO 8601)
- Timestamps: ISO 8601 format with UTC timezone

### Model Methods
- `to_item()`: Convert model to DynamoDB attribute map (includes `password_hash` if present)
- `from_item()`: Parse DynamoDB attribute map to model (returns `Option`)
- Business logic methods (like `calcular_antiguedad()`) belong in model structs
- Security: `password_hash` field is marked `#[serde(skip_serializing)]` to prevent exposure in API responses

## Common Tasks

### Initial Setup: Configure Employee Passwords

After creating employees in the database, configure their passwords:

```bash
cargo run --bin setup_passwords
```

This interactive tool will:
1. List all employees from DynamoDB
2. Show password status for each (configured/not configured)
3. Allow you to set/change passwords with validation
4. Enforce password complexity: 8+ characters, uppercase, lowercase, digits

**First Login:**
- Navigate to `http://localhost:3000/login`
- Use employee email and configured password
- Session persists for 7 days (configurable via `SESSION_TTL_DAYS`)

### Adding a New Route

**Public Route:**
1. Add handler function in `src/handlers/`
2. Add to `public_routes` in `src/routes.rs`
3. No authentication required

**Protected Route:**
1. Add handler with `auth_user: AuthUser` parameter
2. Add to `auth_routes` in `src/routes.rs`
3. Example:
```rust
pub async fn my_handler(
    State(db): State<DynamoDBClient>,
    auth_user: AuthUser,  // Validates authentication
) -> AppResult<impl IntoResponse> {
    // auth_user.empleado contains the authenticated employee
    // Access granted automatically
}
```

**Admin-Only Route:**
1. Add handler with `admin_user: AdminUser` parameter
2. Add to `admin_routes` in `src/routes.rs`
3. Automatically validates `es_admin == true`

### Adding a New Query Pattern
1. Modify service methods in `src/services/`
2. Use DynamoDB Query (for PK + SK prefix) or Scan (for full table with filter)
3. Map `AttributeValue` results using model's `from_item()` method

### Modifying Vacation Calculation Logic
1. Update methods in `src/models/empleado.rs` (`calcular_dias_por_ley()`, etc.)
2. Tests are in the same file under `#[cfg(test)]`
3. Run `cargo test` to verify changes

### Working with Authentication

**To check if user is admin in handler:**
```rust
if auth_user.empleado.es_admin {
    // Admin-specific logic
} else {
    // Regular user logic
}
```

**To get current user's ID:**
```rust
let user_id = auth_user.empleado.id;
```

**To manually change a password via code:**
```rust
let auth_service = AuthService::new(db_client.clone());
auth_service.set_password("employee_id", "new_password").await?;
```

### Deployment Options
- **Native**: Build release binary and run on server (requires AWS credentials)
- **Docker**: Use provided Dockerfile and docker-compose.yml
- **Windows**: Use `build-windows-release.sh` for cross-compilation from Linux/WSL

See BUILD-RELEASE.md and DOCKER.md for detailed deployment instructions.

## Security Considerations

### Production Checklist

Before deploying to production:

1. **Environment Variables:**
   - ✅ Set strong `SESSION_SECRET` (32+ random characters)
   - ✅ Use AWS IAM roles or secure credential management
   - ✅ Never commit `.env` file to version control

2. **Cookie Security:**
   - ✅ Enable secure cookies in `src/main.rs`: `.with_secure(true)` (requires HTTPS)
   - ✅ Verify `with_http_only(true)` is set (prevents XSS access)
   - ✅ Current setting: `SameSite::Lax` (protects against CSRF)

3. **Password Security:**
   - ✅ Bcrypt cost factor: 12 (default in code)
   - ✅ Password requirements enforced: 8+ chars, mixed case, digits
   - ✅ Passwords never logged or exposed in API responses

4. **DynamoDB:**
   - ✅ Use IAM roles with least-privilege permissions
   - ✅ Enable DynamoDB encryption at rest
   - ✅ Optional: Configure DynamoDB TTL on `expires_at` field for auto-cleanup of expired sessions

5. **HTTPS:**
   - ✅ Always use HTTPS in production (required for secure cookies)
   - ✅ Use reverse proxy (nginx/ALB) for TLS termination if needed

### Security Features

**Implemented:**
- Session-based authentication with server-side validation
- HTTP-only cookies prevent XSS attacks
- Bcrypt password hashing (cost=12)
- CSRF protection via SameSite cookie policy
- Automatic session expiration
- Role-based access control (admin vs. regular user)
- Input validation on password complexity
- Authorization checks at handler level via extractors

**Not Implemented (Future Enhancements):**
- Rate limiting on login endpoint (recommended: 5 attempts per 15 min)
- Password reset functionality
- Multi-factor authentication (MFA)
- Session invalidation on password change
- Audit logging for authentication events
- CSRF tokens for state-changing operations

### Common Security Issues to Avoid

1. **Never expose `password_hash`:**
   - Already protected via `#[serde(skip_serializing)]`
   - Never log password hashes
   - Never return in API responses

2. **Validate ownership:**
   - Always check if user owns resource before modification
   - Example: In `crear_solicitud`, verify `solicitud.empleado_id == auth_user.empleado.id`

3. **Use correct extractors:**
   - `AuthUser` for authenticated routes
   - `AdminUser` for admin-only routes
   - Never manually check `es_admin` when `AdminUser` extractor is available

4. **Session security:**
   - Sessions automatically expire after `SESSION_TTL_DAYS`
   - Logout properly destroys session via `session.flush()`
   - Never store sensitive data in session (only `empleado_id`)

## Troubleshooting

### Authentication Issues

**Problem:** "Redirected to /login immediately after logging in"
- Check that `SESSION_SECRET` is set in `.env`
- Verify cookies are enabled in browser
- Check browser console for cookie errors

**Problem:** "Cannot access protected routes"
- Verify employee has `password_hash` configured
- Run `cargo run --bin setup_passwords` to configure
- Check employee record in DynamoDB

**Problem:** "Admin routes return 403 Forbidden"
- Verify `es_admin` field is `true` (Boolean) in DynamoDB
- Check employee record: `aws dynamodb get-item --table-name vacaciones --key '{"PK":{"S":"EMPLEADO#1"},"SK":{"S":"METADATA"}}'`

### Session Issues

**Problem:** "Sessions not persisting"
- Check DynamoDB permissions (PutItem, GetItem, DeleteItem)
- Verify `SESSION#` items exist in DynamoDB table
- Check server logs for session store errors
