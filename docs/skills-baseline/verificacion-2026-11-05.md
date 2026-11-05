# Verificación de líneas base (audio + distribución propia) — 2026-11-05

Alcance ajustado por los CEOs: sin Mac App Store; canal propio (web) para Mac y luego Windows; motor Rust + Tauri. Se eliminan los puntos 10, 11 y 14. El 12 queda solo en notarización fuera de tienda.

Estados: CONFIRMADO / REFUTADO / SIN FUENTE. Notas:
- Los fetch pasan por un resumidor; lo citado entre comillas viene de ese resumen.
- Una fuente de terceros se marca "(3ª)".
- `curl` está bloqueado por el proxy y varias páginas (gnu.org LGPL-2, Amazon, ITU -4 en inglés) no se pudieron leer.
- No se pudo abrir el zip de EBU, así que los nombres de archivo del set quedan sin verificar.

## Bloque A. Loudness (puntos 1-3)

| Pto | Afirmación | Estado | Corrección / dato | URL |
|---|---|---|---|---|
| 1 | Coeficientes K a 48 kHz (shelf b=[1.53512485958697, -2.69169618940638, 1.19839281085285], a1=-1.69065929318241, a2=0.73248077421585; RLB b=[1,-2,1], a1=-1.99004745483398, a2=0.99007225036621) | CONFIRMADO | Tablas 1 y 2 de BS.1770-5 y valores idénticos en ffmpeg `f_ebur128.c` (commit antiguo). No se leyó la tabla numérica de la -4 en inglés. La -4 (PDF francés) confirma "coeficientes previstos para 48 kHz". | itu.int/dms_pubrec/itu-r/rec/bs/R-REC-BS.1770-5-202311-I!!PDF-E.pdf ; .../R-REC-BS.1770-4-201510-S!!PDF-F.pdf ; git.kx.studio/falkTX/FFmpeg (libavfilter/f_ebur128.c) |
| 1 | Otras tasas: recalcular | CONFIRMADO | La ITU exige otros coeficientes que reproduzcan la respuesta de 48 kHz. libebur128 usa: K=tan(π·f0/fs). Etapa 1: f0=1681.974450955533, G=3.999843853973347, Q=0.7071752369554196, Vh=10^(G/20), Vb=Vh^0.4996667741545416, a0=1+K/Q+K². b=[(Vh+Vb·K/Q+K²)/a0, 2(K²−Vh)/a0, (Vh−Vb·K/Q+K²)/a0], a1=2(K²−1)/a0, a2=(1−K/Q+K²)/a0. Etapa 2 (HP): f0=38.13547087602444, Q=0.5003270373238773, b=[1,-2,1], mismo a. Los valores de la línea base (1681.97, +4 dB, 0.7071, 38.135, 0.5003) son esos redondeados. | github.com/jiixyj/libebur128/blob/master/ebur128/ebur128.c |
| 2 | Bloque 400 ms, solape 75 %, gate abs −70, relativo −10 | CONFIRMADO | BS.1770 dice "−70 LKFS" y "−10 dB" respecto al nivel tras el primer umbral. En libebur128, abs = 10^((−70+0.691)/10) sobre energía. | itu.int (-5 PDF-E, -4 PDF-F) ; libebur128 ebur128.c |
| 2 | LRA: ventanas 3 s; paso 1 s? | REFUTADO (el "1 s") | Tech 3342 exige solapamiento mínimo de 2,9 s entre ventanas de 3 s, es decir, actualización ≥10 Hz (paso ≤100 ms). La línea base "usar 100 ms" es correcta. | tech.ebu.ch/files/live/sites/tech/files/shared/tech/tech3342.pdf |
| 2 | LRA: gate −70 abs y −20 LU relativo; P95−P10 | CONFIRMADO | Tech 3342; ffmpeg: `LRA_GATE_THRES −20`, `LRA_LOWER_PRC 10`, `LRA_HIGHER_PRC 95`. | tech3342.pdf ; f_ebur128.c |
| 2 | True peak: 4× a 48 kHz | CONFIRMADO | Annex 2: 4× lleva de 48 a 192 kHz. Antes del sobremuestreo aplica 12,04 dB de atenuación de headroom (la línea base no lo menciona). | BS.1770-5 PDF-E |
| 2 | ¿96 kHz? | CONFIRMADO (corrige "NO SEGURO") | Señales de mayor tasa necesitan "proporcionalmente menos" sobremuestreo. El ejemplo es 96 kHz con solo 2×. Los medidores deben trabajar a ≥192 kHz efectivos. libebur128: 4× por debajo de 96000, 2× de 96000 a <192000, ninguno a ≥192000. Usar 4× siempre es válido y más conservador. | BS.1770-5 PDF-E / -4 PDF-F ; libebur128 |
| 2 | FIR 48 taps, 12 por fase | CONFIRMADO como ejemplo | Annex 2 lo da como "un conjunto" (orden 48, 4 fases), no obligatorio. libebur128 usa `interp_create(49, 4…)`. | BS.1770-5 ; libebur128 |
| 3 | Tolerancia ±0,1 LU (M, S, I) y TP +0,2/−0,4 dB | CONFIRMADO | Tech 3341, 23 casos. Ejemplos: caso 1 estéreo 1 kHz −23,0 dBFS → −23,0 LUFS; caso 2 −33 dBFS → −33 LUFS; casos 15-19 TP −6,0/+3,0 dBTP; casos 20-23 TP 0,0 dBTP. Las señales están a 48 kHz. | tech.ebu.ch/files/live/sites/tech/files/shared/tech/tech3341.pdf |
| 3 | "Estéreo −23 dBFS en ambos canales ≈ −20 LUFS" | REFUTADO | Tech 3341 caso 1: estéreo en fase, sinusoide 1 kHz, −23,0 dBFS de pico por canal = −23,0 LUFS. La cifra −20 LUFS corresponde a −20 dBFS (3ª: tests de exaring/ebur128). | tech3341.pdf ; pkg.go.dev/github.com/exaring/ebur128 (3ª) |
| 3 | "997 Hz 0 dBFS un canal = −3,01 LUFS" | SIN FUENTE | Coherente con la fórmula (mono 0 dBFS pico → −3,01), pero ninguna fuente leída lo afirma. | — |
| 3 | LRA Tech 3342: valores esperados (tolerancia ±1 LU) | CONFIRMADO | 1) sinus 1 kHz −20/−30 dBFS → 10 LU. 2) −20/−15 → 5. 3) −40/−20 → 20. 4) −50/−35/−20/−35/−50 → 15. 5) programa estrecho → 5. 6) programa amplio → 15. | tech3342.pdf |
| 3 | Descarga | CONFIRMADO | `ebu-loudness-test-setv05.zip` (87,4 MB, "70 audio files", cambios en readme.txt dentro del zip). Página: tech.ebu.ch/publications/ebu_loudness_test_set. Aparte: `EBU-reference_listening_signal_pinknoise_500Hz_2kHz_R128.wav` (−23 LUFS, mono). | tech.ebu.ch/files/live/sites/tech/files/shared/testmaterial/ebu-loudness-test-setv05.zip |
| 3 | Nombres de archivo de los 70 ficheros | SIN FUENTE | Solo se vio `seq-3341-1-16bit.wav` en un README de terceros (3ª). Hay que abrir el zip/readme.txt. | components.espressif.com/components/danielmadmon/libebur128_esp32s3 (3ª) |

