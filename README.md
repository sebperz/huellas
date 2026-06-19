![Huellas](huellas.jpg)

# Huellas

Herramienta de escritorio para extraer, validar y reportar la metadata de archivos PDF. Asegura la trazabilidad e integridad de documentos mediante análisis automático.

## Características

- Extrae metadata estándar de PDFs: título, autor, creador, productor, fechas y hash SHA-256
- Valida que todos los campos obligatorios estén presentes
- Interfaz gráfica con drag & drop de carpetas
- Escaneo recursivo de subcarpetas
- Barra de progreso con opción de cancelar
- Exportación de resultados a CSV y Excel
- Funciona en Windows, macOS y Linux
- Sin instalación: un solo ejecutable, doble clic y listo

## Descarga (sin compilar)

Descargá el binario para tu sistema operativo desde [Releases](https://github.com/sebperz/metadata/releases):

| SO | Archivo |
|---|---|
| Linux | `huellas-linux-x86_64` |
| Windows | `huellas-windows-x86_64.exe` |
| macOS Intel | `huellas-macos-x86_64` |
| macOS Apple Silicon | `huellas-macos-aarch64` |

Doble clic en el archivo y la herramienta se abre en tu navegador. Sin instalación.

## Requisitos para compilar

- Rust 1.70+

```bash
git clone https://github.com/sebperz/metadata.git
cd metadata
cargo build --release
```

El binario se genera en `target/release/huellas`.

## Uso

1. Ejecutá `huellas` (doble clic o desde terminal)
2. Arrastrá una carpeta con PDFs a la zona de drop, o usá el botón "Seleccionar carpeta"
3. La herramienta escanea recursivamente todos los PDFs
4. Al finalizar, ves un resumen con la cantidad de archivos OK y con errores
5. Exportá los resultados a CSV o Excel

## Campos analizados

| Campo | Fuente | Descripción |
|---|---|---|
| Título | `/Info` / XMP | Nombre del documento |
| Autor | `/Info` / XMP | Responsable del contenido |
| Creador | `/Info` / XMP | Aplicación origen (Word, Excel, etc.) |
| Productor | `/Info` / XMP | Software que generó el PDF |
| Fecha de creación | `/Info` / XMP | Cuándo se creó el contenido |
| Fecha de modificación | `/Info` / XMP | Última modificación |
| SHA-256 | Calculado | Huella criptográfica de integridad |

## Validación

Un PDF se considera **OK** si los 7 campos están presentes y no vacíos. Casos de error:

- Campos faltantes o vacíos
- PDF encriptado (no se puede leer)
- PDF corrupto o inválido
- Sin permisos de lectura

## Stack

- **Backend**: Rust (axum, lopdf, sha2)
- **Frontend**: HTML/CSS/JS vanilla (servido embebido en el binario)
- **Empaquetado**: Binario único, abre el navegador automáticamente

## Licencia

MIT
