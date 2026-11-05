# nortia. studio · Master Out — Especificación de diseño

Fecha: 05/11/2026 · Estado: borrador para aprobación de Fer · Autores: Fer y Claude (CEOs de nortia.)

## 1. Qué es y para quién

Master Out es una app de Mac que toma el máster de una canción y devuelve, en segundos y en la carpeta que elijas, todos los archivos listos para distribuir, cada uno nombrado con la plataforma a la que va, más un informe de loudness por destino y una ficha de lanzamiento. El máster nunca se altera: la app convierte formato, resolución y contenedor; mide; y avisa.

Para: productores, ingenieros de mastering, A&R y artistas que suben ellos mismos a la distribuidora. Promesa en una frase: "Suelta tu máster. Recibe la entrega perfecta."

Precio: 9,99 € pago único en la Mac App Store. Sin cuenta, sin red, sin suscripción.

## 2. Principios de producto

1. **El máster es sagrado.** Nunca se aplica ganancia, limitación ni normalización. Solo conversión de formato, remuestreo y dither cuando se reduce la profundidad de bits.
2. **Un archivo correcto por destino, no todo para todos.** Spotify pide un único máster y no crear 16 bit desde 24; TuneCore pide no incrustar metadatos. La app genera lo que cada destino quiere y explica por qué.
3. **Todo se ve antes de exportar.** La carpeta de salida se previsualiza en vivo; cada destino enseña qué archivo producirá y su semáforo de loudness.
4. **Nombres que hablan.** Cada archivo lleva la plataforma entre corchetes y su resolución: `Artista - Título [Spotify] 24bit-96kHz.flac`.
5. **Rápido de verdad.** Un máster de 4 minutos a 24/96 debe completar los 10 destinos en menos de 30 s en un Mac Apple Silicon; conversión en paralelo por destino.
6. **Interactiva y atractiva.** La app responde a cada gesto: arrastre con animación, análisis con forma de onda que se dibuja al cargar, tarjetas que cambian de estado al tocar un interruptor, árbol de salida que se reordena en vivo, barra de progreso por archivo y confeti sobrio al terminar. Nada de diálogos modales salvo errores.

## 3. Pantalla única (tres columnas, ventana mínima 1100×700, modo claro y oscuro)

### 3.1 Columna izquierda: el máster
- Zona de arrastre grande con el símbolo de nortia. studio. Acepta WAV, AIFF, FLAC, CAF, ALAC. Rechaza con mensaje claro MP3/AAC como fuente ("Un máster con pérdida no sirve para distribuir; exporta WAV o FLAC desde tu DAW").
- Al soltar: forma de onda que se dibuja de izquierda a derecha (overview por picos), y debajo la ficha técnica: formato, bits, kHz, canales, duración, tamaño, LUFS integrados, LRA, true peak máximo (dBTP), y un semáforo general.
- Botón "Cambiar máster" y "Escuchar" (reproducción simple con posición en la onda).

### 3.2 Columna central: datos y destinos
- **Datos del tema** (6 campos visibles, el resto plegado): Título, Artista principal, Featuring, Versión (p. ej. "Radio Edit"), Año, ISRC (opcional, validado con formato CC-XXX-YY-NNNNN). Plegados: compositores, editorial, sello, ℗ y ©, UPC, género, idioma, explícito. Se recuerdan entre sesiones (último artista/sello).
- **Portada** (opcional): arrastre de imagen; se muestra con su tamaño; aviso si < 3000×3000 o no cuadrada o no sRGB.
- **Destinos** como tarjetas con interruptor, agrupadas: *Streaming* (Spotify, Apple Music / Apple Digital Masters, Amazon Music, Tidal, Deezer, YouTube Music), *Distribuidoras* (Estándar 16/44,1; Hi-Res 24/nativa; TuneCore 24/192), *Promo* (MP3 320), *Vídeo e imagen* (YouTube vídeo con imagen fija 1080p, Spotify Canvas 9:16, Portada 3000×3000 JPG+PNG). Cada tarjeta muestra: icono, nombre, el archivo que generará ("FLAC 24 bit · 96 kHz"), semáforo de loudness para ese destino (verde cumple / ámbar aviso / rojo no cumple, con el motivo en una línea: "TP −0,3 dBTP; Spotify recomienda ≤ −1") y un aviso si el máster no es apto (p. ej. ADM exige 24 bit nativo). Las tarjetas se animan al activarse y la vista previa de la carpeta se actualiza al instante.
- Botón "Preset": guardar/recuperar la selección de destinos ("Mi entrega estándar").

