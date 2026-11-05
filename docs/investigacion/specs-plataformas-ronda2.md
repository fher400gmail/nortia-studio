# Specs de entrega por plataforma (ronda 2, 05/11/2026)

Presupuesto de 22 llamadas agotado. "Confirmado" = leído en la página oficial indicada. Lo no cubierto queda "por verificar" y no se ha rellenado con blogs.

## 1. Loudness y formato
- Amazon Music: la página oficial solo explica cómo activar "Loudness Normalization" en las apps Android/iOS/FireOS; no da objetivo en LUFS. https://digprjsurvey.amazon.com/csad/help/node/TgG1d8mPjGwf885hRP. Objetivo numérico: POR VERIFICAR (sin fuente oficial). Spec de entrega de Amazon: POR VERIFICAR (se entrega vía distribuidora).
- Tidal, Deezer, YouTube Music, SoundCloud: búsqueda sin resultado oficial (solo prensa/blogs/foros). POR VERIFICAR, sin fuente oficial.
- Bandcamp (confirmado): WAV, AIFF o FLAC; mínimo 16 bit / 44,1 kHz lossless; estéreo (no mono ni multicanal); no se indica límite superior ni tamaño; no subir upsampleados desde MP3. https://get.bandcamp.help/en/articles/15263088-how-do-i-upload-music. Otras páginas existentes pero no leídas: https://get.bandcamp.help/en/articles/15263308-can-i-upload-higher-resolution-audio
- Beatport: no se localizó página accesible. POR VERIFICAR.

## 2. Apple: motion artwork (confirmado)
Fuente: https://help.apple.com/itc/albummotionguide/en.lproj/static.html y https://help.apple.com/itc/musicstyleguide/en.lproj/itc7d4b161d8.html
- Entregables: Album Page Motion 3x4 = 2048x2732 px; 1x1 = 3840x3840 px.
- Duración 8–35 s; .mp4 o .mov; H.264, ProRes 422 o ProRes 4444; sin pista de audio; píxel cuadrado 1:1; 45–100 Mbps; 23,976/24/25/29,97/30 fps; Rec 709 o sRGB.
- Contenido: basado en la portada; primer fotograma = portada compuesta (no negro ni color plano); sin cortes ni múltiples ediciones; bucle sin salto; sin drop/hold frames; sin parpadeo frenético; sin texto promocional ni fechas de gira; sin logo Parental Advisory; sin marco ni imitar la UI de Apple Music; zona segura en 3x4; la versión 3x4 debe ser a sangre, no un 1x1 reducido; IA sin degradar texto/rostros.
- Guía de estilo de portada estática (tamaño, formato, espacio de color): la página leída NO lo especifica. POR VERIFICAR (probar otras páginas de help.apple.com/itc/musicstyleguide). Mantener 3000x3000 sRGB como valor provisional.

## 3. Instagram Reels y TikTok
- Instagram: https://help.instagram.com/1038071743007909 existe y trata resolución, relación, duración, códec, bitrate de audio y tamaño, pero el fetch devolvió solo el índice de temas, sin valores. POR VERIFICAR. TikTok: sin fuente oficial localizada. POR VERIFICAR.
- Normalización de loudness en ambas: sin fuente oficial.
- Provisional para el preset (no oficial): 1080x1920, 9:16, H.264 + AAC 48 kHz, TP ≤ −1 dBTP.

