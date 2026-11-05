# Baseline audio Mac/Swift (solo conocimiento propio, sin verificar)

## 1. Conversiones
- Lectura: `AVAudioFile(forReading:)` lee WAV/AIFF/FLAC (FLAC nativo desde macOS 10.13). Procesar en `Float32` no entrelazado (`processingFormat`).
- SRC: `AVAudioConverter` con `sampleRateConverterAlgorithm = AVSampleRateConverterAlgorithm_Mastering`, `sampleRateConverterQuality = .max`. Bucle `convert(to:error:withInputFrom:)` por bloques; vaciar cola al final (`.endOfStream`).
- 16 bit con dither: Float32 -> TPDF propio (suma de dos uniformes +-0,5 LSB), redondear y saturar a Int16. Dither solo en el paso final, una vez. NO SEGURO de que Core Audio exponga dither al bajar de bits; no depender de ello.
- WAV 24/96: PCM Int24 empaquetado little-endian. FLAC 24 bit: `AVAudioFile(forWriting:settings:)` con `AVFormatIDKey: kAudioFormatFLAC`, `AVLinearPCMBitDepthKey: 24`. NO SEGURO de que el codificador de Apple soporte 24 bit en FLAC con ese flujo (en AudioToolbox el soporte de bit depth de FLAC es poco claro); plan B: libFLAC.
- MP3 320 CBR: macOS no trae codificador MP3 (solo decodifica). Requiere LAME: `-b 320 --cbr` (`lame_set_VBR(vbr_off)`, `lame_set_brate(320)`).

## 2. WAV PCM clásico (0x0001)
Escribir la cabecera a mano (`FileHandle`/`Data`): `RIFF`, `WAVE`, `fmt ` de 16 bytes (tag 0x0001, canales, rate, byteRate, blockAlign, bits), `data`. Byte de relleno si el tamaño del chunk es impar. NO SEGURO de cuándo `AVAudioFile`/`ExtAudioFile` emite EXTENSIBLE (creo que para >16 bit o >2 canales), por eso no lo uso. Si pasa de 4 GB haría falta RF64 (no aplica a masters musicales normales).

## 3. Loudness BS.1770-4 / EBU R128
1. K-weighting por canal, biquads en cascada (Direct Form I/II, `vDSP_biquad` en Double). A 48 kHz: etapa 1 (high shelf) b=[1.53512485958697, -2.69169618940638, 1.19839281085285], a=[1, -1.69065929318241, 0.73248077421585]; etapa 2 (RLB, paso alto) b=[1,-2,1], a=[1, -1.99004745483398, 0.99007225036621]. Para 44,1/96 kHz recalcular por transformada bilineal (shelf: f0~1681.97 Hz, G~+4 dB, Q~0.7071; HP: f0~38.135 Hz, Q~0.5003). Valores de memoria: verificar.
2. Pesos de canal: L, R, C = 1,0; Ls, Rs = 1,41; LFE excluido.
3. Bloques de 400 ms, paso de 100 ms (solape 75 %). Energía z_j = media cuadrática por canal, ponderada y sumada. Loudness del bloque: l_j = -0,691 + 10 log10(sum G_i z_ij).
4. Gating absoluto -70 LUFS: descarta bloques por debajo. Umbral relativo = loudness de la media energética de los supervivientes - 10 LU. Integrado = -0,691 + 10 log10(media de z de bloques sobre ambos umbrales).
5. LRA (EBU Tech 3342): ventanas short-term de 3 s; puerta absoluta -70 y relativa -20 LU; LRA = percentil 95 - percentil 10 (en LU) de la distribución. NO SEGURO del paso de ventana exacto (1 s vs 100 ms); usar 100 ms y contrastar con test.
6. True peak: sobremuestreo 4x con FIR polifásico (Annex 2 de BS.1770-4: 48 taps, 12 por fase), máx |x|, dBTP = 20 log10. NO SEGURO de si a 96 kHz basta 2x; usar 4x siempre (más conservador). Medir en la frecuencia original.
- Tests: tonos y ficheros del "EBU loudness test set" (Tech 3341/3342); tolerancia +-0,1 LU en integrado, true peak +0,2/-0,4 dB. Sinusoide 997 Hz a 0 dBFS en un solo canal = -3,01 LUFS; estéreo -23 dBFS en ambos canales = aprox. -20 LUFS (comprobar aritmética en el test). Contraste cruzado con `ffmpeg -i f.wav -af ebur128=peak=true -f null -` y libebur128 en CI.

