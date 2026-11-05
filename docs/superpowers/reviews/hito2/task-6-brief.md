### Task 6: Prueba de humo automatizada en macOS y lista de verificación manual para Fer

**Files:** Create `.github/workflows/app-smoke.yml` (o job en app-build), `app/scripts/smoke-mac.sh`, `docs/pruebas/hito2-checklist.md`.

- [ ] **Step 1:** En el job de macOS, tras el build: generar un máster sintético de 10 s (`ffmpeg -f lavfi -i "sine=1000:duration=10" -ar 48000 -ac 2 -c:a pcm_s24le master.wav`), lanzar el binario con la variable `MASTERCHECK_SMOKE=<ruta.wav>;<carpeta>` (modo headless implementado en `src-tauri/src/lib.rs`: si existe la variable, ejecuta analyze+plan+export por el bridge sin abrir ventana y sale con 0/1 imprimiendo el `DeliveryDto`), y comprobar que existen `01 Spotify/… [Spotify] 24bit-48kHz.flac`, `README.txt` y `informe-loudness.txt`.
- [ ] **Step 2:** `docs/pruebas/hito2-checklist.md`: 12 comprobaciones para Fer en su Mac (abrir con clic derecho, soltar máster real, onda, pastilla, activar/desactivar, cambiar carpeta, crear, abrir carpeta, cancelar a mitad, modo oscuro, idioma del sistema en inglés, tamaño mínimo de ventana).
- [ ] **Step 3:** commit `test(app): humo en macOS y checklist de pruebas`.

---

## Autorevisión
- Cobertura: integración motor+UI (T1, T2, T4), ventana = tarjeta y Windows (T3), idioma del sistema (T1, T4), build descargable (T5), revisión final del hito 1 I3/I4/I5 (T2), contrato §4 inglés (T2), prueba en Mac real sin Xcode (T5/T6). Vídeo, Canvas, portada, ADM/afclip, firma/notarización, pasarela y web: hito 3 y 4.
- Tipos: `MasterDto`, `WaveformDto`, `ReleaseMetaDto`, `PlanDto`, `JobDto`, `ProgressDto`, `DeliveryDto`, `BridgeError`, `ExportRequestDto` definidos en T1/T2 y usados con el mismo nombre en T3/T4.
- Review Focus: los cinco puntos tienen test o verificación asignada (T1, T2, T2, T2, T3/T5).
