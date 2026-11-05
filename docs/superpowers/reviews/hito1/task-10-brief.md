### Task 10: Exportación de punta a punta (WAV, FLAC, MP3, documentos)

**Files:**
- Create: `engine/masterout-core/src/mp3.rs`, `engine/masterout-core/src/export.rs`
- Modify: `src/lib.rs` (re-exports)
- Test: `engine/masterout-core/tests/export.rs`

**Interfaces:**
- Consumes: todo lo anterior.
- Produces:
  ```rust
  pub struct ExportRequest { pub master_path: PathBuf, pub meta: ReleaseMeta, pub active_ids: Vec<String>, pub out_root: PathBuf, pub lang: Lang, pub threads: Option<usize> }
  pub enum Progress { Analyzed(MasterInfo), Planned(Vec<Resolved>), JobStarted { preset_id: String }, JobProgress { preset_id: String, fraction: f32 }, JobDone { preset_id: String, path: PathBuf, bytes: u64 }, JobFailed { preset_id: String, error: String }, Finished(Delivery) }
  pub struct Delivery { pub folder: PathBuf, pub files: Vec<(PathBuf, u64)>, pub failed: Vec<(String, String)>, pub elapsed_ms: u128 }
  pub fn export::analyze(master_path: &Path) -> Result<(AudioBuffer, MasterInfo), AudioError>
  pub fn export::run(req: &ExportRequest, on_progress: &(dyn Fn(Progress) + Sync)) -> Result<Delivery, AudioError>
  pub fn mp3::encode(wav_16_44_path: &Path, out: &Path, tags: &Id3Tags, cover: Option<&Path>) -> Result<(), AudioError>  // busca `lame` (PATH o junto al ejecutable) y ejecuta `lame -b 320 --cbr -m j -q 0 --add-id3v2 --id3v2-only --tt … --ta … --tl … --ty … --tc … in.wav out.mp3`; después reescribe/añade frames con el crate `id3` (TSRC, TCOM, TPUB, TCOP, APIC) en ID3v2.3; si no hay `lame` → Err(UnsupportedFormat("lame not found"))
  ```
  Reglas de `run`: carpeta = `out_root/delivery_folder(...)`; si existe, usar `… (2)`, `… (3)`; crear solo subcarpetas con jobs; por job: remuestrear si hace falta → cuantizar (dither según job) → escribir contenedor; paralelo con `rayon` (threads = `req.threads` o núcleos físicos); el MP3 parte de la versión 16/44,1 ya calculada si existe el job de Deezer/distribuidora, si no la genera; al final escribe README.txt, informe-loudness.txt, ficha-lanzamiento.csv, ficha-lanzamiento.txt en la raíz de la entrega; devuelve `Delivery`.

- [ ] **Step 1: Tests que fallan**:
  ```rust
  #[test] fn exports_full_tree_for_synthetic_master() // máster sintético 96/24 de 5 s; active = spotify, apple_adm, deezer, distributor_std, tunecore, mp3_promo (mp3 solo si hay lame) → existen: "01 Spotify/… [Spotify] 24bit-96kHz.flac", "02 Apple Music/… [Apple Digital Masters] 24bit-96kHz.wav", "05 Deezer/… [Deezer] 16bit-44kHz.wav", "07 Distribuidora/… [Distribuidora] 16bit-44kHz.wav", "07 Distribuidora/… [Distribuidora Hi-Res]…" NO (no activo), README.txt, informe-loudness.txt, ficha-lanzamiento.csv, ficha-lanzamiento.txt; ninguna subcarpeta vacía
  #[test] fn written_wavs_have_expected_headers() // probe() de cada WAV: tag 0x0001, bits y sr del job; el de TuneCore no tiene LIST ni bext
  #[test] fn existing_delivery_folder_gets_numbered_suffix() // segunda exportación → "… (2)"
  #[test] fn progress_events_cover_every_job_once() // JobStarted y JobDone por preset activo; Finished al final
  #[test] fn real_master_end_to_end_under_time_budget() // fixture m96_24 (2:22) con 6 destinos → < 60 s en este contenedor (orientativo; el objetivo de la spec es < 30 s en M1 para 10 destinos); loudness del WAV 16/44 exportado difiere < 0,2 LU del máster
  #[test] fn mp3_has_id3v23_frames() // si hay lame: id3::Tag::read_from_path → TIT2, TPE1, TYER, TSRC; versión Id3v23
  ```
- [ ] **Steps 2–5.** Commit `feat(core): exportación completa con progreso y documentos`.

---

