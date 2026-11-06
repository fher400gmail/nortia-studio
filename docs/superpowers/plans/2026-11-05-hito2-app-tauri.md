# master check · Hito 2 · App Tauri (Mac y Windows) — Plan de implementación

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Una app de escritorio instalable (macOS 14+, Windows 10/11) que une el motor `masterout-core` con la interfaz de `design/final/`: sueltas el máster, ves onda y loudness reales, activas plataformas, pulsas "Crear carpeta de entrega" y la carpeta aparece; con build automática en GitHub Actions descargable para probar.

**Architecture:** Tres capas. (1) `engine/masterout-core` (hito 1) + un crate nuevo `engine/mastercheck-bridge` (Rust puro, serde): DTOs JSON estables, análisis + forma de onda, plan, exportación con progreso, cancelación, errores localizados; **testable en Linux sin Tauri**. (2) `app/src-tauri`: comandos Tauri 2 finísimos que llaman al bridge y emiten eventos; configuración de ventana según `design/final/DISEÑO-FINAL.md`; capacidades mínimas. (3) `app/ui`: los archivos de `design/final/` con un adaptador `backend.js` que elige `mock` (navegador/prototipo) o `tauri` (app real), de modo que el prototipo sigue vivo. Sin sandbox del SO; sin red.

**Tech Stack:** Rust 2021, Tauri 2.x (`tauri`, `tauri-plugin-dialog`, `tauri-plugin-opener`, `tauri-plugin-os`), serde/serde_json, `@tauri-apps/api` v2 vía `withGlobalTauri` (sin bundler: HTML/CSS/JS planos), GitHub Actions con `tauri-apps/tauri-action`. Iconos desde `brand/iconos/`. Fuentes Hanken Grotesk locales.

**Spec:** `docs/superpowers/specs/2026-11-05-master-out-design.md` (+ adendas 14:30, 15:49, 15:50–15:52, 17:50), `design/final/DISEÑO-FINAL.md`, `brand/BRAND.md`, `brand/DESIGN.md`, revisión final del hito 1 `docs/superpowers/reviews/hito1/final-review.md` (I3 memoria, I4 errores, I5 panic/cancel).

## Global Constraints

- Nombre de la app y del producto: `master check` (minúsculas) · marca `ferboix. studio` · `brand/BRAND.md` y `brand/DESIGN.md` mandan en todo lo visible (colores de `brand/tokens.css`, Hanken Grotesk, acento cian solo donde hay significado, botón primario en tinta, sin logos de terceros).
- Ventana: 880×860 por defecto, mínimo 760×720; macOS `decorations:true, titleBarStyle:"Overlay", hiddenTitle:true, transparent:false`; Windows `decorations:false, transparent:true, shadow:true` con controles HTML. La ventana es la tarjeta.
- Idioma: `Lang` del sistema (`tauri-plugin-os` locale → `es*`→Es, otro→En); toda cadena visible pasa por i18n (motor: `i18n/*.json`; UI: `app/ui/i18n/{es,en}.json` con las mismas claves donde coincidan).
- Motor intocable en nivel: nunca ganancia. Másters > 96 kHz rechazados con `err.rate_over_96k`. Contrato de nombres §4 (español) y su equivalente inglés (abajo).
- Sin red. Sin GPL. `cargo fmt`, `cargo clippy --all-targets -- -D warnings` limpios en `engine/` y `app/src-tauri/`.
- Nada se construye con Tauri en el contenedor (faltan libs GTK): el bridge se prueba aquí; `src-tauri` y la UI se verifican en Actions (macOS y Windows) y con `tauri::test` donde sea posible.

### Contrato §4 en inglés (Lang::En)
Carpeta `{Artist} - {Title}{ (Version)}/`; subcarpetas `01 Spotify · 02 Apple Music · 03 Amazon Music · 04 Tidal · 05 Deezer · 06 YouTube · 07 Distributor · 08 Promo · 09 Artwork`; etiquetas `[Distributor]`, `[Distributor Hi-Res]`, `[Cover]`; documentos `README.txt · loudness-report.txt · release-sheet.csv · release-sheet.txt`. En español se mantiene §4 tal cual (`07 Distribuidora`, `09 Imagen`, `[Portada]`, `informe-loudness.txt`, `ficha-lanzamiento.*`). Las etiquetas de plataforma (`[Spotify]`, `[Apple Digital Masters]`, …) y `README.txt` no cambian.

