# SDD ledger — plan: docs/superpowers/plans/2026-11-05-hito2-app-tauri.md
Rama hito2-app (worktree .worktrees/hito2). Spec: master-out-design.md + adendas; DISEÑO-FINAL.md; BRAND/DESIGN; final-review hito1.

## Pre-flight scan
| Par / tarea | Produce vs consume | Hallazgo |
|---|---|---|
| T1→T3/T4 | DTOs serde (MasterDto, PlanDto, JobDto, ProgressDto, DeliveryDto, BridgeError) | Nombres idénticos en comandos y backend.js; JSON camel/snake: Ruling → snake_case en JSON tal cual los campos Rust (sin rename), la UI usa snake_case |
| T2 presets.json | nuevo `subfolder_key` sustituye a `subfolder` literal | Rompe la tabla de contrato de hito 1 (tests/plan.rs) → Ruling: la tabla pasa a comprobar `naming::subfolder(key, Es)`; mismo literal en español |
| T2 ExportRequest.cancel | campo nuevo Option<Arc<AtomicBool>> | Tests existentes construyen ExportRequest con struct literal → añadir `..Default` o actualizar; aceptable |
| T3 sin build local | tauri no compila en el contenedor | Verificación en T5; T3 solo fmt + tests puros (lame.rs, map_locale) ejecutados como crate normal si el `[dev]` lo permite; si tauri no compila ni para tests → mover lame.rs y map_locale al bridge (Ruling) |
| T4 ↔ T3 | nombres de comandos y evento "mastercheck://progress" | Fijados en el plan |
| T5 | tauri-action vs cargo tauri | Ruling: usar `tauri-apps/tauri-action@v0` con `projectPath: app` y `tauriScript: npx tauri` |
| T6 | modo headless MASTERCHECK_SMOKE | Necesita que lib.rs lo soporte: añadir en T3 el hook (env var) para no reabrir T3 |