## Bloque B. Core Audio, codecs y metadatos (puntos 4-6, 9)

| Pto | Afirmación | Estado | Corrección / dato | URL |
|---|---|---|---|---|
| 4 | AVAudioFile/ExtAudioFile escriben WAV 24 bit con 0x0001 o EXTENSIBLE | SIN FUENTE | Lo único hallado es un foro de desarrolladores (3ª, no Apple): AVAssetWriter con 16 bit emite WAVE_FORMAT_EXTENSIBLE (subformato PCM) y rellena con chunk `FLLR` hasta alinear a 4 KB. No hay dato de 24 bit ni de AVAudioFile. La decisión de escribir la cabecera a mano sigue siendo válida. | developer.apple.com/forums/thread/105039 (3ª) |
| 4 | Flag para evitar el relleno | SIN FUENTE | `kAudioFileFlags_DontPageAlignAudioData`: la página de Apple no se pudo abrir. QA1534 (archivo) sí documenta en `afconvert` el flag `--no-filler` ("disables page-aligning audio data"). | developer.apple.com/library/archive/qa/qa1534/_index.html |
| 5 | FLAC en Core Audio | CONFIRMADO (existencia) | `kAudioFormatFLAC` desde macOS 10.13; "usa flags para indicar la profundidad de bits del material fuente". Sin fuente sobre: codificación 24 bit, tasas admitidas, Vorbis comments (SIN FUENTE los tres). El `afconvert` documentado (mirror ss64) no lista FLAC como formato de salida. Plan B libFLAC sigue justificado. | developer.apple.com/documentation/coreaudiotypes/kaudioformatflac ; uucp.hu/mirror/ss64.com/osx/afconvert.html |
| 5 | "NO SEGURO de que Core Audio exponga dither" | REFUTADO | Existe `kAudioConverterPropertyDithering` (AudioToolbox, macOS) y `afconvert --dither algorithm` (valores 1-2). El comportamiento y los valores de la propiedad son SIN FUENTE (la página no trae descripción). | developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertydithering ; ss64 mirror |
| 5 | `kAudioConverterPropertyBitDepthHint` | CONFIRMADO | UInt32 con la profundidad de bits fuente a preservar. Es una pista para codificadores (ej. Apple Lossless); no es un control de dither. | developer.apple.com/documentation/audiotoolbox/kaudioconverterpropertybitdepthhint |
| 5 | Calidad SRC | CONFIRMADO (afconvert) | `-r/--src-quality` 0-127 (por defecto 127) y `--src-complexity line\|norm\|bats`. Los valores de `AVSampleRateConverterAlgorithm_Mastering` y `kAudioConverterSampleRateConverterQuality` no se verificaron (página no abierta): SIN FUENTE. | QA1534 ; ss64 mirror |
| 6 | macOS no codifica MP3 | CONFIRMADO (débil) | Hilo de coreaudio-api (2012): "el codificador no está incluido en el sistema" por licencias de patente. No hay doc actual de Apple sobre `kAudioFormatMPEGLayer3` como encoder. | lists.apple.com/archives/coreaudio-api/2012/Jun/msg00032.html |
| 6 | LAME LGPL-2 | CONFIRMADO / matiz | README del mirror: "GNU Library General Public License" (v2), con el decoder MPGLIB bajo GPL. La web oficial dice solo "LGPL" (sin versión) y anuncia v4.0 (jul 2026), según la página leída. | github.com/rbrito/lame ; lame.sourceforge.io |
| 6 | Texto LGPL-2 sobre enlace dinámico (§6) | SIN FUENTE | gnu.org no se pudo leer. Mantener la condición "dinámico + posibilidad de sustituir la lib". FFmpeg indica lo mismo para sus builds LGPL (enlazar DLL). | ffmpeg.org/legal.html |
| 6 | Patentes MP3 caducadas (2017) | SIN FUENTE | El README de LAME aún avisa de posibles patentes. | github.com/rbrito/lame |
| 6 | Práctica en apps de Mac App Store con LAME | SIN FUENTE (y ya fuera de alcance) | — | — |
| 9 | BWF `bext` no tiene ISRC | CONFIRMADO | Campos: Description, Originator, OriginatorReference, fecha/hora, TimeReference, Version, UMID, cinco campos de loudness, CodingHistory. Sin ISRC. | tech.ebu.ch/docs/tech/tech3285.pdf |
| 9 | RIFF INFO `ISRC` | REFUTADO como contenedor del código | En RIFF INFO el id `ISRC` significa "Source", no el código ISRC. Hilo JUCE: guardaban el código ahí por error y lo corrigieron ("Disambiguate ISRC and source information"). JUCE sí lo escribe en `axml`. | forum.juce.com/t/isrc-code-in-riff-info-tags/50067 (3ª) |
| 9 | WAV: chunk `id3 ` con TSRC; iXML | SIN FUENTE | No se leyó ninguna convención oficial ni el nombre de tag en iXML. | — |
| 9 | FLAC `ISRC` en Vorbis comment | CONFIRMADO | Campo sugerido "ISRC number for the track". | xiph.org/vorbis/doc/v-comment.html |
| 9 | MP3 TSRC (ID3v2.3) | CONFIRMADO | "International Standard Recording Code (ISRC) (12 characters)". TYER = año numérico de 4 caracteres. Codificación Unicode `$01` con BOM. El tamaño de cabecera es de 28 bits; en los frames es un campo de 4 bytes sin esa restricción (lectura del layout; la spec no usa "synchsafe"). | id3.org/id3v2.3.0 |

