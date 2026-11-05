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

