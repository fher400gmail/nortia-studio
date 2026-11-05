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