## Bloque C. afconvert, vídeo y plataformas (puntos 7-8)

| Pto | Afirmación | Estado | Corrección / dato | URL |
|---|---|---|---|---|
| 7 | Flujo ADM del PDF de Apple | CONFIRMADO | (a) fuente a tasa final: `afconvert source.wav intermediate.caf -d 0 -f caff --soundcheck-generate`. (b) fuente a más tasa: `afconvert source.wav -d LEF32@xxxxx -f caff --soundcheck-generate --src-complexity bats -r 127 intermediate.caf`. (c) `afconvert intermediate.caf -d aac -f m4af -u pgcm 2 --soundcheck-read -b 256000 -q 127 -s 2 final.m4a`. (d) auditar: `afconvert final.m4a decode.wav -d LEI24 -f WAVE`. | apple.com/itunes/docs/apple-digital-masters.pdf |
| 7 | Flags `-s 3` | REFUTADO | El PDF usa `-s 2`. Según QA1534/ss64: 0=CBR, 1=ABR, 2=VBR restringido, 3=VBR. `-r 127` es la calidad del SRC y `-q 127` la del codec. La línea base (`-s 2`, `-q 127`, `-b 256000`) coincide con el PDF, pero omitía `--soundcheck-*` y el CAF intermedio. | PDF ADM ; QA1534 ; ss64 mirror |
| 7 | Requisitos ADM | CONFIRMADO | Solo fuentes de 24 bit, sin subir tasa ni rellenar bits. El PDF nombra 48, 88,2, 96 y 192 kHz como tasas altas. Dejar ≥1 dB de headroom. El PDF indica `afclip` para recortes intermuestra y AURoundTripAAC para comparar. | PDF ADM |
| 7 | `afclip` existe y su uso | CONFIRMADO (según PDF) | `afclip <archivo>` y `afclip -h`. Que siga instalado en macOS actual: SIN FUENTE (el PDF parece antiguo: menciona 10.6.8). | PDF ADM |
| 8 | YouTube: vídeo a 1 fps | SIN FUENTE | La guía oficial no habla de imagen fija ni de fps bajos. Dice "codificar y subir a la misma tasa de grabación" (24-60 fps), MP4 sin edit lists y moov al inicio (fast start). Audio AAC-LC u Opus a 48 kHz, estéreo 384 kbps recomendado. Bitrate de vídeo 1080p recomendado: 8 Mbps (24-30 fps). | support.google.com/youtube/answer/1722171 |
| 8 | "~2 Mbps, AAC 256" | REFUTADO como recomendación oficial | La recomendación es 8 Mbps y 384 kbps de audio. Que 2 Mbps y 256 kbps sirvan para imagen fija es SIN FUENTE. | idem |
| 8 | Spotify Canvas | CONFIRMADO | 3 a 8 s, vertical 9:16, altura 720-1080 px, MP4 o JPG; tamaño máximo no indicado. | support.spotify.com/us/artists/article/canvas-guidelines/ |
| 8 | Spotify loudness | CONFIRMADO | Normal −14, Loud −11 (limitador desde −1 dB, ataque 5 ms, release 100 ms), Quiet −19 LUFS. Máster a ≤−1 dBTP; si es más fuerte que −14 LUFS, ≤−2 dBTP. | support.spotify.com/us/artists/article/loudness-normalization/ |
| 8 | Apple Music −16, YouTube −14, Tidal −14, Amazon −14/−2 dBTP, Deezer | SIN FUENTE | Ninguna página oficial leída da cifras. Amazon bloquea el robot (robots_disallowed). Todo es de terceros. | — |

