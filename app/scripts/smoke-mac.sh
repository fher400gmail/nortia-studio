#!/usr/bin/env bash
# Humo headless de master check en macOS. Uso: smoke-mac.sh "<ruta al .app>" "<carpeta de trabajo>"
# Fuerza el idioma a español (LANG) para que los documentos se llamen README.txt e
# informe-loudness.txt aunque el runner de CI esté en inglés.
set -euo pipefail
APP="$1"; WORK="$2"
EXE="$APP/Contents/MacOS/mastercheck-app"
mkdir -p "$WORK"
WAV="$WORK/máster de prueba.wav"
OUT="$WORK/salida humo"
rm -rf "$OUT"

# WAV de 10 s, 24 bit, 48 kHz, estéreo: seno de 1 kHz a -12 dBFS (sin dependencias).
python3 - "$WAV" <<'PY'
import math, struct, sys, wave
rate, secs = 48000, 10
w = wave.open(sys.argv[1], "wb")
w.setnchannels(2); w.setsampwidth(3); w.setframerate(rate)
amp = int(8388607 * 0.25)
buf = bytearray()
for n in range(rate * secs):
    v = int(amp * math.sin(2 * math.pi * 1000 * n / rate))
    b = struct.pack("<i", v)[:3]
    buf += b + b
w.writeframes(bytes(buf)); w.close()
PY
ls -la "$WAV"

set +e
LANG=es_ES.UTF-8 LC_ALL=es_ES.UTF-8 MASTERCHECK_SMOKE="$WAV;$OUT" perl -e 'alarm 60; exec @ARGV' "$EXE"
CODE=$?
set -e
echo "código de salida: $CODE"
test "$CODE" -eq 0

RESULT="$OUT/smoke-result.json"
test -f "$RESULT"
python3 - "$RESULT" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
assert d["cancelled"] is False, "cancelled"
assert d["failed"] == [], d["failed"]
assert d["files"], "sin archivos"
print("entrega:", d["folder"], "-", len(d["files"]), "archivos")
PY

find "$OUT" -type f | sort | tee "$WORK/listado.txt"
find "$OUT" -type f -path "*/01 Spotify/*" -name "* [[]Spotify[]] 24bit-48kHz.flac" | grep -q .
find "$OUT" -type f -name "README.txt" | grep -q .
find "$OUT" -type f -name "informe-loudness.txt" | grep -q .
echo "HUMO OK"