### 3.3 Columna derecha: salida
- Selector de carpeta destino (se recuerda la última).
- **Árbol de la carpeta de salida en vivo**: `Artista - Título/` → subcarpetas numeradas por destino → nombres finales de archivo con tamaño estimado. Clic en un archivo muestra qué contendrá y por qué.
- Botón **Exportar** grande. Durante la exportación: progreso por archivo con tiempo restante; al terminar, resumen (n archivos, tamaño total, tiempo), botón "Mostrar en Finder" y animación de cierre sobria.
- Archivos siempre generados: `README.txt` (qué archivo va a qué plataforma y por qué; en español o inglés según el sistema), `informe-loudness.txt` (medidas del máster y veredicto por destino), `ficha-lanzamiento.csv` y `ficha-lanzamiento.txt` (los datos del tema en el orden que piden las distribuidoras).

## 4. Nombres de archivo y carpeta (contrato)

```
{Artista} - {Título}{ (Versión)}/
  01 Spotify/{Artista} - {Título} [Spotify] 24bit-96kHz.flac
  02 Apple Music/{Artista} - {Título} [Apple Digital Masters] 24bit-96kHz.wav
  03 Amazon Music/{Artista} - {Título} [Amazon Music] 24bit-96kHz.flac
  04 Tidal/{Artista} - {Título} [Tidal] 24bit-96kHz.flac
  05 Deezer/{Artista} - {Título} [Deezer] 16bit-44kHz.wav
  06 YouTube/{Artista} - {Título} [YouTube] 1080p.mp4
  07 Distribuidora/{Artista} - {Título} [Distribuidora] 16bit-44kHz.wav
  07 Distribuidora/{Artista} - {Título} [Distribuidora Hi-Res] 24bit-96kHz.wav
  08 Promo/{Artista} - {Título} [MP3 320].mp3
  09 Imagen/{Artista} - {Título} [Portada] 3000x3000.jpg (+ .png)
  09 Imagen/{Artista} - {Título} [Canvas] 1080x1920.mp4
  README.txt · informe-loudness.txt · ficha-lanzamiento.csv · ficha-lanzamiento.txt
```
Reglas: caracteres inválidos en nombres sustituidos por `-`; la resolución del nombre refleja la real del archivo; las subcarpetas solo se crean si el destino está activo; los presets exactos (bits/kHz por destino) salen de la tabla verificada en `docs/investigacion/` y se actualizan sin tocar la interfaz.

Regla de resolución por destino (decide qué se genera a partir del máster de entrada):
- Si el destino pide "nativa" (Spotify, Apple, Amazon, Tidal, Hi-Res): se conservan los bits y kHz del máster; nunca se sube la resolución para estos destinos (Spotify y Apple lo desaconsejan). Si el máster es 16 bit, la tarjeta de Apple Digital Masters se desactiva con el motivo.
- Si el destino fija una resolución (Deezer 16/44,1, Distribuidora estándar 16/44,1, TuneCore 24/192, YouTube 48 kHz): se convierte a esa resolución; al bajar bits se aplica dither TPDF; al subir (p. ej. 48 → 192 para TuneCore) se permite y el README lo marca como upsampling sin ganancia de calidad.
- Las resoluciones exactas de cada destino son datos de configuración (`presets.json` dentro de la app) con su fuente; un destino "por verificar" se muestra con la etiqueta "recomendación" hasta que se confirme.

