### Task 4: UI real: `backend.js` (mock|tauri), análisis/plan/exportación reales, idioma del sistema

**Files:** Create `app/ui/` (copia de `design/final/` sin `capturas/`, `*.py`, `final.png`), `app/ui/backend.js`, `app/ui/i18n/{es,en}.json`; Modify `app/ui/app.js`, `app/ui/index.html`; Test `app/ui/tests/backend.spec.mjs` (Node 22 `node --test` con el mock) y un `scripts/ui-check.mjs` que carga `index.html` en Playwright con `?backend=mock` y recorre vacío → soltar → crear → listo sin errores de consola.

**Interfaces:**
```js
// backend.js
export const backend = detect(); // window.__TAURI_INTERNALS__ ? tauriBackend : mockBackend; forzable con ?backend=mock|tauri
backend.analyze(path, bins) -> Promise<{master, waveform}>        // tauri: invoke('analyze', {path, bins})
backend.plan(master, meta, activeIds) -> Promise<PlanDto>
backend.export(req, onProgress) -> Promise<DeliveryDto>           // tauri: listen('mastercheck://progress', e => onProgress(e.payload)); invoke('export', {req})
backend.cancelExport() -> Promise<void>
backend.pickFolder() -> Promise<string|null>
backend.openPath(path) -> Promise<void>
backend.systemLang() -> Promise<'es'|'en'>
backend.appInfo() -> Promise<{version, os, lame_available}>
// app.js: el estado S pasa a guardar master (MasterDto), plan (PlanDto), lang; la pastilla usa master.integrated_lufs/true_peak_dbtp con formato por idioma; la onda usa waveform.bins; las fichas se habilitan/deshabilitan con plan.jobs[].enabled y muestran disabled_reason en tooltip; la lista "creando" usa ProgressDto; "listo" usa DeliveryDto (botón "Abrir carpeta" → openPath). Drag & drop en Tauri: onDragDropEvent de @tauri-apps/api/webview (paths) además del HTML5 del mock. Portada: path real (cover_path) → <img src=convertFileSrc(path)>.
// i18n UI: t(key) con es/en; claves de las fichas/estados/botones; detectar lang vía backend.systemLang(); `?lang=` fuerza.
```
- [ ] **Step 1: Tests que fallan** — `backend.spec.mjs`: `mock_analyze_returns_bins_and_master`, `mock_export_emits_progress_then_finished`, `format_lufs_es_uses_comma_and_minus` ("−9,1 LUFS"), `format_lufs_en_uses_dot`; `ui-check.mjs` (Playwright): flujo completo con `?backend=mock&lang=es` y `&lang=en` sin `pageerror`, textos en el idioma correcto, chips deshabilitadas con tooltip cuando el mock devuelve `enabled:false`.
- [ ] **Step 2–5** → commit `feat(ui): adaptador de backend, datos reales del motor e idioma del sistema`. Regenerar capturas con el mock para comprobar que el diseño no se ha roto (mirar 02/03/06/07).

---

