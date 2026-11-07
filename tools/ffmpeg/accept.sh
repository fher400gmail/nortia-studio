#!/usr/bin/env bash
# Pruebas de aceptación del sidecar recién construido.
# uso: accept.sh <ffmpeg> <ffprobe> <encoder h264: h264_videotoolbox|h264_mf> <out> [workdir]
# Variables opcionales: ACCEPT_ARCH=x86_64|arm64 (ejecuta bajo `arch -<x>`; buildconf-<arch>.txt)
# Escribe <out>/buildconf[-<arch>].txt. Sale con error si algo no cuadra.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
source "$HERE/common.sh"
FF=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
FP=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
ENC=${3:?encoder}; OUT=${4:?out}
mkdir -p "$OUT"; OUT=$(cd "$OUT" && pwd)
WORKDIR=${5:-$(mktemp -d)}; mkdir -p "$WORKDIR"; cd "$WORKDIR"
RUN=(); BCNAME=buildconf.txt
if [ -n "${ACCEPT_ARCH:-}" ]; then RUN=(arch "-$ACCEPT_ARCH"); BCNAME="buildconf-$ACCEPT_ARCH.txt"; fi
ff() { ${RUN[@]+"${RUN[@]}"} "$FF" "$@"; }
fp() { ${RUN[@]+"${RUN[@]}"} "$FP" "$@"; }
PY=$(command -v python3 || command -v python) || { echo "falta python"; exit 1; }
fail() { echo "FALLO: $*" >&2; exit 1; }

echo "== versión y licencia =="
ff -hide_banner -version | head -n 3
{ echo "### ffmpeg -L"; ff -hide_banner -L; echo; echo "### ffmpeg -buildconf"; ff -hide_banner -buildconf; } > "$OUT/$BCNAME" 2>&1
cat "$OUT/$BCNAME"
grep -q -- '--disable-gpl' "$OUT/$BCNAME" || fail "buildconf sin --disable-gpl"
grep -q -- '--enable-gpl' "$OUT/$BCNAME" && fail "buildconf con --enable-gpl"
grep -q -- '--enable-nonfree' "$OUT/$BCNAME" && fail "buildconf con --enable-nonfree"
grep -q -- '--enable-version3' "$OUT/$BCNAME" && fail "buildconf con --enable-version3"
grep -qi 'Lesser General Public' "$OUT/$BCNAME" || fail "licencia -L no es LGPL"
ENCLIST=$(ff -hide_banner -encoders 2>/dev/null)
grep -q " $ENC " <<<"$ENCLIST" || fail "encoder $ENC no está en el binario"
grep -q " libmp3lame " <<<"$ENCLIST" || fail "libmp3lame no está en el binario"
for banned in libx264 libx265 libfdk_aac; do
  if grep -q " $banned " <<<"$ENCLIST"; then fail "encoder prohibido $banned"; fi
done

ff -hide_banner -h "encoder=$ENC" || true

echo "== inventario de componentes (lista blanca) =="
HEVC_ENC=${ENC/h264/hevc}
export ALLOW_ENC="$ALLOW_ENCODERS_BASE,$ENC,$HEVC_ENC"
ff -hide_banner -filters > inv.filters 2>&1
ff -hide_banner -encoders > inv.encoders 2>&1
ff -hide_banner -decoders > inv.decoders 2>&1
ff -hide_banner -muxers > inv.muxers 2>&1
ff -hide_banner -demuxers > inv.demuxers 2>&1
ff -hide_banner -bsfs > inv.bsfs 2>&1
ff -hide_banner -protocols > inv.protocols 2>&1
ff -hide_banner -devices 2>&1 | grep -q " lavfi" || fail "indev lavfi ausente"
export ALLOW_FILTERS ALLOW_DECODERS ALLOW_MUXERS ALLOW_DEMUXERS ALLOW_BSFS ALLOW_PROTOCOLS
"$PY" - <<'PYEOF'
import os, re, sys
def names(kind):
    out = set()
    for line in open("inv." + kind, errors="replace"):
        line = line.rstrip("\n")
        if kind == "bsfs":
            if re.fullmatch(r"[a-z0-9_]+", line.strip()): out.add(line.strip())
            continue
        if kind == "protocols":
            continue
        if kind in ("muxers", "demuxers"):
            toks = line.split()
            while toks and len(toks[0]) <= 2 and toks[0] != "--": toks = toks[1:]  # flags D/E/d
            if not toks or line.lstrip().startswith(("D.", ".E", "..", "-", "File", "Formats")): continue
            out.add(toks[0].split(",")[0]); continue
        m = re.match(r"^\s+([A-Z.|]{1,6})\s+(\S+)", line)
        if not m or m.group(2) == "=": continue
        out.add(m.group(2).split(",")[0])
    return out
