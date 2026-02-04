# Etapa 1: Builder - Compilación de la aplicación
FROM rust:1.89-slim AS builder

# Instalar dependencias necesarias para compilar
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Crear directorio de trabajo
WORKDIR /app

# Copiar archivos de configuración de Cargo
COPY Cargo.toml Cargo.lock ./

# Copiar templates (necesarios para la compilación de askama)
COPY templates ./templates

# Copiar el código fuente
COPY src ./src

# Compilar en modo release
RUN cargo build --release

# Etapa 2: Runtime - Imagen final ligera
FROM debian:sid-slim

# Instalar dependencias de runtime necesarias
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario no-root para ejecutar la aplicación
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Copiar el binario compilado desde la etapa builder
COPY --from=builder /app/target/release/vacaciones-app /app/vacaciones-app

# Copiar archivos estáticos y templates
COPY static ./static
COPY templates ./templates

# Cambiar permisos
RUN chown -R appuser:appuser /app

# Cambiar a usuario no-root
USER appuser

# Exponer el puerto (puede ser configurado con SERVER_PORT)
EXPOSE 3000

# Variables de entorno por defecto
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000
ENV RUST_LOG=info

# Comando para ejecutar la aplicación
CMD ["/app/vacaciones-app"]