## Review Focus

1. Soltar un archivo que no es WAV (MP3, AIFF, carpeta): mensaje claro en el idioma del sistema, nada se cuelga. → T1 `analyze_dto_rejects_non_wav_with_localized_message`, T4 manual en CI (smoke).
2. Cancelar a mitad de exportación: no quedan `.part`, la carpeta parcial se elimina si no tiene archivos completos, la UI vuelve a "normal". → T2 `cancel_mid_export_leaves_no_partials`.
3. Carpeta de salida sin permiso de escritura o disco lleno: `Finished` con `failed` y mensaje localizado; sin panic. → T2 `unwritable_out_root_reports_failure`.
4. Máster de 4 min a 96/24 con 9 destinos: memoria pico < 4× el tamaño del máster en f32 (antes 10×). → T2 `peak_memory_under_4x_master` (Linux, /proc/self/status VmHWM).
5. Windows: rutas con espacios y acentos, `lame.exe` junto al ejecutable, controles de ventana HTML. → T3 `tauri::test` de `resolve_lame_path` y T5 build en windows-latest.

---

## Estructura de archivos

```
engine/
  Cargo.toml                       # workspace: ["masterout-core", "mastercheck-bridge"]
  mastercheck-bridge/
    Cargo.toml                     # deps: masterout-core, serde, serde_json, thiserror
    src/lib.rs                     # pub mod dto, analysis, planning, exporting, errors, lang
    src/dto.rs                     # structs serde: MasterDto, WaveformDto, PlanDto, JobDto, ProgressDto, DeliveryDto, ReleaseMetaDto, ExportRequestDto
    src/analysis.rs                # analyze(path, lang, bins) -> Result<(MasterDto, WaveformDto), BridgeError>
    src/planning.rs                # plan(master, meta, active, lang) -> PlanDto
    src/exporting.rs               # export(req, on_progress, cancel) -> DeliveryDto
    src/errors.rs                  # BridgeError { key, message(lang), detail }
    src/lang.rs                    # Lang::from_locale(&str)
    tests/*.rs
app/
  ui/                              # copia de design/final + backend.js, i18n/, platform glyphs
    index.html  styles.css  app.js  backend.js  i18n/es.json  i18n/en.json  fonts/
  src-tauri/
    Cargo.toml                     # tauri 2, plugins dialog/opener/os, mastercheck-bridge
    tauri.conf.json                # ventana, bundle (icons de brand/), identifier "studio.ferboix.mastercheck"
    capabilities/default.json
    src/main.rs  src/lib.rs        # comandos: analyze, plan, export, cancel_export, pick_folder, open_path, system_lang, app_info
    icons/                         # generados con `tauri icon` desde brand/iconos/macos/studio-*-1024.png
.github/workflows/app-build.yml    # tauri-action en macos-15 y windows-latest; artefactos
```

---

### Task 1: Crate `mastercheck-bridge` — DTOs, análisis con forma de onda, plan, errores localizados

**Files:** Create `engine/mastercheck-bridge/{Cargo.toml,src/lib.rs,src/dto.rs,src/analysis.rs,src/planning.rs,src/errors.rs,src/lang.rs}`; Modify `engine/Cargo.toml` (workspace); Test `engine/mastercheck-bridge/tests/{analysis.rs,planning.rs,errors.rs}`.