## Bloque D. Notarización, CI y venta (puntos 12-13, 15)

| Pto | Afirmación | Estado | Corrección / dato | URL |
|---|---|---|---|---|
| 12 | notarytool + stapler fuera de tienda | CONFIRMADO (secundaria) | `xcrun notarytool submit X.zip --keychain-profile P --wait` (perfil con `store-credentials`); `xcrun stapler staple X.dmg\|.pkg\|.app`. La doc de Apple es solo JavaScript y no se pudo leer. | docs.bastion.tech/devices/apple/signing-notarizing (3ª) |
| 12 | Apple dice que Developer ID obliga a notarizar | CONFIRMADO | Tauri: "la notarización es obligatoria para certificados Developer ID Application"; una cuenta gratuita no puede notarizar. | v2.tauri.app/distribute/sign/macos/ |
| 13 | Runners macOS | CONFIRMADO | arm64 (3 vCPU M1, 7 GB): `macos-latest`, `macos-14`, `macos-15`, `macos-26`. Intel (4 vCPU, 14 GB): `macos-15-intel`, `macos-26-intel`. Según runner-images: `macos-latest` = macOS 26 arm64, y macos-14 figura deprecado. | docs.github.com/en/actions/reference/runners/github-hosted-runners ; github.com/actions/runner-images |
| 13 | Xcode preinstalado | CONFIRMADO | macOS 26 image (26.6.2): Xcode 26.6 por defecto (también 26.5, 26.4.1, 26.3…). macOS 15 image (15.7.9): Xcode 16.4 por defecto (hasta 26.3). | runner-images …/macos-26-arm64-Readme.md ; …/macos-15-arm64-Readme.md |
| 13 | "Multiplicador 10×" y minutos gratis | REFUTADO en forma | Los docs ya no hablan de multiplicadores ni de minutos: el uso incluido está en dólares/mes (Free 12 USD, Pro 18, Team 18, Enterprise Cloud 300). Precios por minuto: Linux 2-core 0,006 USD; macOS estándar 0,062 (≈10,3×); macOS grande 12-core 0,077; xlarge M2 Pro 0,102; Windows 2-core 0,010. Los minutos incluidos no valen para runners grandes. Con 12 USD salen unos 190 min macOS al mes (cálculo propio). Repos públicos: runners estándar gratis. | docs.github.com/en/billing/concepts/product-billing/github-actions ; …/billing/reference/actions-runner-pricing |
| 13 | `windows-latest` | CONFIRMADO | Aparece en el ejemplo oficial de tauri-action y en la guía de pipelines (matrix: macos-latest ×2 targets, ubuntu-22.04, windows-latest). | v2.tauri.app/distribute/pipelines/github/ ; github.com/tauri-apps/tauri-action |
| 13 | Firma macOS en Tauri/CI | CONFIRMADO | Variables: `APPLE_CERTIFICATE` (.p12 en base64: `openssl base64 -A -in cert.p12`), `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY`, notarización con `APPLE_ID`+`APPLE_PASSWORD` (contraseña de app) o API key (`APPLE_API_ISSUER`, `APPLE_API_KEY`, `APPLE_API_KEY_PATH`). `APPLE_TEAM_ID` no aparece en esa página. El ejemplo importa el certificado en un keychain temporal (`KEYCHAIN_PASSWORD` es un secret de CI, no de Tauri). `--skip-stapling` sirve para una primera pasada. README de tauri-action: no lista secrets de Apple. | v2.tauri.app/distribute/sign/macos/ |
| 13 | Firma en CI genérica (GitHub) | CONFIRMADO | Secrets `BUILD_CERTIFICATE_BASE64`, `P12_PASSWORD`, `KEYCHAIN_PASSWORD`; `security create-keychain`, `security import … -f pkcs12`, `security set-key-partition-list -S apple-tool:,apple:`. Los runners alojados se destruyen tras cada job. | docs.github.com/en/actions/how-tos/deploy/deploy-to-third-party-platforms/sign-xcode-applications |
| 13 | Firma Windows (Tauri) | CONFIRMADO | Opciones: certificado OV/EV (`certificateThumbprint`, `digestAlgorithm`, `timestampUrl`), Azure Key Vault (`relic`), Azure Artifact Signing (antes Trusted Signing, con `artifact-signing-cli` y `signCommand`), o `signCommand` propio. En CI: `.pfx` en base64 en `WINDOWS_CERTIFICATE` y `WINDOWS_CERTIFICATE_PASSWORD`, importado con `Import-PfxCertificate`. | v2.tauri.app/distribute/sign/windows/ |
| 13 | Azure Artifact Signing: coste | CONFIRMADO | Basic 9,99 USD/mes (5.000 firmas, +0,005 USD cada extra); Premium 99,99 USD/mes (100.000 firmas). Requiere suscripción de pago. Individuos: validación con Authenticator y Verified ID más un documento oficial con dirección. El nombre CN es el nombre legal validado. Lista de países: SIN FUENTE (remite al quickstart). | azure.microsoft.com/en-us/products/artifact-signing ; learn.microsoft.com/en-us/azure/trusted-signing/faq |
| 13 | Precio EV/OV | SIN FUENTE | Tauri dice solo "OV es generalmente más barato que EV". | v2.tauri.app/distribute/sign/windows/ |
| 13 | Sin firmar en Windows | CONFIRMADO | Sin firma se puede ejecutar aceptando el aviso de SmartScreen en descargas por navegador. Windows 11 Smart App Control bloquea los no firmados siempre. Desde 2024 un EV ya no da reputación inmediata: la reputación se acumula con el tiempo (FAQ de Microsoft: se construye "automáticamente" por historial de descargas del hash). | Tauri sign/windows ; learn.microsoft.com FAQ |
| 15 | Lemon Squeezy | CONFIRMADO | Es merchant of record. Comisión 5 % + 0,50 USD por transacción, con posibles extras en pagos no-US. License API (activate, validate, deactivate) en `api.lemonsqueezy.com`, límite 60 req/min, estados inactive/active/expired/disabled. Claves por variante ("Generate license keys"). Archivos por producto hasta 5 GB en total; actualizar un archivo lo da también a compradores anteriores. | lemonsqueezy.com/pricing ; docs.lemonsqueezy.com/api/license-api ; docs.lemonsqueezy.com/help/products/adding-products |
| 15 | Polar | CONFIRMADO | Es merchant of record. Starter 5 % + 0,50 USD; Pro (20 USD/mes) 3,8 % + 0,40; Growth (100 USD/mes) 3,6 % + 0,35; Scale (400 USD/mes) 3,4 % + 0,30. Extras: +1,5 % tarjetas internacionales, 15 USD por disputa, y payout de Stripe (2 USD/mes activo, 0,25 % + 0,25 USD por payout, conversión 0,25 % UE hasta 1 %). Early Member (4 % + 0,40 más 0,5 % de suscripción) solo para organizaciones creadas antes del 27-may-2026. Claves: `validate` y `activate` en `/v1/customer-portal/license-keys/` (los ejemplos no llevan cabecera de auth); `deactivate` no está documentado como API (solo en el portal). Descargas: hasta 10 GB por archivo, URL firmada personal, SHA-256, escaneo antimalware. | polar.sh/docs/merchant-of-record/fees ; polar.sh/docs/features/benefits/license-keys ; polar.sh/docs/features/benefits/file-downloads |
| 15 | Paddle | CONFIRMADO / matiz | Merchant of record, 5 % + 0,50 USD por checkout, sin cuotas mensuales; para productos <10 USD, precio a medida. La página de precios no menciona claves de licencia. El único documento de licencias hallado es de Paddle Classic (entrega por lista .txt o webhook, no API de licencias). Licencias en Paddle Billing: SIN FUENTE. | paddle.com/pricing ; paddle.com/support/can-i-sell-my-desktop-app-using-my-own-trials-and-licensing |

