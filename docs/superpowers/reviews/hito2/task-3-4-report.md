# Informe tareas 3 y 4 · hito 2

STATUS: DONE_WITH_CONCERNS · commits 27b2eac (T3) y el siguiente `feat(ui)` (T4) en rama hito2-app.

## Tarea 3: scaffold Tauri 2
- `app/src-tauri` (crate propio con `[workspace]` vacío; lib `mastercheck_app_lib`). Feature `app` (por defecto) enlaza Tauri; con `--no-default-features` queda solo la lógica pura. `build.rs` llama a `tauri_build::build()` solo con la feature.
- Lógica pura en el puente: `mastercheck_bridge::app_support::{map_locale, os_name, AppInfo, app_info, lame::{resolve_lame_path, resolve_lame_in, path_with_dir_first}, smoke::{parse_spec, run_smoke, run_and_report}}` (único cambio fuera de src-tauri: `pub mod app_support;` en `engine/mastercheck-bridge/src/lib.rs` + módulo nuevo; el motor `masterout-core` no se toca). `src-tauri/src/{lame,locale}.rs` reexportan.
- Comandos (nombres de la brief): analyze, plan, export (emite `mastercheck://progress`, bandera en `State<ExportState>`), cancel_export, pick_folder, open_path (reveal_item_in_dir), system_lang, app_info.
  Añadidos NO previstos en la brief, necesarios: `pick_file(kind)` (el `<input type=file>` del webview no da ruta real; sin él no se puede abrir un máster ni una portada en la app), `set_lang(tag)` (el idioma de los mensajes del motor sigue al de la UI, `?lang=` incluido; el idioma inicial sale de `tauri-plugin-os`), `frontend_ready` (muestra la ventana oculta; además hay un respaldo a los 4 s). `AppInfo` lleva un campo extra `default_out_root` (Escritorio/master check).
- `lame`: el motor solo busca junto al ejecutable y en PATH; para macOS (`Contents/Resources`) la app antepone la carpeta de `lame` a `PATH` al arrancar (`prepare_lame_path`), sin tocar el motor.
- Humo: `MASTERCHECK_SMOKE="<wav>;<salida>"` en `lib.rs::run()` antes de crear nada de Tauri: analiza + plan + exporta (spotify, apple_adm, deezer, distributor_std, + mp3_promo si hay lame), imprime el DeliveryDto JSON, lo escribe también en `<salida>/smoke-result.json` (en Windows release la app no tiene consola), sale 0/1. Probado aquí con `cargo run --no-default-features` y la fixture m44_24.wav (exit 0; exit 1 con spec inválida).
- Config: `tauri.conf.json` (productName, identifier, 880×860 mín 760×720, `visible:false`, withGlobalTauri, CSP con `asset:`, assetProtocol scope `**`), `tauri.macos.conf.json` (Overlay, hiddenTitle, transparent false, targets dmg+app), `tauri.windows.conf.json` (sin decoraciones, transparent, shadow, nsis). Los arrays se reemplazan al fusionar, por eso cada fichero de plataforma repite la ventana completa.
- Capabilities exactas de la brief. Iconos: `npx @tauri-apps/cli@^2 icon ../../brand/iconos/macos/nortia-studio-claro-1024.png -o icons` (el nombre real es `nortia-studio-claro-1024.png`, no `studio-claro-1024.png`); borrados android/ios/Square*.

## Tarea 4: UI
- `app/ui` = design/final (sin capturas, *.py, final.png) + `backend.js` (mock | tauri, `detect()`, `?backend=`), `format.js` (−9,1 LUFS / -9.1 LUFS, kHz, MB, specs), `i18n.js`, `i18n/{es,en}.json` (mismas claves, comprobado por test), `boot.js` (script clásico previo al pintado: tema/os/native; fuera del HTML por la CSP), `app.js` como módulo ES.
- Estado S con master/waveform/plan/lang; onda real (bins con máximo por barra), pastilla y popover desde MasterDto, fichas desde `plan.jobs[]` (la UI pide el plan de las 10 fichas y selecciona en local; se recalcula al cambiar artista/título/versión/portada), `disabled_reason` como tooltip + `aria-disabled`, progreso desde ProgressDto, "listo" desde DeliveryDto (Abrir carpeta → openPath), fallos por destino en una línea de aviso, botón Cancelar durante la creación (vuelve a normal). Drag & drop Tauri vía `onDragDropEvent` (paths, posición/dpr); HTML5 en mock. Portada: `convertFileSrc`. Reproducción: audio real por asset protocol en Tauri; simulada en mock.
- Fichas ↔ presets: spotify, apple→apple_adm, amazon, tidal, deezer, ytm→youtube_music, dist→distributor_std, mp3→mp3_promo, portada→cover, canvas→canvas. El motor devuelve cover y canvas como `not_in_this_build`, así que salen deshabilitadas (el mock replica esto, y Apple se desactiva con un WAV de 16 bit). MP3 se deshabilita con aviso si `appInfo.lame_available` es false.
- El estado por defecto del mock sigue siendo "normal" (prototipo); en Tauri arranca vacío.