**Interfaces:**
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)] pub struct MasterDto { pub path: String, pub file_name: String, pub sample_rate: u32, pub bits: u16, pub is_float: bool, pub channels: u16, pub frames: u64, pub duration_secs: f64, pub integrated_lufs: f64, pub true_peak_dbtp: f64, pub lra_lu: f64, pub overs_count: u64, pub suggested_artist: Option<String>, pub suggested_title: Option<String> }
pub struct WaveformDto { pub bins: Vec<f32> /* 0..1, pico absoluto por bin, len == bins pedidos */ }
pub struct ReleaseMetaDto { artist, title, version, featuring, album, year, isrc, composer, label, copyright, cover_path: Option<String> }  // todos Option<String> salvo artist/title: String
pub struct JobDto { pub preset_id: String, pub subfolder: String, pub file_name: String, pub container: String /* "wav"|"flac"|"mp3"|"mp4"|"jpg"|"png" */, pub bits: u16, pub sample_rate: u32, pub verdict: String /* "ok"|"warn"|"fail" */, pub verdict_text: String /* localizado */, pub estimated_bytes: u64, pub enabled: bool, pub disabled_reason: Option<String> /* localizado */ }
pub struct PlanDto { pub delivery_folder: String, pub jobs: Vec<JobDto>, pub total_files: u32 /* jobs habilitados activos + 4 documentos */, pub total_bytes: u64 }
pub struct BridgeError { pub key: String /* p. ej. "err.rate_over_96k", "err.not_wav", "err.io" */, pub message: String /* en el idioma pedido */, pub detail: Option<String> }
pub fn analysis::analyze(path: &Path, lang: Lang, bins: usize) -> Result<(MasterDto, WaveformDto), BridgeError>   // llama a masterout_core::export::analyze; bins: pico por tramo; suggested_* desde naming::fallback_title + patrón "Artista - Título"
pub fn planning::plan(master: &MasterDto, meta: &ReleaseMetaDto, active_ids: &[String], lang: Lang) -> PlanDto   // usa plan::resolve + naming; verdict_text y disabled_reason con i18n::t(lang, key)
pub fn lang::Lang::from_locale(tag: &str) -> masterout_core::i18n::Lang
pub fn errors::localize(err: &AudioError, lang: Lang) -> BridgeError   // mapea UnsupportedFormat("err.*") → su texto; Io → "err.io" + detalle; NotRiffWave → "err.not_wav"
```
- [ ] **Step 1: Tests que fallan** — `analyze_real_or_synthetic_master_returns_waveform_with_requested_bins` (bins 600 → len 600, todos en 0..=1, máximo ≈ pico), `analyze_dto_rejects_non_wav_with_localized_message` (archivo .txt → key "err.not_wav", message es contiene "WAV"), `analyze_rejects_192k_with_key` ("err.rate_over_96k"), `suggested_artist_title_from_file_name` ("Vera Alcón - Luz de Noviembre (master v3).wav" → Some("Vera Alcón"), Some("Luz de Noviembre")), `plan_dto_marks_disabled_apple_for_16bit_with_reason`, `plan_dto_totals_count_documents` (3 jobs → total_files 7), `plan_dto_english_names_use_english_contract` (Lang::En → subfolder "07 Distributor", file_name contiene "[Distributor]"), `lang_from_locale` ("es-ES"→Es, "es_MX.UTF-8"→Es, "en-US"→En, ""→En).
- [ ] **Step 2–5:** ejecutar (falla) → implementar → ejecutar (pasa) → commit `feat(bridge): DTOs, análisis con onda, plan y errores localizados`.

---

### Task 2: Motor: nombres en inglés, memoria, cancelación, panics, exportación por el bridge

**Files:** Modify `engine/masterout-core/src/{naming.rs,presets.rs,plan.rs,reports.rs,export.rs,i18n/*.json}`; Create `engine/mastercheck-bridge/src/exporting.rs`; Test `engine/masterout-core/tests/{naming.rs,export.rs}` (añadir), `engine/mastercheck-bridge/tests/exporting.rs`.

**Interfaces:**
```rust
// masterout-core
pub fn naming::subfolder(preset_subfolder_key: &str, lang: Lang) -> String   // presets.json pasa a tener subfolder_key ("spotify"… "distributor", "promo", "artwork"); i18n: "folder.distributor" = "07 Distribuidora" / "07 Distributor", etc.
pub fn naming::document_names(lang: Lang) -> DocumentNames { readme, loudness_report, sheet_csv, sheet_txt }
pub struct ExportRequest { …, pub lang: Lang, pub cancel: Option<Arc<AtomicBool>> }   // nuevo campo
pub enum Progress { …, Cancelled { files_kept: u32 } }
// cache compartida: HashMap<(bits, rate, dither), Arc<Vec<Vec<i32>>>> calculada una vez por combinación; FLAC escribe por bloques desde &[Vec<i32>] sin copia entrelazada completa
// cada job dentro de std::panic::catch_unwind → JobFailed { error: "err.internal" } ; callback de progreso también protegido
// mastercheck-bridge
pub fn exporting::export(req: &ExportRequestDto, lang: Lang, on_progress: &(dyn Fn(ProgressDto) + Sync), cancel: Arc<AtomicBool>) -> Result<DeliveryDto, BridgeError>
pub struct ProgressDto { pub kind: String /* "analyzed"|"planned"|"job_started"|"job_progress"|"job_done"|"job_failed"|"cancelled"|"finished" */, pub preset_id: Option<String>, pub fraction: Option<f32>, pub path: Option<String>, pub bytes: Option<u64>, pub error: Option<String>, pub delivery: Option<DeliveryDto> }
pub struct DeliveryDto { pub folder: String, pub files: Vec<(String, u64)>, pub failed: Vec<(String, String)>, pub elapsed_ms: u64 }
```
- [ ] **Step 1: Tests que fallan** — `english_contract_folder_and_document_names` (Lang::En: "07 Distributor", "09 Artwork", "[Cover]", "loudness-report.txt"); `spanish_contract_unchanged`; `peak_memory_under_4x_master` (Linux: máster sintético 60 s 96k/24 estéreo = 46 MB f32; exportar 9 destinos; leer VmHWM antes/después; delta < 4× 46 MB; `#[cfg(target_os="linux")]`); `cancel_mid_export_leaves_no_partials` (cancelar desde el callback al primer JobDone → evento Cancelled, sin `.part`, sin subcarpetas vacías); `unwritable_out_root_reports_failure` (out_root es un archivo, no un directorio → Err BridgeError key "err.io", sin panic); `panicking_progress_callback_does_not_abort_run` (callback que hace panic una vez → run termina con Finished); bridge `export_emits_progress_dtos_in_order_and_returns_delivery`.
- [ ] **Step 2–5** → commit `feat(core,bridge): contrato en inglés, caché de muestras, cancelación y protección de panics`.

