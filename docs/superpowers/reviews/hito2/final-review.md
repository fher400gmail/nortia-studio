# Revisión final de rama · hito 2 (master check) · cea58de..ffbcea5

Comprobado aquí: `cargo test --all-features` en engine/ (con fixtures), 142 tests en verde · `cargo fmt --check` y `clippy -D warnings` limpios (engine y src-tauri) · `cargo test --no-default-features` en src-tauri en verde · `npm test` 16/16 · Playwright `ui-check` ok en es y en en · licencias: flac-codec y bitstream-io son MIT/Apache, tauri 2.12.5 y los plugins dialog/opener/os son Apache-2.0 OR MIT; no hay GPL (r-efi es MIT OR Apache OR LGPL y se usa con MIT).

## 1. Veredicto

**APPROVE WITH FIXES.** El contrato de DTOs, la cancelación, la memoria y el ámbito de seguridad son coherentes de punta a punta y están probados. Quedan cinco arreglos pequeños, cada uno de menos de 30 minutos (i18n, validación de una entrada, una regla de marca, CI y checklist). Ninguno toca la lógica.

## 2. Hallazgos (por prioridad)

### Antes del merge
1. **Cadena visible fuera de i18n.** `engine/mastercheck-bridge/src/app_support/mod.rs:52-62`: los textos de `busy_error` están en el código. Esto incumple la Global Constraint «toda cadena visible pasa por i18n». Arreglo: añadir `"err.busy"` a `masterout-core/i18n/{es,en}.json` y usar `BridgeError::from_key("err.busy", lang, None)`. El test `busy_error_is_localized_with_stable_key` debe seguir comprobando los literales.
2. **`bins` sin límite desde JS.** `app/src-tauri/src/commands.rs:48-52` llega a `analysis.rs:91`: un `bins` de 4·10⁹ reserva 16 GB y termina en OOM/abort. Arreglo: en `analysis::analyze`, `let bins = bins.clamp(1, 8192);`, con un test `analyze_clamps_bins`.
3. **Marca: el símbolo flota en bucle y lleva sombra.** `app/ui/styles.css:187` (y también `design/final/styles.css:187`) aplica `.dzone .mk{animation:mkb 4s … infinite; filter:drop-shadow(…)}`. Eso choca con BRAND §4.5 («el punto no se anima en bucle») y §5 (sin sombras en el símbolo). DISEÑO-FINAL solo pide que «respire» el halo de la zona, no el símbolo. Arreglo: quitar `animation` y `filter` de `.dzone .mk`. El halo `breathe` del borde se queda.
4. **La UI no se prueba en CI.** `npm test` y `ui-check` solo corren a mano. Arreglo: añadir a `.github/workflows/app-build.yml` (job `check`) `actions/setup-node@v4` (22) y `npm test` en `app/`. Si se quiere también `ui-check`, hace falta `npx playwright install --with-deps chromium`.
5. **El checklist no cubre lo que solo se ve en la app real.** `docs/pruebas/hito2-checklist.md` deja fuera el asset protocol y las rutas raras, que no verifican ni el humo ni Playwright. Hay que añadir 4 puntos:
   - reproducir el máster (play y saltar en la onda);
   - arrastrar una portada `.png`/`.jpg` y elegirla con el selector;
   - soltar un máster cuyo nombre tenga `[ ]`, acentos y espacios (esto cierra el deferred de `allow_file`);
   - soltar un MP3 o una carpeta (Review Focus 1): mensaje en el idioma y nada colgado.

### Hito 3
6. **Sin progreso mientras se prepara la caché.** `export.rs:588` llama a `build_cache` (remuestreo y cuantización de todas las combinaciones) antes de emitir ningún `job_started`. Con un máster de 4 min a 96k la UI se queda en «0 %» varios segundos. Arreglo: emitir `job_progress` (o un `kind:"preparing"` con `fraction`) por cada combinación calculada, o construir la caché por combinación justo antes de sus destinos.
7. **El humo no pasa por Tauri.** `app/src-tauri/src/lib.rs:51-53` hace que el binario salga antes del `Builder`: el humo prueba el ejecutable real y el puente, pero no los comandos, las capacidades, la CSP ni el asset protocol. Además `app/scripts/smoke-mac.sh:44-56` no comprueba el número exacto de archivos (8 sin lame) ni los 4 audios, ni que el FLAC decodifique. Arreglo: `assert len(files)==8`, comprobar los nombres de `02/05/07` y ejecutar `afinfo` sobre el FLAC. Más adelante, un humo de UI con `tauri-driver` o WebDriver.
8. **No hay humo en Windows.** Review Focus 5 (rutas con espacios y acentos, `lame.exe`) solo se cubre con la build. Arreglo: un paso en el job `windows` que ejecute `target/release/mastercheck-app.exe` con `MASTERCHECK_SMOKE` y lea `smoke-result.json`.
9. **Carrera mínima al cancelar.** Entre `backend.js:217` (`await listen`) y el registro del slot en `commands.rs:81-88`, un `cancel_export` (`commands.rs:106`) no hace nada y la exportación termina como «listo». La ventana es de unos pocos ms de IPC. Arreglo: que `Create.onEvent` reenvíe `cancelExport()` con el primer evento si `S.canceling`.
10. **Textos sin pasar por i18n en la UI.** En `app/ui/app.js`:
    - `' %'` / `'0 %'` (líneas 210, 375, 378): formato por idioma, «45 %» frente a «45%»;
    - `'×'` (374);
    - `'&lt;1 KB'` (294);
    - `WAV … bit` / `'32 bit float'` (201);
    - el id `"documents"` aparece en crudo en la línea de fallos (230). Hace falta una clave `chip.long.documents`.
