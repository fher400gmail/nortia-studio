# Master Out · Hito 1 · Motor en Rust — Plan de implementación

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Un crate Rust `masterout-core` que, dado un máster WAV y unos datos de lanzamiento, mide loudness/true peak, decide qué archivo toca por destino, escribe WAV/FLAC/MP3 con nombres y metadatos correctos, y genera README, informe y ficha en el idioma pedido — sin interfaz, con tests.

**Architecture:** Un crate de biblioteca puro (sin Tauri) en `engine/masterout-core`, organizado por módulos con una responsabilidad cada uno (`wav`, `dither`, `loudness`, `resample`, `flac`, `mp3`, `naming`, `presets`, `plan`, `i18n`, `reports`, `export`). Audio interno en `f32` planar (`Vec<Vec<f32>>`, un vector por canal) con `AudioBuffer`. La exportación produce una lista de `Job` (uno por destino) y los ejecuta en paralelo con `rayon`, emitiendo `Progress` por callback. Las piezas solo-macOS (afconvert/afclip) quedan fuera de este hito.

**Tech Stack:** Rust 2021 (stable 1.9x), crates: `ebur128` 0.1 (MIT), `rubato` 0.16/5 (MIT/Apache), `flacenc` 0.5 (Apache-2.0) con plan B `libflac-sys` (BSD-3), `metaflac` 0.2, `id3` 1.x, `rayon`, `serde`+`serde_json`, `thiserror`, `rand` (solo dither), `csv`. MP3 mediante el programa `lame` como proceso aparte (LGPL). Tests con `cargo test`; referencias externas con `ffmpeg -af ebur128` (ya medidas).

**Spec:** `docs/superpowers/specs/2026-11-05-master-out-design.md` (secciones 2, 4, 5, 7 y la adenda del 05/11 14:30). Verificación técnica: `docs/skills-baseline/verificacion-2026-11-05.md`. Investigación de plataformas: `docs/investigacion/`.

## Global Constraints

- El máster nunca se modifica en nivel: ninguna etapa aplica ganancia, limitación ni normalización. Solo conversión de formato/resolución y dither al bajar bits.
- WAV de salida siempre PCM `0x0001` con `fmt ` de 16 bytes (nunca WAVE_FORMAT_EXTENSIBLE), sin chunks de relleno, `data` como último chunk salvo que se escriban `bext`/`LIST` antes de él.
- Dither TPDF solo al bajar de bits (24→16). Nunca al mantener o subir.
- Loudness según ITU-R BS.1770-4/EBU R128: bloques 400 ms, solape 75 %, gate abs −70 LUFS, rel −10 LU; LRA (Tech 3342) ventanas 3 s, paso 100 ms, gate −20 LU, P10–P95; true peak con sobremuestreo 4× (2× basta a 96 kHz; 4× siempre válido). Tolerancias de test: ±0,1 LU (I), ±1 LU (LRA), +0,2/−0,4 dB (TP) frente a Tech 3341/3342; ±0,15 LU y ±0,3 dB frente a ffmpeg en másters reales.
- Regla de resolución por destino (spec §4): destinos "nativa" (Spotify, Apple, Amazon, Tidal, Hi-Res) conservan bits y kHz del máster, nunca suben; destinos "fija" (Deezer 16/44,1, Distribuidora estándar 16/44,1, TuneCore 24/192, YouTube 48 kHz) convierten. Si el máster es 16 bit, el destino Apple Digital Masters queda `Disabled { reason }`.
- Contrato de nombres (spec §4), exacto: carpeta `{Artista} - {Título}{ (Versión)}/`, subcarpetas `01 Spotify/ … 09 Imagen/`, archivo `{Artista} - {Título} [Etiqueta] {bits}bit-{kHz}kHz.{ext}` (p. ej. `Vera Alcón - Luz de Noviembre [Spotify] 24bit-96kHz.flac`); kHz se escribe `44kHz`, `48kHz`, `96kHz`, `192kHz`; caracteres `/ \ : * ? " < > |` → `-`; sin espacios ni puntos finales; acentos se conservan; nombre de archivo ≤ 200 bytes UTF-8.
- Metadatos: WAV lleva `bext` (Description "Artista - Título", Originator "nortia. studio", OriginatorReference vacío, fecha/hora) y `LIST/INFO` (`INAM`, `IART`, `IPRD`, `ICRD` año, `ISFT` "Master Out"); el ISRC NO va en `INFO/ISRC`. FLAC: Vorbis comments `TITLE`, `ARTIST`, `ALBUM`, `DATE`, `ISRC`, `COPYRIGHT`. MP3: ID3v2.3 `TIT2`, `TPE1`, `TALB`, `TYER`, `TSRC`, `TCOM`, `TPUB`, `TCOP`, `APIC` ($03). Preset TuneCore: sin metadatos incrustados.
- Idioma: toda cadena visible al usuario pasa por `i18n::t(lang, key)`; `es` y `en` obligatorios; `Lang::from_system()` elige por `LANG`/locale y cae a `en`.
- Licencias: nada GPL. Sin red.
- Los nombres de fixtures reales no se suben al repo (`*.wav` en `.gitignore`); los tests que los usan se saltan con mensaje si falta `MASTEROUT_FIXTURES`.

