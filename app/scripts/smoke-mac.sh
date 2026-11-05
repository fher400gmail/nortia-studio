#!/usr/bin/env bash
# Humo headless de master check en macOS. Uso: smoke-mac.sh "<ruta al .app>" "<carpeta de trabajo>"
# Ejecuta el binario real (analizar + planificar + exportar) con portada sintética y comprueba
# [Portada], [Canvas], [YouTube], [MP3 320] y Apple Motion con el ffprobe del propio sidecar
# empaquetado en Contents/MacOS. Variables:
#   ADM=required   exige «0 muestras con clipping» de Apple Digital Masters en el informe
#   (lo pone el job solo si instaló afclip; si no, el informe dirá «no disponible» y hay aviso)
# Fuerza el idioma a español (LANG) para que los documentos se llamen README.txt e
# informe-loudness.txt aunque el runner de CI esté en inglés.
set -euo pipefail
APP="$1"; WORK="$2"
HERE="$(cd "$(dirname "$0")" && pwd)"
EXE="$APP/Contents/MacOS/mastercheck-app"
test -x "$EXE" || { echo "no existe el ejecutable: $EXE"; ls -la "$APP/Contents/MacOS" || true; exit 2; }
# el sidecar empaquetado (externalBin) va junto al ejecutable, sin sufijo de triple
for b in ffmpeg ffprobe; do
  test -x "$APP/Contents/MacOS/$b" || { echo "::error::falta $b junto al ejecutable (externalBin)"; ls -la "$APP/Contents/MacOS"; exit 2; }
  echo "$b: $(lipo -archs "$APP/Contents/MacOS/$b")"
done
mkdir -p "$WORK"
WAV="$WORK/máster de prueba.wav"
COVER="$WORK/portada de prueba.png"
OUT="$WORK/salida humo"
rm -rf "$OUT"

python3 -I "$HERE/smoke_assets.py" wav "$WAV"
python3 -I "$HERE/smoke_assets.py" cover "$COVER"
ls -la "$WAV" "$COVER"

set +e
LANG=es_ES.UTF-8 LC_ALL=es_ES.UTF-8 MASTERCHECK_SMOKE_COVER="$COVER" MASTERCHECK_SMOKE="$WAV;$OUT" "$EXE" &
PID=$!
( sleep 900; kill -9 "$PID" 2>/dev/null ) &
WATCHDOG=$!
wait "$PID"
CODE=$?
kill "$WATCHDOG" 2>/dev/null
set -e
echo "código de salida: $CODE"
test "$CODE" -eq 0

RESULT="$OUT/smoke-result.json"
test -f "$RESULT"
# tiempo de capabilities() y avisos: también al resumen del job si GitHub lo ofrece
python3 -I - "$RESULT" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
c = (d.get("smoke") or {}).get("capabilities") or {}
print("capabilities() en frío (macOS):", c)
PY

find "$OUT" -type f | sort | tee "$WORK/listado.txt"
find "$OUT" -type f -path "*/01 Spotify/*" -name "* [[]Spotify[]] 24bit-48kHz.flac" | grep -q .
python3 -I "$HERE/smoke_verify.py" --out "$OUT" --platform mac --adm "${ADM:-any}"
echo "HUMO OK"
