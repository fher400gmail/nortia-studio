### Task 5: Build en GitHub Actions (macOS y Windows) con artefactos descargables

**Files:** Create `.github/workflows/app-build.yml`; Modify `.github/workflows/engine-tests.yml` (añadir el bridge al `cargo test`).

- [ ] **Step 1:** Workflow `app` en `push` a `main`/`hito*` y `workflow_dispatch`: matrix `macos-15` (targets `aarch64-apple-darwin` y `x86_64-apple-darwin` → `universal-apple-darwin` si tauri-action lo soporta sin firma) y `windows-latest`; pasos: checkout, `dtolnay/rust-toolchain@stable` con targets, `Swatinem/rust-cache@v2`, `npm i -g @tauri-apps/cli@^2` (sin package.json en la UI), `cargo tauri build --config app/src-tauri/tauri.<os>.conf.json` (o `tauri-apps/tauri-action@v0` con `projectPath: app`), `actions/upload-artifact@v4` con `app/src-tauri/target/**/bundle/**/*.{dmg,zip,msi,exe}`; `timeout-minutes: 45`; sin firma ni notarización (variables vacías) — esta build es para pruebas internas; en Mac se abre con clic derecho → Abrir (documentarlo en el resumen del run).
- [ ] **Step 2:** Push, vigilar con `gh run watch`/`gh run view`, corregir fallos de compilación de `src-tauri` (es la primera vez que compila) hasta 3 iteraciones, descargar el artefacto de macOS con `gh run download` y comprobar que el `.app` contiene `Contents/MacOS/master check` y `Contents/Info.plist` con `LSMinimumSystemVersion 14.0`.
- [ ] **Step 3:** commit `ci: build de la app para macOS y Windows con artefactos`.

---

