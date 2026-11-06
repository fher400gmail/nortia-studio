# master check · lista de pruebas del hito 3 (en tu Mac)

Para Fer. Lo que el CI no puede mirar: oído, ojo y un Mac con las herramientas de Apple. Unos 35 minutos. Necesitas macOS 14 o superior y un máster real en WAV (24 bit, 44,1 o 48 kHz, de 3 a 5 minutos), una portada cuadrada (JPG o PNG) y otra NO cuadrada.

Qué ya comprueba el CI y no hace falta repetir: el sidecar descargado coincide con su SHA256; `ffmpeg` y `ffprobe` van dentro de la app (universal); el humo exporta [Portada] 3000×3000, [Canvas] 1080×1920 sin audio, [YouTube] 1080p con AAC 48 kHz, [MP3 320] y [Portada animada]; el tiempo de `capabilities()` queda anotado en `smoke-result.json` (artefacto `master-check-humo`).

## Antes de empezar

1. Descarga el artefacto `master-check-macos` de la última ejecución verde de «app» en GitHub (Actions, rama `hito3-media`) y abre `master check` con clic derecho → **Abrir** (la app no está firmada).
2. Mira en el resumen de esa ejecución si hay avisos amarillos. Si hay uno que dice **«afclip no está en este runner»**, la prueba 9 es obligatoria. Si no, el CI ya la hizo (afclip viene de serie en macOS) y puedes saltarla.

Responde con una línea por prueba: **funciona** o **falla: qué viste** (con captura si puedes).

---

## Comprobaciones

### 1. Acerca de
- Haz: pulsa **Acerca de** en la barra superior.
- Esperado: versión `0.1.0`, el componente «Vídeo y MP3» con `ffmpeg-sidecar-v7.1.5-3a21763` (no «No disponible»), el texto de licencias (FFmpeg LGPL, LAME) y el enlace **Licencias y código fuente**. Esc lo cierra.
- Pulsa **Licencias y código fuente**: se abre `terceros.txt` en TextEdit con la tabla de componentes.

### 2. Los vídeos y el MP3 están disponibles
- Haz: suelta tu máster. Mira las fichas de Extras.
- Esperado: MP3 habilitada; Portada, Canvas, YouTube (vídeo) y Portada animada deshabilitadas con el aviso «Falta la portada…» y con borde discontinuo y la pista «Arrastra la portada» (también en oscuro). Ninguna dice «falta ffmpeg».

### 3. Primer plan: tiempo de arranque
- Haz: cierra la app, ábrela y cronometra desde que sueltas el máster hasta que aparece el resumen («N archivos · tamaño»). Es la primera vez que se sondean los codificadores de vídeo.
- Esperado: menos de 3 segundos de espera añadida; nunca más de 8. Anota los segundos.

### 4. Portada cuadrada y no cuadrada
- Haz: arrastra la portada cuadrada sobre el recuadro **«Arrastrar portada»** (a la izquierda del nombre de la canción). Luego cámbiala por la no cuadrada, soltándola en el mismo recuadro. Crea la carpeta con Portada activa.
- Esperado: la cuadrada no da avisos; la no cuadrada avisa «se ha recortado desde el centro». En `09 Imagen/` hay un JPG y un PNG de 3000×3000 (Información → Más info en Finder). El JPG se ve nítido, sin bandas.

### 5. Canvas
- Haz: con portada, crea la carpeta con Canvas activo. Abre `[Canvas] 1080x1920.mp4` con QuickTime.
- Esperado: vídeo vertical de unos 8 s sin audio, movimiento suave de la portada, sin parpadeos ni saltos al repetirse.

### 5b. Material propio en Canvas
- Haz: suelta un vídeo tuyo (MP4/MOV) sobre la ficha **«Canvas»** (en Extras) y crea la carpeta.
- Esperado: la ficha muestra su miniatura y una ×; el Canvas resultante es tu vídeo recodificado a 1080×1920.

### 6. Vídeo de YouTube
- Haz: abre `06 YouTube/… [YouTube] 1080p.mp4` en QuickTime y salta al minuto 2.
- Esperado: portada con audio de la canción, sonido sincronizado y sin chasquidos, empieza al instante (no tarda en cargar), dura lo mismo que el máster (±1 s).