def protocols():
    sec, got = None, {"Input": set(), "Output": set()}
    for line in open("inv.protocols", errors="replace"):
        l = line.strip()
        if l.endswith(":") and l[:-1] in got: sec = l[:-1]; continue
        if sec and l: got[sec].add(l)
    return got["Input"] | got["Output"]
# Implícitos de libavfilter, siempre compilados (no dependen de --enable-filter).
# + hflip/vflip/transpose/rotate: los arrastra el propio ejecutable ffmpeg (autorotate); son LGPL.
IMPLICIT_FILTERS = {"abuffer", "abuffersink", "buffer", "buffersink", "hflip", "vflip", "transpose", "rotate"}
# vp8: decodificador LGPL que arrastra la dependencia de matroska/vp9 en configure.
IMPLICIT_DECODERS = {"vp8"}
checks = [
  ("filters",  names("filters"),  set(os.environ["ALLOW_FILTERS"].split(",")) | IMPLICIT_FILTERS),
  ("encoders", names("encoders"), set(os.environ["ALLOW_ENC"].split(","))),
  ("decoders", names("decoders"), set(os.environ["ALLOW_DECODERS"].split(",")) | IMPLICIT_DECODERS),
  ("muxers",   names("muxers"),   set(os.environ["ALLOW_MUXERS"].split(","))),
  ("demuxers", names("demuxers"), {d.replace("image_", "") for d in os.environ["ALLOW_DEMUXERS"].split(",")} | {"lavfi"}),
  ("bsfs",     names("bsfs"),     set(os.environ["ALLOW_BSFS"].split(","))),
  ("protocols", protocols(),      set(os.environ["ALLOW_PROTOCOLS"].split(","))),
]
bad = False
for kind, got, want in checks:
    miss, extra = sorted(want - got), sorted(got - want)
    print("%-9s esperados=%s" % (kind, ",".join(sorted(want))))
    print("%-9s presentes =%s" % (kind, ",".join(sorted(got))))
    if miss or extra:
        bad = True
        print("FALLO inventario %s: faltan=%s sobran=%s" % (kind, miss, extra))
        print(open("inv." + kind, errors="replace").read()[:4000])
if bad: sys.exit(1)
print("inventario ok")
PYEOF

echo "== material sintético =="
"$PY" - <<'PYEOF'
import random, struct, wave, zlib
# WAV 8 s, 44,1 kHz, estéreo, 16 bit
sr, secs = 44100, 8
w = wave.open("in.wav", "wb"); w.setnchannels(2); w.setsampwidth(2); w.setframerate(sr)
buf = bytearray()
# Ruido blanco determinista, canales independientes (obliga al AAC a gastar el bitrate pedido).
rng = random.Random(1234)
for i in range(sr * secs):
    buf += struct.pack("<hh", rng.randint(-9000, 9000), rng.randint(-9000, 9000))
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
  h264_videotoolbox) EXTRA=(-allow_sw 1); PROFILE=high ;;
  # h264_mf no define la constante "high"; se usa el valor numérico AV_PROFILE_H264_HIGH (100)
  h264_mf) EXTRA=(); PROFILE=100 ;;
  *) EXTRA=(); PROFILE=high ;;
esac
run() { echo "+ ffmpeg $*"; ff -hide_banner -y -loglevel warning "$@"; }

