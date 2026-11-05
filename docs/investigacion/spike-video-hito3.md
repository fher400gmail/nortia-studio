# Spike hito 3 (master check): cómo producir cover, Canvas, YouTube, Apple motion y MP3

Fecha: 2026-11-05. Marcas: CONFIRMADO = leído en la URL citada en esta sesión. SIN FUENTE = conocimiento general no verificado aquí (o página bloqueada); verificar antes de decidir.
Páginas que no se pudieron abrir (permiso no concedido): github.com/BtbN/FFmpeg-Builds/.../50-openh264.sh, crates.io/api/v1/crates/image y /openh264, community.chocolatey.org (lame). Quedan como SIN FUENTE.

## 0. Especificaciones de destino (lo que hay que producir)

| Salida | Especificación | Estado |
|---|---|---|
| Apple motion | 3840x3840 (1x1) y 2048x2732 (3x4); H.264 (o ProRes 422/4444); .mp4/.mov; 8–35 s; 23.976/24/25/29.97/30 fps; sin audio; Rec.709 o sRGB; píxel 1:1; bitrate mínimo 45 Mbps, máximo 100 Mbps | CONFIRMADO https://help.apple.com/itc/albummotionguide/en.lproj/static.html |
| YouTube | MP4, moov al principio (faststart), sin edit lists; H.264 High; AAC-LC; 48 kHz; estéreo 384 kbps; 1080p SDR 8 Mbps (24/25/30 fps); 4:2:0; 2 B-frames; GOP cerrado = mitad del fps; CABAC; BT.709 | CONFIRMADO https://support.google.com/youtube/answer/1722171?hl=en |
| Spotify Canvas | Vertical 9:16; altura entre 720 y 1080 px; 3–8 s; MP4 o JPG. Audio no especificado (recomiendan material sin voz). OJO: la página NO exige 1080x1920 exacto; 1080x1920 cumple. | CONFIRMADO https://support.spotify.com/fj/artists/article/canvas-guidelines/ |
| Cover | 3000x3000, 72 dpi, JPG, RGB (no CMYK); perfil de color y regla de no-upscaling no especificados (distribuidor Massive Music). Playlists de curadores de Apple: mínimo 3000x3000, PNG sin transparencia, RGB. | CONFIRMADO https://distribution-support.studio.massivemusic.com/kb/a69/album-cover-artwork-specifications-and-guidelines.aspx ; https://help.apple.com/itc/musiccuratorbestpractices/en.lproj/static.html |
| Cover: páginas oficiales de Spotify y Amazon (mín/máx px, sRGB, upscaling) | No localizadas | SIN FUENTE |

Consecuencia de diseño: el preset Apple exige 45–100 Mbps. Con un encoder hardware (VideoToolbox / Media Foundation) hay que fijar bitrate explícito; hay que validar con ffprobe tras exportar.

## 1. ffmpeg como sidecar, build LGPL