## 5. Motor

- **Lectura y conversión:** AVFoundation / Core Audio (`AVAudioFile`, `AVAudioConverter`) para WAV, AIFF, CAF, FLAC y ALAC; remuestreo con la máxima calidad del convertidor; salida WAV como PCM 0x0001 (no EXTENSIBLE) para cumplir Spotify; FLAC vía Core Audio (macOS 10.13+).
- **Dither:** TPDF solo al bajar de 24 a 16 bit; nunca al mantener o subir resolución. Subir resolución (p. ej. 44,1 → 96) se permite pero el README lo marca como "upsampling, sin ganancia de calidad" y las tarjetas de Spotify/Apple avisan de que prefieren la tasa nativa.
- **Medición:** LUFS integrados, LRA y true peak según ITU-R BS.1770-4 / EBU R128 (gating −70 LUFS absoluto y −10 LU relativo; true peak con sobremuestreo 4×), implementado en Swift y verificado en tests contra referencias de la EBU y contra `ffmpeg -af ebur128`.
- **MP3:** LAME enlazado dinámicamente (LGPL) a 320 kbps CBR, joint stereo, ID3v2.3 con título, artista, álbum, año, ISRC, compositor, editorial, copyright y portada.
- **Vídeo:** AVAssetWriter; H.264 High, 1080p (y 4K opcional), GOP cerrado, audio AAC-LC 48 kHz 384 kbps, moov al inicio; imagen fija = portada centrada sobre fondo desenfocado. Canvas: recorte 9:16 de la portada con zoom lento de 6 s, 1080×1920, sin audio. Apple motion artwork (destino opcional en "Vídeo e imagen"): 1×1 de 3840×3840 y 3×4 de 2048×2732, 8–35 s, H.264, sin audio, primer fotograma igual a la portada, bucle suave, 45–100 Mbps, 23,976–30 fps (spec oficial de Apple, confirmada en ronda 2).
- **Apple Digital Masters:** flujo oficial afconvert → CAF con Sound Check → AAC 256 → `afclip`; el resultado (sin clipping / n muestras con clipping) se muestra en la tarjeta de Apple y en el informe. Si `afclip` no está disponible en el sistema, la tarjeta lo indica y ofrece el enlace a las herramientas de Apple.
- **Metadatos incrustados:** BWF `bext` en WAV (Description con "Artista - Título", Originator "nortia. studio", fecha y hora; el chunk `bext` NO tiene campo ISRC según EBU Tech 3285, así que el ISRC va en `axml`/iXML si se confirma el esquema, y siempre en la ficha de lanzamiento); Vorbis comments en FLAC (TITLE, ARTIST, DATE, ISRC, COPYRIGHT); ID3v2.3 en MP3 (TIT2, TPE1, TALB, TYER, TCON, TSRC, TCOM, TPUB, TCOP, APIC tipo $03). Se omiten en el preset TuneCore (lo pide así).
- **Paralelismo:** una tarea por destino con `TaskGroup`; límite de concurrencia = núcleos de rendimiento; progreso por archivo.
- **Sin red.** Nada sale del Mac. Sandbox de App Store con acceso a archivos elegidos por el usuario (bookmarks de seguridad para la carpeta de salida).

## 6. Fuera de la v1 (y cuándo)

Ajuste automático de loudness (nunca por defecto; quizá como herramienta aparte), Dolby Atmos / ADM BWF, lotes de varios temas (v1.1 si hay ventas), integración con distribuidoras y DDEX, letras, generación de ISRC, versión iPhone/iPad (v2 si la de Mac funciona), Windows.

## 7. Calidad y verificación