## Review Focus

1. WAV con chunk `JUNK`/`LIST`/`bext` antes de `fmt `, o `fmt ` EXTENSIBLE de 40 bytes (Pro Tools, Logic): debe leerse igual. → Task 1 test `reads_wav_with_junk_chunk_before_fmt` y `reads_extensible_24bit`.
2. Máster con picos > 0 dBTP (medidos +1,2 dBTP en másters reales): la medición no satura ni recorta; el informe lo dice. → Task 4 test con fixture `m48_24` (TP +1,2 ±0,3).
3. Artista o título con `/`, `:` o `?` ("AC/DC", "¿Qué?"), o vacíos: nombres válidos y nunca una ruta que escape de la carpeta. → Task 7 tests `sanitizes_path_separators`, `empty_title_falls_back_to_file_stem`.
4. Carpeta de salida que ya contiene una entrega anterior con el mismo nombre: no se sobrescribe en silencio. → Task 10 test `existing_delivery_folder_gets_numbered_suffix` (`… (2)/`).
5. Máster mono o con > 2 canales: mono se acepta (se mide y se exporta mono); > 2 canales se rechaza con error claro. → Task 1 test `rejects_more_than_two_channels`, Task 4 test `measures_mono`.

---

## Estructura de archivos

```
engine/
  Cargo.toml                     # workspace: members = ["masterout-core"]
  masterout-core/
    Cargo.toml
    presets.json                 # datos de destinos (include_str!)
    i18n/es.json  i18n/en.json   # cadenas
    src/lib.rs                   # pub mod … ; pub use tipos principales
    src/audio.rs                 # AudioBuffer, SampleFormat, errores comunes
    src/wav.rs                   # read_wav / write_wav (+ bext, LIST INFO)
    src/dither.rs                # quantize con TPDF
    src/loudness.rs              # measure → LoudnessReport
    src/resample.rs              # resample
    src/flac.rs                  # write_flac (+ Vorbis comments)
    src/mp3.rs                   # encode_mp3 vía `lame` + ID3
    src/naming.rs                # sanitize, file_name, folder_name, subfolder
    src/presets.rs               # Destination, Preset, carga de presets.json
    src/plan.rs                  # resolve(master, meta, active) → Vec<Job> / Disabled
    src/i18n.rs                  # Lang, t()
    src/reports.rs               # readme, informe-loudness, ficha csv/txt
    src/export.rs                # run(plan, out_dir, progress) → Delivery
    tests/fixtures/              # WAVs sintéticos pequeños generados por tests (tmp) — nada binario en git
    tests/references.json        # loudness de referencia de los másters reales (ffmpeg)
```

Fixtures reales (fuera del repo): `/home/claude/testdata/masters/{m44_24,m48_24,m96_24,m44_24_adm}.wav`; variable `MASTEROUT_FIXTURES=/home/claude/testdata/masters`. Referencias ffmpeg (ebur128, peak=true): m44_24 I −6,4 LRA 3,5 TP +0,1 · m48_24 I −9,1 LRA 5,5 TP +1,2 · m96_24 I −6,2 LRA 3,5 TP 0,0 · m44_24_adm I −8,1 LRA 6,8 TP −1,0.