Hechos:
- BtbN/FFmpeg-Builds publica variantes `gpl`, `lgpl`, `nonfree` y sus `-shared`. La variante lgpl "carece de libs solo-GPL, sobre todo libx264 y libx265". Targets: win64, win32, linux64, linuxarm64; auto-builds solo win(arm)64 y linux(arm)64. CONFIRMADO https://github.com:443/BtbN/FFmpeg-Builds
- No hay assets de macOS en BtbN (solo Windows y Linux). CONFIRMADO https://github.com/BtbN/FFmpeg-Builds/releases/tag/latest
- Tamaños: win64 lgpl estático 169,8 MiB (n8.1: 162,7; n9.0: 163,6); win64 lgpl-shared 78,4 MiB (zip). CONFIRMADO misma URL. Son builds "todo incluido", demasiado grandes para empaquetar tal cual.
- Si BtbN lgpl incluye libopenh264, h264_mf o h264_videotoolbox: la página no lo dice (README remite a `scripts.d`). SIN FUENTE.
- evermeet.cx / Homebrew: licencia y flags no verificados. Un build de ejemplo para Apple Silicon (zff.dev) usa `--enable-gpl --enable-version3` (GPLv3) e incluye h264_videotoolbox; tarda "varias horas". CONFIRMADO https://zff.dev/taagedal/ffmpeg-apple-silicon . Que Homebrew/evermeet sean GPL por defecto: SIN FUENTE, pero asumir que sí (traen x264). No usar.
- Build propio: LGPL = compilar sin `--enable-gpl` ni `--enable-nonfree` (punto 1 del checklist). CONFIRMADO https://www.ffmpeg.org/legal.html
- Flags propuestos (SIN FUENTE, a validar en CI): `--disable-gpl --disable-nonfree --disable-autodetect --disable-everything` + habilitar solo lo necesario: demuxers image2/mov/wav/flac/mp3/aiff; decoders png/mjpeg/h264/aac/pcm_*/flac/mp3; encoders aac, libmp3lame, mjpeg, png, h264_videotoolbox (mac) / h264_mf (win); filtros scale/zoompan/gblur/overlay/format/boxblur/crop/fps; muxers mp4/mp3/image2; `--enable-videotoolbox` (mac), `--enable-mediafoundation` (win), `--enable-libmp3lame`. Tamaño estimado de un build mínimo: pocos MB (SIN FUENTE; medir).
- Encoder `aac` nativo de FFmpeg es LGPL y suficiente para AAC-LC 384 kbps/48 kHz. SIN FUENTE (la página legal no menciona aac, VideoToolbox, Media Foundation ni OpenH264).
- Patentes: FFmpeg dice no saber si usa algoritmos patentados; uso comercial implica más riesgo y MPEG LA cobra. CONFIRMADO https://www.ffmpeg.org/legal.html

