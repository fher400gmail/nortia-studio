#!/usr/bin/env bash
# Pruebas de aceptación del sidecar recién construido.
# uso: accept.sh <ffmpeg> <ffprobe> <encoder h264: h264_videotoolbox|h264_mf> <out> [workdir]
# Escribe <out>/buildconf.txt. Sale con error si algo no cuadra.
set -euo pipefail
FF=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
FP=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
ENC=${3:?encoder}; OUT=${4:?out}
mkdir -p "$OUT"; OUT=$(cd "$OUT" && pwd)
WORKDIR=${5:-$(mktemp -d)}; mkdir -p "$WORKDIR"; cd "$WORKDIR"
PY=$(command -v python3 || command -v python) || { echo "falta python"; exit 1; }
fail() { echo "FALLO: $*" >&2; exit 1; }

echo "== versión y licencia =="
"$FF" -hide_banner -version | head -n 3
{ echo "### ffmpeg -L"; "$FF" -hide_banner -L; echo; echo "### ffmpeg -buildconf"; "$FF" -hide_banner -buildconf; } > "$OUT/buildconf.txt" 2>&1
cat "$OUT/buildconf.txt"
grep -q -- '--disable-gpl' "$OUT/buildconf.txt" || fail "buildconf sin --disable-gpl"
grep -q -- '--enable-gpl' "$OUT/buildconf.txt" && fail "buildconf con --enable-gpl"
grep -q -- '--enable-nonfree' "$OUT/buildconf.txt" && fail "buildconf con --enable-nonfree"
grep -q -- '--enable-version3' "$OUT/buildconf.txt" && fail "buildconf con --enable-version3"
grep -qi 'Lesser General Public' "$OUT/buildconf.txt" || fail "licencia -L no es LGPL"
ENCLIST=$("$FF" -hide_banner -encoders 2>/dev/null)
grep -q " $ENC " <<<"$ENCLIST" || fail "encoder $ENC no está en el binario"
grep -q " libmp3lame " <<<"$ENCLIST" || fail "libmp3lame no está en el binario"
for banned in libx264 libx265 libfdk_aac; do
  if grep -q " $banned " <<<"$ENCLIST"; then fail "encoder prohibido $banned"; fi
done

echo "== material sintético =="
"$PY" - <<'PYEOF'
import math, struct, wave, zlib
# WAV 8 s, 44,1 kHz, estéreo, 16 bit
sr, secs = 44100, 8
w = wave.open("in.wav", "wb"); w.setnchannels(2); w.setsampwidth(2); w.setframerate(sr)
buf = bytearray()
for i in range(sr * secs):
    v = int(12000 * math.sin(2*math.pi*440*i/sr) + 6000 * math.sin(2*math.pi*1320*i/sr))
    buf += struct.pack("<hh", v, -v)
w.writeframes(bytes(buf)); w.close()
# PNG 1600x1600 degradado
n = 1600
def chunk(t, d):
    c = struct.pack(">I", len(d)) + t + d
    return c + struct.pack(">I", zlib.crc32(t + d) & 0xffffffff)
raw = bytearray()
for y in range(n):
    raw.append(0)
    raw += bytes(((x * 255) // n, (y * 255) // n, ((x + y) * 255) // (2 * n))[k] for x in range(n) for k in range(3))
png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", n, n, 8, 2, 0, 0, 0)) \
    + chunk(b"IDAT", zlib.compress(bytes(raw), 6)) + chunk(b"IEND", b"")
open("in.png", "wb").write(png)
PYEOF
ls -l in.wav in.png

case "$ENC" in
  h264_videotoolbox) EXTRA=(-allow_sw 1) ;;
  *) EXTRA=() ;;
esac
run() { echo "+ ffmpeg $*"; "$FF" -hide_banner -y -loglevel warning "$@"; }

echo "== canvas =="
run -framerate 30 -loop 1 -i in.png -t 6 -an \
  -vf "scale=1080:1920:force_original_aspect_ratio=increase,crop=1080:1920,setsar=1,format=yuv420p" \
  -r 30 -c:v "$ENC" ${EXTRA[@]+"${EXTRA[@]}"} -profile:v high -b:v 5M -movflags +faststart canvas.mp4

echo "== youtube =="
run -framerate 30 -loop 1 -i in.png -i in.wav -t 8 \
  -vf "scale=1920:1080:force_original_aspect_ratio=increase,crop=1920:1080,setsar=1,format=yuv420p" \
  -r 30 -c:v "$ENC" ${EXTRA[@]+"${EXTRA[@]}"} -g 15 -bf 2 -profile:v high -b:v 8M \
  -c:a aac -b:a 384k -ar 48000 -movflags +faststart youtube.mp4