Cargo usa el proxy de la sesión (`~/.cargo/config.toml` ya configurado). Commit tras cada tarea con mensaje `feat(core): …` o `test(core): …`.

---

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

### Task 3: Cuantización y dither TPDF

**Files:**
- Create: `engine/masterout-core/src/dither.rs`
- Test: `engine/masterout-core/tests/dither.rs`

**Interfaces:**
- Produces:
  ```rust
  pub enum Dither { None, Tpdf { seed: u64 } }
  pub fn quantize(channels: &[Vec<f32>], target_bits: u16, source_bits: u16, source_float: bool) -> (Vec<Vec<i32>>, Dither)  // elige TPDF solo si target_bits < source_bits o source_float; redondeo al más cercano; clamp a [−2^(b−1), 2^(b−1)−1]
  pub fn quantize_with(channels: &[Vec<f32>], target_bits: u16, dither: Dither) -> Vec<Vec<i32>>
  ```
  TPDF = suma de dos uniformes en [−0,5, 0,5) LSB (amplitud pico ±1 LSB), RNG determinista (`rand` `StdRng::seed_from_u64`), independiente por canal.

- [ ] **Step 1: Tests que fallan**:
  ```rust
  #[test] fn same_bits_no_dither_is_exact() // 24→24 de valores que ya son múltiplos de 2^-23 → enteros exactos, Dither::None
  #[test] fn downgrade_24_to_16_uses_tpdf() // devuelve Dither::Tpdf
  #[test] fn upgrade_16_to_24_no_dither() // Dither::None y valores exactos (x·2^23)
  #[test] fn tpdf_noise_statistics() // señal constante 0,25 a 16 bit con TPDF, 200k muestras: media del error ∈ [−0,02, 0,02] LSB; RMS del error ∈ [0,38, 0,44] LSB (teórico √(1/6+1/12)=0,50 para TPDF+redondeo → usar [0,45, 0,55])
  #[test] fn clamps_overs() // 1,5 a 16 bit → 32767; −1,5 → −32768
  #[test] fn deterministic_with_seed() // dos llamadas con seed 7 → idénticas
  ```
- [ ] **Step 2: Ejecutar** → FALLA. **Step 3: Implementar.** **Step 4: Ejecutar** → PASA (ajusta el intervalo del RMS al valor teórico de tu implementación y documenta el cálculo en un comentario). **Step 5: Commit** `feat(core): cuantización con dither TPDF solo al bajar bits`.

---

### Task 4: Medición de loudness (BS.1770-4 / R128) con `ebur128`

**Files:**
- Create: `engine/masterout-core/src/loudness.rs`, `engine/masterout-core/tests/references.json`
- Test: `engine/masterout-core/tests/loudness.rs`

**Interfaces:**
- Consumes: `AudioBuffer`.
- Produces:
  ```rust
  pub struct LoudnessReport { pub integrated_lufs: f64, pub lra_lu: f64, pub true_peak_dbtp: f64, pub sample_peak_dbfs: f64, pub momentary_max_lufs: f64, pub short_term_max_lufs: f64, pub overs_count: u64 /* muestras de true peak > 0 dBTP */ }
  pub fn measure(buf: &AudioBuffer) -> Result<LoudnessReport, AudioError>
  pub fn verdict(report: &LoudnessReport, target_lufs: f64, max_tp_dbtp: f64) -> Verdict  // enum Verdict { Ok, Warn(String key), Fail(String key) } con claves i18n: "tp_over_platform", "louder_than_target", "quieter_than_target"
  ```
  Implementación: `ebur128::EbuR128::new(ch, sr, Mode::I | Mode::LRA | Mode::TRUE_PEAK | Mode::SAMPLE_PEAK | Mode::M | Mode::S)`, alimentar con `add_frames_planar_f32` en bloques de 4096 frames; `overs_count` = número de bloques de 100 ms cuyo true peak > 0 dBTP (documentarlo así en el informe: "tramos de 100 ms").