---

### Task 3: Scaffold Tauri 2 (`app/src-tauri`) con comandos, ventana, capacidades e iconos

**Files:** Create `app/src-tauri/{Cargo.toml,tauri.conf.json,build.rs,capabilities/default.json,src/main.rs,src/lib.rs,src/commands.rs,src/lame.rs,icons/*}`; Test `app/src-tauri/src/commands.rs` (unit, `#[cfg(test)]` con `tauri::test::mock_builder` donde aplique) y `app/src-tauri/tests/lame_path.rs`.

**Interfaces (comandos; nombres exactos, usados por T4):**
```rust
#[tauri::command] async fn analyze(path: String, bins: u32) -> Result<AnalyzeResponse { master: MasterDto, waveform: WaveformDto }, BridgeError>
#[tauri::command] fn plan(master: MasterDto, meta: ReleaseMetaDto, active_ids: Vec<String>) -> PlanDto
#[tauri::command] async fn export(app: AppHandle, req: ExportRequestDto) -> Result<DeliveryDto, BridgeError>   // emite "mastercheck://progress" con ProgressDto; guarda el AtomicBool en State<ExportState>
#[tauri::command] fn cancel_export(state: State<ExportState>)
#[tauri::command] async fn pick_folder(app: AppHandle) -> Option<String>      // tauri-plugin-dialog
#[tauri::command] fn open_path(app: AppHandle, path: String) -> Result<(), BridgeError>   // tauri-plugin-opener (revela carpeta)
#[tauri::command] fn system_lang() -> String   // "es"|"en" desde tauri-plugin-os locale
#[tauri::command] fn app_info() -> AppInfo { version, os: "mac"|"win"|"linux", lame_available: bool }
pub fn lame::resolve_lame_path(exe_dir: &Path) -> Option<PathBuf>   // junto al ejecutable (mac: Contents/MacOS, Contents/Resources; win: mismo dir), luego PATH
```
- `tauri.conf.json`: `productName "master check"`, `identifier "studio.ferboix.mastercheck"`, ventana según DISEÑO-FINAL (mac overlay; win sin decoraciones; `label "main"`, `visible:false` y `show()` tras `ready` del frontend), `app.withGlobalTauri: true`, `bundle.targets ["dmg","app","nsis"]`, `bundle.icon` generado con `cargo tauri icon brand/iconos/macos/studio-verde…` → **usar el icono de marca v2** `brand/iconos/macos/studio-claro-1024.png` (o el nombre real en `brand/iconos/macos/`), `bundle.resources` incluye `lame` si existe `app/vendor/lame/<target>/` (vacío por ahora; el MP3 se desactiva con aviso si no hay `lame`), `bundle.macOS.minimumSystemVersion "14.0"`, `bundle.licenseFile` LGPL de lame cuando se incluya.
- `capabilities/default.json`: `core:default`, `core:window:allow-minimize|toggle-maximize|close|start-dragging|set-title`, `dialog:allow-open`, `opener:allow-reveal-item-in-dir`, `os:allow-locale`; `fs` NO (el acceso a disco lo hace Rust con rutas elegidas).
- Config por plataforma: `tauri.macos.conf.json` y `tauri.windows.conf.json` para `decorations/titleBarStyle/transparent/shadow`.
- [ ] **Step 1: Tests que fallan** — `resolve_lame_path_prefers_bundle_dir` (crea `exe_dir/lame` ejecutable falso → lo devuelve), `resolve_lame_path_falls_back_to_path_env`, `system_lang_maps_locale` (función pura `map_locale("es-ES")=="es"`), `plan_command_roundtrips_dtos` (serde de MasterDto/PlanDto JSON ↔ struct).
- [ ] **Step 2:** `cargo check` del bridge pasa aquí; `src-tauri` solo `cargo fmt`/lectura; la compilación real ocurre en T5. **Step 3:** commit `feat(app): scaffold Tauri 2 con comandos, ventana y capacidades`.

