### Task 1: Workspace, `AudioBuffer` y lector WAV

**Files:**
- Create: `engine/Cargo.toml`, `engine/masterout-core/Cargo.toml`, `engine/masterout-core/src/lib.rs`, `engine/masterout-core/src/audio.rs`, `engine/masterout-core/src/wav.rs`
- Test: `engine/masterout-core/tests/wav_read.rs` (+ helper `tests/common/mod.rs` que escribe WAVs sintéticos a un `tempdir`)

**Interfaces:**
- Produces:
  ```rust
  pub struct AudioBuffer { pub sample_rate: u32, pub channels: Vec<Vec<f32>>, pub source_bits: u16, pub source_float: bool }
  impl AudioBuffer { pub fn frames(&self) -> usize; pub fn duration_secs(&self) -> f64; pub fn num_channels(&self) -> usize }
  #[derive(thiserror::Error, Debug)] pub enum AudioError { Io(#[from] std::io::Error), NotRiffWave, MissingFmt, MissingData, UnsupportedFormat(String), TooManyChannels(u16), Rf64Unsupported }
  pub struct WavInfo { pub sample_rate: u32, pub channels: u16, pub bits: u16, pub is_float: bool, pub frames: u64, pub format_tag: u16, pub extra_chunks: Vec<String> }
  pub fn wav::probe(path: &Path) -> Result<WavInfo, AudioError>
  pub fn wav::read(path: &Path) -> Result<AudioBuffer, AudioError>
  ```
- Escala: enteros de 16/24/32 bit se convierten a `f32` dividiendo por 2^(bits−1) (−1.0 ≤ x < 1.0); float32 se copia tal cual (puede superar ±1.0).

- [ ] **Step 1: Crear el workspace** con `engine/Cargo.toml` (`[workspace] members = ["masterout-core"] resolver = "2"`) y el crate (`cargo new --lib masterout-core` dentro de `engine/`). Dependencias iniciales: `thiserror = "2"`, `tempfile = "3"` (dev). Ejecutar `cargo test` → 0 tests, compila.

- [ ] **Step 2: Escribir los tests que fallan** en `tests/wav_read.rs`, con un helper `common::write_wav(path, sr, bits, channels: &[Vec<i32>], opts)` que escribe bytes a mano (sin usar `wav.rs`) y admite `opts.junk_before_fmt: bool`, `opts.extensible: bool`:
  ```rust
  #[test] fn reads_pcm16_stereo_sine() // 48000 Hz, 1 kHz, 0,5 FS, 4800 frames → sample_rate 48000, 2 canales, frames 4800, source_bits 16, pico ≈ 0,5 (±1e-3)
  #[test] fn reads_pcm24_scaled_to_unit() // muestra 0x7FFFFF → 1.0 − 2^-23 (±1e-6); 0x800000 → −1.0
  #[test] fn reads_wav_with_junk_chunk_before_fmt() // JUNK de 64 bytes antes de fmt → se lee igual; extra_chunks contiene "JUNK"
  #[test] fn reads_extensible_24bit() // fmt de 40 bytes, tag 0xFFFE, subformat PCM → format_tag 0xFFFE, bits 24, datos correctos
  #[test] fn reads_float32() // tag 0x0003, valores 1.5 → se conservan (source_float true)
  #[test] fn rejects_more_than_two_channels() // 6 canales → Err(TooManyChannels(6))
  #[test] fn rejects_rf64() // cabecera "RF64" → Err(Rf64Unsupported)
  #[test] fn probe_matches_read() // probe().frames == read().frames()
  #[test] #[ignore_if_no_fixtures] fn reads_real_masters() // para cada fixture: sr/bits esperados (m96_24: 96000/24; m48_24: 48000/24; m44_24 y m44_24_adm: 44100/24), 2 canales, duración ±0,01 s de ffprobe (142,41 / 185,41 / 129,33 / 187,20)
  ```
  (`ignore_if_no_fixtures`: el test devuelve pronto con `eprintln!("skip: MASTEROUT_FIXTURES no definido")` si falta la variable.)

- [ ] **Step 3: Ejecutar** `cargo test --test wav_read` → FALLA (módulos inexistentes).

- [ ] **Step 4: Implementar** `audio.rs` y `wav.rs`: lector por chunks (`RIFF`/`WAVE`, iterar `id`+`size` con padding a par; aceptar `fmt ` de 16/18/40 bytes; `0xFFFE` → leer subformato GUID y tratar como PCM o float según los dos primeros bytes `0x0001`/`0x0003`; `data` puede no ser el último chunk; si `size` de `data` es 0 o 0xFFFFFFFF, usar hasta EOF). Lectura por bloques de 1 MiB, nunca el archivo entero en un `Vec<u8>`.

- [ ] **Step 5: Ejecutar** `cargo test --test wav_read` → PASA; también `MASTEROUT_FIXTURES=/home/claude/testdata/masters cargo test --test wav_read -- --nocapture`.

- [ ] **Step 6: Commit** `feat(core): AudioBuffer y lector WAV robusto (JUNK, EXTENSIBLE, float, RF64 error)`.

---

