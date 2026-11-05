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

