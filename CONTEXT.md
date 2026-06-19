# Metadata Analyzer

Herramienta para extraer, validar y reportar metadata de archivos PDF, asegurando su trazabilidad e integridad.

## Language

**Análisis**:
Proceso de extraer metadata de PDFs, validarla contra reglas definidas, y generar un reporte consolidado.
_Avoid_: Escaneo, revisión, inspección

**Metadata**:
Conjunto de campos estándar del diccionario `/Info` y XMP de un PDF que describen su procedencia y características.
_Avoid_: Metadatos, propiedades, atributos

**Trazabilidad**:
Capacidad de establecer la procedencia de un PDF — quién lo creó (Author), con qué aplicación origen (Creator), qué software lo generó (Producer), y cuándo (CreationDate, ModDate).
_Avoid_: Rastreabilidad, seguimiento

**Integridad**:
Doble verificación: (1) que todos los campos obligatorios de metadata estén presentes y no vacíos, y (2) que el archivo no haya sido alterado, mediante hash criptográfico SHA-256.
_Avoid_: Completitud, validez

**Reporte**:
Resultado consolidado del análisis, presentado como tabla interactiva en pantalla y exportable a CSV, Excel, o PDF.
_Avoid_: Informe, salida, resultado

**Regla de validación**:
Criterio fijo que verifica la presencia y contenido de un campo de metadata. Si un campo obligatorio falta o está vacío, se marca como error.
_Avoid_: Check, verificación, condición

## Required fields

Los siguientes campos son obligatorios para todo PDF analizado:

| Campo | Fuente PDF | Propósito |
|---|---|---|
| **Title** | `/Info` / XMP dc:title | Identificación del documento |
| **Author** | `/Info` / XMP dc:creator | Responsable del contenido |
| **Creator** | `/Info` / XMP xmp:CreatorTool | Aplicación origen del contenido |
| **Producer** | `/Info` / XMP pdf:Producer | Software que generó el PDF |
| **CreationDate** | `/Info` / XMP xmp:CreateDate | Fecha de creación del contenido |
| **ModDate** | `/Info` / XMP xmp:ModifyDate | Fecha de última modificación |
| **SHA-256** | Calculado sobre el archivo | Huella criptográfica de integridad |

## Relationships

- Un **Análisis** procesa uno o más PDFs y produce un **Reporte**
- Cada PDF es validado contra las **Reglas de validación** fijas
- Un campo de **Metadata** faltante o vacío genera un error en el **Reporte**
- El hash **SHA-256** se calcula siempre y se muestra, pero no se compara contra una referencia externa
- El escaneo de la carpeta de entrada es recursivo — incluye todas las subcarpetas
- Los archivos no-PDF se ignoran silenciosamente; solo se procesan archivos `.pdf`
- Si no se encuentra ningún PDF, se muestra un mensaje y se vuelve al inicio
- Los enlaces simbólicos y accesos directos se ignoran

### Validación

- Si el diccionario `/Info` no existe, los 6 campos de metadata se marcan como error; el **SHA-256** se calcula igual
- Si un campo existe tanto en `/Info` como en XMP, el valor de XMP tiene prioridad (estándar moderno)
- La validación de valores solo verifica presencia (campo existe y no está vacío); no valida formatos de fecha ni contenido semántico
- Un PDF corrupto, encriptado, o sin permisos de lectura se marca como error y el **Análisis** continúa
- El tamaño del archivo no es un límite — la metadata se lee de los primeros/últimos KB

### Flujo de trabajo

- Los PDFs se procesan secuencialmente (uno por uno)
- El **Análisis** se puede cancelar en cualquier momento; se muestran resultados parciales con opción de exportar
- Al finalizar, un botón "Nuevo análisis" permite procesar otra carpeta sin reiniciar la app
- Entre sesiones no se guarda historial — cada **Análisis** es independiente

### Reporte y exportaciones

- El **Reporte** en pantalla muestra un resumen simple: total de PDFs procesados, OK, y con errores
- Las exportaciones (CSV, Excel, PDF) incluyen todos los PDFs con sus 7 campos y una columna de resultado (OK/Error)
- La ruta mostrada es relativa a la carpeta raíz arrastrada
- El usuario elige dónde guardar cada exportación mediante diálogo del sistema operativo
- El reporte PDF contiene portada con resumen + tabla completa de resultados

## Example dialogue

> **Dev:** "Cuando un PDF no tiene **Title**, ¿eso es un error bloqueante?"
> **Domain expert:** "Sí — todo campo obligatorio faltante es un error. El **Reporte** lo marca, pero el **Análisis** continúa con el resto de archivos."
>
> **Dev:** "¿Y si un PDF tiene /Info/Author = 'Juan' pero XMP dc:creator = 'Pedro'?"
> **Domain expert:** "Siempre gana XMP. Es el estándar más moderno. Si XMP no tiene el campo, ahí sí se usa /Info como respaldo."
>
> **Dev:** "El usuario arrastra una carpeta con PDFs, DOCs e imágenes. ¿Qué ve?"
> **Domain expert:** "Solo los PDFs. Los demás archivos se ignoran — no aparecen en el **Reporte** ni en ningún mensaje."
>
> **Dev:** "¿Y si cancela a la mitad?"
> **Domain expert:** "Se detiene el proceso, se muestran los resultados de lo ya analizado, y puede exportarlos."

## Flagged ambiguities

- "integridad" se usó inicialmente para significar tanto "hash del archivo" como "campos completos" — resuelto: el término **Integridad** cubre ambas verificaciones como un solo concepto.