## Progreso
Task 1-2: implementados (e06a20c, b14e82f) por a973e7d024e45ea3f; DONE_WITH_CONCERNS (ExportRequestDto definido por el implementador; -inf acotado a -200; sin rojo previo). Memoria: pico ~2× el máster (antes ~10×). Revisión despachada.
Task 1-2: revisión: spec ✅, calidad aprobada con fixes. Fix round 1 despachado (I1 errores por clave, I2 MP3 comparte 16/44 con dither, I3 checkpoints de cancelación en FLAC/resample; M1 drop guard, M2 documentos bajo catch_unwind, M3 DeliveryDto.cancelled, M6 título vacío sin duplicar artista, M7 comentario; tests de panic en job y onda mono/bins>frames/0 frames).
Task 1-2: Ruling: carpeta de salida no escribible → Err(err.io) (como el brief), no Finished; T3/T4 muestran el error. Deferred: `panic = "abort"` prohibido en el perfil release de src-tauri (anotar en T3); set_hook de panics en la app; M8 JobDto deshabilitado con verdict "ok" → la UI comprueba `enabled` primero.
Task 1-2: fix round 1/5 (commit fca27bf; todos addressed según implementador; DTOs ganan error_key/error_detail, cancelled, failed_keys/details). Re-revisión despachada.
Task 1-2: fix round 1/5 (todos addressed, 0 open; b14e82f..fca27bf). Re-revisión: sí. Deferred: TEST_PANIC_PRESET sin cfg/env guard (poner feature `test-hooks` en T5); test de cancelación a mitad de resample; textos internos de mp3/wav en `detail`.
Task 1: complete (commits cea58de..e06a20c + fix fca27bf, review clean)
Task 2: complete (commits e06a20c..b14e82f + fix fca27bf, review clean)
Task 3-4: implementados (27b2eac, 3376be5) por acf15a8d3980fd0ca; DONE_WITH_CONCERNS (comandos extra pick_file/set_lang/frontend_ready; compilación Tauri solo en CI; FLAC de m44_24 ocupa 75–89 MB > WAV 34 MB → investigar). Revisión despachada.
Task 3-4: revisión: spec ✅, aprobada condicionada a I1 (carrera de cancel en UI) e I2 (assetProtocol scope "**"). Fix round 1 despachado (I1, I2, M1 capacidades sobrantes, M2 exportación concurrente → err.busy, M4 frontendDist sin tests/capturas, M5 mock completo, M6 catch en drop, M9 smoke-result.json).
Task 3-4: Ruling M7: glifos neutros de plataformas se mantienen en esta build; antes de la salida pública se sustituyen por los activos oficiales de cada plataforma según sus guías de marca (TODO(brand)). CI-only: compilación src-tauri, semáforos vs logo, transparencia Windows, reproducción WAV vía convertFileSrc, reveal de carpeta, lame.
Investigación FLAC (a6455689b4a3e1111): FLAC de m44_24 ocupa 75–89 MB > 34 MB WAV → en curso.
Task 3-4: fix round 1/5 (commit 4da3596; todos addressed según implementador). Re-revisión despachada.
FLAC: causa raíz flacenc 0.5.1 (Rice de 4 bits, parámetro ≤14 → residuos mayores que PCM en material fuerte); sustituido por flac-codec 1.3.3 (MIT/Apache, Rust puro): m44_24 pasa de 89,2 MB (259 %) a 28,3 MB (82 %), igual que ffmpeg; decodificación bit-idéntica. Commit dd10c1d. Ruling: umbral del test real 85 % (el máster es muy fuerte). Re-revisión conjunta despachada.
Task 3-4: fix round 1/5 (todos addressed; 3376be5..4da3596). Re-revisión: sí. FLAC dd10c1d: re-revisión sí (MD5, tags, 16 bit, cancel, >96k OK).
Task 3: complete (commit 27b2eac + fix 4da3596, review clean; compilación real pendiente de T5/CI)
Task 4: complete (commit 3376be5 + fix 4da3596, review clean)
Task 5: implementador despachado (a79dfbd03fba24207): workflow app-build.yml, primera compilación de src-tauri en CI, artefactos.
Deferred: allow_file con rutas con '[' o '*' (probar en CI); err.busy no limpia el slot ajeno (correcto).
Task 5: complete (dfca07b, 111850d): app-build.yml; src-tauri compiló a la primera en CI; runs 42085232876 y 42088328399 verdes; artefactos master-check-macos (dmg 5,7 MB + app.zip, universal) y master-check-windows (setup.exe 2,4 MB). Deferred: binario mastercheck-app (mainBinaryName), lame no empaquetado, capacidad macos-15 intermitente, descarga de artefactos bloqueada desde el contenedor (Fer descarga desde la web).
Task 6: complete (01e4d24, ffbcea5): humo en macOS verde (run 42095911188: binario real, 8 archivos, 318 ms, ruta con espacios y acentos, LANG es_ES); docs/pruebas/hito2-checklist.md. Revisión final de rama despachada.
Revisión final de rama (opus): APPROVE WITH FIXES. Before merge (fix wave a98e9433964c2c7ef): err.busy por i18n; tope de bins 8192; símbolo sin animación en bucle ni sombra (BRAND §4.5/§5); npm test en CI + fmt src-tauri + cli fijado + glob Windows; checklist +4 puntos.
Hito 3 (del revisor final): progreso "preparando" durante la caché; humo que pase por Tauri (comandos/CSP/asset) y en Windows; carrera mínima de cancel antes del registro del slot; textos sueltos en app.js (%, ×, <1 KB, 32 bit float, "documents"); claves err.lame_failed/err.id3; open_path limitado a la última entrega; tope de duración del máster (err.too_long); set_hook; feature test-hooks; mainBinaryName; lame empaquetado + LGPL; glifos oficiales (bloquea salida pública); humo Windows.
Fix wave final: 44f42c1 (5/5 según implementador; CI app y engine verdes). Re-revisión despachada.
Fix wave final: re-revisión sí (5/5). Hito 2 mergeable.