## Bloque E. Punto 16 — Crates Rust del motor

Las versiones y fechas vienen de la API de crates.io y reflejan el estado a 2026-11-05.

| Crate | Versión / licencia / última actualización | Estado | Notas | URL |
|---|---|---|---|---|
| hound | 3.5.1 · Apache-2.0 · 2023-09 | CONFIRMADO | Mantenimiento lento (más de 3 años sin publicar). Lectura/escritura WAV. El soporte de 24 bit y de cabecera 0x0001/EXTENSIBLE no se verificó: SIN FUENTE. | crates.io/crates/hound |
| wav | no consultado | SIN FUENTE | — | — |
| ebur128 | 0.1.11 · MIT · 2026-10-26 | CONFIRMADO | Port de libebur128. Su README dice "EBU R128" (no cita BS.1770-4/5), con true peak, LRA (Tech 3342), cualquier tasa (recalcula coeficientes) y "pasa todos los tests de Tech 3341 y 3342". Mantenimiento activo. | github.com/sdroege/ebur128 |
| flacenc | 0.5.1 · Apache-2.0 · 2025-12 | CONFIRMADO / sin datos | Encoder FLAC en Rust puro, pre-1.0, MSRV 1.83. El README no confirma 24 bit ni tasas, y no menciona Vorbis comments. | github.com/yotarok/flacenc-rs |
| libflac-sys | 0.3.4 · BSD-3-Clause · 2025-10 | CONFIRMADO | FFI a libFLAC (es la opción para 24 bit seguro y metadatos; compila C). | crates.io/crates/libflac-sys |
| mp3lame-encoder | 0.3.0 · LGPL-3.0 (crate) · 2026-11-03 | CONFIRMADO | Wrapper de mp3lame-sys; el README dice "LAME library is under LGPL". Enlace estático o dinámico de mp3lame-sys: SIN FUENTE (README no lo dice). Ojo: la licencia del crate es LGPL-3.0, no 2. | github.com/DoumanAsh/mp3lame-encoder |
| symphonia | 0.6.1 · MPL-2.0 · 2026-08-13 | CONFIRMADO | Decodificación y contenedores en Rust puro. | crates.io/crates/symphonia |
| rubato | 5.0.1 · MIT OR Apache-2.0 · 2026-10-01 | CONFIRMADO | Resampling asíncrono. | github.com/HEnquist/rubato |
| id3 | 1.17.2 · MIT · 2026-09-22 | CONFIRMADO | — | crates.io/crates/id3 |
| metaflac | 0.2.8 · MIT · 2025-01 | CONFIRMADO | — | crates.io/crates/metaflac |
| openh264 | 0.9.9 · BSD-2-Clause · 2026-10-15 | CONFIRMADO (crate) | Bindings de OpenH264. Patentes y licencia del binario Cisco: SIN FUENTE. | crates.io/crates/openh264 |
| mp4 | 0.14.0 · MIT · 2023-08 | CONFIRMADO | Lector/escritor de contenedor, sin mantenimiento reciente. | crates.io/crates/mp4 |
| dither en Rust ("hound dither") | — | SIN FUENTE | No se encontró un crate de dither; la línea base ya recomienda TPDF propio. | — |
| ffmpeg como sidecar | — | CONFIRMADO | Tauri: `bundle.externalBin` con sufijo de target triple (`-aarch64-apple-darwin`…), `shell:allow-execute` con `sidecar: true` y restricción de `args`. FFmpeg es LGPL 2.1+ solo si se compila sin `--enable-gpl` ni `--enable-nonfree`, con enlace dinámico (DLL en Windows) y sin libx264. | v2.tauri.app/develop/sidecar/ ; ffmpeg.org/legal.html |
| LGPL con enlace dinámico en Tauri | — | SIN FUENTE | No se encontró doc de LAME/Tauri. Un sidecar (proceso aparte) o una dylib/DLL aparte es lo que FFmpeg recomienda; por ser un crate de Rust (estático por defecto) habría que verificar el build. | — |
| AVAssetWriter solo Mac | — | SIN FUENTE nueva | No se verificó otra cosa que lo ya sabido; no es multiplataforma. | — |

