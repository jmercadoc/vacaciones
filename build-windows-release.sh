#!/bin/bash
# Script para crear release de Windows desde Linux/WSL

set -e

echo "🔨 Compilando para Windows (x86_64-pc-windows-gnu)..."
cargo build --release --target x86_64-pc-windows-gnu

RELEASE_DIR="release-windows"
VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)

echo "📦 Creando paquete de release v${VERSION}..."

# Limpiar directorio de release anterior
rm -rf $RELEASE_DIR
mkdir -p $RELEASE_DIR

# Copiar binario
echo "  → Copiando binario..."
cp target/x86_64-pc-windows-gnu/release/vacaciones-app.exe $RELEASE_DIR/

# Copiar templates
echo "  → Copiando templates..."
cp -r templates $RELEASE_DIR/

# Copiar archivos estáticos
echo "  → Copiando static..."
cp -r static $RELEASE_DIR/

# Copiar archivo de configuración de ejemplo
echo "  → Copiando .env.example..."
cp .env.example $RELEASE_DIR/

# Copiar README
echo "  → Copiando README.md..."
cp README.md $RELEASE_DIR/

# Crear archivo .env vacío
touch $RELEASE_DIR/.env

# Crear script de inicio para Windows
cat > $RELEASE_DIR/start.bat << 'EOFBAT'
@echo off
echo ====================================
echo Sistema de Gestion de Vacaciones
echo ====================================
echo.

REM Verificar si existe .env
if not exist .env (
    echo [ERROR] Archivo .env no encontrado
    echo.
    echo Por favor:
    echo 1. Copia .env.example a .env
    echo 2. Configura tus variables de entorno
    echo.
    pause
    exit /b 1
)

echo Iniciando servidor...
echo.
echo Servidor disponible en: http://127.0.0.1:3000
echo Presiona Ctrl+C para detener
echo.

vacaciones-app.exe

pause
EOFBAT

# Crear README de instalación para Windows
cat > $RELEASE_DIR/INSTALL-WINDOWS.md << 'EOFREADME'
# Instalación en Windows

## Requisitos previos

- AWS CLI instalado y configurado
- Tabla de DynamoDB creada (ver README.md)

## Pasos de instalación

### 1. Configurar variables de entorno

Copia el archivo `.env.example` a `.env`:

```
copy .env.example .env
```

Edita `.env` con tus valores:

```env
AWS_PROFILE=tu-perfil
AWS_REGION=us-east-1
DYNAMODB_TABLE_NAME=vacaciones
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
```

### 2. Verificar AWS CLI

Abre PowerShell o CMD y verifica que AWS CLI esté configurado:

```powershell
aws configure list
```

### 3. Iniciar la aplicación

Haz doble clic en `start.bat` o ejecuta desde CMD:

```
start.bat
```

La aplicación estará disponible en: http://127.0.0.1:3000

## Solución de problemas

### Error: "AWS credentials not found"

Configura AWS CLI:

```powershell
aws configure --profile tu-perfil
```

### Error: "DynamoDB table not found"

Crea la tabla usando AWS CLI (ver README.md para el comando completo).

### El servidor no inicia

Verifica que:
1. El archivo `.env` existe y está configurado correctamente
2. El puerto 3000 no esté en uso por otra aplicación
3. Tienes permisos para ejecutar el programa

### Cambiar el puerto

Edita `.env` y cambia `SERVER_PORT=3000` al puerto deseado.
EOFREADME

echo "✅ Release creado en: $RELEASE_DIR/"
echo ""
echo "📁 Contenido del release:"
ls -lh $RELEASE_DIR/

# Crear archivo ZIP
ZIP_NAME="vacaciones-app-v${VERSION}-windows-x64.zip"
echo ""
echo "📦 Creando archivo ZIP: $ZIP_NAME"
cd $RELEASE_DIR
zip -r ../$ZIP_NAME * > /dev/null
cd ..

echo "✅ Archivo ZIP creado: $ZIP_NAME"
echo ""
echo "📤 Para distribuir:"
echo "   1. Sube el archivo $ZIP_NAME a GitHub Releases"
echo "   2. O compártelo directamente"
echo ""
echo "🎉 Release completado!"