- TDD: pruebas unitarias de medición (señales sintéticas de −23 LUFS y −1 dBTP; referencias EBU), de conversión (bits/kHz/formato de salida, PCM 0x0001), de dither (solo cuando toca), de nombres de archivo (caracteres raros, acentos, longitud) y de la estructura de carpeta; pruebas de integración con 3 másters reales de Fer (44,1/16, 48/24, 96/24).
- Rendimiento: medición del tiempo total con el máster de 4 min a 96/24; objetivo < 30 s para 10 destinos en M1 o superior.
- Revisión de código por un agente que no lo escribió; verificación antes de cada hito (superpowers verification-before-completion).
- Revisión de la interfaz con `mobile-app-design:ios-review`/`swiftui-macos` adaptado a Mac y con una crítica a ciegas (`design:design-critique`) antes de enviar a la App Store.
- Capturas y texto de la ficha de la App Store pasados por `product-legal:marketing-claims-review`.

## 8. Marca, nombre y tienda

- Marca: "nortia. studio" (minúsculas, punto), símbolo N2 de nortia. en el color propio de studio (a elegir entre dos propuestas) y wordmark "nortia. studio". Icono de app en el mismo sistema.
- Nombre de la app: provisional "Master Out"; se eligen 3 opciones (comprobadas en App Store y dominios) antes de enviar.
- App Store: publicada desde la cuenta de desarrollador de Fer. El nombre del vendedor que muestra Apple es el titular de la cuenta; para que aparezca "nortia." haría falta cuenta de empresa (D-U-N-S). Decisión pendiente, no bloquea el desarrollo.
- Textos de la ficha en español e inglés; 5 capturas; vídeo de 15 s.

## 9. Riesgos y cómo se mitigan

- Un preset mal verificado hace que la app entregue algo incorrecto → cada preset cita su fuente oficial en `docs/investigacion/` y lo que sea "por verificar" se marca en la tarjeta como "recomendación" hasta confirmarlo.
- FLAC en Core Audio con limitaciones (metadatos) → si falta algo, se enlaza libFLAC (BSD) en v1.1.
- Rechazo de la App Store por el enlace dinámico de LAME → alternativa: AAC en lugar de MP3 en el preset promo, o MP3 vía framework propio; se prueba en TestFlight antes.
- Que el cliente espere que la app "arregle" el loudness → la promesa y la ficha dejan claro que mide y avisa, y que el máster no se toca.

## 10. Plan de alto nivel (el detalle, en writing-plans)

Hito 1 (semana 1): motor de medición y conversión con tests, sin interfaz. Hito 2 (semana 2): interfaz de tres columnas con vista previa en vivo y exportación. Hito 3 (semana 3): vídeo, Canvas, portada, ADM/afclip, informe y ficha; pruebas con másters reales; revisión de diseño. Hito 4 (semana 4): marca, icono, ficha de la App Store, TestFlight, envío.

---

## Adenda 05/11/2026 14:30 — cambios de alcance decididos por los CEOs (sustituyen a lo que contradigan arriba)

1. **Canal de venta: solo propio.** No se publica en la Mac App Store por ahora. Venta desde la web de nortia. con pasarela que actúa de vendedor (merchant of record) y entrega archivo + clave de licencia. Mac: build firmada con Developer ID y notarizada (obligatorio para que Gatekeeper la abra). Sección 8 ("tienda") y los puntos de sandbox/App Review de la sección 5 quedan anulados.
2. **Plataformas: Mac primero, después Windows.** iPhone/iPad/Android fuera.
3. **Stack: motor en Rust + interfaz Tauri 2** (HTML/CSS/JS en webview nativo), un solo código para Mac y Windows. Sustituye a SwiftUI/AVFoundation/Core Audio de la sección 5. Las piezas solo-Mac (afconvert/afclip para Apple Digital Masters) se activan solo en macOS; en Windows la tarjeta de Apple entrega el WAV 24 bit nativo y marca la validación AAC como "no disponible en Windows".
4. **Fábrica: GitHub Actions** (runners macOS y Windows); tests del motor también en Linux. Fer no tiene Xcode.
5. **Idioma: la app habla el idioma del sistema** (mínimo español e inglés en v1; textos externalizados). README, informe de loudness y ficha de lanzamiento se generan en ese idioma.
6. **Mínimo macOS 14**; Windows 10 (1803+) / 11 con WebView2.
7. **Lectura de WAV robusta:** los másters reales traen chunks `JUNK` antes de `fmt ` (Pro Tools/Logic) y picos > 0 dBTP (medidos: hasta +1,2 dBTP). El lector recorre chunks; la medición y los avisos deben funcionar con esos archivos.
8. **Licencias:** nada GPL en el bundle. FLAC con `flacenc` (Apache-2.0) o `libflac-sys` (BSD-3); MP3 con el programa `lame` como proceso aparte (LGPL, con licencia y enlace al código) o AAC si no es viable; vídeo H.264 con codificador del sistema (VideoToolbox en Mac, Media Foundation en Windows) o `openh264` (BSD) — se decide en el hito 3 con un spike.