- [ ] **Step 1: Tests que fallan** (señales sintéticas generadas en el test, 48 kHz salvo indicación):
  ```rust
  #[test] fn tech3341_case1_stereo_minus23_dbfs_is_minus23_lufs() // seno 1 kHz, −23 dBFS pico en L y R, 20 s → I = −23,0 ±0,1
  #[test] fn tech3341_case2_minus33() // −33 dBFS → −33,0 ±0,1
  #[test] fn tech3341_true_peak_minus6() // seno 997 Hz a −6 dBFS (fase tal que el pico cae entre muestras) → TP −6,0 (+0,2/−0,4)
  #[test] fn true_peak_of_full_scale_sine_between_samples_is_positive() // seno 11025 Hz a 0 dBFS en 44,1 kHz con fase π/4 → TP > +0,5 dBTP (inter-sample over)
  #[test] fn tech3342_case1_lra_is_10() // 1 kHz −20 dBFS 20 s + −30 dBFS 20 s → LRA 10 ±1
  #[test] fn gating_ignores_silence() // 20 s de −23 dBFS + 20 s de silencio digital → I = −23,0 ±0,1
  #[test] fn measures_mono() // mono −23 dBFS → I = −26,0 ±0,1 (un canal: −3,01 LU respecto al par)
  #[test] fn works_at_96k_and_44k() // mismo seno a 96000 y 44100 → mismas I ±0,1
  #[test] fn verdict_rules() // (−9,1, TP +1,2) con target −14 / max −1 → Fail("tp_over_platform"); (−14,0, −1,5) → Ok; (−18, −2) → Warn("quieter_than_target")
  #[test] fn real_masters_match_ffmpeg() // si hay fixtures: comparar con tests/references.json (I ±0,15; LRA ±0,5; TP ±0,3)
  ```
  `references.json`: `{"m44_24":{"i":-6.4,"lra":3.5,"tp":0.1},"m48_24":{"i":-9.1,"lra":5.5,"tp":1.2},"m96_24":{"i":-6.2,"lra":3.5,"tp":0.0},"m44_24_adm":{"i":-8.1,"lra":6.8,"tp":-1.0}}`.
- [ ] **Step 2: Ejecutar** → FALLA. **Step 3: Implementar.** **Step 4: Ejecutar** (con y sin fixtures) → PASA. **Step 5: Commit** `feat(core): medición R128 con ebur128 y veredicto por destino`.

---

### Task 5: Remuestreo

**Files:**
- Create: `engine/masterout-core/src/resample.rs`
- Test: `engine/masterout-core/tests/resample.rs`

**Interfaces:**
- Produces: `pub fn resample(buf: &AudioBuffer, target_rate: u32) -> Result<AudioBuffer, AudioError>` (si `target_rate == buf.sample_rate` devuelve clon; usa `rubato::SincFixedIn` con `SincInterpolationParameters { sinc_len: 256, f_cutoff: 0.95, oversampling_factor: 256, interpolation: Cubic, window: BlackmanHarris2 }`, procesando en bloques y compensando la latencia para que `frames_out == round(frames_in * target/source)` ±2).

- [ ] **Step 1: Tests que fallan**:
  ```rust
  #[test] fn length_matches_ratio() // 96000→44100, 96000 frames → 44100 ±2
  #[test] fn preserves_tone_level_and_frequency() // seno 1 kHz −6 dBFS 96k→44.1k: RMS ±0,1 dB; frecuencia estimada por cruces por cero 1000 ±1 Hz
  #[test] fn upsample_44_to_192_is_allowed() // 44100→192000 longitud ×4,353 ±2
  #[test] fn same_rate_is_identity() // puntero/valores idénticos
  #[test] fn no_gain_change_on_real_master() // fixture m96_24 → 44,1: loudness integrado (Task 4) difiere < 0,1 LU
  ```
- [ ] **Steps 2–5** como en las tareas anteriores. Commit `feat(core): remuestreo con rubato sin cambio de nivel`.

---

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

### Task 7: Contrato de nombres

**Files:**
- Create: `engine/masterout-core/src/naming.rs`
- Test: `engine/masterout-core/tests/naming.rs`

