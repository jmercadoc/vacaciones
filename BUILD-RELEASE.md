# Guía para crear Release de Windows

## Opción 1: Cross-compilación desde WSL/Linux (Recomendada)

### Paso 1: Instalar MinGW

```bash
sudo apt-get update
sudo apt-get install -y mingw-w64
```

### Paso 2: Ejecutar el script de build

```bash
./build-windows-release.sh
```

Esto creará:
- Una carpeta `release-windows/` con todos los archivos necesarios
- Un archivo ZIP `vacaciones-app-v0.1.0-windows-x64.zip` listo para distribuir

### Paso 3: Distribuir

El archivo ZIP contiene:
- `vacaciones-app.exe` - El ejecutable de Windows
- `templates/` - Templates HTML
- `static/` - Archivos CSS/JS
- `.env.example` - Ejemplo de configuración
- `start.bat` - Script para iniciar la aplicación fácilmente
- `INSTALL-WINDOWS.md` - Instrucciones de instalación
- `README.md` - Documentación completa

## Opción 2: Compilar directamente en Windows

Si prefieres compilar directamente en Windows:

### Paso 1: Instalar Rust en Windows

Descarga e instala desde: https://rustup.rs/

### Paso 2: Clonar el proyecto

```powershell
git clone <url-del-repo>
cd vacaciones
```

### Paso 3: Compilar

```powershell
cargo build --release
```

El ejecutable estará en: `target\release\vacaciones-app.exe`

### Paso 4: Crear carpeta de distribución

```powershell
mkdir release-windows
copy target\release\vacaciones-app.exe release-windows\
xcopy templates release-windows\templates\ /E /I
xcopy static release-windows\static\ /E /I
copy .env.example release-windows\
copy README.md release-windows\
```

## Estructura del Release

```
vacaciones-app-v0.1.0-windows-x64/
├── vacaciones-app.exe       # Ejecutable principal
├── templates/               # Templates HTML
│   ├── base.html
│   ├── home.html
│   ├── empleados.html
│   ├── empleado_detalle.html
│   ├── solicitudes.html
│   └── nueva_solicitud.html
├── static/                  # Archivos estáticos
│   ├── css/
│   └── js/
├── .env.example             # Configuración de ejemplo
├── .env                     # (Usuario debe configurar)
├── start.bat                # Script de inicio
├── INSTALL-WINDOWS.md       # Instrucciones de instalación
└── README.md                # Documentación completa
```

## Uso para el usuario final

1. Descomprimir el ZIP
2. Copiar `.env.example` a `.env`
3. Configurar credenciales de AWS en `.env`
4. Ejecutar `start.bat`
5. Abrir navegador en http://127.0.0.1:3000

## Notas

- El ejecutable es estático y no requiere dependencias adicionales de Rust
- El usuario final SOLO necesita:
  - Windows 10/11 (64-bit)
  - AWS CLI configurado
  - Tabla de DynamoDB creada

## GitHub Release (Opcional)

Para crear un release en GitHub:

1. Crea un tag:
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

2. Ve a GitHub → Releases → New Release

3. Sube el archivo ZIP `vacaciones-app-v0.1.0-windows-x64.zip`

4. Agrega notas del release:
   ```markdown
   ## Características
   - Gestión de empleados
   - Solicitudes de vacaciones
   - Cálculo automático de días según antigüedad
   - Exclusión de fines de semana

   ## Instalación
   Ver `INSTALL-WINDOWS.md` incluido en el ZIP.
   ```
