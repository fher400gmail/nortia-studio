#!/usr/bin/env python3
"""Comprueba el resultado del humo de master check (macOS y Windows) con el ffprobe del propio sidecar.

Uso: smoke_verify.py --out <carpeta de salida> --platform mac|win [--adm required|any]

Lee <out>/smoke-result.json (entrega + bloque `smoke`), exige [Portada], [Canvas], [YouTube], [MP3 320]
y Apple Motion, los sondea con ffprobe y anota el tiempo de media::capabilities() en frío.
Los avisos van como ::warning:: de GitHub Actions; un fallo sale con código 1.
"""
import argparse
import glob
import json
import os
import subprocess
import sys

CAPS_TARGET_MS = 3000       # objetivo del sondeo en frío
CAPS_DEADLINE_MS = 16000    # 2 códecs x 8 s de plazo: más allá hay algo colgado

errors = []


def fail(msg):
    errors.append(msg)
    print(f"::error::{msg}")


def warn(msg):
    print(f"::warning::{msg}")


def probe(ffprobe, path):
    r = subprocess.run(
        [ffprobe, "-v", "error", "-print_format", "json", "-show_streams", "-show_format", path],
        capture_output=True, text=True, timeout=60,
    )
    if r.returncode != 0:
        fail(f"ffprobe no pudo leer {os.path.basename(path)}: {r.stderr.strip()[:300]}")
        return None
    return json.loads(r.stdout)


def streams(info, kind):
    return [s for s in info["streams"] if s.get("codec_type") == kind]


def one(files, tag, ext):
    tags = tag if isinstance(tag, tuple) else (tag,)
    hits = [f for f in files if any(f"[{x}]" in os.path.basename(f) for x in tags) and f.lower().endswith(ext)]
    if len(hits) != 1:
        fail(f"se esperaba 1 archivo [{tag}] *{ext} y hay {len(hits)}")
        return None
    return hits[0]


def want(cond, msg):
    if not cond:
        fail(msg)