**Interfaces:**
- Produces:
  ```rust
  pub fn sanitize_component(s: &str) -> String   // reemplaza / \ : * ? " < > | y controles por '-', colapsa espacios, recorta espacios y puntos finales, limita a 200 bytes sin partir un carácter
  pub fn khz_label(sample_rate: u32) -> String   // 44100→"44kHz", 48000→"48kHz", 88200→"88kHz", 96000→"96kHz", 176400→"176kHz", 192000→"192kHz"
  pub fn delivery_folder(artist: &str, title: &str, version: Option<&str>) -> String   // "Artista - Título" o "Artista - Título (Versión)"
  pub fn audio_file_name(artist: &str, title: &str, tag: &str, bits: u16, sample_rate: u32, ext: &str) -> String  // "Artista - Título [Tag] 24bit-96kHz.flac"
  pub fn media_file_name(artist: &str, title: &str, tag: &str, suffix: &str, ext: &str) -> String  // "Artista - Título [Portada] 3000x3000.jpg", "[YouTube] 1080p.mp4", "[MP3 320].mp3" (suffix vacío → sin espacio extra)
  ```
- [ ] **Step 1: Tests que fallan** (valores exactos del contrato):
  ```rust
  #[test] fn spotify_example_matches_spec() // ("Vera Alcón","Luz de Noviembre","Spotify",24,96000,"flac") == "Vera Alcón - Luz de Noviembre [Spotify] 24bit-96kHz.flac"
  #[test] fn deezer_example_44k() // == "Vera Alcón - Luz de Noviembre [Deezer] 16bit-44kHz.wav"
  #[test] fn folder_with_version() // ("Vera Alcón","Luz de Noviembre",Some("Acústico")) == "Vera Alcón - Luz de Noviembre (Acústico)"
  #[test] fn sanitizes_path_separators() // artist "AC/DC", title "¿Qué?: remix" → "AC-DC - ¿Qué-- remix [Tidal] 24bit-48kHz.flac" (regla: cada carácter prohibido → '-', sin colapsar guiones)
  #[test] fn trims_trailing_dots_and_spaces() // "Título..." → "Título"
  #[test] fn keeps_accents_and_non_latin() // "Björk – Jóga 東京" se conserva
  #[test] fn limits_length_to_200_bytes_without_splitting_utf8()
  #[test] fn empty_title_falls_back_to_file_stem() // naming::fallback_title("", Path("Mi Cancion (master v3).wav")) == "Mi Cancion"  (quita sufijos entre paréntesis que contengan "master", "mix", "v\d+")
  #[test] fn media_examples() // "[Portada] 3000x3000.jpg", "[Canvas] 1080x1920.mp4", "[MP3 320].mp3"
  ```
- [ ] **Steps 2–5.** Commit `feat(core): contrato de nombres de carpeta y archivos`.

---

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

### Task 9: i18n y documentos (README, informe, ficha)

**Files:**
- Create: `engine/masterout-core/i18n/es.json`, `engine/masterout-core/i18n/en.json`, `engine/masterout-core/src/i18n.rs`, `engine/masterout-core/src/reports.rs`
- Test: `engine/masterout-core/tests/reports.rs`

**Interfaces:**
- Produces:
  ```rust
  #[derive(Clone, Copy)] pub enum Lang { Es, En }
  impl Lang { pub fn from_tag(tag: &str) -> Lang /* "es-ES"→Es, "en"→En, otro→En */; pub fn from_system() -> Lang /* LANG/LC_ALL; en Mac/Windows lo pasará la app */ }
  pub fn i18n::t(lang: Lang, key: &str) -> String  // falta la clave → devuelve "⟦key⟧" y lo registra; test exige que es.json y en.json tengan EXACTAMENTE las mismas claves
  pub fn i18n::tf(lang: Lang, key: &str, args: &[(&str, &str)]) -> String // sustituye {name}
  pub struct DeliverySummary<'a> { pub meta: &'a ReleaseMeta, pub master: &'a MasterInfo, pub jobs: &'a [Resolved], pub created_at: String /* ISO local */, pub app_version: &'a str }
  pub fn reports::readme(lang: Lang, s: &DeliverySummary) -> String
  pub fn reports::loudness_report(lang: Lang, s: &DeliverySummary) -> String
  pub fn reports::release_sheet_csv(s: &DeliverySummary) -> String   // cabeceras en inglés fijas (las distribuidoras las leen): artist,title,version,featuring,album,year,isrc,composer,label,copyright,duration_sec,sample_rate,bits,integrated_lufs,true_peak_dbtp,lra_lu
  pub fn reports::release_sheet_txt(lang: Lang, s: &DeliverySummary) -> String
  ```
  Contenido fijado: el README empieza con la línea `{Artista} - {Título}` y después, por cada subcarpeta, una línea `01 Spotify/  →  …flac  (24 bit · 96 kHz · FLAC)` y la frase de i18n `readme.master_untouched` ("Tu máster no se ha modificado. Master Out mide y avisa."). El informe incluye I, LRA, TP, sample peak, overs y una línea por destino con el veredicto. Nada de frases largas.

