# SDD ledger — plan: docs/superpowers/plans/2026-11-05-hito1-motor-rust.md
Spec: docs/superpowers/specs/2026-11-05-master-out-design.md (+ adenda). Rama: hito1-motor (worktree .worktrees/hito1).

## Pre-flight scan
| Par / tarea | Produce vs consume | Hallazgo |
|---|---|---|
| T1→T2 (wav.rs) | T1 `read/probe/AudioBuffer`; T2 añade `write(samples_i32…)` en el mismo archivo | OK, T2 modifica archivo de T1; se despachan al mismo implementador (lote T1–T3) |
| T2↔T3 | T2 recibe `&[Vec<i32>]` ya cuantizados; T3 produce `Vec<Vec<i32>>` | Coherente |
| T1→T4/T5 | `AudioBuffer` f32 planar | Coherente; ebur128 `add_frames_planar_f32` acepta planar |
| T4→T8 | `LoudnessReport`, `Verdict` (claves i18n) consumidos por `MasterInfo`/`Job` | Coherente; claves de veredicto: tp_over_platform, louder_than_target, quieter_than_target, tp_over_zero — T8 usa tp_over_zero que T4 no lista → Ruling: `verdict()` acepta `max_tp` y devuelve `Warn("tp_over_zero")` cuando `target_lufs` es None y TP > 0; añadido a T4 por el despacho — coste si mal: una clave más |
| T7→T8/T10 | `audio_file_name/media_file_name/delivery_folder` | Coherente |
| T8→T9/T10 | `Resolved/Job/ReleaseMeta/MasterInfo` | Coherente |
| T6→T10 | `flac::write(samples_i32, sr, bits, tags)` | Coherente |
| T9 CSV | cabecera fija inglés | Coherente con spec (ficha para distribuidoras) |
| T10 MP3 | `lame` ausente en el contenedor; ffmpeg presente | Ruling: en tests, si no hay `lame` se salta MP3 (como dice el plan). No usar ffmpeg como sustituto en producción. |
| T3 RMS TPDF | el plan da dos intervalos distintos | Ruling: usar el teórico de la implementación (TPDF ±1 LSB + redondeo ⇒ RMS ≈ 0,50 LSB) con margen [0,45, 0,55]; el implementador documenta el cálculo |
| T11 | ninguna colisión | OK |

