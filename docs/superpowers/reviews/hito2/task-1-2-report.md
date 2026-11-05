# Informe tareas 1 y 2 (hito 2)

STATUS: DONE_WITH_CONCERNS

Commits:
- e06a20c feat(bridge): DTOs, análisis con onda, plan y errores localizados
- b14e82f feat(core,bridge): contrato en inglés, caché de muestras, cancelación y protección de panics

Tests: 118 pasan, 0 fallan, tanto con MASTEROUT_FIXTURES como sin él. fmt y clippy --all-targets -D warnings limpios. MP3 se omite (sin lame).

## Memoria
Máster 60 s 96k/24 estéreo (46 MB en f32), 9 destinos, 4 hilos: delta de VmHWM ~95 MB (~2x), límite 184 MB.
Antes de la mejora el mismo caso daba ~460 MB.
Cómo: caché compartida por (bits, frecuencia, dither), calculada antes de los jobs; la última combinación a frecuencia nativa consume el Vec f32 (`dither::quantize_owned`, misma salida bit a bit), las remuestreadas van primero; cada entrada se libera al terminar su último destino. FLAC escribe bloque a bloque a disco (encode_fixed_size_frame + cabecera al final), sin entrelazado ni flujo completo en memoria. El test usa un WAV escrito en streaming y reinicia el pico con /proc/self/clear_refs; vive en `tests/memory.rs` (binario propio) para que el VmHWM de otros tests no lo contamine.

## Decisiones y desviaciones
- Los cambios del contrato de nombres (subfolder_key, naming::subfolder, resolve_in) van en el commit 1 porque el test de plan en inglés del bridge los necesita. El commit 2 trae el resto del motor.
- `presets.json`: `subfolder` pasa a `subfolder_key`; nuevo `tag_key` opcional (distributor_std, distributor_hires, cover) para `[Distributor]`, `[Distributor Hi-Res]`, `[Cover]`. `Preset.tag` sigue en español. `plan::resolve` se mantiene (Es) y se añade `plan::resolve_in(.., lang)`. Tests de tabla actualizados con `naming::subfolder(key, Es)`, mismos literales.
- `ExportRequest::new(..)`, `.with_threads`, `.with_cancel`; `cancel: Option<Arc<AtomicBool>>`.
- Cancelación: se comprueba entre destinos y antes de escribir; los archivos completos se conservan, no se escriben documentos, se borran subcarpetas vacías y la carpeta de entrega si queda vacía; `run` devuelve Ok(Delivery parcial) tras emitir `Cancelled { files_kept }`.
- Cambio de comportamiento: antes el CD (16/44,1) calculaba el dither con su propia regla (`16 < bits del máster`); ahora todos los destinos usan `job.needs_dither` del plan (que además dithera al remuestrear a <= 24 bit). Solo difiere para másters de 16 bit remuestreados a 44,1.
- `lang::Lang` del bridge es una unidad con `from_locale` que devuelve el `Lang` del motor (se reexporta como `EngineLang`).
- Valores no finitos (silencio, -inf) se acotan a -200 en el MasterDto para que el JSON sea válido (null no deserializa en f64).
- `analyze` del bridge hace `wav::probe` antes de leer: rechaza no-WAV, carpetas (err.not_wav) y > 96 kHz sin cargar el audio.
- Claves nuevas en i18n del motor (es/en): folder.*, tag.*, doc.*, err.not_wav, err.io, err.internal, err.generic, err.too_many_channels, err.cancelled, err.rf64_unsupported.
- Disabled en JobDto: file_name "", bits/sample_rate 0, verdict "ok" con texto vacío, `disabled_reason` localizado.
- `ExportRequestDto { master_path, out_root, meta, active_ids }` definido en dto.rs (el brief solo lo nombraba). ProgressDto "cancelled" no lleva `files_kept`; el DeliveryDto devuelto trae los archivos conservados.

## Concerns
- No se vio el rojo previo de los tests de cada paso (se escribieron junto a la implementación); todos pasan ahora.
- El test de cancelación del bridge no puede fijar hilos; solo exige al menos un archivo y evento `cancelled` final. La versión determinista (1 hilo) está en el motor.
- Un panic dentro de un job imprime el mensaje por stderr (hook por defecto); la app Tauri podría querer silenciarlo/registrarlo.

---
# Ronda de correcciones 1

STATUS: DONE_WITH_CONCERNS (clippy del bridge falla solo por `app_support/lame.rs`, del otro implementador; core y mis archivos limpios)

- I1: los errores de destino son claves. `AudioError::key_and_detail()` (audio.rs) da clave + detalle; `Progress::JobFailed { error: clave, detail }`; `Delivery.failed` = (id, clave), `failed_details`, `cancelled`. Textos de flac.rs, lame, contenedor, hilos y documentos pasan a claves (`err.flac: detalle`, `err.lame_missing`, `err.container`, `err.threadpool`, `err.documents`, i18n es/en). Bridge: `ProgressDto.error` localizado + `error_key` + `error_detail`; `DeliveryDto.failed` localizado + `failed_keys` + `failed_details`. `bridge::errors::localize` usa `key_and_detail`.
- I2: `plan.rs` aplica la regla de dither también al MP3. `export::plan_sample_keys` (doc hidden) permite comprobar que deezer + mp3_promo + distributor_std comparten una sola clave (16, 44100, true).
- I3: `AudioError::Cancelled`; `flac::write_cancellable` (cada 64 bloques), `resample::resample_cancellable` (cada bloque), `dither::quantize_cancellable` / `quantize_owned_cancellable` (cada 64 Ki muestras, conservan el reuso de memoria: pico sigue en ~95 MB). Un job con `Cancelled` se mapea a Skipped. Test con 60 s: cancelar al JobStarted tarda 1,0 s frente a 5,2 s completo; el checkpoint de FLAC se prueba directamente (se detiene en la 3.ª comprobación).
- M1: guard `TempGuard` en `atomic` y para el WAV intermedio del MP3. M2: escritura de documentos dentro de catch_unwind. M3: `DeliveryDto.cancelled`; `folder` vacío si se borró. M6: título vacío usa el sugerido sin duplicar artista, tanto en `plan` como en `exporting::export` (test corregido). M7: comentario sobre la clave de caché y `DITHER_SEED`.
- Tests nuevos: claves de error (core y bridge, inglés), caché compartida MP3, panic en un job (hook de prueba `export::TEST_PANIC_PRESET`; sin `.part`), cancelación temporizada, onda mono / bins > frames / cero frames / 0 bins, entrega cancelada, título vacío.
- Fuera de la lista permitida pero necesario: audio.rs, dither.rs, resample.rs, bridge errors.rs/analysis.rs (`suggest` pasa a pub(crate))/planning.rs, tests/flac.rs. No toqué `lib.rs` del bridge (cambio del otro implementador).
- Verificación: 138 tests pasan con fixtures (incluye los del otro implementador); fmt limpio; clippy limpio en masterout-core.
