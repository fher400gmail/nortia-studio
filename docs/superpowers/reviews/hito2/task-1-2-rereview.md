# Re-review tareas 1-2 (b14e82f..fca27bf)

Tests: `cargo test -p masterout-core -p mastercheck-bridge` en verde (todos los binarios ok, incl. cancelación 60 s y panic en job).

## Hallazgos
- I1 ADDRESSED. audio.rs:58-79 `key_and_detail()` cubre las 10 variantes de AudioError (Io, NotRiffWave, MissingFmt/Data, UnsupportedFormat con y sin "err.", TooManyChannels, Rf64, Cancelled); export.rs usa claves (lame_missing, container, threadpool, rate_over_96k, internal, documents); flac.rs `err.flac: detalle`; i18n es/en con todas las claves (err.rate_over_96k ya existía :60). Bridge: exporting.rs ProgressDto.error localizado + error_key/error_detail; DeliveryDto.failed localizado + failed_keys/failed_details; errors.rs usa key_and_detail. Test en inglés: tests/export.rs `job_errors_are_keys_never_spanish_text` (se salta si hay lame) + bridge `failed_job_carries_localized_message_key...`.
- I2 ADDRESSED. plan.rs `dithers = is_audio_pcm || Mp3`; export.rs `plan_sample_keys`; test `mp3_shares_the_dithered_cd_version_with_deezer` comprueba una sola clave (16,44100,true).
- I3 ADDRESSED. flac.rs `write_cancellable` cada 64 bloques; resample.rs por bloque (CHUNK=1024) y en el vaciado; dither.rs cada 65536 muestras (no por muestra); `AudioError::Cancelled` mapeado a Skipped (export.rs ~625). Tests: `cancel_is_checked_every_64_blocks` (para en la 3.ª comprobación), tiempo de cancelación.
- M1 ADDRESSED (TempGuard, export.rs ~195 y WAV intermedio MP3). M2 ADDRESSED (catch_unwind en documentos). M3 ADDRESSED (DeliveryDto.cancelled). M6 ADDRESSED (planning.rs y exporting.rs, sin duplicar artista). M7 ADDRESSED (comentario SampleKey/DITHER_SEED). Tests de onda mono / bins>frames / cero frames presentes (bridge tests/analysis.rs:130).

## Nuevo daño Critical/Important
Ninguno.
- Los campos nuevos del DTO son aditivos y snake_case (serde por defecto). `cancelled` es bool obligatorio al deserializar (solo importa si algo antiguo parsea DeliveryDto).
- Cancelación en la caché deja `Err("err.cancelled")` en la entrada, pero `render` comprueba `cancelled()` antes de leerla, así que nunca sale como JobFailed.

## Diferidos (una línea)
- `TEST_PANIC_PRESET` (export.rs:230) es `pub static` con `#[doc(hidden)]` sin `cfg(test)` ni guarda por entorno (los tests de integración no pueden usar cfg(test)): en release existe pero solo dispara si código en proceso lo activa; valorar `cfg(feature="test-hooks")` o guarda por env.
- El test de tiempo de cancelación cancela en JobStarted, cuando la caché ya está calculada: no ejercita los checkpoints de resample/quantize en vuelo (solo el de FLAC por test directo).
- Errores de mp3.rs (id3, lame_missing en `encode_with`, estado de lame) siguen siendo texto localizado dentro de UnsupportedFormat -> `err.generic` con ese texto como detalle; no se filtra a `error` pero el detalle va en idioma del motor. Mensajes de wav.rs/loudness/resample también en español como detalle de err.generic.
- Test `job_errors_are_keys...` se omite si hay lame instalado.

## Veredicto final: SÍ