## Verificado aquí
- `cargo fmt --check` y `cargo clippy --all-targets -- -D warnings` limpios en `engine/` y `app/src-tauri` (`--no-default-features`).
- `cargo test -p mastercheck-bridge`: 33 tests OK (11 nuevos en `tests/app_support.rs`); `app/src-tauri` `cargo test --no-default-features`: 3 OK (`tests/lame_path.rs`).
- `node --test tests/backend.spec.mjs`: 13 OK. `node scripts/ui-check.mjs` (Playwright, Chromium): es y en OK, 0 pageerror/console error; cubre vacío → error no-WAV → soltar → crear → listo → nueva canción, chips deshabilitadas con tooltip, lame ausente, Apple off con 16 bit.
- Capturas del mock en `app/ui/capturas-mock/` (02, 03, 06, 07, es) revisadas con Read: el diseño no se ha roto (Portada/Canvas salen atenuadas por el motor; "Cancelar" nuevo en la barra de creando).
- Las APIs usadas de tauri 2.12 / plugin-dialog 2.8 / opener 2.7 / os 2.4 se contrastaron leyendo sus fuentes descargadas (`cargo fetch`), no compilando.

## NO verificado (comprobar en CI / máquina real)
1. `cargo check/build` de `app/src-tauri` con la feature `app` (no hay GTK/WebKit aquí): commands.rs, lib.rs, build.rs con `tauri_build`, generate_context!, permisos de capabilities (`dialog:allow-open`, `opener:allow-reveal-item-in-dir`, `os:allow-locale`, `core:window:*`), el `$schema` de capabilities apunta a `gen/` (se genera en el build).
2. Ventana: overlay macOS (comprobar que los semáforos no pisan el logo; sin `trafficLightPosition`), Windows transparente + sombra, controles HTML, arrastre `data-tauri-drag-region`, `visible:false` + `frontend_ready`.
3. `onDragDropEvent` (posición física/dpr), `convertFileSrc` y CSP (`asset:`, `http://asset.localhost`) con portada y audio; reproducción WAV 24/96 en WKWebView/WebView2.
4. `reveal_item_in_dir` sobre una carpeta (revela en el padre con la carpeta seleccionada; si se prefiere abrirla, `open_path`).
5. Empaquetado de `lame`: `bundle.resources` no está en el JSON (un glob vacío rompe el build): Task 5 debe pasarlo por `--config` cuando exista `app/vendor/lame/<target>/` (y `bundle.licenseFile`). El resolver ya busca `Contents/Resources[/lame]` y junto al exe.
6. El humo en CI: `MASTERCHECK_SMOKE="<wav>;<salida>" ./master check` debe leerse de `<salida>/smoke-result.json` en Windows.

## Preocupaciones
- `frontendDist: "../ui"` incrusta toda la carpeta (tests/, scripts/, capturas-mock/, DISEÑO-FINAL.md, package.json). Funciona; Task 5 puede preparar una carpeta limpia.
- `assetProtocol.scope = ["**"]` (lectura de cualquier ruta elegida por la persona para portada/audio); acotar si se prefiere.
- Los módulos ES y `fetch` de i18n no funcionan con `file://`: el prototipo se abre sirviendo `app/ui` por http (`python3 -m http.server`); los scripts llevan su propio servidor.
- Observación del motor (no tocado): con el commit b14e82f, el FLAC de `m44_24.wav` (WAV de 34 MB) ocupaba ~75–89 MB en una exportación de humo; parece que el escritor FLAC no comprime. Reproducir con el commit fca27bf (el humo en dev tarda >100 s con esa fixture) y revisar.
- Otro agente modificaba `engine/masterout-core` en el mismo worktree durante mi trabajo (luego commit fca27bf); verifiqué sobre una instantánea limpia de HEAD y de nuevo sobre el worktree ya estable.

## Fix round 1
- I1: `app/ui/flow.js::isCancelled(flag, delivery)`; `Create.start` usa `isCancelled(S.cancelled, d)` y nunca dibuja "Listo" si la entrega está cancelada. Test `cancel_race_delivery_flag_wins_when_event_never_arrives` (mock con `dropCancelEvent`).
- I2: `assetProtocol.scope` = `[]`; `allow_file` en `pick_file`, en `analyze` tras éxito y nuevo comando `allow_asset(path)` (solo .png/.jpg/.jpeg, `app_support::is_image_path`, con test); `backend.allowAsset` se llama antes de `convertFileSrc` en `loadCover`.
- M1: capabilities reducidas a `core:default` + `core:window:*`. M2: `export` rechaza una segunda con `err.busy` (`app_support::busy_error`, es/en, test; reserva bajo el mismo cerrojo). M4: tests/scripts/capturas/DISEÑO/contraste movidos a `app/ui-tests/`, `package.json` a `app/`; `frontendDist ../ui` solo lleva la UI; los scripts sirven `app/ui` por http. M5: mock con `cancelled`, `failed_keys`, `failed_details`. M6: `.catch` en los handlers de soltar; imagen sin canción → pista `drop.need_song` (es/en) y ui-check lo cubre. M7: `// TODO(brand)` sobre los glifos. M9: el humo ya escribía `<salida>/smoke-result.json` (DeliveryDto en éxito); el test ahora lo parsea como `DeliveryDto`.
- Verificado: node --test 16 OK; ui-check es+en OK; bridge tests OK (13 en app_support); src-tauri `--no-default-features` 3 OK; fmt/clippy limpios (rustfmt solo sobre mis ficheros; `engine/` tiene cambios ajenos sin commitear en masterout-core/flac que no he tocado). Sin compilar con Tauri: `allow_asset`, `asset_protocol_scope().allow_file` (API leída en tauri 2.12.5) y el `??` de `analyze`.
