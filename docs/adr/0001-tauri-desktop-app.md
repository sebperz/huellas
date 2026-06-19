# Tauri como framework de escritorio cross-platform

El proyecto requiere una aplicación GUI que funcione en Windows, macOS y Linux, con instalación cero para el usuario final (descargar un ejecutable, doble clic, funciona). Tras evaluar opciones, elegimos Tauri sobre Electron y Python+PyInstaller.

## Considered options

**Electron**: maduro, ecosistema enorme, PDF parsing trivial con librerías Node. Descartado por el tamaño del binario (80+ MB) y consumo de RAM, inaceptables para una herramienta simple de metadata.

**Python + PyInstaller**: librerías PDF excelentes (pikepdf, pypdf), desarrollo rápido. Descartado por binarios de 40-80 MB y UI menos pulida (tkinter se ve anticuada, alternativas como Flet aún son inmaduras para empaquetado).

**Tauri**: binario de 3-8 MB, UI web moderna con drag & drop nativo, usa el webview del sistema operativo. El costo es que el parsing de PDF en Rust tiene menos librerías — aceptable porque solo necesitamos extraer metadata del diccionario `/Info` (no renderizar ni manipular PDFs). La librería `lopdf` cubre este caso.