11. **Errores de MP3 con texto ya traducido dentro de `err.generic`.** En `masterout-core/src/mp3.rs:30,163`, `tf(lang,"export.err.*")` acaba en `UnsupportedFormat` sin el prefijo `err.`, así que se convierte en `err.generic` con el texto traducido como `detail`. Arreglo: claves `err.lame_failed` / `err.id3` con el detalle aparte.
12. **`open_path` revela cualquier ruta.** `commands.rs:167-176` no limita qué se revela. Solo muestra en el Finder, no ejecuta, así que el riesgo es bajo. Arreglo: guardar en `ExportState` la última `delivery.folder` y revelar solo esa o algo dentro de ella.
13. **Sin tope de tamaño del máster.** `wav::read` carga cualquier WAV de hasta 4 GB: un float32 largo junto con la caché puede agotar la memoria de un Mac de 8 GB. Arreglo: rechazar en `analyze` a partir de N minutos o M bytes, con `err.too_long`.
14. **Higiene de CI y config:**
    - `npx @tauri-apps/cli@^2` sin versión fija (`app-build.yml:62,115`);
    - falta `cargo fmt --check` para src-tauri;
    - `msi/*.msi` en el upload de Windows, que nunca existe (solo hay `nsis`);
    - `core:window:allow-set-title` no se usa (`capabilities/default.json:12`);
    - `failed_details` no va alineado con `failed` (`dto.rs:113`): documentarlo o hacerlo `Option` por posición.

## 3. Triage del ledger (deferred / parked)

| Línea del ledger | Decisión | Nota |
|---|---|---|
| `panic = "abort"` prohibido en el release de src-tauri | drop (cumplido) | `Cargo.toml:44-48` no lo pone; dejarlo como convención |
| `set_hook` de panics en la app | hito 3 | registrar el panic en un log junto a la app antes de soporte a usuarios |
| M8 JobDto deshabilitado con verdict "ok" | drop | la UI comprueba `enabled` primero (`app.js:173`); documentado en `planning.rs:105` |
| TEST_PANIC_PRESET sin guard (feature `test-hooks`) | hito 3 | sigue `pub static` sin cfg (`export.rs:230`); poner `#[cfg(any(test, feature="test-hooks"))]`, ya que CI usa `--all-features` |
| Test de cancelación a mitad de resample | hito 3 | el checkpoint existe (`resample.rs:57`) pero no tiene test |
| Textos internos de mp3/wav en `detail` | hito 3 | ver hallazgo 11 |
| Ruling M7: glifos neutros con TODO(brand) | hito 3 (bloquea la salida pública) | `app.js:28` |
| CI-only: semáforos, transparencia Windows, reproducción WAV, reveal, lame | antes del merge (checklist) | ver hallazgo 5; transparencia y controles de Windows en hito 3 (no hay Windows en el checklist) |
| `allow_file` con `[` o `*` | antes del merge (checklist) | ver hallazgo 5 |
| err.busy no limpia el slot ajeno | drop | es correcto |
| Binario `mastercheck-app` (mainBinaryName) | hito 3 | decidir el nombre del proceso; el humo lo tiene fijo en `smoke-mac.sh:7` |
| lame no empaquetado | hito 3 | MP3 desactivado con aviso; al incluirlo, añadir `licenseFile` LGPL |
| Capacidad intermitente de macos-15 | drop | es de infraestructura |
| Descarga de artefactos bloqueada desde el contenedor | drop | Fer descarga desde la web |
| FLAC: umbral del test real al 85 % | drop (ruling vigente) | flac-codec es igual a `flac -5` y decodifica bit a bit igual |

## 4. Tres convenciones que merece la pena mantener

1. **Errores como claves estables de punta a punta.** El motor solo emite `err.*`; el puente los localiza en el borde como `BridgeError{key,message,detail}`; la UI muestra `message` y, si falta, `t(key)`. Los tests comprueban claves y literales del contrato, nunca textos que salgan de la implementación.
2. **Lógica sin Tauri en `mastercheck-bridge::app_support`, con la feature `app`.** Los comandos son una capa fina, todo se prueba en Linux sin GTK y el humo reutiliza el mismo código que el binario real.
3. **Ámbitos que empiezan cerrados y escrituras atómicas.** El asset scope arranca en `[]` y se abre con `allow_file` solo para lo que el usuario eligió. Las capacidades no dan fs ni permisos de plugins a JS. Cada archivo se escribe a `.part`, con `TempGuard` y `catch_unwind` por destino: un error, un panic o una cancelación nunca dejan archivos a medias.
