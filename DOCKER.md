# Guía de Docker para Vacaciones App

Esta guía explica cómo construir y ejecutar la aplicación usando Docker.

## Requisitos Previos

- Docker instalado (versión 20.10 o superior)
- Docker Compose instalado (versión 2.0 o superior)
- Credenciales de AWS configuradas (si usas DynamoDB en AWS)

## Construcción de la Imagen

### Construcción básica

```bash
docker build -t vacaciones-app .
```

### Construcción con nombre y etiqueta específica

```bash
docker build -t vacaciones-app:v1.0.0 .
```

## Ejecución con Docker

### Opción 1: Usando variables de entorno directamente

```bash
docker run -d \
  --name vacaciones-app \
  -p 3000:3000 \
  -e AWS_REGION=us-east-1 \
  -e AWS_ACCESS_KEY_ID=tu_access_key \
  -e AWS_SECRET_ACCESS_KEY=tu_secret_key \
  -e DYNAMODB_TABLE_NAME=vacaciones \
  vacaciones-app
```

### Opción 2: Usando archivo .env

```bash
docker run -d \
  --name vacaciones-app \
  -p 3000:3000 \
  --env-file .env \
  vacaciones-app
```

### Opción 3: Montando credenciales de AWS

```bash
docker run -d \
  --name vacaciones-app \
  -p 3000:3000 \
  -v ~/.aws:/home/appuser/.aws:ro \
  -e AWS_PROFILE=devops \
  -e AWS_REGION=us-east-1 \
  -e DYNAMODB_TABLE_NAME=vacaciones \
  vacaciones-app
```

## Ejecución con Docker Compose

### Desarrollo con DynamoDB Local

1. Crea un archivo `.env` basado en `.env.example`:
   ```bash
   cp .env.example .env
   ```

2. Edita el `.env` para desarrollo local:
   ```env
   AWS_REGION=us-east-1
   AWS_ACCESS_KEY_ID=dummy
   AWS_SECRET_ACCESS_KEY=dummy
   DYNAMODB_TABLE_NAME=vacaciones
   AWS_ENDPOINT_URL=http://dynamodb-local:8000
   SERVER_PORT=3000
   ```

3. Inicia los servicios:
   ```bash
   docker-compose up -d
   ```

4. Crea la tabla en DynamoDB Local:
   ```bash
   docker exec -it vacaciones-app bash
   # Dentro del contenedor, ejecutar el script de inicialización si existe
   ```

### Producción con DynamoDB en AWS

1. Configura el archivo `.env` con credenciales reales de AWS

2. Edita `docker-compose.yml` y comenta la sección de `dynamodb-local` y `depends_on`

3. Inicia la aplicación:
   ```bash
   docker-compose up -d app
   ```

## Comandos Útiles

### Ver logs

```bash
# Ver logs de la aplicación
docker logs -f vacaciones-app

# Ver logs con docker-compose
docker-compose logs -f app
```

### Detener y eliminar contenedores

```bash
# Con Docker
docker stop vacaciones-app
docker rm vacaciones-app

# Con Docker Compose
docker-compose down
```

### Eliminar volúmenes (DynamoDB Local data)

```bash
docker-compose down -v
```

### Reconstruir la imagen

```bash
docker-compose up -d --build
```

### Acceder al contenedor

```bash
docker exec -it vacaciones-app bash
```

## Verificación

Una vez iniciada la aplicación, verifica que esté funcionando:

```bash
curl http://localhost:3000/health
```

## Configuración de Puertos

Por defecto, la aplicación se expone en el puerto 3000. Para cambiar el puerto del host:

```bash
# Opción 1: En docker-compose.yml
ports:
  - "8080:3000"

# Opción 2: Variable de entorno
SERVER_PORT=8080 docker-compose up -d
```

## Solución de Problemas

### Error de conexión a DynamoDB

Si usas DynamoDB en AWS, verifica:
- Las credenciales de AWS son correctas
- El usuario tiene permisos para acceder a DynamoDB
- La región configurada es correcta
- El nombre de la tabla existe

### Error de puerto en uso

Si el puerto 3000 está ocupado:
```bash
# Cambiar el puerto del host
docker run -p 8080:3000 vacaciones-app
```

### Logs de errores

```bash
# Ver logs detallados
docker logs vacaciones-app --tail 100

# Ver logs en tiempo real
docker logs -f vacaciones-app
```

## Tamaño de la Imagen

La imagen utiliza multi-stage build para optimizar el tamaño:
- Etapa de compilación: ~2-3 GB (rust:1.83-slim)
- Imagen final: ~100-150 MB (debian:bookworm-slim)

Para ver el tamaño de tu imagen:
```bash
docker images vacaciones-app
```

## Optimizaciones Adicionales

### Usar caché de dependencias de Cargo

Para acelerar la compilación, puedes crear una capa separada para las dependencias:

```dockerfile
# En el Dockerfile, después de COPY Cargo.toml Cargo.lock
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Luego copiar el código real
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release
```

## Seguridad

- La aplicación se ejecuta con un usuario no-root (`appuser`)
- Los certificados CA están incluidos para conexiones HTTPS
- El archivo `.env` está excluido en `.dockerignore`
- Las credenciales nunca deben estar en el código o en el Dockerfile

## Variables de Entorno Disponibles

| Variable | Descripción | Valor por defecto |
|----------|-------------|-------------------|
| `AWS_REGION` | Región de AWS | `us-east-1` |
| `AWS_ACCESS_KEY_ID` | Access Key de AWS | - |
| `AWS_SECRET_ACCESS_KEY` | Secret Key de AWS | - |
| `AWS_PROFILE` | Perfil de AWS (alternativa a keys) | - |
| `AWS_ENDPOINT_URL` | URL de DynamoDB (para local) | - |
| `DYNAMODB_TABLE_NAME` | Nombre de la tabla | `vacaciones` |
| `SERVER_HOST` | Host del servidor | `0.0.0.0` |
| `SERVER_PORT` | Puerto del servidor | `3000` |
| `RUST_LOG` | Nivel de logs | `info` |
