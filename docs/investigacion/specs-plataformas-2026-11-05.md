# Specs de entrega por plataforma (ronda 1, 05/11/2026)

Solo "confirmado" lo leído en página oficial. Lo demás "por verificar".

## Spotify (confirmado)
- FLAC preferido, WAV aceptado; 16 o 24 bit; estéreo; 44,1 kHz o superior; mantener resolución nativa. WAV PCM 0x0001 (no WAVE_FORMAT_EXTENSIBLE), sin datos tras el chunk data. Un único máster por tema; no crear 16 bit desde 24.
- Loudness: normaliza a −14 LUFS; recomienda TP < −1 dBTP (< −2 si el máster es más fuerte que −14 LUFS). Web player y terceros no normalizan.
- Fuentes: providersupport.spotify.com/article/audio-and-artwork-specifications · support.spotify.com/al/artists/article/audio-file-formats/ · support.spotify.com/us/artists/article/loudness-normalization/
- Portada: TIFF/PNG/JPEG, sRGB 24 bit, mín. 640 px, máx. 10.000 px, 1:1, sin upscaling.
- Canvas: MP4 o JPG, 9:16, 3–8 s, 720–1080 px de alto; sin voz, sin nombre de artista/canción. support.spotify.com/us/artists/article/canvas-guidelines/

## Apple Digital Masters (confirmado; PDF apple.com/apple-music/apple-digital-masters/docs/apple-digital-masters.pdf)
- Solo fuentes 24 bit nativas, sin bit-padding; tasa nativa más alta, sin upsampling. Mínimo 1 dB de headroom bajo 0 dBFS (clipping inter-sample); comprobar también el AAC codificado.
- Herramientas: afconvert, afclip, Droplet ADM, AURoundTripAAC. Flujo: afconvert → CAF con soundcheck → AAC 256 kbps → afclip.
- Por verificar: "−16 LUFS Sound Check", "≥ 44,1 kHz" literal.

## Distribuidoras (confirmado)
- DistroKid: WAV/MP3/M4A/FLAC/AIFF/WMA; 1 GB por archivo; sin requisitos de loudness. support.distrokid.com/hc/en-us/articles/360013647753-
- TuneCore: WAV; recomendado 24/192; aceptado 16/44,1; NO embeber metadatos. support.tunecore.com/hc/en-us/articles/115006499327
- CD Baby: WAV/FLAC preferidos, 16/44,1; MP3 aceptado no recomendado. support.cdbaby.com/hc/en-us/articles/204498645
- Por verificar: Ditto, Amuse, Altafonte, Believe/Orchard, Beatport (robots.txt), AWAL, Revelator.

## YouTube subida (confirmado; support.google.com/youtube/answer/1722171)
- MP4 moov al inicio; H.264 High, progresivo, 4:2:0, 2 B-frames, GOP cerrado = fps/2, VBR. Audio AAC-LC 48 kHz, estéreo 384 kbps. Vídeo 1080p 8 Mbps; 4K 35–45 Mbps.

## Por verificar (ronda 2)
Amazon, Tidal, Deezer, YouTube Music loudness, SoundCloud, Bandcamp, Beatport; Apple motion artwork (help.apple.com/itc/albummotionguide) y reglas de portada Apple; Instagram/TikTok; plantillas de label copy y DDEX; dither, ID3 y bitrate MP3; ADM BWF/Atmos.

## Tabla de presets recomendados (v1)
| Preset | Formato | Bits/kHz | Loudness | TP | Dither | Metadatos | Destino |
|---|---|---|---|---|---|---|---|
| Streaming Hi-Res | FLAC + WAV PCM | 24 / nativa | medir; objetivo −14 | ≤ −1 (≤ −2 si > −14) | no | opcional | Spotify, distribuidoras 24 bit |
| Apple Digital Masters | WAV/AIFF 24 | 24 / nativa | medir | ≥ 1 dB headroom; afclip en AAC | no | opcional | Apple Music |
| Validación AAC | M4A 256 | tasa final | — | afclip limpio | — | — | interno |
| Distribuidora estándar | WAV | 16 / 44,1 | medir | ≤ −1 | TPDF (por verificar) | sin embeber | CD Baby, DistroKid |
| TuneCore alta | WAV | 24 / 192 | medir | ≤ −1 | no | sin embeber | TuneCore |
| YouTube imagen fija | MP4 H.264 + AAC 384k | 48 kHz | medir | ≤ −1 | — | — | YouTube |
| Canvas | MP4/JPG 9:16 | 720–1080 alto, 3–8 s | — | — | — | — | Spotify |
| Portada | JPG/PNG sRGB | 3000×3000 | — | — | — | — | todos |
| MP3 promo | MP3 320 CBR | 44,1 | medir | ≤ −1 | TPDF | ID3 completo | promo |

Nota de método: los fetch a URLs adivinadas fallan; partir de URLs devueltas por búsqueda.
