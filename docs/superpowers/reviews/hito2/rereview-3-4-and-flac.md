# Re-review: fix round 1 de tareas 3-4 (4da3596) y fix FLAC (dd10c1d)

## (A) 4da3596
- I1 ADDRESSED: app/ui/flow.js:5 `isCancelled`; usado en app/ui/app.js:402 (nunca "Listo" si `d.cancelled`). Test `cancel_race_delivery_flag_wins_when_event_never_arrives` (mock `dropCancelEvent`, resuelve sin evento).
- I2 ADDRESSED: tauri.conf.json:28 `scope: []`; `allow_file` en commands.rs:58 (analyze), :143 (pick_file); `allow_asset` commands.rs:149 con `is_image_path` (bridge app_support/mod.rs, test `allow_asset_accepts_only_images`); `loadCover` llama `backend.allowAsset` antes de `convertFileSrc` (app.js ~451+). Sin compilar con Tauri (API vista en tauri src/scope/fs.rs:370).
- M1 ADDRESSED: capabilities/default.json solo core:default + core:window:*.
- M2 ADDRESSED: commands.rs:80-88 comprobar y reservar bajo el mismo cerrojo; el early return no toca el slot de la otra exportación; test `busy_error_is_localized_with_stable_key`.
- M4 ADDRESSED: frontendDist ../ui ahora solo app.js, backend.js, boot.js, flow.js, format.js, i18n*, index.html, styles.css, tokens.json, fonts; tests en app/ui-tests; package.json en app/.
- M5 ADDRESSED: mock DeliveryDto completo (backend.js), test `mock_delivery_has_full_dto_shape`.
- M6 ADDRESSED: app.js:451 `drop.need_song` (es/en), rejection de drop contenido en `Promise.resolve(run).catch`; ui-check lo cubre.
- M7 ADDRESSED: app.js:28 TODO(brand).
- M9 ADDRESSED: tests/app_support.rs parsea smoke-result.json como DeliveryDto.

Verificado: `npm test` 16/16; `node ui-tests/scripts/ui-check.mjs` es+en OK; bridge tests OK (app_support 13); src-tauri --no-default-features 3 OK.

## (B) dd10c1d FLAC
- Tests `--test flac --test export` con fixtures: 9/9 y 20/20 OK, incl. ambos tests de compresion (synthetic <80 %, real master <85 %), lossless, 24 bit.
- Vorbis comments: ADDRESSED. `writes_vorbis_comments` (metaflac) pasa; comprobado a mano: bloques STREAMINFO + VORBIS_COMMENT, sin PADDING/SEEKTABLE, tag con acento OK.
- 16 bit: OK (roundtrip y a16.flac decodifica con ffmpeg `-err_detect crccheck+aggressive`).
- MD5 de STREAMINFO: poblado y IGUAL al MD5 del PCM entrelazado, en 24 y 16 bit (comprobado con crate de prueba en scratchpad + python).
- Cancelacion: flac.rs `blocks.is_multiple_of(64) && should_cancel()` sigue; test `cancel_is_checked_every_64_blocks` pasa (3 llamadas).
- >96 kHz: error limpio `err.rate_over_96k`, sin archivo creado (el chequeo va antes de crear el writer); test pasa.
- flacenc: grep sin rastro en .rs/.toml/.lock del worktree (engine y app/src-tauri); solo queda mencion en un comentario.

## Nuevo Critical/Important
Ninguno.

## Diferidos (una linea)
- Cargo.lock de app/src-tauri en 4da3596 ya llevaba flac-codec mientras masterout-core aun pedia flacenc (arrastre del otro agente); consistente en dd10c1d, solo importa si alguien construye 4da3596 con --locked.
- `allow_file` con rutas con caracteres glob (`[`, `*`) no verificado (tauri sin compilar): probar en CI con un nombre asi.
- `Options::overwrite()` + cancelacion deja archivo parcial: ya lo borra el llamador (atomic), igual que antes.
- Siguen sin verificar los puntos de "NO verificado" del informe (build Tauri real, ventana, drag-drop, CSP).
- ui-check conserva un `.catch(() => {})` inocuo en `setInputFiles('#fcov', [])`.

## Veredicto final
- (A): SI
- (B): SI
