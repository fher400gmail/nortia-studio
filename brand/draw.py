"""Utilidades de dibujo SVG compartidas por assets.py y sheets.py."""
import math
from brand import *
from brand import _M

FAM = "Hanken Grotesk"


def T(x, y, s, size=14, w=400, fill="#000", anchor="start", ls=0, op=1, fam=FAM, extra=""):
    s = s.replace("&", "&amp;")
    return (f'<text x="{x:.1f}" y="{y:.1f}" font-family="\'{fam}\'" font-size="{size}" font-weight="{w}" fill="{fill}" '
            f'fill-opacity="{op}" text-anchor="{anchor}" letter-spacing="{ls}em" {extra}>{s}</text>')


def head(x, y, text, size, fill, dot=None, w=600, track=-0.035):
    """Titular como contornos (kerning propio). x = borde de tinta. Si dot: punto final circular de la marca. Devuelve (svg, ancho)."""
    sp = dict(SPEC, axes=dict(wght=w))
    f = font(SPEC["font"], wght=w)
    sc = size / f["head"].unitsPerEm
    lsb = bounds(f, gname(f, text[0]))[0] * sc
    p, wd = text_outline(sp, text, size, x - lsb, y, fill, track=track, kern={})
    svg = p
    end = wd - lsb
    if dot:
        lb = bounds(f, gname(f, "l"))
        D = (lb[2] - lb[0]) * sc * SPEC["dot_k"]
        r = D / 2
        cx = x - lsb + wd + SPEC["dot_gap"] * size * 0.55 + r - track * size * 0
        svg += f'<circle cx="{cx:.2f}" cy="{y - r + 0.006 * size:.2f}" r="{r:.2f}" fill="{dot}"/>'
        end = cx + r - x
    return svg, end


def panel(x, y, w, h, fill, r=28, stroke=None):
    st = f' stroke="{stroke}" stroke-width="1"' if stroke else ""
    return f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{r}" fill="{fill}"{st}/>'


_uid = [0]


def uid(p="u"):
    _uid[0] += 1
    return f"{p}{_uid[0]}"


def clip(x, y, w, h, r, inner):
    i = uid("c")
    return (f'<clipPath id="{i}"><rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{r}"/></clipPath>'
            f'<g clip-path="url(#{i})">{inner}</g>')


def pills(x, base, n, D, gap, mode, accent_fill, accent_i, seed=0, maxh=None, grow=None):
    """Fila de barras: el punto crece a barra y vuelve a punto (nivel de señal). Una barra en acento."""
    th = THEME[mode]
    maxh = maxh or D * 6
    out = []
    for k in range(n):
        t = k / (n - 1)
        env = math.sin(math.pi * t) ** 1.3
        wob = 0.55 + 0.45 * math.sin(k * 1.9 + 0.7 + seed) ** 2
        h = D + (maxh - D) * env * wob
        col = th["wv"]
        if k == accent_i:
            h = maxh
            col = accent_fill
        out.append(f'<rect x="{x + k * (D + gap):.1f}" y="{base - h:.1f}" width="{D}" height="{h:.1f}" rx="{D / 2}" fill="{col}"/>')
    return "".join(out)


def ic_check(cx, cy, sz, col, w=2.2):
    k = sz / 2
    return (f'<path d="M{cx - k * 0.55:.1f},{cy + k * 0.02:.1f} L{cx - k * 0.12:.1f},{cy + k * 0.45:.1f} L{cx + k * 0.6:.1f},{cy - k * 0.4:.1f}" '
            f'stroke="{col}" stroke-width="{w}" fill="none" stroke-linecap="round" stroke-linejoin="round"/>')
