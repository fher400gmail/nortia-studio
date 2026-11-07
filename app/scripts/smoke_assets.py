#!/usr/bin/env python3
"""Material sintético del humo (sin dependencias). Uso:
  smoke_assets.py wav   <salida.wav>   10 s, 24 bit, 48 kHz, estéreo: seno de 1 kHz a -12 dBFS
  smoke_assets.py cover <salida.png>   3000x3000 RGB (degradado con cuadrícula)
"""
import math
import struct
import sys
import wave
import zlib


def make_wav(path, rate=48000, secs=10):
    w = wave.open(path, "wb")
    w.setnchannels(2)
    w.setsampwidth(3)
    w.setframerate(rate)
    amp = int(8388607 * 0.25)
    buf = bytearray()
    for n in range(rate * secs):
        v = int(amp * math.sin(2 * math.pi * 1000 * n / rate))
        b = struct.pack("<i", v)[:3]
        buf += b + b
    w.writeframes(bytes(buf))
    w.close()


def _chunk(tag, data):
    c = struct.pack(">I", len(data)) + tag + data
    return c + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)


def make_cover(path, side=3000):
    """PNG RGB de 8 bit: degradado diagonal con líneas cada 250 px (se ve en el vídeo)."""
    rows = bytearray()
    for y in range(side):
        row = bytearray(b"\x00")  # filtro 0
        gy = y * 255 // side
        line_y = (y % 250) < 4
        # fila por patrones de 250 px: más rápido que píxel a píxel
        seg = bytearray()
        for x in range(0, side, 250):
            gx = x * 255 // side
            px = bytes((gx, gy, 128)) * min(250, side - x)
            if line_y:
                px = bytes((30, 30, 30)) * min(250, side - x)
            else:
                px = bytes((30, 30, 30)) * 4 + px[12:]
            seg += px
        row += seg
        rows += row
    ihdr = struct.pack(">IIBBBBB", side, side, 8, 2, 0, 0, 0)
    png = b"\x89PNG\r\n\x1a\n" + _chunk(b"IHDR", ihdr) + _chunk(b"IDAT", zlib.compress(bytes(rows), 6)) + _chunk(b"IEND", b"")
    with open(path, "wb") as f:
        f.write(png)


if __name__ == "__main__":
    if len(sys.argv) != 3 or sys.argv[1] not in ("wav", "cover"):
        sys.exit(__doc__)
    (make_wav if sys.argv[1] == "wav" else make_cover)(sys.argv[2])