def video_check(ffprobe, path, codec, w, h, audio):
    info = probe(ffprobe, path)
    if not info:
        return
    v = streams(info, "video")
    name = os.path.basename(path)
    want(len(v) == 1, f"{name}: se esperaba 1 pista de vídeo")
    if v:
        want(v[0]["codec_name"] == codec, f"{name}: códec {v[0]['codec_name']} (se esperaba {codec})")
        want((v[0]["width"], v[0]["height"]) == (w, h), f"{name}: tamaño {v[0]['width']}x{v[0]['height']} (se esperaba {w}x{h})")
    a = streams(info, "audio")
    if audio:
        want(len(a) == 1 and a[0]["codec_name"] == "aac" and int(a[0]["sample_rate"]) == 48000,
             f"{name}: se esperaba AAC a 48 kHz, hay {[(s['codec_name'], s.get('sample_rate')) for s in a]}")
    else:
        want(not a, f"{name}: no debe llevar audio")
    print(f"ok   {name}: {codec} {w}x{h}" + (" + aac 48k" if audio else ""))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--platform", choices=["mac", "win", "linux"], required=True)
    ap.add_argument("--adm", choices=["required", "any"], default="any")
    args = ap.parse_args()

    res_path = os.path.join(args.out, "smoke-result.json")
    d = json.load(open(res_path, encoding="utf-8"))
    want(d.get("cancelled") is False, "la entrega está cancelada")
    want(d.get("failed") == [], f"destinos fallidos: {d.get('failed')}")
    files = [f[0] for f in d.get("files", [])]
    want(bool(files), "sin archivos")
    for f in files:
        want(os.path.isfile(f), f"el informe lista {f} y no existe")
    print(f"entrega: {d.get('folder')} - {len(files)} archivos")

    base = lambda f: os.path.basename(f)
    want(any("[Spotify]" in base(f) and f.endswith(".flac") for f in files), "falta [Spotify] .flac")
    want(any(base(f) == "README.txt" for f in files), "falta README.txt")
    report = [f for f in files if base(f) == "informe-loudness.txt"]
    want(bool(report), "falta informe-loudness.txt")

    smoke = d.get("smoke") or {}
    side = smoke.get("sidecar")
    want(bool(side), "el humo no encontró el sidecar ffmpeg/ffprobe junto al ejecutable")
    caps = smoke.get("capabilities") or {}
    want(isinstance(caps.get("millis"), int), "smoke-result.json no trae el tiempo de capabilities()")
    if errors:
        return finish()
    ffprobe = side["ffprobe"]
    print(f"sidecar: {side['ffmpeg']}")
    ms = caps["millis"]
    print(f"capabilities() en frío: {ms} ms (h264={caps['h264']}, hevc={caps['hevc']}; objetivo < {CAPS_TARGET_MS} ms)")
    if ms >= CAPS_TARGET_MS:
        warn(f"capabilities() tardó {ms} ms en frío en {args.platform} (objetivo < {CAPS_TARGET_MS} ms; plazo 8 s por códec)")
    want(ms < CAPS_DEADLINE_MS, f"capabilities() tardó {ms} ms: un codificador se colgó")
    want(caps["h264"] is True, "el sondeo dice que no hay codificador H.264: Canvas y YouTube no se pueden crear")
    if args.platform == "mac":
        want(caps["hevc"] is True, "macOS debe ofrecer HEVC (hevc_videotoolbox)")

    # portada
    jpg = one(files, "Portada", ".jpg")
    png = one(files, "Portada", ".png")
    for p, codec in ((jpg, "mjpeg"), (png, "png")):
        if p:
            info = probe(ffprobe, p)
            if info:
                s = streams(info, "video")[0]
                want((s["width"], s["height"]) == (3000, 3000) and s["codec_name"] == codec,
                     f"{base(p)}: {s['codec_name']} {s['width']}x{s['height']}")
                print(f"ok   {base(p)}: {codec} 3000x3000")
    # vídeos
    p = one(files, "Canvas", ".mp4")
    if p:
        video_check(ffprobe, p, "h264", 1080, 1920, audio=False)
    p = one(files, "YouTube", ".mp4")
    if p:
        video_check(ffprobe, p, "h264", 1920, 1080, audio=True)
    # MP3
    p = one(files, "MP3 320", ".mp3")
    if p:
        info = probe(ffprobe, p)
        if info:
            a = streams(info, "audio")[0]
            # bit_rate de la pista de audio: el del formato incluye la portada incrustada (mjpeg)
            br = int(a.get("bit_rate") or 0)
            want(a["codec_name"] == "mp3" and int(a["sample_rate"]) == 44100 and 315000 <= br <= 325000,
                 f"{base(p)}: {a['codec_name']} {a['sample_rate']} Hz {br} bps (se esperaba mp3 44100 Hz 320 kbps)")
            print(f"ok   {base(p)}: mp3 44.1 kHz {br // 1000} kbps")
    # portada animada (es: «Portada animada»; en: «Motion artwork»)
    motion = ("Portada animada", "Motion artwork")
    small = one(files, motion, "2048x2732.mp4")
    if small:
        video_check(ffprobe, small, "h264", 2048, 2732, audio=False)
    square = [f for f in files if any(f"[{x}]" in base(f) for x in motion) and "3840x3840" in base(f)]
    if caps["hevc"]:
        if len(square) == 1:
            # en Linux (solo desarrollo) el «HEVC» es libx264: nunca se distribuye
            video_check(ffprobe, square[0], "h264" if args.platform == "linux" else "hevc", 3840, 3840, audio=False)
        else:
            fail("HEVC disponible y falta la portada animada 3840x3840.mp4")
    else:
        warn("HEVC no disponible en este equipo: la portada animada solo sale en 2048x2732 (instala «HEVC Video Extensions» para el cuadrado de 3840x3840)")
        want(not square, "sin HEVC no debe haber 3840x3840")

    # Apple Digital Masters en el informe de loudness
    txt = open(report[0], encoding="utf-8").read() if report else ""
    adm_lines = [l for l in txt.splitlines() if "Apple Digital Masters" in l]
    print("informe ADM:", adm_lines)
    if args.adm == "required":
        want(any("0 muestras con clipping" in l for l in adm_lines),
             f"el informe debe decir '0 muestras con clipping' (hay: {adm_lines})")
    elif args.platform == "mac" and adm_lines and any("no disponible" in l for l in adm_lines):
        warn("Apple Digital Masters no validado en el humo: " + adm_lines[0])
    return finish()


def finish():
    if errors:
        print(f"HUMO FALLA: {len(errors)} problema(s)")
        return 1
    print("HUMO VERIFICADO")
    return 0


if __name__ == "__main__":
    sys.exit(main())