- [ ] **Step 1: Tests que fallan**:
  ```rust
  #[test] fn es_and_en_have_identical_key_sets()
  #[test] fn missing_key_is_visible_not_panic() // t(Es,"nope") == "⟦nope⟧"
  #[test] fn lang_from_tag() // "es-MX"→Es, "EN_us"→En, "fr"→En
  #[test] fn readme_lists_each_active_job_once_in_order() // 3 jobs → 3 líneas con "01 Spotify/", "05 Deezer/", "08 Promo/" en ese orden; contiene t(lang,"readme.master_untouched")
  #[test] fn readme_marks_upsampling_and_disabled() // tunecore → contiene t("readme.upsampling"); apple Disabled → contiene t("readme.disabled") + motivo
  #[test] fn loudness_report_has_numbers_with_comma_in_es_and_dot_in_en() // "−9,1 LUFS" en es, "-9.1 LUFS" en en
  #[test] fn csv_has_fixed_header_and_one_row() // primera línea exacta; campos vacíos permitidos; comas escapadas
  #[test] fn release_sheet_txt_in_both_languages_no_missing_keys() // no contiene "⟦"
  ```
- [ ] **Steps 2–5.** Commit `feat(core): i18n es/en y documentos de entrega`.

---

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

### Task 11: CI en GitHub Actions (Linux, macOS, Windows)

**Files:**
- Create: `.github/workflows/engine-tests.yml`

- [ ] **Step 1:** Workflow `on: [push, pull_request]`, `paths: engine/**`; matrix `ubuntu-latest`, `macos-latest`, `windows-latest`; `dtolnay/rust-toolchain@stable`; `Swatinem/rust-cache@v2`; `cargo test --manifest-path engine/Cargo.toml --all-features` (sin fixtures); `cargo clippy -- -D warnings`; `cargo fmt --check`. En macOS y Linux instalar `lame` (`brew install lame` / no disponible en apt del contenedor → en CI `sudo apt-get install -y lame`), en Windows `choco install lame` si existe; si no, los tests de MP3 se saltan.
- [ ] **Step 2:** Push y comprobar en `gh run list`/`gh run view` que las tres plataformas pasan. Si Windows falla por rutas o CRLF, corregir en el código, no en el test.
- [ ] **Step 3: Commit** `ci: tests del motor en Linux, macOS y Windows`.

---

## Autorevisión (hecha al escribir el plan)

- Cobertura de la spec: §4 contrato → T7/T8/T10; §5 motor (lectura, dither, medición, FLAC, metadatos, paralelismo) → T1–T6, T10; MP3 → T10; vídeo/portada/Canvas/afclip → fuera de este hito (hito 3), presets presentes pero `not_in_this_build`; §7 calidad (TDD, 3 másters reales, rendimiento) → tests con fixtures y T10; adenda (idioma) → T9; adenda (JUNK, picos > 0) → T1/T4.
- Consistencia de tipos: `AudioBuffer`, `AudioError`, `LoudnessReport`, `Verdict`, `MasterInfo`, `ReleaseMeta`, `Resolved`, `Job`, `Lang`, `Progress`, `Delivery` se definen una vez y se reutilizan con el mismo nombre.
- Review Focus: los cinco puntos tienen test asignado (T1, T4, T7, T10, T1/T4).
- Proporción: el plan fija interfaces y tests; los cuerpos los escribe el implementador.