## 4. MP3 e ID3
- LAME 320 CBR joint stereo: no se consultó documentación de LAME. POR VERIFICAR.
- ID3v2.3.0 (confirmado, https://id3.org/id3v2.3.0): TIT2 título; TPE1 artista principal (separador "/"); TALB álbum; TYER año de 4 caracteres; TCON género (texto o "(n)" ID3v1, más RX/CR); TSRC ISRC de 12 caracteres; TCOM compositores ("/"); TPUB sello/editor; TCOP debe empezar por "AAAA " (año y espacio); APIC: codificación, MIME ("image/png" o "image/jpeg" recomendados), tipo de imagen ($03 = portada frontal), descripción ≤ 64 caracteres, datos. Codificación $00 ISO-8859-1 o $01 Unicode con BOM.
- ID3v2.4 (confirmado parcial, https://id3.org/id3v2.4.0-frames): TDRC = timestamp de grabación (formato exacto en el documento de estructura, no leído); en 2.4 TCON es texto con valores separados por nulo; TSRC igual (12 caracteres). Que TDRC sustituye a TYER: el texto leído no lo dice, POR VERIFICAR. Recomendación práctica: escribir ID3v2.3 con TYER para máxima compatibilidad.

## 5. Dither 24 → 16 bit
Sin fuente técnica seria consultada. POR VERIFICAR. Nota: la ronda 1 recoge que Spotify indica no crear 16 bit a partir de 24 para su entrega; el dither solo aplica a destinos que exijan 16 bit (CD Baby, Ditto 16/44,1 aceptado).

## 6. Medición
- EBU R128 (v4, ago. 2020; confirmado, https://tech.ebu.ch/docs/r/r128v4_0.pdf): objetivo programa −23,0 LUFS; tolerancia ±1,0 LU (directo) o ±0,2 LU (flujos de nivelación); TP máximo −1 dBTP (tolerancia de medición ±0,3 dB); LRA según Tech 3342; gating según ITU-R BS.1770; no cita revisión concreta de BS.1770. Existe una v5 en https://tech-qual.ebu.ch/files/live/sites/tech/files/shared/r/r128v5_0.pdf (no leída).
- EBU Tech 3341 (confirmado, https://tech.ebu.ch/docs/tech/tech3341.pdf): puerta absoluta −70 LUFS; puerta relativa −10 LU; momentáneo 400 ms sin puerta; corto plazo 3 s sin puerta; bloques de 400 ms con 75 % de solapamiento; medidores a ≥ 10 Hz; tolerancias de test ±0,1 LU/LUFS y, para true peak, +0,2/−0,4 dBTP; indica usar "las versiones más recientes" de las Recomendaciones ITU-R.
- Sobremuestreo 4× para true peak: Tech 3341 lo remite al Anexo 2 de BS.1770 y no lo fija. ITU-R BS.1770-4/5: NO consultado, enlace oficial POR VERIFICAR (itu.int). Tech 3341 incluye señales de test (usar para el test automático del medidor).
- Valor de ejemplo para la suite de tests: tono 1 kHz sinusoidal −23 dBFS estéreo = −23,0 LUFS (verificar contra la tabla de Tech 3341).

## 7. BWF / bext
- EBU Tech 3285 (confirmado, https://knowledgehub.ebu.ch/files/live/sites/tech/files/shared/tech/tech3285.pdf): campos de bext = Description (ASCII libre, hasta 256 caracteres), Originator, OriginatorReference, OriginationDate, OriginationTime, TimeReference, Version, UMID, cinco valores de loudness, Reserved, CodingHistory. NO hay campo ISRC en bext.
- Título: no hay campo propio; solo cabe en Description (libre). Suplemento de ISRC: no localizado. Supplements 1–6 listados (MPEG, Capturing Report, Peak Envelope, Link, axml, Dolby). ISRC en WAV vía axml (Suppl. 5, https://tech.ebu.ch/publications/tech3285s5) o iXML: no leído, POR VERIFICAR. Recomendación: para ISRC/título usar CSV/label copy; en WAV no embeber (TuneCore lo prohíbe).

## 8. Distribuidoras
- Ditto (confirmado, https://support.dittomusic.com/en/articles/4283815-what-format-does-my-audio-need-to-be-in): WAV o MP3 estéreo, ≥ 16 bit y ≥ 44,1 kHz; subir temas completos sin silencios largos. Sin dato de tamaño ni loudness.
- Amuse, Altafonte, The Orchard, Believe: no consultadas. POR VERIFICAR. Plantillas CSV/Excel de label copy y DDEX: no consultadas.

## TABLA DE PRESETS v2
Etiqueta de plataforma entre corchetes en el nombre de archivo. {art}=artista, {tit}=título.

| Preset | Formato | Bits/kHz | Loudness objetivo | TP máx. | Dither | Metadatos | Nombre sugerido | Destino | Fuente |
|---|---|---|---|---|---|---|---|---|---|
| Streaming Hi-Res | FLAC + WAV PCM 0x0001 | 24 / nativa | medir; −14 LUFS (referencia Spotify) | −1 (−2 si > −14) | no | opcional | {art} - {tit} [Spotify].flac | Spotify, distribuidoras 24 bit | Ronda 1 |
| Apple Digital Masters | WAV/AIFF | 24 / nativa, sin padding | medir | ≥ 1 dB headroom; afclip en AAC | no | opcional | {art} - {tit} [ADM].wav | Apple Music | Ronda 1 (PDF Apple) |
| Validación AAC | M4A 256 kbps | tasa final | — | afclip limpio | — | — | {art} - {tit} [AAC-check].m4a | interno | Ronda 1 |
| Distribuidora estándar | WAV | 16 / 44,1 | medir | −1 | TPDF (recomendación no verificada) | sin embeber | {art} - {tit} [CDBaby].wav | CD Baby, DistroKid, Ditto | R1 + Ditto |
| TuneCore alta | WAV | 24 / 192 (aceptado 16/44,1) | medir | −1 | no | NO embeber | {art} - {tit} [TuneCore].wav | TuneCore | Ronda 1 |
| Bandcamp | WAV/AIFF/FLAC estéreo | ≥ 16 / ≥ 44,1; 24 / nativa | medir; sin norma | −1 (prudente) | no (si 16 desde 24: TPDF) | opcional (tags) | {art} - {tit} [Bandcamp].flac | Bandcamp | Bandcamp help |
| YouTube imagen fija | MP4 H.264 High + AAC-LC 384k | 48 kHz | medir | −1 | — | — | {art} - {tit} [YouTube].mp4 | YouTube | Ronda 1 |
| Canvas | MP4/JPG 9:16, sin audio/voz | 720–1080 alto, 3–8 s | — | — | — | — | {tit} [Canvas].mp4 | Spotify | Ronda 1 |
| Apple Motion 3x4 | MP4/MOV H.264 o ProRes, sin audio | 2048x2732, 8–35 s, 45–100 Mbps, 24–30 fps | — | — | — | — | {tit} [AppleMotion3x4].mov | Apple Music | Apple albummotionguide |
| Apple Motion 1x1 | idem | 3840x3840 | — | — | — | — | {tit} [AppleMotion1x1].mov | Apple Music | Apple albummotionguide |
| Reels/TikTok (provisional) | MP4 H.264 + AAC | 1080x1920 9:16; 48 kHz | medir | −1 | — | — | {tit} [Reels].mp4 | Instagram, TikTok | SIN fuente oficial |
| Portada | JPG/PNG sRGB | 3000x3000 (Spotify 640–10000 px) | — | — | — | — | {tit} [Cover].jpg | todos | Spotify; Apple por verificar |
| MP3 promo | MP3 320 CBR joint stereo | 44,1 | medir | −1 | TPDF | ID3v2.3: TIT2 TPE1 TALB TYER TCON TSRC TCOM TPUB TCOP APIC($03) | {art} - {tit} [Promo].mp3 | promo | id3.org; LAME por verificar |
| Medición/QC | informe | — | EBU R128: −23 LUFS (broadcast, solo referencia) | −1 | — | — | {tit} [QC].json | interno | EBU R128 v4, Tech 3341 |

## Sigue sin fuente oficial
1. Loudness de Amazon, Tidal, Deezer, YouTube Music, SoundCloud, Instagram y TikTok: sin cifra oficial.
2. Beatport, Amuse, Altafonte, Believe, The Orchard y plantillas CSV/DDEX de label copy: no consultados.
3. Dither TPDF/noise shaping y recomendación LAME 320 CBR joint stereo: sin fuente técnica leída.
4. ITU-R BS.1770-4/5 (itu.int) y el sobremuestreo 4× del true peak: sin texto oficial leído.
5. ISRC/título en WAV: bext no tiene campo ISRC; axml/iXML y suplemento de ISRC sin leer.
6. Specs de Instagram Reels y TikTok, y tamaño/formato de portada estática de Apple: página sin valores o no localizada.
