# Componentes de terceros · master check (nortia. studio)

Documento de cumplimiento de licencias. Se actualiza con cada cambio de dependencias. Estado: **borrador técnico, pendiente de revisión legal** (ver el final).

## Componentes

| Componente | Uso | Licencia | Cómo se distribuye |
|---|---|---|---|
| FFmpeg 7.x (libavcodec, libavformat, libavfilter, libswscale, libswresample; ejecutables `ffmpeg` y `ffprobe`) | Vídeo (Canvas, YouTube, Apple Motion), AAC y MP3 | LGPL 2.1 o posterior. Compilado con `--disable-gpl --disable-nonfree`, sin x264/x265/fdk-aac/openh264 | Ejecutable aparte (sidecar), invocado como proceso; nunca enlazado con la app. Construido en CI desde fuente |
| LAME 3.100 (libmp3lame) | Codificación MP3 dentro del sidecar | LGPL 2 o posterior | Enlazado estáticamente dentro del ejecutable ffmpeg |
| VideoToolbox (macOS) / Media Foundation (Windows) | Codificación H.264 del sistema operativo, a través de `h264_videotoolbox` / `h264_mf` | Componentes del SO (licencia del sistema) | No se distribuyen; los aporta el sistema operativo |
| zlib | PNG dentro de ffmpeg | zlib | Librería del sistema (macOS) / enlazada estáticamente (Windows) |
| flac-codec | Lectura/escritura FLAC | MIT / Apache-2.0 | Compilado en el motor |
| ebur128 | Sonoridad EBU R128 | MIT | Compilado en el motor |
| rubato | Remuestreo | MIT | Compilado en el motor |
| image | Portadas | MIT / Apache-2.0 | Compilado en el motor |
| Tauri | Interfaz de escritorio | Apache-2.0 / MIT | Compilado en la app |
| Hanken Grotesk | Tipografía de la interfaz | SIL Open Font License 1.1 | Incluida con la app |

Las licencias completas de las dependencias de Rust se regeneran en cada release con `cargo about`/`cargo deny` y se añaden a este documento.

## Obligaciones y cómo se cumplen (FFmpeg/LAME, LGPL)

1. **Sin GPL ni nonfree**: el CI comprueba que `ffmpeg -buildconf` contiene `--disable-gpl` y no contiene `--enable-gpl`, `--enable-nonfree` ni `--enable-version3`; también que no hay codificadores x264/x265/fdk-aac. El resultado se guarda como `buildconf-<os>.txt`.
2. **Código fuente exacto**: cada release del sidecar (`ffmpeg-sidecar-v<versión>-<hash>`) adjunta el tarball de FFmpeg y el de LAME tal como se compilaron (verificados por SHA256), más `configure_line-<os>.txt`. No hay parches propios; si los hubiera, se adjuntaría el diff.
3. **Mismo servidor**: el fuente se aloja junto a los binarios (misma página de release), y la web de descargas enlaza a ese release.
4. **Licencias**: se distribuyen `LICENSE.LGPLv2.1` (FFmpeg) y `COPYING.LGPL` (LAME) junto al binario y dentro de la app.
5. **Sustituibilidad**: el sidecar es un ejecutable independiente que el motor invoca como proceso. Quien quiera reemplazarlo por su propia compilación puede hacerlo (ruta del sidecar junto a la app); el EULA no prohíbe la ingeniería inversa para depurar esa modificación.
6. **Marca y atribución**: se escribe "FFmpeg"; no se afirma propiedad de FFmpeg ni de LAME; no se usa el logo de FFmpeg.
7. **Aviso visible** en "Acerca de", en la web de descargas y en el EULA (texto abajo).

## Texto para "Acerca de"

> **Componentes de terceros**
> master check usa **FFmpeg** (https://ffmpeg.org), licenciado bajo la GNU Lesser General Public License, versión 2.1 o posterior, y la biblioteca **LAME** (https://lame.sourceforge.io), licenciada bajo la LGPL 2. FFmpeg se distribuye como un programa aparte, compilado sin componentes GPL, y no se modifica. Puedes obtener el código fuente exacto, la línea de configuración y las licencias en: *{URL del release ffmpeg-sidecar-v…}*. La codificación H.264 la realizan los componentes del sistema operativo (VideoToolbox en macOS, Media Foundation en Windows). FFmpeg es una marca de Fabrice Bellard, originalmente creador del proyecto. Otras licencias: Tauri (Apache-2.0/MIT), flac-codec, ebur128, rubato, image (MIT/Apache-2.0) y la tipografía Hanken Grotesk (SIL OFL 1.1).

## Pendiente de consulta legal (antes de vender)

- **Patentes H.264 y AAC**: FFmpeg avisa de que no puede garantizar la ausencia de algoritmos patentados y de que el uso comercial conlleva más riesgo. Se mitiga usando los codificadores H.264 del sistema operativo, pero el AAC es el codificador nativo de FFmpeg; hay que confirmar con un asesor si se requiere licencia.
- **LGPL con enlace estático**: el ejecutable `ffmpeg` incluye LAME enlazado estáticamente y se publica el fuente exacto para poder reconstruirlo; hay que confirmar que esta forma satisface la LGPL 2.1 para ambas bibliotecas (alternativa: build `--enable-shared` y entregar DLL/dylib).
- Texto final del EULA y de la página de descargas.