---

### Task 4: UI real: `backend.js` (mock|tauri), análisis/plan/exportación reales, idioma del sistema

**Files:** Create `app/ui/` (copia de `design/final/` sin `capturas/`, `*.py`, `final.png`), `app/ui/backend.js`, `app/ui/i18n/{es,en}.json`; Modify `app/ui/app.js`, `app/ui/index.html`; Test `app/ui/tests/backend.spec.mjs` (Node 22 `node --test` con el mock) y un `scripts/ui-check.mjs` que carga `index.html` en Playwright con `?backend=mock` y recorre vacío → soltar → crear → listo sin errores de consola.

**Interfaces:**
```js
// backend.js
export const backend = detect(); // window.__TAURI_INTERNALS__ ? tauriBackend : mockBackend; forzable con ?backend=mock|tauri
backend.analyze(path, bins) -> Promise<{master, waveform}>        // tauri: invoke('analyze', {path, bins})
backend.plan(master, meta, activeIds) -> Promise<PlanDto>
backend.export(req, onProgress) -> Promise<DeliveryDto>           // tauri: listen('mastercheck://progress', e => onProgress(e.payload)); invoke('export', {req})
backend.cancelExport() -> Promise<void>
backend.pickFolder() -> Promise<string|null>
backend.openPath(path) -> Promise<void>
backend.systemLang() -> Promise<'es'|'en'>
backend.appInfo() -> Promise<{version, os, lame_available}>
// app.js: el estado S pasa a guardar master (MasterDto), plan (PlanDto), lang; la pastilla usa master.integrated_lufs/true_peak_dbtp con formato por idioma; la onda usa waveform.bins; las fichas se habilitan/deshabilitan con plan.jobs[].enabled y muestran disabled_reason en tooltip; la lista "creando" usa ProgressDto; "listo" usa DeliveryDto (botón "Abrir carpeta" → openPath). Drag & drop en Tauri: onDragDropEvent de @tauri-apps/api/webview (paths) además del HTML5 del mock. Portada: path real (cover_path) → <img src=convertFileSrc(path)>.
// i18n UI: t(key) con es/en; claves de las fichas/estados/botones; detectar lang vía backend.systemLang(); `?lang=` fuerza.
```
- [ ] **Step 1: Tests que fallan** — `backend.spec.mjs`: `mock_analyze_returns_bins_and_master`, `mock_export_emits_progress_then_finished`, `format_lufs_es_uses_comma_and_minus` ("−9,1 LUFS"), `format_lufs_en_uses_dot`; `ui-check.mjs` (Playwright): flujo completo con `?backend=mock&lang=es` y `&lang=en` sin `pageerror`, textos en el idioma correcto, chips deshabilitadas con tooltip cuando el mock devuelve `enabled:false`.
- [ ] **Step 2–5** → commit `feat(ui): adaptador de backend, datos reales del motor e idioma del sistema`. Regenerar capturas con el mock para comprobar que el diseño no se ha roto (mirar 02/03/06/07).