### 7. Portada animada (apágala y enciéndela)
- Haz: enciende la ficha **Portada animada** (nace apagada) y crea la carpeta.
- Esperado: dos archivos en `02 Apple Music/`: `… [Portada animada] 3840x3840.mp4` y `… [Portada animada] 2048x2732.mp4`; ambos abren en QuickTime sin color apagado o verdoso.

### 8. MP3 320
- Haz: abre `08 Promo/… [MP3 320].mp3` en Música o VLC.
- Esperado: suena igual que el máster, 320 kbps (Cmd + I → Más información), y muestra artista, título y la portada incrustada.

### 9. Comprobación de Apple (afclip) (obligatoria solo si el CI avisó de que falta afclip)
`afconvert` y `afclip` vienen con macOS (`/usr/bin`; el CI los encontró en macos-15 y el flujo real pasó). Si el CI no avisó, esta prueba es opcional y solo sirve para ver el informe con tu máster.
- Haz: en Terminal, `which afclip`. Si responde una ruta (lo normal), sigue; si no responde nada, anótalo como «falla: no hay afclip» y salta el resto (instalar herramientas de Apple no entra en esta prueba). Con Apple Music activo, crea la carpeta con tu máster y abre `informe-loudness.txt`.
- Esperado: una línea «Comprobación con las herramientas de Apple (AAC 256): N muestras con clipping». Con un máster que supere −1 dBTP, N > 0; con uno limpio, 0.
- Si ves «Comprobación con las herramientas de Apple: no disponible: no se entendió la respuesta de afclip», el formato de salida no es el que esperamos. Entonces pega en Terminal esto con tu máster (cámbiale la ruta) y envíame la salida completa:
  ```
  cd "$(mktemp -d)" && afconvert "/ruta/máster.wav" i.caf -d 0 -f caff --soundcheck-generate && afconvert i.caf f.m4a -d aac -f m4af -u pgcm 2 --soundcheck-read -b 256000 -q 127 -s 2 && afconvert f.m4a d.wav -d LEF32 -f WAVE && afclip d.wav
  ```
  (Para másters de más de 48 kHz, en el primer paso usa `-d LEF32@44100 -f caff --soundcheck-generate --src-complexity bats -r 127`, o `@48000` si tu máster es de la familia 48.)
- Anota también si `afclip` termina con código distinto de 0 cuando hay clips (`echo $?` justo después).
- Con un máster que dé N > 0, al terminar la pantalla «Listo» muestra una línea con aviso ámbar (no roja): «Revisa: Apple Music: pasa de 0 dB al codificar». No debe salir como fallo ni cambiar el recuento de archivos.

### 10. Cancelar durante un vídeo
- Haz: con todo activo y un máster largo, pulsa **Cancelar** mientras se crea el vídeo de YouTube.
- Esperado: se detiene en pocos segundos, sin ficheros `.part` ni vídeos a medias en la carpeta, y la app sigue respondiendo.

### 10b. Preparando y «Abrir carpeta»
- Haz: con un máster de 4 minutos o más a 96 kHz, pulsa **Crear carpeta de entrega** y mira el botón de progreso.
- Esperado: primero «Preparando… N %» subiendo (no se queda en 0 %), luego «Creando carpeta…». Al terminar, **Abrir carpeta** muestra la entrega en el Finder.

### 11. Una instalación sin sidecar (opcional, 2 minutos)
- Haz: borra `master check.app/Contents/MacOS/ffmpeg` en una copia de la app y ábrela.
- Esperado: MP3 y los vídeos salen deshabilitados con «falta ffmpeg en esta instalación»; Acerca de dice «No disponible en esta instalación». El resto funciona.

---

## Pendiente que no es de esta lista
- Firma y notarización de la app (el CI compila sin firma): sin hacer.
- Windows (instalador NSIS) lo prueba solo el CI hasta que haya un PC; HEVC en Windows requiere «HEVC Video Extensions» del sistema.
- Huecos de prueba automática conocidos (humo vía Tauri, plazo/kill con `.cmd` en Windows): `docs/pruebas/huecos.md`.