## Progreso
Task 1-3: implementados (58324fb, b8b090d, 4b01c1c) por a862cc08eb1fd827c; DONE_WITH_CONCERNS (duraciones del brief intercambiadas m44/m96 → test usa ffprobe; T2 sin rojo previo). Revisión despachada.
Task 1: complete (commits c17c0e8..58324fb, review clean)
Task 2: complete (commits 58324fb..b8b090d, review clean)
Task 3: complete (commits b8b090d..4b01c1c, review clean)
Task 1-3: minor (deferred): checked_mul en byte_rate (wav.rs:876); tests de chunk impar / data no último / fmt 18 / EXTENSIBLE float / escritor mono impar; data size 0 → EOF solo si no hay más chunks; semilla de dither por canal con mezcla (seed ^ c·0x9E3779B97F4A7C15); considerar ChaCha8Rng para estabilidad; escribir a temporal+rename en la capa de exportación (T10).
Task 1-3: ⚠️ resueltos por el controlador: float32→24 bit con TPDF es correcto (float cuenta como mayor resolución); bext v1 con campos de loudness a 0 es intencionado.
Task 4-5: implementados (5b5e17f, ba019d4) por abcb0a86246c6d2ac; DONE_WITH_CONCERNS (test TP intersample reinterpretado con amplitud √2 — Ruling: aceptado, el brief era físicamente imposible tal cual; tolerancia ±1 LU en louder/quieter; opt-level 2 en dev). Revisión despachada.
Task 4-5: revisión: spec ✅, calidad NO aprobada por Important 1 (resample Err con frames múltiplo de 1024 / buffer vacío). Fix round 1 despachado al implementador original (abcb0a86246c6d2ac).
Task 4-5: minor (deferred): epsilon en comparaciones TP > 0 (Ruling: se mantiene estricto; un TP > 0 por 1e-7 es información válida, revisar en T10 si molesta en UI); comentario del test 997 Hz; aserción redundante en upsample; variante AudioError::Dsp; invariante de canales de igual longitud (documentar en audio.rs en T10); a ≥192 kHz ebur128 no sobremuestrea (aceptado).
Task 4-5: fix round 1/5 (3 addressed según implementador, pendiente re-revisión; commit 2945c91). Re-revisión despachada.
Task 4-5: fix round 1/5 (3 addressed, 0 open; commit ba019d4..2945c91). Re-revisión: todo resuelto. Deferred: bucle while en resample.rs:58 sin tope de iteraciones.
Task 4: complete (commits 4b01c1c..5b5e17f + fix 2945c91, review clean tras fix round 1)
Task 5: complete (commits 5b5e17f..ba019d4 + fix 2945c91, review clean tras fix round 1)
Task 6-9: implementados (47970d9 T7, d438891 T8, 7cba0c1 T9, 8524582 T6) por ab2949c27b2eef294; DONE_WITH_CONCERNS (subcarpetas renumeradas 09 Hi-Res/10 TuneCore: contradice el contrato §4 → el revisor lo evaluará; Tidal/Deezer/YT Music sin URL oficial → status recommendation, aceptado). Revisión despachada.
Task 6-9: revisión: spec ❌ (Critical: subcarpetas/tags fuera del contrato §4; Important: panic en delivery_folder con versión larga; nombres > 200 bytes con tag largo). Fix round 1 despachado al implementador original (ab2949c27b2eef294) con Rulings: contrato §4 se restablece (06 YouTube incluye youtube_music; 07 Distribuidora incluye std, Hi-Res y TuneCore; 09 Imagen); distributor_std mantiene metadatos; "Master Out" → "master check" en todas las cadenas; campo source_kind url|note.
Task 6-9: minor (deferred): nombres reservados de Windows (CON, NUL) y controles bidi; distributor_hires cita DistroKid sin requisito hi-res (buscar fuente mejor); preset.name solo en español (i18n de nombres en T10/UI); presets::all() re-parsea JSON (cachear con OnceLock); neutralizar campos CSV que empiezan por =+-@; sin seek table en FLAC (aceptado).
Task 6-9: fix round 1/5 (según implementador 4 addressed; commit 5ab1718). Re-revisión despachada.
Task 6-9: fix round 1/5 (4 addressed, 0 open; commit 8524582..5ab1718). Re-revisión: sí. Deferred: test del conjunto ordenado de subcarpetas; caso UTF-8 en truncado; lib.rs y fixtures aún dicen "Master Out"; UI no debe agrupar solo por nombre de carpeta.
Task 6: complete (commit 8524582 + fix 5ab1718, review clean tras fix round 1)
Task 7: complete (commit 47970d9 + fix 5ab1718, review clean tras fix round 1)
Task 8: complete (commit d438891 + fix 5ab1718, review clean tras fix round 1)
Task 9: complete (commit 7cba0c1 + fix 5ab1718, review clean tras fix round 1)
Task 10: implementado (ff325fd, e4edd44) por a34e8f8613251a020; DONE_WITH_CONCERNS (tag Apple 'Apple Music' ≠ contrato '[Apple Digital Masters]' → a corregir en fix round; UTC; lame real no probado). Revisión despachada.
Task 10: revisión: spec ❌ (Critical: tag Apple "Apple Music" ≠ "[Apple Digital Masters]", oculto por tests que asertaban el valor erróneo; Important: fallo al escribir documentos aborta run sin Finished; dither no sale de job.needs_dither; cadenas de error fuera de i18n). Fix round 1 despachado al implementador original (a34e8f8613251a020). Ruling: float→24 bit sin dither aceptado en v1; UTC aceptado.
Task 10: minor (deferred): JobProgress grueso (0/0,6/0,8/1); callback de progreso que haga panic aborta el pool (sin catch_unwind); lame real no probado (solo wrapper ffmpeg) — verificar en CI macOS con brew lame; APIC con portada real sin probar.
Task 10: fix round 1/5 (según implementador todos addressed; commit 872fccd). Ruling: float32→24 bit CON dither (el plan lo marca así y es coherente con 'float = mayor resolución'); se anula la expectativa contraria del despacho. Re-revisión despachada.
Task 10: fix round 1/5 (todos addressed, 0 open; commit e4edd44..872fccd). Re-revisión: sí. Deferred: test de limpieza del WAV temporal y de localización de errores; comentario now_utc.
Task 10: complete (commits 5ab1718..872fccd, review clean tras fix round 1)
Task 11: implementador despachado (a7a844b838d2b0502): workflow engine-tests.yml, push de hito1-motor y vigilancia de runs.
Task 11: implementado (21a436b + fix 09eacad); CI: ubuntu y windows PASS; macos-15 PASS en el run 42061507022 (según revisor final). complete.
Revisión final de rama (opus): APPROVE WITH FIXES. Before merge: C1 FLAC falla > 96 kHz; I1 tag Amazon "Amazon" ≠ "[Amazon Music]"; I2 informes con nombres de preset solo en español y OK/WARN/FAIL fijos. Fix wave despachado (a0346388b5bbde35b).
Rulings finales: (a) másters > 96 kHz: los destinos FLAC entregan WAV con la misma etiqueta y nota en README (libflac-sys queda para hito 2 si hace falta FLAC hi-res); (b) dither siempre que se baje de bits, la fuente sea float o haya remuestreo con destino ≤ 24 bit; (c) nombres de carpeta/documentos en inglés para usuarios en inglés: hito 2 (extender contrato §4 con equivalentes); (d) memoria (1,1 GB pico), errores localizados, panic/cancel token: hito 2, primero memoria.
Fix wave final: commit 77ac9cf (C1 según override de Fer: >96 kHz rechazado; TuneCore 24 bit nativo; I1; I2; dither). Re-revisión final despachada.
