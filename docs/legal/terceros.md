# Componentes de terceros · master check (nortia. studio)

Documento de cumplimiento de licencias. Se actualiza con cada cambio de dependencias. Estado: **borrador técnico, pendiente de revisión legal** (ver el final).

## Componentes

| Componente | Uso | Licencia | Cómo se distribuye |
|---|---|---|---|
| FFmpeg 7.x (libavcodec, libavformat, libavfilter, libswscale, libswresample; ejecutables `ffmpeg` y `ffprobe`) | Vídeo (Canvas, YouTube, Apple Motion), AAC y MP3 | LGPL 2.1 o posterior. Compilado con `--disable-gpl --disable-nonfree`, sin x264/x265/fdk-aac/openh264 | Ejecutable aparte (sidecar), invocado como proceso; nunca enlazado con la app. Construido en CI desde fuente |
| LAME 3.100 (libmp3lame) | Codificación MP3 dentro del sidecar | LGPL 2 o posterior | Enlazado estáticamente dentro del ejecutable ffmpeg |
| VideoToolbox (macOS) / Media Foundation (Windows) | Codificación H.264 y HEVC del sistema operativo, a través de `h264_videotoolbox`/`hevc_videotoolbox` y `h264_mf`/`hevc_mf` | Componentes del SO (licencia del sistema) | No se distribuyen; los aporta el sistema operativo |
| zlib | PNG dentro de ffmpeg | zlib | Librería del sistema (macOS) / enlazada estáticamente (Windows) |
| flac-codec | Lectura/escritura FLAC | MIT / Apache-2.0 | Compilado en el motor |
| ebur128 | Sonoridad EBU R128 | MIT | Compilado en el motor |
| rubato | Remuestreo | MIT | Compilado en el motor |
| image | Portadas | MIT / Apache-2.0 | Compilado en el motor |
| Tauri | Interfaz de escritorio | Apache-2.0 / MIT | Compilado en la app |
| Hanken Grotesk | Tipografía de la interfaz | SIL Open Font License 1.1 | Incluida con la app |

Las licencias completas de las dependencias de Rust se regeneran en cada release con `cargo about`/`cargo deny` y se añaden a este documento.

## Obligaciones y cómo se cumplen (FFmpeg/LAME, LGPL)

1. **Sin GPL ni nonfree** (hecho en CI): el CI comprueba que `ffmpeg -buildconf` contiene `--disable-gpl` y no contiene `--enable-gpl`, `--enable-nonfree` ni `--enable-version3`; también que no hay codificadores x264/x265/fdk-aac, y compara el inventario real de filtros, codificadores, decodificadores, muxers, demuxers, bsfs y protocolos con la lista blanca (falla si falta o sobra algo; p. ej. `boxblur` es solo GPL y no se incluye). El resultado se guarda como `buildconf-<os>.txt`.
2. **Código fuente exacto** (hecho en CI): cada release del sidecar (`ffmpeg-sidecar-v<versión>-<hash>`) adjunta el tarball de FFmpeg (con su firma `.asc`, verificada contra la huella de la clave de releases de FFmpeg) y el de LAME tal como se compilaron (SHA256 fijado), más `configure_line-<os>.txt`. Los scripts de compilación y de aceptación de `tools/ffmpeg/` forman parte de la oferta de fuente (se publica el commit del repositorio en la nota del release o se adjuntan).
   - **Parches propios**: no se parchea FFmpeg. En LAME 3.100 el script de compilación borra la línea `lame_init_old` de `include/libmp3lame.sym` (`sed '/lame_init_old/d'`) para que enlace en macOS y mingw; es un parche de compilación y queda documentado aquí y en `tools/ffmpeg/build-*.sh`.
3. **Mismo servidor** (pendiente): hoy el fuente está en la página del release del repositorio de desarrollo, que es privado. Antes de vender, los assets del release (binarios, fuentes, licencias, `configure_line`) se replicarán en una ubicación **pública** (repositorio `nortia-studio-sources` o la página de descargas), junto a los binarios que se distribuyan. No se ha creado nada público todavía.
4. **Licencias** (pendiente de incluir en la app): se publican `LICENSE.LGPLv2.1` (FFmpeg), `COPYING.LGPL` y `LICENSE.lame` (LAME) junto al binario en el release; se copiarán también dentro de la app y de la página de descargas.
5. **Sustituibilidad** (diseño): el sidecar es un ejecutable independiente que el motor invoca como proceso. Quien quiera reemplazarlo por su propia compilación podrá hacerlo (ruta del sidecar junto a la app); el EULA no prohibirá la ingeniería inversa para depurar esa modificación. Pendiente de confirmar con el asesor (enlace estático de LAME).
6. **Marca y atribución**: se escribe "FFmpeg"; no se afirma propiedad de FFmpeg ni de LAME; no se usa el logo de FFmpeg.
7. **Aviso visible** (pendiente de implementar): "Acerca de", web de descargas y EULA incluirán el texto de abajo cuando existan.

## Texto para "Acerca de"

> **Componentes de terceros**
> master check usa **FFmpeg** (https://ffmpeg.org), licenciado bajo la GNU Lesser General Public License, versión 2.1 o posterior, y la biblioteca **LAME** (https://lame.sourceforge.io), licenciada bajo la LGPL 2. FFmpeg se distribuye como un programa aparte, compilado sin componentes GPL, y no se modifica. Puedes obtener el código fuente exacto, la línea de configuración y las licencias en: *{URL del release ffmpeg-sidecar-v…}*. La codificación H.264 la realizan los componentes del sistema operativo (VideoToolbox en macOS, Media Foundation en Windows). FFmpeg es una marca de Fabrice Bellard, originalmente creador del proyecto. Otras licencias: Tauri (Apache-2.0/MIT), flac-codec, ebur128, rubato, image (MIT/Apache-2.0) y la tipografía Hanken Grotesk (SIL OFL 1.1).

## Pendiente de consulta legal (antes de vender)

- **Patentes H.264 y AAC**: FFmpeg avisa de que no puede garantizar la ausencia de algoritmos patentados y de que el uso comercial conlleva más riesgo. Se mitiga usando los codificadores H.264 del sistema operativo, pero el AAC es el codificador nativo de FFmpeg; hay que confirmar con un asesor si se requiere licencia.
- **LGPL con enlace estático**: el ejecutable `ffmpeg` incluye LAME enlazado estáticamente y se publica el fuente exacto para poder reconstruirlo; hay que confirmar que esta forma satisface la LGPL 2.1 para ambas bibliotecas (alternativa: build `--enable-shared` y entregar DLL/dylib).
- **Ubicación pública del fuente**: replicar los assets del release a un repositorio público `nortia-studio-sources` (o a la página de descargas) antes de la primera venta, y enlazar esa URL en "Acerca de", la web y el EULA.
- Texto final del EULA y de la página de descargas.