echo "== canvas =="
run -framerate 30 -loop 1 -i in.png -t 6 -an \
  -vf "scale=1080:1920:force_original_aspect_ratio=increase,crop=1080:1920,setsar=1,format=yuv420p" \
  -r 30 -c:v "$ENC" ${EXTRA[@]+"${EXTRA[@]}"} -profile:v "$PROFILE" -b:v 5M -movflags +faststart canvas.mp4

echo "== youtube =="
run -framerate 30 -loop 1 -i in.png -i in.wav -t 8 \
  -vf "scale=1920:1080:force_original_aspect_ratio=increase,crop=1920:1080,setsar=1,format=yuv420p" \
  -r 30 -c:v "$ENC" ${EXTRA[@]+"${EXTRA[@]}"} -g 15 -bf 2 -profile:v "$PROFILE" -b:v 8M \
  -c:a aac -b:a 384k -ar 48000 -movflags +faststart youtube.mp4

echo "== mp3 =="
run -i in.wav -vn -c:a libmp3lame -b:a 320k out.mp3

echo "== canvas con zoompan + cadena de color real (setparams) =="
MOV="+faststart+negative_cts_offsets"
run -framerate 30 -loop 1 -i in.png -t 6 -an \
  -vf "scale=1080:1920:force_original_aspect_ratio=increase,crop=1080:1920,zoompan=z='min(zoom+0.0008,1.1)':d=1:x='iw/2-(iw/zoom/2)':y='ih/2-(ih/zoom/2)':s=1080x1920:fps=30,setsar=1,scale=out_color_matrix=bt709:out_range=tv,format=yuv420p,setparams=colorspace=bt709:color_primaries=bt709:color_trc=bt709:range=tv" \
  -r 30 -c:v "$ENC" ${EXTRA[@]+"${EXTRA[@]}"} -profile:v "$PROFILE" -b:v 5M -movflags "$MOV" -use_editlist 0 canvas_zoom.mp4

echo "== youtube con split + gblur + overlay (grafo real) =="
run -framerate 30 -loop 1 -i in.png -i in.wav -t 8 \
  -filter_complex "[0:v]setpts=PTS-STARTPTS,split=2[bg][fg];[bg]scale=1920:1080:force_original_aspect_ratio=increase,crop=1920:1080,gblur=sigma=30[b];[fg]scale=1080:1080[f];[b][f]overlay=(W-w)/2:(H-h)/2,fade=t=in:st=0:d=0.5,setsar=1,scale=out_color_matrix=bt709:out_range=tv,format=yuv420p,setparams=colorspace=bt709:color_primaries=bt709:color_trc=bt709:range=tv[v]" \
  -map "[v]" -map 1:a -r 30 -c:v "$ENC" ${EXTRA[@]+"${EXTRA[@]}"} -g 15 -bf 2 -profile:v "$PROFILE" -b:v 8M \
  -c:a aac -b:a 384k -ar 48000 -movflags "$MOV" -use_editlist 0 youtube_graph.mp4

echo "== HEVC 3840x3840 (motion artwork) =="
HEVC_OK=1
if ! run -framerate 24 -loop 1 -i in.png -t 2 -an \
  -vf "scale=3840:3840,setsar=1,format=yuv420p" -r 24 -c:v "$HEVC_ENC" ${EXTRA[@]+"${EXTRA[@]}"} -b:v 45M -tag:v hvc1 -movflags "$MOV" -use_editlist 0 hevc.mp4; then
  HEVC_OK=0
  case "$ENC" in
    h264_mf) echo "AVISO: $HEVC_ENC no pudo codificar (la extensión HEVC de Windows no está garantizada en el runner); no es fatal en Windows" ;;
    *) fail "$HEVC_ENC no pudo codificar 3840x3840" ;;
  esac
fi
export HEVC_OK

for f in canvas.mp4 canvas_zoom.mp4 youtube.mp4 youtube_graph.mp4 out.mp3; do fp -v error -show_format -show_streams -of json "$f" > "$f.json"; done
[ "${HEVC_OK:-0}" = 1 ] && fp -v error -show_format -show_streams -of json hevc.mp4 > hevc.mp4.json || true
fp -v error -show_streams -select_streams v:0 -show_entries stream=profile,has_b_frames,r_frame_rate,avg_frame_rate -of default youtube.mp4 || true

