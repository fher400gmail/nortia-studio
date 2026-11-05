### Task 2: Escritor WAV (PCM 0x0001, `bext`, `LIST/INFO`)

**Files:**
- Modify: `engine/masterout-core/src/wav.rs`
- Test: `engine/masterout-core/tests/wav_write.rs`

**Interfaces:**
- Consumes: `AudioBuffer`, `AudioError` (Task 1).
- Produces:
  ```rust
  pub struct WavMetadata { pub title: String, pub artist: String, pub album: Option<String>, pub year: Option<u16>, pub bext_description: String, pub bext_originator: String /* "nortia. studio" */, pub origination: Option<chrono-free (u16,u8,u8,u8,u8,u8)> }
  pub struct WavWriteOptions { pub bits: u16 /* 16 | 24 */, pub metadata: Option<WavMetadata> }
  pub fn wav::write(path: &Path, samples_i32: &[Vec<i32>], sample_rate: u32, opts: &WavWriteOptions) -> Result<(), AudioError>
  ```
  (`samples_i32` ya cuantizados por Task 3; `write` solo serializa. Orden de chunks: `fmt `, `bext`?, `LIST`?, `data`.)

- [ ] **Step 1: Tests que fallan** en `tests/wav_write.rs`:
  ```rust
  #[test] fn writes_pcm24_header_0x0001_fmt16() // bytes 20..22 == 0x0001, size de fmt == 16, bits 24, block_align 6, byte_rate sr*6
  #[test] fn roundtrip_pcm16_and_pcm24() // write → read (Task 1) devuelve los mismos enteros (comparar tras re-escalar)
  #[test] fn data_is_last_chunk_and_sizes_consistent() // RIFF size == file_len − 8; data size == frames*block_align; sin chunk tras data
  #[test] fn writes_bext_and_list_info_when_metadata() // existe chunk "bext" de 602 bytes con Description/Originator ASCII rellenos a 0; chunk LIST tipo INFO con INAM, IART, ICRD, ISFT="Master Out"; NO existe sub-chunk "ISRC"
  #[test] fn no_metadata_means_only_fmt_and_data() // ids de chunks == ["fmt ", "data"]
  ```
- [ ] **Step 2: Ejecutar** → FALLA.
- [ ] **Step 3: Implementar** `write` (escritura secuencial con `BufWriter`; `bext` según EBU Tech 3285 v1: Description 256, Originator 32, OriginatorReference 32, OriginationDate 10 "YYYY-MM-DD", OriginationTime 8 "HH:MM:SS", TimeReference u64 0, Version u16 1, UMID 64 bytes 0, loudness fields 5×i16 0, Reserved 180, CodingHistory vacío → 602 bytes).
- [ ] **Step 4: Ejecutar** → PASA.
- [ ] **Step 5: Commit** `feat(core): escritor WAV PCM 0x0001 con bext y LIST/INFO`.

---