---

### Task 5: Build en GitHub Actions (macOS y Windows) con artefactos descargables

**Files:** Create `.github/workflows/app-build.yml`; Modify `.github/workflows/engine-tests.yml` (añadir el bridge al `cargo test`).

- [ ] **Step 1:** Workflow `app` en `push` a `main`/`hito*` y `workflow_dispatch`: matrix `macos-15` (targets `aarch64-apple-darwin` y `x86_64-apple-darwin` → `universal-apple-darwin` si tauri-action lo soporta sin firma) y `windows-latest`; pasos: checkout, `dtolnay/rust-toolchain@stable` con targets, `Swatinem/rust-cache@v2`, `npm i -g @tauri-apps/cli@^2` (sin package.json en la UI), `cargo tauri build --config app/src-tauri/tauri.<os>.conf.json` (o `tauri-apps/tauri-action@v0` con `projectPath: app`), `actions/upload-artifact@v4` con `app/src-tauri/target/**/bundle/**/*.{dmg,zip,msi,exe}`; `timeout-minutes: 45`; sin firma ni notarización (variables vacías) — esta build es para pruebas internas; en Mac se abre con clic derecho → Abrir (documentarlo en el resumen del run).
- [ ] **Step 2:** Push, vigilar con `gh run watch`/`gh run view`, corregir fallos de compilación de `src-tauri` (es la primera vez que compila) hasta 3 iteraciones, descargar el artefacto de macOS con `gh run download` y comprobar que el `.app` contiene `Contents/MacOS/master check` y `Contents/Info.plist` con `LSMinimumSystemVersion 14.0`.
- [ ] **Step 3:** commit `ci: build de la app para macOS y Windows con artefactos`.

---

### Task 6: Prueba de humo automatizada en macOS y lista de verificación manual para Fer

**Files:** Create `.github/workflows/app-smoke.yml` (o job en app-build), `app/scripts/smoke-mac.sh`, `docs/pruebas/hito2-checklist.md`.

- [ ] **Step 1:** En el job de macOS, tras el build: generar un máster sintético de 10 s (`ffmpeg -f lavfi -i "sine=1000:duration=10" -ar 48000 -ac 2 -c:a pcm_s24le master.wav`), lanzar el binario con la variable `MASTERCHECK_SMOKE=<ruta.wav>;<carpeta>` (modo headless implementado en `src-tauri/src/lib.rs`: si existe la variable, ejecuta analyze+plan+export por el bridge sin abrir ventana y sale con 0/1 imprimiendo el `DeliveryDto`), y comprobar que existen `01 Spotify/… [Spotify] 24bit-48kHz.flac`, `README.txt` y `informe-loudness.txt`.
- [ ] **Step 2:** `docs/pruebas/hito2-checklist.md`: 12 comprobaciones para Fer en su Mac (abrir con clic derecho, soltar máster real, onda, pastilla, activar/desactivar, cambiar carpeta, crear, abrir carpeta, cancelar a mitad, modo oscuro, idioma del sistema en inglés, tamaño mínimo de ventana).
- [ ] **Step 3:** commit `test(app): humo en macOS y checklist de pruebas`.

---

## Autorevisión
- Cobertura: integración motor+UI (T1, T2, T4), ventana = tarjeta y Windows (T3), idioma del sistema (T1, T4), build descargable (T5), revisión final del hito 1 I3/I4/I5 (T2), contrato §4 inglés (T2), prueba en Mac real sin Xcode (T5/T6). Vídeo, Canvas, portada, ADM/afclip, firma/notarización, pasarela y web: hito 3 y 4.
- Tipos: `MasterDto`, `WaveformDto`, `ReleaseMetaDto`, `PlanDto`, `JobDto`, `ProgressDto`, `DeliveryDto`, `BridgeError`, `ExportRequestDto` definidos en T1/T2 y usados con el mismo nombre en T3/T4.
- Review Focus: los cinco puntos tienen test o verificación asignada (T1, T2, T2, T2, T3/T5).