Obligaciones LGPL (checklist oficial, https://www.ffmpeg.org/legal.html, CONFIRMADO): no usar `--enable-gpl/nonfree`; "enlazar dinámicamente" (en Windows, DLLs); distribuir el código fuente exacto de los binarios (aunque no se modifique), con diff y archivo con la línea de configure; alojar el fuente en el mismo servidor que el binario; aviso LGPLv2.1 y enlace al fuente en descargas, About y EULA; no prohibir ingeniería inversa en el EULA; no afirmar propiedad de FFmpeg; escribir "FFmpeg"; revisar otras libs LGPL enlazadas (cita LAME).
Riesgo de interpretación: el checklist recomienda enlace dinámico; un ejecutable estático invocado como proceso separado es práctica común, pero no es lo que dice el checklist. Mitigación: ofrecer fuente + script de build reproducible, y/o enviar build `shared` (DLL/dylib) junto al exe. Decisión legal pendiente (SIN FUENTE jurídica).

## 2. Ruta Rust pura / plataforma

- `openh264` 0.9.9 (wrapper BSD-2, núcleo Cisco BSD-2). Feature `source` (por defecto) compila el código OpenH264 incluido; feature `libloading` exige la librería binaria de Cisco. Encoder disponible (desde 0.2). Aarch64 macOS "compila, no probado". CONFIRMADO https://docs.rs/openh264/ y https://docs.rs/crate/openh264/latest/features
- Licencia binaria Cisco: el binario debe descargarse por separado al dispositivo del usuario; no puede integrarse ni combinarse con software de terceros antes de llegar al usuario; el usuario debe poder activarlo/desactivarlo; hay que mostrar "OpenH264 Video Codec provided by Cisco Systems, Inc." y reproducir el texto en el EULA. La cobertura de patentes de Cisco (MPEG LA, sin coste) solo aplica si se cumplen esas condiciones. CONFIRMADO https://github.com/Kurento/openh264/blob/master/BINARY_LICENSE.txt (copia de la licencia)
- Implicación crítica: la app "sin red" no puede cumplir la licencia binaria de Cisco (ni empaquetarlo ni descargarlo en runtime). Compilar openh264 desde fuente (`source`) es BSD-2 pero NO tiene la cobertura de patentes de Cisco: la responsabilidad de patentes H.264 sería nuestra (SIN FUENTE sobre importes de licencia MPEG LA/Via LA). Además OpenH264 solo ofrece perfil Constrained Baseline/Main en encoder, no High (SIN FUENTE; verificar, YouTube pide High, Apple no pide perfil).
- Muxers: crate `mp4` 0.14.0, MIT, última actualización 2023-08-01 (estancado). CONFIRMADO https://crates.io/api/v1/crates/mp4 . `re_mp4` (Rerun): solo parseo; la página no menciona escritura. CONFIRMADO https://github.com/rerun-io/re_mp4 . `mp4-atom`: no verificado, SIN FUENTE. `muxide`, `mp4forge` aparecen en búsqueda, no evaluados.
- AAC sin ffmpeg: fdk-aac es non-free (SIN FUENTE aquí; conocido); AudioToolbox vía `coreaudio-rs` en mac y Media Foundation vía crate `windows` son viables pero no verificados (SIN FUENTE).
- VideoToolbox H.264 vía `objc2`/`core-video`: no verificado; VideoToolbox sí está en ffmpeg (h264_videotoolbox, ver zff.dev arriba, CONFIRMADO).
- Esfuerzo estimado (SIN FUENTE, juicio propio): escribir dos backends nativos (VT+AudioToolbox, MF) + muxer MP4 propio con faststart, B-frames/edit lists correctos, más pruebas: 4–8 semanas y alto riesgo de incompatibilidades de plataforma (YouTube exige sin edit lists, moov delante, GOP cerrado). Frente a ffmpeg: no compensa en v1.

## 3. Imágenes

- Crate `image`: resize Lanczos3 (`FilterType::Lanczos3`), JPEG con calidad configurable, PNG; versión/licencia (MIT/Apache) no verificadas: página crates.io bloqueada. SIN FUENTE.
- Orientación EXIF: `image` no aplica la orientación sola; leer con `kamadak-exif` o usar `ImageDecoder::orientation()` en versiones recientes. SIN FUENTE (verificar versión).
- Perfil ICC: para entrada con perfil no sRGB (Display P3, Adobe RGB) hay que convertir a sRGB, no solo borrar el perfil. Opciones: `lcms2` (envoltorio de Little CMS, licencia MIT; necesita compilar C) o `moxcms` (Rust puro). Ambas SIN FUENTE. Después guardar sin perfil o con sRGB incrustado.
- No-upscaling: ninguna fuente oficial hallada. Recomendación de producto: avisar si el lado corto <3000 y marcar "reescalado" en el informe (regla propia, no de plataforma). Canvas JPG y cover: ver sección 0.
- Para Canvas/Apple motion desde imagen: zoompan de ffmpeg; para evitar tembleque del zoom, trabajar sobre frames sobremuestreados (técnica conocida, SIN FUENTE) o generar los frames con `image` y pasarlos a ffmpeg por stdin (más control, más CPU).

## 4. LAME

- Versión estable 3.100 disponible como paquete en múltiples distros y en Chocolatey (3.100.20190524). CONFIRMADO (existencia) https://chocolatey.org/packages/lame/3.100.20190524 (redirige a community.chocolatey.org, no abierto) ; https://packages.guix.gnu.org/packages/lame/3.100
- Fuente oficial lame.sourceforge.io y licencia LGPL (v2 o posterior): SIN FUENTE (no abierto aquí). Binarios oficiales precompilados no los distribuye el proyecto (solo fuente); los de Chocolatey son de terceros. SIN FUENTE.
- Obligaciones LGPL para exe incluido: licencia, aviso, oferta de fuente de la versión exacta, no impedir sustitución. Mismo patrón que ffmpeg.
- Opción más simple: `--enable-libmp3lame` dentro del propio ffmpeg (la guía de ffmpeg cita LAME como lib LGPL compatible, CONFIRMADO https://www.ffmpeg.org/legal.html). Un sidecar menos. Comando: `-c:a libmp3lame -b:a 320k` (CBR 320). Alternativa: `lame` CLI como segundo externalBin compilado en CI con `./configure --enable-static --disable-shared --disable-frontend`? (el frontend es necesario para CLI; flags SIN FUENTE).
- Tauri `bundle.externalBin`: cada binario necesita copia con sufijo `-$TARGET_TRIPLE` (p. ej. `aarch64-apple-darwin`); ruta relativa a `src-tauri`; permisos `shell:allow-execute` con `"sidecar": true`; Rust: `app.shell().sidecar("nombre")`. CONFIRMADO https://v2.tauri.app/es/develop/sidecar/ . La página NO cubre universal macOS ni firma de sidecars (SIN FUENTE: habitual es `x86_64-apple-darwin` + `aarch64-apple-darwin`, o `universal-apple-darwin` con `lipo`; Windows `x86_64-pc-windows-msvc`).

## 5. Recomendación v1

Arquitectura: sidecar `ffmpeg` propio, LGPL, build mínimo por plataforma, sin x264 ni openh264; H.264 por encoder del sistema operativo (h264_videotoolbox en macOS, h264_mf en Windows); AAC con el encoder nativo `aac`; MP3 con `libmp3lame` dentro del mismo ffmpeg; covers con crate `image` (+ conversión de color a sRGB). Un solo sidecar (+ ffprobe opcional).

Razón: openh264 es inviable sin red por su licencia binaria (sección 2); x264 es GPL; el muxer Rust está estancado; los encoders del SO evitan licencia de patente propia en la mayoría de casos (SIN FUENTE: confirmar con asesor), dan velocidad y permiten bitrates 45–100 Mbps.

Pasos de CI (GitHub Actions) (todo SIN FUENTE, a probar):
1. Job macOS arm64 (`macos-14`) y x86_64 (`macos-13`/cross): descargar tarball ffmpeg y lame 3.100 desde fuente oficial, verificar hash; compilar lame estático; configurar ffmpeg con flags de sección 1; `make`; `strip`; `lipo` opcional.
2. Job Windows: MSYS2/MinGW (o cross desde Linux con mingw-w64) para x86_64; mismo configure con `--enable-mediafoundation`. Si se opta por cumplimiento "dinámico", build `--enable-shared` y copiar DLLs.
3. Guardar artefactos con `ffmpeg-<triple>[.exe]`; adjuntar `FFMPEG_SOURCE.tar.xz` + `configure_line.txt` + `LICENSE.LGPL` + `THIRD_PARTY.md`; publicar fuente en el mismo servidor de descargas.
4. Firma: firmar cada sidecar con Developer ID y hardened runtime antes de empaquetar y notarizar el .app/.dmg (cada Mach-O anidado debe estar firmado); Windows: firmar .exe/.dll y el instalador. Requisito general, SIN FUENTE concreta para Tauri.
5. Test de aceptación automático: ffprobe sobre cada salida contra las tablas de sección 0 (resolución, codec, perfil, fps, duración, audio ausente en Canvas/Apple, bitrate Apple 45–100 Mbps, faststart en YouTube).

Riesgos:
- Patentes H.264/AAC: no resueltos jurídicamente (ffmpeg.org avisa del riesgo comercial). Consultar asesor antes de vender.
- h264_mf: depende de Media Foundation (ediciones Windows N sin Media Feature Pack; calidad/control de perfil High y GOP variables según GPU/driver). SIN FUENTE; probar en Win10 y Win11 reales. Plan B: pedir al usuario instalar Media Feature Pack.
- h264_videotoolbox: perfil High disponible, pero garantía de GOP cerrado/B-frames no verificada; validar contra requisitos YouTube.
- Interpretación LGPL del exe estático (sección 1).
- Tamaño: build mínimo pequeño (estimado pocos MB), frente a 163–170 MiB de BtbN completo.
- Notarización: fallo típico si un sidecar no está firmado con hardened runtime.
- Mantenimiento: seguir CVEs de ffmpeg; fijar versión y hash.

Descartado para v1: BtbN lgpl tal cual (sin macOS, enorme), openh264 (licencia/red), mp4 crate (estancado), Homebrew/evermeet (GPL probable).
