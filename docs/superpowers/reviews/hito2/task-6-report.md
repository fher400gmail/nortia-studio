# Informe tarea 6 · humo en macOS y checklist

STATUS: DONE. Commits 01e4d24 (humo + checklist) y el siguiente "test(app): humo sin perl y con comprobación del ejecutable".
Run verde: https://github.com/fher400gmail/nortia-studio/actions/runs/42095911188 (check, macos y windows OK). El primer intento (42094363324) falló en el humo por mi wrapper `perl -e 'alarm 60; exec ...'` (salió 0 sin ejecutar el binario); sustituido por background + watchdog `sleep 60; kill -9`. Sin cambios en el código de la app.

## Humo (job macos, tras la verificación del bundle)
`app/scripts/smoke-mac.sh`: genera con Python (wave, sin instalar nada) un WAV de 10 s, 24 bit, 48 kHz, estéreo en ruta con espacios y acentos ("máster de prueba.wav"); lanza `Contents/MacOS/mastercheck-app` con `LANG/LC_ALL=es_ES.UTF-8` (forzado, el runner está en inglés; así los documentos son README.txt e informe-loudness.txt) y 60 s de límite. Comprueba exit 0, smoke-result.json con cancelled=false, failed vacío, y existencia de `01 Spotify/… [Spotify] 24bit-48kHz.flac`, README.txt, informe-loudness.txt. Artefacto `master-check-humo` (listado.txt + smoke-result.json).
Resultado: exit 0, 8 archivos (Spotify FLAC 667 KB, Apple ADM wav, Deezer 16/44, Distribuidora, README, informe-loudness, ficha csv/txt) en 318 ms; failed=[]. MP3 no se ejercita (sin lame, esperado).

## Checklist
`docs/pruebas/hito2-checklist.md`: 12 pruebas para Fer con resultado esperado y plantilla de respuesta "funciona / falla: qué viste".
