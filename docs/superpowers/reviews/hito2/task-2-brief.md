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