## 4. Metadatos
- WAV: `LIST` tipo `INFO` con `INAM` (título), `IART` (artista), `ICRD` (fecha/año), `ICMT`. ISRC: NO SEGURO de que exista subchunk INFO estándar; más fiable un chunk `id3 ` con ID3v2 (frame `TSRC`), o `bext`/`iXML` si es broadcast. `bext` (BWF) tiene Description, Originator, OriginationDate, pero no ISRC.
- FLAC: bloque `VORBIS_COMMENT` (tipo 4): `TITLE`, `ARTIST`, `DATE`, `ISRC` (UTF-8). Escribible a mano o con libFLAC/TagLib.
- MP3: ID3v2.3 (máxima compatibilidad): `TIT2`, `TPE1`, `TYER` (año), `TSRC` (ISRC). En v2.4 el año es `TDRC` y los tamaños son synchsafe; en v2.3 no. Cabecera "ID3", versión 03 00, tamaño synchsafe. Codificación 0x01 (UTF-16 con BOM) en v2.3. Añadir tras codificar.

## 5. Apple Digital Masters
Requisitos que recuerdo: master 24 bit, 44,1-192 kHz, sin clipping tras codificar a AAC, idealmente headroom (true peak <= -1 dBTP). Herramientas del paquete Mastered for iTunes / Apple Digital Masters: `afinfo master.wav` (formato), `afclip master.wav` (detecta recortes tras simular AAC), `afconvert` para generar el AAC de prueba. Ejemplo de memoria, NO SEGURO en flags: `afconvert master.wav -d aac -f m4af -u pgcm 2 -b 256000 -q 127 -s 2 out.m4a`. NO SEGURO de la sintaxis exacta de `afclip` y de que siga en el instalador actual; comprobar con `afclip` sin argumentos y `man afconvert`.

## 6. MP4 H.264 1080p + AAC
`AVAssetWriter(url:, fileType: .mp4)`. Vídeo: `AVVideoCodecKey: AVVideoCodecType.h264`, 1920x1080, `AVVideoCompressionPropertiesKey` con `AVVideoProfileLevelKey: AVVideoProfileLevelH264HighAutoLevel`, `AVVideoAverageBitRateKey` ~2 Mbps, `AVVideoMaxKeyFrameIntervalDurationKey: 2`. `AVAssetWriterInputPixelBufferAdaptor` (32BGRA); dibujar la imagen con `CGContext` sobre `CVPixelBuffer`; añadir frames a 1-2 fps (o primero y último a t=duración). Audio: `AVAssetWriterInput` con `AVFormatIDKey: kAudioFormatMPEG4AAC`, 48000 Hz, 2 canales, 256000 bps, alimentado por `AVAssetReader` con salida PCM. `shouldOptimizeForNetworkUse = true`. `startSession(atSourceTime: .zero)`, `endSession(atSourceTime:)`, `finishWriting`. NO SEGURO de que YouTube acepte bien 1 fps; con 2-5 fps no he visto problemas.

## 7. Qué cabe en Apple puro
- Sí: lectura, SRC, WAV/AIFF, MP4/H.264/AAC, DSP de loudness (Accelerate), dither (código propio), cabeceras WAV y tags WAV/FLAC (a mano).
- Dudoso: FLAC 24 bit de escritura (ver 1).
- No: MP3 (LAME), ID3 robusto (TagLib o a mano).
- Licencias: LAME LGPL-2 (enlazar dinámico/dylib o proceso aparte; el estático LGPL complica App Store); patentes MP3 caducadas (2017, creo). libFLAC: BSD-3 (utilizable sin problema). TagLib: LGPL-2.1/MPL-1.1. libebur128: MIT. ffmpeg: LGPL/GPL según build, evitar GPL.

## 8. Objetivos de plataformas (memoria; confirmar en sus webs)
| Plataforma | LUFS integrado | True peak |
|---|---|---|
| Spotify | -14 (Normal; -11 Loud, -19 Quiet) | -1 dBTP (-2 si más fuerte que -14) |
| Apple Music (Sound Check) | -16 | -1 dBTP |
| YouTube | -14 (solo atenúa) | -1 dBTP recomendado, sin requisito oficial |
| Tidal | -14 | -1 dBTP |
| Amazon Music | -14 (NO SEGURO) | -2 dBTP (NO SEGURO) |
