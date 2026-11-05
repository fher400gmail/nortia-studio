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
- `tauri.conf.json`: `productName "master check"`, `identifier "studio.nortia.mastercheck"`, ventana según DISEÑO-FINAL (mac overlay; win sin decoraciones; `label "main"`, `visible:false` y `show()` tras `ready` del frontend), `app.withGlobalTauri: true`, `bundle.targets ["dmg","app","nsis"]`, `bundle.icon` generado con `cargo tauri icon brand/iconos/macos/studio-verde…` → **usar el icono de marca v2** `brand/iconos/macos/studio-claro-1024.png` (o el nombre real en `brand/iconos/macos/`), `bundle.resources` incluye `lame` si existe `app/vendor/lame/<target>/` (vacío por ahora; el MP3 se desactiva con aviso si no hay `lame`), `bundle.macOS.minimumSystemVersion "14.0"`, `bundle.licenseFile` LGPL de lame cuando se incluya.
- `capabilities/default.json`: `core:default`, `core:window:allow-minimize|toggle-maximize|close|start-dragging|set-title`, `dialog:allow-open`, `opener:allow-reveal-item-in-dir`, `os:allow-locale`; `fs` NO (el acceso a disco lo hace Rust con rutas elegidas).
- Config por plataforma: `tauri.macos.conf.json` y `tauri.windows.conf.json` para `decorations/titleBarStyle/transparent/shadow`.
- [ ] **Step 1: Tests que fallan** — `resolve_lame_path_prefers_bundle_dir` (crea `exe_dir/lame` ejecutable falso → lo devuelve), `resolve_lame_path_falls_back_to_path_env`, `system_lang_maps_locale` (función pura `map_locale("es-ES")=="es"`), `plan_command_roundtrips_dtos` (serde de MasterDto/PlanDto JSON ↔ struct).
- [ ] **Step 2:** `cargo check` del bridge pasa aquí; `src-tauri` solo `cargo fmt`/lectura; la compilación real ocurre en T5. **Step 3:** commit `feat(app): scaffold Tauri 2 con comandos, ventana y capacidades`.

---