echo "== aserciones =="
"$PY" - <<'PYEOF'
import json, os, struct, sys
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
CONT = {"moov", "trak", "edts", "mdia", "minf", "stbl"}
def boxes(path):
    """Cajas anidadas (ruta tipo moov/trak/edts/elst)."""
    found = []
    data = open(path, "rb").read()
    def walk(start, end, prefix):
        pos = start
        while pos + 8 <= end:
            size, typ = struct.unpack(">I4s", data[pos:pos+8]); typ = typ.decode("latin1"); hdr = 8
            if size == 1: size = struct.unpack(">Q", data[pos+8:pos+16])[0]; hdr = 16
            elif size == 0: size = end - pos
            if size < hdr: break
            found.append(prefix + typ)
            if typ in CONT: walk(pos + hdr, pos + size, prefix + typ + "/")
            pos += size
    walk(0, len(data), "")
    return found
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
chk(s and s[0].get("channels") == 2, "youtube audio 2 canales")
br = int(s[0].get("bit_rate", 0)) if s else 0
chk(340000 <= br <= 420000, "youtube audio bitrate 340k-420k (medido %d)" % br)
chk(j["format"]["nb_streams"] == 2, "youtube nb_streams=2")
d = float(j["format"]["duration"]); chk(abs(d - 8) <= 0.1, "youtube duración 8 s (%.3f)" % d)
a = atoms("youtube.mp4"); chk("moov" in a and "mdat" in a and a.index("moov") < a.index("mdat"), "youtube moov antes de mdat %s" % a)

j = load("out.mp3"); s = j["streams"]
chk(len(s) == 1 and s[0]["codec_name"] == "mp3", "mp3 codec mp3, 1 stream")
chk(316800 <= int(j["format"]["bit_rate"]) <= 323200, "mp3 320 kbps CBR, margen 1 por ciento (%s)" % j["format"]["bit_rate"])
d = float(j["format"]["duration"]); chk(abs(d - 8) <= 0.1, "mp3 duración 8 s (%.3f)" % d)

for name, dur in (("canvas_zoom.mp4", 6), ("youtube_graph.mp4", 8)):
    j = load(name); v = vs(j)[0]; s = au(j)
    w, h = (1080, 1920) if name.startswith("canvas") else (1920, 1080)
    chk(v["codec_name"] == "h264" and v.get("profile") == "High", name + " h264 High (%s)" % v.get("profile"))
    chk((v["width"], v["height"]) == (w, h), name + " tamaño %dx%d" % (w, h))
    chk(abs(fps(v) - 30) < 0.01, name + " 30 fps (%s)" % v["r_frame_rate"])
    d = float(j["format"]["duration"]); chk(abs(d - dur) <= 0.1, name + " duración %d s (%.3f)" % (dur, d))
    if name.startswith("canvas"):
        chk(len(s) == 0 and j["format"]["nb_streams"] == 1, name + " sin audio")
    else:
        chk(len(s) == 1 and s[0]["codec_name"] == "aac" and int(s[0]["sample_rate"]) == 48000, name + " audio aac 48 kHz")
    top = [b for b in boxes(name) if "/" not in b]
    chk(top.index("moov") < top.index("mdat"), name + " moov antes de mdat %s" % top)
    allb = boxes(name)
    chk(not any(b.endswith("elst") for b in allb), name + " sin caja elst (negative_cts_offsets)")

if os.environ.get("HEVC_OK") == "1":
    j = load("hevc.mp4"); v = vs(j)[0]
    chk(v["codec_name"] == "hevc" and (v["width"], v["height"]) == (3840, 3840), "hevc 3840x3840 (%s %sx%s)" % (v["codec_name"], v["width"], v["height"]))
    chk(len(au(j)) == 0, "hevc sin audio")
else:
    print("hevc: omitido (codificación no disponible en este runner)")

if errs:
    print("\n%d aserciones fallidas" % len(errs)); sys.exit(1)
print("\nTODAS LAS ACEPTACIONES OK")
PYEOF
