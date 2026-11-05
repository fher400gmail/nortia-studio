# Revisión tareas 1-2 (hito 2) · cea58de..b14e82f

Ejecutado: `cargo test` en engine con MASTEROUT_FIXTURES: todo en verde (bridge 18, core incl. memory 1). No se modificó nada.

## Spec compliance: ✅ (con una desviación aceptada y dos huecos de contrato)

Presente y con los valores del brief: los 8 tests de T1, los 7 de T2 (`peak_memory_under_4x_master` en `masterout-core/tests/memory.rs`, binario propio; delta ~2x vs límite 4x), DTOs y campos snake_case, `analyze` vía `export::analyze` (con `wav::probe` antes: >96k -> `err.rate_over_96k`, no-WAV/carpeta -> `err.not_wav`), contrato inglés/español, nunca ganancia, dither desde `job.needs_dither`.
- Extra aceptado: `ExportRequestDto` (dto.rs:284), `BridgeError::from_key`, `ExportRequest::new/with_*`, `resolve_in`, `quantize_owned`.
- Decisión `Cancelled` sin `files_kept` en `ProgressDto`: OK (engine.rs lo conserva; el `DeliveryDto` devuelto trae los archivos). Ver M1: el DTO no dice que fue cancelado.
- Brecha: Review Focus 3 del plan dice "`Finished` con `failed`" para carpeta no escribible; brief y test dicen `Err(err.io)`. Se cumple el brief; T3/T4 deben mostrar el Err (no habrá evento `finished`).

## Findings

### Critical
Ninguno.

### Important
- **I1. Mensajes por destino sin localizar / clave filtrada** (export.rs ~2127 `e.to_string()`; exporting.rs:390 `localize_message`). `AudioError` Display está en español fijo y `UnsupportedFormat` añade prefijo: un destino cuya caché falló o entró en pánico (`samples()` -> `unsupported("err.internal")`) sale como `"formato no soportado: err.internal"` (no empieza por `err.`, no se traduce); un `Io` (disco lleno) como `"error de E/S: ..."` en español para usuario inglés; los textos de flac.rs también son español fijo. Rompe "errores localizados" y Review Focus 3. Fix: en `run`, construir `error` con una función `job_error_key(&AudioError)`: `UnsupportedFormat(s) if s.starts_with("err.") => s`, `Io(_) => "err.io"` (detalle aparte), resto `err.generic`; o que el bridge reciba la clave y llame a `t()`. Añadir test: caché que falla -> `failed[0].1` sin "formato no soportado".
- **I2. MP3 ya no comparte la versión CD y pierde dither** (export.rs `sample_key`, plan.rs:~2520). `needs_dither` exige `is_audio_pcm` (Wav|Flac), así que para MP3 es siempre `false`: su clave es `(16,44100,false)` mientras Deezer/distribuidora usan `(16,44100,true)`. Resultado: segundo remuestreo + segunda copia i32 (más memoria) y MP3 cuantizado a 16 bit sin dither (antes usaba la CD con dither si el máster >16 bit). El informe solo menciona la diferencia para másters 16 bit. Fix: para MP3 usar `needs_dither = master_bits > 16 || is_float || needs_resample&&...` (misma regla que el destino 16/44,1) o derivar la clave del dither de la regla de 16 bit; añadir test que cuente combinaciones en caché para `deezer + mp3_promo`.
- **I3. Cancelación sin puntos dentro de bucles largos.** Comprobaciones: entre claves de `build_cache`, al inicio de job y tras `samples`, antes de crear archivos. Nada dentro de `resample`, `quantize`, `wav::write` ni el bucle FLAC por bloques (flac.rs ~2351). Cancelar durante un FLAC 96k/24 de 4 min espera a que termine el destino (segundos). No viola el brief, pero la UI "vuelve a normal" tarda. Fix barato: pasar `&dyn Fn()->bool` a `flac::write` y comprobar cada N bloques devolviendo un error de cancelación (`atomic` borra el `.part`); mapear a `Skipped`.

### Minor
- **M1.** Un panic dentro del closure de `atomic` (export.rs ~187) salta el `remove_file(&tmp)`: queda `.part` y la subcarpeta no vacía (tampoco se limpia `wav_tmp` de MP3). `catch_unwind` lo convierte en `err.internal` pero deja basura. Fix: guard RAII (`Drop` que borra `tmp` salvo tras rename).
- **M2.** Documentos (`reports::*`, export.rs ~2194) fuera de `catch_unwind`: un panic ahí aborta `run` sin `Finished`. Envolver cada generación.
- **M3.** `DeliveryDto` no distingue cancelado de terminado y, si no hay archivos, `folder` apunta a una ruta ya borrada. T3 debe decidirlo por el último evento (`cancelled`) y tratar `files.is_empty()`; o añadir `cancelled: bool` a `DeliveryDto`.
- **M4.** Sin test de panic dentro de un job (solo del callback) ni de que la caché se comparte (p. ej. `Arc::strong_count`/contador). La memoria lo cubre indirectamente.
- **M5.** Waveform correcta por inspección (pico sobre frames de todos los canales, `.max(start+1).min(frames)`, tramo final parcial, mono, `bins>frames`, clamp a 1), pero sin tests de mono, `bins>frames` ni longitud 0. Añadir uno.
- **M6.** `plan_dto_untitled_uses_file_name_fallback` fija "Vera Alcón - Vera Alcón - Luz de Noviembre" (artista duplicado). Comportamiento heredado del motor (`run` hace igual), pero conviene usar `suggest()` para título vacío; al menos no blindarlo con test.
- **M7.** Clave de caché `(bits, rate, bool)` es correcta solo porque `DITHER_SEED` es constante; comentarlo para que un cambio de semilla por job obligue a ampliar la clave.
- **M8.** Disabled `JobDto`: `verdict:"ok"` + texto vacío puede pintar un "OK" verde si la UI no mira `enabled`; documentado, avisar en T4.

Verificado sin problemas: caché por `Arc<Vec<Vec<i32>>>` (clon de Arc, no de datos), liberada por contador `remaining` (también en fallo/skip/panic); FLAC streaming (`scratch` de un bloque, cabecera al final con MD5 y total, sin entrelazado completo); `catch_unwind(AssertUnwindSafe)` en build_cache, cada job y cada `emit`; el mutex de la caché tolera poisoning; un callback que hace panic no envenena rayon (se captura dentro del worker); `Finished` siempre salvo cancelación/Err inicial; orden de limpieza en cancelación (subcarpetas vacías, luego carpeta si vacía, luego `Cancelled`); sin `unwrap/expect` nuevos en rutas de datos; no hay español nuevo salvo los mensajes de flac.rs/AudioError (I1).

## ⚠️ No verificable desde el diff / aquí
- Que el perfil release de `app/src-tauri` no ponga `panic = "abort"` (anularía todo el `catch_unwind`); `engine/Cargo.toml` no lo define.
- Mensaje de panic por stderr (hook por defecto): T3 puede querer `panic::set_hook`.
- Comportamiento de la memoria en macOS/Windows y con mp3 (sin `lame` aquí, MP3 se omite).
- Flac con último bloque parcial decodificado por reproductores externos (los tests de `tests/flac.rs` pasan con claxon; no revisé su cobertura del MD5).

## Task quality: **Approved with fixes** (corregir I1 e I2 antes de T3; I3, M1-M3 pueden ir en T3/T4 o un commit de seguimiento).