## Bloque F. Punto 17 — Tauri 2 en 2026

| Afirmación | Estado | Dato | URL |
|---|---|---|---|
| Versión estable | CONFIRMADO | crate `tauri` v2.11.5 ("Latest", 1 jul); `tauri-cli` v2.11.4 (28 jun). El año no aparece en la página. | github.com/tauri-apps/tauri/releases |
| WebView2 en Windows | CONFIRMADO | Obligatorio. Ya incluido en Windows 10 desde abril 2018 (1803) y en Windows 11. Modos de instalación (`webviewInstallMode`): `downloadBootstrapper` (por defecto, +0 MB, necesita Internet), `embedBootstrapper` (~1,8 MB), `offlineInstaller` (~127 MB), `fixedRuntime` (~180 MB), `skip`. | v2.tauri.app/start/prerequisites/ ; v2.tauri.app/distribute/windows-installer/ |
| Mínimos del sistema | CONFIRMADO | macOS 10.15 o superior. En la página de requisitos se lee "Windows 7 y posteriores". | v2.tauri.app/start/prerequisites/ |
| Tamaño típico del binario | SIN FUENTE | La guía solo dice "binarios muy pequeños por defecto" y sugiere LTO, `codegen-units=1`, `opt-level="s"`, `panic="abort"`, strip y `removeUnusedCommands` (requiere tauri 2.4+). Sin cifras en MB. | v2.tauri.app/concept/size/ |
| Instaladores Windows | CONFIRMADO | NSIS (`-setup.exe`, se puede cruzar-compilar) y WiX v3 (`.msi`, solo en Windows). | v2.tauri.app/distribute/windows-installer/ |
| Drag & drop de archivos | CONFIRMADO | `getCurrentWebview().onDragDropEvent`: eventos `over` (con `position`) y `drop` (con `paths`) y cancelación. El flag `dragDropEnabled` y su conflicto con el HTML5 drag: SIN FUENTE (página truncada). | v2.tauri.app/reference/javascript/api/namespacewebviewwindow/ |
| Acceso a disco sin sandbox | CONFIRMADO / matiz | Tauri no usa sandbox del SO en desktop, pero el plugin fs bloquea por defecto: hay que dar permisos y scopes (allow/deny con variables como `$HOME`). Lectura por defecto solo en directorios de la app. El código Rust con `std::fs` no pasa por esos scopes. Cómo se concede acceso a rutas elegidas con el diálogo: SIN FUENTE. | v2.tauri.app/plugin/file-system/ |
| Updater integrado | CONFIRMADO | La firma es obligatoria y no se puede desactivar. Claves con `tauri signer generate`, `pubkey` en `tauri.conf.json`, `TAURI_SIGNING_PRIVATE_KEY` en el build, `createUpdaterArtifacts: true`. Endpoints con TLS; JSON estático con `version`, `platforms.<os-arch>.url` y `.signature`. Servidor dinámico: 204 sin update, 200 con JSON. Si se pierde la clave privada no se puede actualizar a los usuarios existentes. | v2.tauri.app/plugin/updater/ |
| Firma del instalador | CONFIRMADO | Ver bloque D (macOS Developer ID, Windows OV/EV/Azure). Hay aparte una firma del updater que no sustituye a la del SO. | v2.tauri.app/distribute/sign/ |

