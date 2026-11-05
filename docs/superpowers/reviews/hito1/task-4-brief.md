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

