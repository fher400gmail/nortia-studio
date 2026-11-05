### Task 6: FLAC 24 bit con Vorbis comments

**Files:**
- Create: `engine/masterout-core/src/flac.rs`
- Test: `engine/masterout-core/tests/flac.rs`

**Interfaces:**
- Produces:
  ```rust
  pub struct FlacTags { pub title: String, pub artist: String, pub album: Option<String>, pub date: Option<String>, pub isrc: Option<String>, pub copyright: Option<String> }
  pub fn flac::write(path: &Path, samples_i32: &[Vec<i32>], sample_rate: u32, bits: u16, tags: Option<&FlacTags>) -> Result<(), AudioError>
  ```
- Decisión en esta tarea: primero `flacenc` (Rust puro). Criterio de aceptación: el test de 24 bit pasa y `ffmpeg -i out.flac -f null -` decodifica sin error con `bits_per_raw_sample=24`. Si `flacenc` no cumple 24 bit, cambiar a `libflac-sys` (feature `build-flac`) y dejarlo anotado en el commit.

- [ ] **Step 1: Tests que fallan**:
  ```rust
  #[test] fn writes_24bit_flac_decodable_by_ffmpeg() // escribe seno 24 bit 96 kHz; `ffprobe` (si está en PATH; si no, skip) reporta sample_fmt s32, bits_per_raw_sample 24, sample_rate 96000
  #[test] fn roundtrip_is_lossless() // decodificar con `ffmpeg -i out.flac -f s32le -` o con el crate `claxon` (dev-dep) → enteros idénticos a la entrada (desplazados a 24 bit)
  #[test] fn writes_vorbis_comments() // metaflac::Tag::read_from_path → TITLE, ARTIST, DATE, ISRC presentes con los valores dados
  #[test] fn writes_16bit_flac_too()
  ```
- [ ] **Steps 2–5.** Commit `feat(core): FLAC 16/24 bit con Vorbis comments (flacenc|libflac)`.

---