## Adenda 05/11/2026 15:49 — fichas de imagen y vídeo aceptan material propio (Fer)

Regla: **cada ficha de imagen o vídeo (Portada, Canvas, YouTube vídeo, Apple motion artwork) es una zona de soltar.** El usuario puede arrastrar su propio archivo (imagen o vídeo) sobre la ficha; la app lo conforma a la norma de la plataforma (recorte al formato, escalado, duración —recortando si sobra—, sin audio donde se exige, códec y contenedor correctos) y muestra una miniatura en la ficha de lo que va a usar. Si no se suelta nada, la app fabrica el entregable a partir de la portada (Canvas: recorte 9:16 con zoom lento de 6 s; YouTube: portada centrada sobre fondo desenfocado; motion artwork: zoom lento 10 s en 1:1 y 3:4). La Portada se suelta en el cuadrado junto a Artista/Título y se entrega a 3000×3000 sRGB en JPG y PNG, con aviso si la fuente es pequeña o no cuadrada. Diseño: el departamento de diseño añade el estado "ficha con material propio" (miniatura + aspa para quitarlo) y el halo de arrastre sobre las fichas de imagen/vídeo. Motor: hito 3 (vídeo) incluye `conform_video(input, spec)` y `conform_image(input, spec)` con tests por plataforma.

## Adenda 05/11/2026 15:50 — nombre de la app: **Master Check** (Fer)

La app se llama **Master Check** (sustituye al provisional "Master Out" en toda la spec, el diseño y los textos; el crate interno `masterout-core` se renombrará a `mastercheck-core` al cerrar el hito 1). Pendiente: comprobar dominio y conflictos de marca de "Master Check" antes de publicar.

## Adenda 05/11/2026 15:51 — nombre definitivo: **Master Ready** (Fer)

Sustituye a "Master Check" (15:50) y a "Master Out". Crate interno pasará a `masterready-core` al cerrar el hito 1. Pendiente: dominio y conflictos de marca de "Master Ready".

## Adenda 05/11/2026 15:52 — nombre definitivo: **master check** (en minúsculas, Fer)

Sustituye a las adendas de 15:50 y 15:51. Se escribe siempre en minúsculas, como la marca ("nortia. studio · master check"). Crate interno pasará a `mastercheck-core` al cerrar el hito 1. Pendiente: dominio y conflictos de marca.

## Adenda 05/11/2026 17:50 — límite de 96 kHz (Fer)

master check **no admite másters por encima de 96 kHz**: la app los rechaza al analizar con el mensaje "master check admite másters de hasta 96 kHz" (clave `err.rate_over_96k`). Desaparece cualquier ruta de 192 kHz: el preset TuneCore pasa a 24 bit a la frecuencia nativa del máster (sin metadatos incrustados). Sin notas en el README al respecto. La regla de resolución de §4 se completa: los destinos "nativa" nunca superan 96 kHz porque la entrada no puede superarlos.
