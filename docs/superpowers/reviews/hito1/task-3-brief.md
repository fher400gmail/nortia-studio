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