## Lo que cambia respecto a las líneas base

1. LRA: paso de ventana ≤100 ms (no 1 s); el "−20 LU y P10/P95" es correcto.
2. Test "−23 dBFS estéreo ≈ −20 LUFS" es falso: da −23,0 LUFS (Tech 3341 caso 1). Añadir los 23 casos de 3341 y los 6 de 3342 (±1 LU) como tests.
3. True peak: a 96 kHz basta 2× según BS.1770 (libebur128 también); 4× sigue siendo válido. Añadir la atenuación de 12,04 dB del Annex 2.
4. Dither: Core Audio sí lo expone (`kAudioConverterPropertyDithering`, `afconvert --dither`), aunque el comportamiento sigue sin documentar. `BitDepthHint` no es dither.
5. ADM: confirmar el flujo CAF con `--soundcheck-generate` y luego `--soundcheck-read`; `-s 2` (no 3); `-r 127` y `--src-complexity bats` solo al bajar la tasa.
6. ISRC en WAV: no usar RIFF INFO `ISRC` (es "source"). `bext` no tiene campo. Mejor `axml`/`iXML` y/o `id3 ` (estos dos sin fuente oficial), y TSRC (MP3) e `ISRC` (Vorbis comment, FLAC) confirmados.
7. YouTube recomienda 8 Mbps en 1080p y 384 kbps de audio; 1 fps sigue sin fuente. Cifras de plataformas distintas de Spotify: SIN FUENTE.
8. Costes de GitHub Actions: ahora en dólares; macOS 0,062 USD/min (≈10×), Windows 0,010; no hay "minutos gratis" documentados. `macos-latest` ya es macOS 26 (Xcode 26.6) y macos-14 está deprecado.
9. Con Tauri, el canal propio exige: Developer ID + notarización (obligatoria) y firma de Windows (Azure Artifact Signing desde 9,99 USD/mes, o OV/EV con precio SIN FUENTE). Sin firma en Windows: aviso de SmartScreen y bloqueo en Windows 11 con Smart App Control.
10. LAME: el crate `mp3lame-encoder` es LGPL-3.0 y la forma de enlace no está documentada; la vía más segura es un sidecar o una DLL/dylib aparte. FLAC: `libflac-sys` (BSD-3) para 24 bit seguro; `flacenc` es pre-1.0 y sin confirmar 24 bit.
11. Para MP4 de imagen fija: ffmpeg sidecar solo si es LGPL (sin `--enable-gpl`, sin x264, enlace dinámico); `openh264` y `mp4` no verificados para este uso.
12. Pasarelas: Lemon Squeezy (5 % + 0,50) y Polar (5 % + 0,50, o desde 3,4 % + 0,30 en plan de pago) tienen API de licencias y descargas. Paddle: sin API de claves confirmada en Billing.
13. Eliminados por alcance: App Sandbox, App Review, Small Business, DSA, capturas.
