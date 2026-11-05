### Task 8: Presets de destinos y resolución del plan

**Files:**
- Create: `engine/masterout-core/presets.json`, `engine/masterout-core/src/presets.rs`, `engine/masterout-core/src/plan.rs`
- Test: `engine/masterout-core/tests/plan.rs`

**Interfaces:**
- Produces:
  ```rust
  pub enum Container { Wav, Flac, Mp3, Mp4, Jpg, Png }
  pub enum BitsPolicy { Native, Fixed(u16) }      pub enum RatePolicy { Native, Fixed(u32) }
  pub enum Status { Confirmed, Recommendation }
  pub struct Preset { pub id: String /* "spotify" */, pub name: String, pub subfolder: String /* "01 Spotify" */, pub tag: String /* "Spotify" */, pub container: Container, pub bits: BitsPolicy, pub rate: RatePolicy, pub target_lufs: Option<f64>, pub max_tp_dbtp: Option<f64>, pub embed_metadata: bool, pub status: Status, pub source_url: String, pub group: String /* "streaming" | "distributor" | "extras" */, pub default_on: bool }
  pub fn presets::all() -> Vec<Preset>   // desde include_str!("../presets.json")
  pub struct MasterInfo { pub sample_rate: u32, pub bits: u16, pub is_float: bool, pub channels: u16, pub frames: u64, pub loudness: LoudnessReport }
  pub struct ReleaseMeta { pub artist: String, pub title: String, pub version: Option<String>, pub featuring: Option<String>, pub album: Option<String>, pub year: Option<u16>, pub isrc: Option<String>, pub composer: Option<String>, pub label: Option<String>, pub copyright: Option<String>, pub cover: Option<PathBuf> }
  pub enum Resolved { Job(Job), Disabled { preset_id: String, reason_key: String } }
  pub struct Job { pub preset_id: String, pub subfolder: String, pub file_name: String, pub container: Container, pub bits: u16, pub sample_rate: u32, pub needs_resample: bool, pub needs_dither: bool, pub is_upsampling: bool, pub embed_metadata: bool, pub verdict: Verdict }
  pub fn plan::resolve(master: &MasterInfo, meta: &ReleaseMeta, active_ids: &[&str]) -> Vec<Resolved>
  pub fn plan::estimate_bytes(job: &Job, frames: u64, channels: u16) -> u64  // WAV exacto; FLAC ≈ 0,62·WAV; MP3 320 = 40 kB/s
  ```
- `presets.json` (v1, con `source_url` y `status` tomados de `docs/investigacion/`): spotify (Flac, Native, Native, −14, −1, confirmed), apple_adm (Wav, Native con mínimo 24 → si máster 16 bit: Disabled "apple_needs_24bit", Native, None, −1, confirmed), amazon (Flac, Native, Native, −14, −2, recommendation), tidal (Flac, Native, Native, −14, −1, recommendation), deezer (Wav, Fixed 16, Fixed 44100, −15, −1, recommendation), youtube_music (Wav, Native(24), Fixed 48000, −14, −1, recommendation), distributor_std (Wav, Fixed 16, Fixed 44100, None, −1, confirmed CD Baby/DistroKid), distributor_hires (Wav, Native, Native, None, −1, confirmed), tunecore (Wav, Fixed 24, Fixed 192000, embed_metadata false, confirmed), mp3_promo (Mp3, —, Fixed 44100, confirmed), cover (Jpg+Png), canvas (Mp4), youtube_video (Mp4). En este hito los presets `cover`, `canvas`, `youtube_video` existen en el JSON pero `resolve` los devuelve `Disabled { reason_key: "not_in_this_build" }`.

- [ ] **Step 1: Tests que fallan**:
  ```rust
  #[test] fn presets_json_loads_and_has_13_entries_with_sources() // cada preset tiene source_url no vacío
  #[test] fn native_destinations_keep_master_resolution() // máster 96/24 → spotify Job 24/96 flac, needs_resample false, needs_dither false, file_name == "… [Spotify] 24bit-96kHz.flac", subfolder "01 Spotify"
  #[test] fn fixed_destinations_convert_with_dither_when_lowering_bits() // 96/24 → deezer 16/44100, needs_resample true, needs_dither true
  #[test] fn apple_disabled_for_16bit_master() // máster 44,1/16 → Disabled{"apple_adm","apple_needs_24bit"}
  #[test] fn tunecore_marks_upsampling_and_no_metadata() // 48/24 → 24/192, is_upsampling true, embed_metadata false
  #[test] fn never_upsample_native() // máster 44,1/16 → spotify 16/44,1 (no sube a 24 ni a 96)
  #[test] fn verdict_uses_preset_targets() // loudness (−9,1, +1,2) → spotify Fail; youtube_music Warn o Fail según regla; distributor_std con target None y TP +1,2 → Warn("tp_over_zero")
  #[test] fn inactive_ids_are_not_resolved() // active_ids = ["spotify"] → 1 Resolved
  #[test] fn estimate_bytes_wav_exact() // 96k·24bit·2ch·60 s = 34 560 000 B
  ```
- [ ] **Steps 2–5.** Commit `feat(core): presets con fuente y resolución del plan por destino`.

---