echo "== mp3 =="
run -i in.wav -vn -c:a libmp3lame -b:a 320k out.mp3

for f in canvas.mp4 youtube.mp4 out.mp3; do "$FP" -v error -show_format -show_streams -of json "$f" > "$f.json"; done
"$FP" -v error -show_streams -select_streams v:0 -show_entries stream=profile,has_b_frames,r_frame_rate,avg_frame_rate -of default youtube.mp4 || true

echo "== aserciones =="
"$PY" - <<'PYEOF'
import json, struct, sys
errs = []
def load(f): return json.load(open(f + ".json"))
def chk(c, m):
    print(("ok   " if c else "FAIL ") + m)
    if not c: errs.append(m)
def fps(s):
    n, d = s["r_frame_rate"].split("/"); return float(n) / float(d)
def atoms(path, limit=16):
    out = []
    with open(path, "rb") as fh:
        size_total = fh.seek(0, 2); fh.seek(0)
        while len(out) < limit:
            pos = fh.tell()
            h = fh.read(8)
            if len(h) < 8: break
            size, typ = struct.unpack(">I4s", h); typ = typ.decode("latin1")
            if size == 1: size = struct.unpack(">Q", fh.read(8))[0]
            elif size == 0: size = size_total - pos
            out.append(typ)
            if size < 8: break
            fh.seek(pos + size)
    return out
def vs(j): return [s for s in j["streams"] if s["codec_type"] == "video"]
def au(j): return [s for s in j["streams"] if s["codec_type"] == "audio"]

j = load("canvas.mp4"); v = vs(j)[0]
chk(v["codec_name"] == "h264", "canvas codec h264 (%s)" % v["codec_name"])
chk(v.get("profile") == "High", "canvas profile High (%s)" % v.get("profile"))
chk((v["width"], v["height"]) == (1080, 1920), "canvas 1080x1920 (%sx%s)" % (v["width"], v["height"]))
chk(abs(fps(v) - 30) < 0.01, "canvas 30 fps (%s)" % v["r_frame_rate"])
chk(len(au(j)) == 0 and j["format"]["nb_streams"] == 1, "canvas sin audio, nb_streams=1 (%s)" % j["format"]["nb_streams"])
d = float(j["format"]["duration"]); chk(abs(d - 6) <= 0.1, "canvas duración 6 s (%.3f)" % d)
a = atoms("canvas.mp4"); chk("moov" in a and "mdat" in a and a.index("moov") < a.index("mdat"), "canvas moov antes de mdat %s" % a)

j = load("youtube.mp4"); v = vs(j)[0]; s = au(j)
chk(v["codec_name"] == "h264", "youtube codec h264")
chk(v.get("profile") == "High", "youtube profile High (%s)" % v.get("profile"))
chk((v["width"], v["height"]) == (1920, 1080), "youtube 1920x1080 (%sx%s)" % (v["width"], v["height"]))
chk(abs(fps(v) - 30) < 0.01, "youtube 30 fps (%s)" % v["r_frame_rate"])
chk(len(s) == 1 and s[0]["codec_name"] == "aac", "youtube audio aac")
chk(s and int(s[0]["sample_rate"]) == 48000, "youtube audio 48 kHz")
chk(s and s[0].get("profile") == "LC", "youtube AAC-LC (%s)" % (s[0].get("profile") if s else None))
if s and s[0].get("bit_rate"):
    br = int(s[0]["bit_rate"]); chk(br <= 430000, "youtube audio bitrate <= ~384k (medido %d; el aac nativo no es CBR estricto)" % br)
chk(j["format"]["nb_streams"] == 2, "youtube nb_streams=2")
d = float(j["format"]["duration"]); chk(abs(d - 8) <= 0.1, "youtube duración 8 s (%.3f)" % d)
a = atoms("youtube.mp4"); chk("moov" in a and "mdat" in a and a.index("moov") < a.index("mdat"), "youtube moov antes de mdat %s" % a)

j = load("out.mp3"); s = j["streams"]
chk(len(s) == 1 and s[0]["codec_name"] == "mp3", "mp3 codec mp3, 1 stream")
chk(316800 <= int(j["format"]["bit_rate"]) <= 323200, "mp3 320 kbps CBR, margen 1 por ciento (%s)" % j["format"]["bit_rate"])
d = float(j["format"]["duration"]); chk(abs(d - 8) <= 0.1, "mp3 duración 8 s (%.3f)" % d)

if errs:
    print("\n%d aserciones fallidas" % len(errs)); sys.exit(1)
print("\nTODAS LAS ACEPTACIONES OK")
PYEOF
