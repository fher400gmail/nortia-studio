"""Núcleo de gen.py: fuentes, contornos de texto (wordmarks a medida), formas y render.
Todo el texto de marca se dibuja como PATH (kerning y punto propios); el texto de interfaz usa @font-face."""
import os, math, asyncio
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.pens.boundsPen import BoundsPen

HERE = os.path.dirname(os.path.abspath(__file__))
FONTS = os.path.join(HERE, "fonts")
BASE = "https://raw.githubusercontent.com/google/fonts/main/ofl"
FONT_URLS = {
    "HankenGrotesk.ttf": "hankengrotesk/HankenGrotesk%5Bwght%5D.ttf",
}


def ensure_fonts():
    os.makedirs(FONTS, exist_ok=True)
    import subprocess
    for name, rel in FONT_URLS.items():
        p = os.path.join(FONTS, name)
        if not os.path.exists(p):
            subprocess.run(["curl", "-sf", "-m", "90", "-o", p, f"{BASE}/{rel}"], check=True)


_cache = {}


def font(fname, **axes):
    key = (fname, tuple(sorted(axes.items())))
    if key in _cache:
        return _cache[key]
    f = TTFont(os.path.join(FONTS, fname))
    if "fvar" in f:
        have = {a.axisTag: a for a in f["fvar"].axes}
        loc = {k: v for k, v in axes.items() if k in have}
        # los ejes no indicados se quedan en su valor por defecto
        f = instancer.instantiateVariableFont(f, loc, inplace=False)
    _cache[key] = f
    return f


def _cmap(f):
    return f.getBestCmap()


def gname(f, ch):
    return _cmap(f)[ord(ch)]


def adv(f, g):
    return f["hmtx"][g][0]


def bounds(f, g):
    gs = f.getGlyphSet()
    bp = BoundsPen(gs)
    gs[g].draw(bp)
    return bp.bounds  # xmin,ymin,xmax,ymax o None


def gpath(f, g, x, y, s):
    """path SVG del glifo, origen (x,y) en baseline, escala s (unidades->px)."""
    gs = f.getGlyphSet()
    pen = SVGPathPen(gs, ntos=lambda v: f"{v:.2f}")
    gs[g].draw(TransformPen(pen, (s, 0, 0, -s, x, y)))
    return pen.getCommands()


def kern_gpos(f, a, b):
    return 0  # kerning se resuelve a mano (par a par) en cada dirección


class Mark:
    """Resultado de dibujar un texto: svg (paths), ancho, métricas."""
    def __init__(self, svg, w, xh, cap, r, cx, cy):
        self.svg, self.w, self.xh, self.cap, self.r, self.cx, self.cy = svg, w, xh, cap, r, cx, cy


def text_outline(spec, text, size, x, y, fill, track=None, kern=None, weight=None, opacity=1.0):
    """Contornos de `text` con la fuente/ejes de spec. track en em; kern dict {'or': em}."""
    axes = dict(spec["axes"])
    if weight is not None:
        axes["wght"] = weight
    f = font(spec["font"], **axes)
    upm = f["head"].unitsPerEm
    s = size / upm
    track = spec.get("track", 0) if track is None else track
    kern = spec.get("kern", {}) if kern is None else kern
    cx = x
    out = []
    for i, ch in enumerate(text):
        if ch == " ":
            cx += adv(f, gname(f, " ")) * s + track * size
            continue
        g = gname(f, ch)
        out.append(gpath(f, g, cx, y, s))
        cx += adv(f, g) * s + track * size
        if i + 1 < len(text):
            cx += kern.get(ch + text[i + 1], 0) * size
    d = " ".join(out)
    return f'<path d="{d}" fill="{fill}" fill-opacity="{opacity}"/>', cx - x


def wordmark(spec, size, x, y, ink, accent, suffix=None, suffix_ink=None, dot_shape="circle", pill=1.0, suffix_op=1.0):
    """'ferboix.' con punto a medida. y = baseline. Devuelve Mark.
    spec: font, axes, track, kern, dot_k (diámetro/asta), dot_gap (em), dotless_i (bool)."""
    f = font(spec["font"], **spec["axes"])
    upm = f["head"].unitsPerEm
    s = size / upm
    track, kern = spec.get("track", 0), spec.get("kern", {})
    letters = spec.get("letters", "ferboix")
    cx = x
    parts = []
    xh = f["OS/2"].sxHeight * s
    cap = f["OS/2"].sCapHeight * s
    lb = bounds(f, gname(f, "l"))
    stem = (lb[2] - lb[0]) * s
    D = stem * spec.get("dot_k", 1.2)
    r = D / 2
    last_right = x
    for i, ch in enumerate(letters):
        gch = ch
        g = gname(f, ch)
        if ch == "i" and spec.get("dotless_i"):
            g = [n for n in f.getGlyphOrder() if n in ("dotlessi", "idotless", "uni0131")][0]
        parts.append(gpath(f, g, cx, y, s))
        b = bounds(f, g)
        last_right = cx + b[2] * s
        if ch == "i" and spec.get("dotless_i"):
            ib = bounds(f, g)
            icx = cx + (ib[0] + ib[2]) / 2 * s
            top = bounds(f, gname(f, "i"))[3] * s  # techo del tittle original
            tr = D * spec.get("tittle_k", 1.0) / 2
            parts_dot_i = (icx, y - top + tr, tr)
        cx += adv(f, g) * s + track * size
        if i + 1 < len(letters):
            cx += kern.get(ch + letters[i + 1], 0) * size
    svg = f'<path d="{" ".join(parts)}" fill="{ink}"/>'
    if spec.get("dotless_i"):
        ix, iy, ir = parts_dot_i
        svg += f'<circle cx="{ix:.2f}" cy="{iy:.2f}" r="{ir:.2f}" fill="{ink}"/>'
    over = 0.006 * size
    dcx = last_right + spec.get("dot_gap", 0.07) * size + r
    dcy = y - r + over
    if dot_shape == "pill":      # punto estirado en vertical (barra de nivel)
        h = D * pill
        svg += f'<rect x="{dcx - r:.2f}" y="{y + over - h:.2f}" width="{D:.2f}" height="{h:.2f}" rx="{r:.2f}" fill="{accent}"/>'
        dcy = y + over - h / 2
    elif dot_shape == "hpill":   # punto estirado en horizontal
        wd = D * pill
        svg += f'<rect x="{dcx - r:.2f}" y="{y + over - D:.2f}" width="{wd:.2f}" height="{D:.2f}" rx="{r:.2f}" fill="{accent}"/>'
        dcx = dcx - r + wd - r
    else:
        svg += f'<circle cx="{dcx:.2f}" cy="{dcy:.2f}" r="{r:.2f}" fill="{accent}"/>'
    w = dcx + r - x
    if suffix:
        sx = dcx + r + spec.get("suffix_gap", 0.28) * size
        sspec = dict(spec)
        sspec["axes"] = dict(spec["axes"], **spec.get("suffix_axes", {}))
        sp, sw = text_outline(sspec, suffix, size * spec.get("suffix_scale", 1.0), sx, y, suffix_ink or ink,
                              track=spec.get("suffix_track", spec.get("track", 0)), kern={}, opacity=suffix_op)
        svg += sp
        w = sx + sw - x
    return Mark(svg, w, xh, cap, r, dcx, dcy)


def squircle_path(cx, cy, size, n=5.0, steps=240):
    """superelipse |x|^n+|y|^n=1: aproxima el squircle continuo de macOS."""
    a = size / 2
    pts = []
    for i in range(steps):
        t = 2 * math.pi * i / steps
        c, s_ = math.cos(t), math.sin(t)
        x = a * math.copysign(abs(c) ** (2 / n), c)
        y = a * math.copysign(abs(s_) ** (2 / n), s_)
        pts.append((cx + x, cy + y))
    return "M" + " L".join(f"{px:.2f},{py:.2f}" for px, py in pts) + "Z"


FONT_FACE_CSS = """
@font-face{font-family:'Hanken Grotesk';src:url('fonts/HankenGrotesk.ttf');font-weight:100 900;}
"""


def render_svgs(jobs, scale=1):
    """jobs: [(svg, w, h, out_png[, transparent])] -> PNG con Chromium (Playwright); fuentes de fonts/."""
    from playwright.async_api import async_playwright

    async def run():
        async with async_playwright() as p:
            b = await p.chromium.launch()
            pg = await b.new_page(viewport={"width": 100, "height": 100}, device_scale_factor=scale)
            hp = os.path.join(HERE, "_render.html")
            for job in jobs:
                svg, w, h, out = job[:4]
                tr = job[4] if len(job) > 4 else False
                bg = "transparent" if tr else "#fff"
                html = f"<!doctype html><meta charset=utf-8><style>{FONT_FACE_CSS}html,body{{margin:0;background:{bg}}}svg{{display:block}}</style>{svg}"
                open(hp, "w", encoding="utf-8").write(html)
                await pg.set_viewport_size({"width": int(w), "height": int(h)})
                await pg.goto("file://" + hp)
                await pg.evaluate("document.fonts.ready")
                await pg.wait_for_timeout(60)
                os.makedirs(os.path.dirname(out), exist_ok=True)
                await pg.screenshot(path=out, clip={"x": 0, "y": 0, "width": int(w), "height": int(h)}, omit_background=tr)
            await b.close()
    asyncio.run(run())
    try:
        os.remove(os.path.join(HERE, "_render.html"))
    except OSError:
        pass


def render_html(jobs):
    """jobs: [(html_path, w, h, png_out, pdf_out|None)] -> PNG (y PDF A4 si se indica)."""
    from playwright.async_api import async_playwright

    async def run():
        async with async_playwright() as p:
            b = await p.chromium.launch()
            for hp, w, h, png, pdf in jobs:
                pg = await b.new_page(viewport={"width": w, "height": h}, device_scale_factor=2)
                await pg.goto("file://" + hp)
                await pg.evaluate("document.fonts.ready")
                await pg.wait_for_timeout(150)
                if png:
                    await pg.screenshot(path=png, full_page=False)
                if pdf:
                    await pg.pdf(path=pdf, format="A4", print_background=True, margin=dict(top="0", bottom="0", left="0", right="0"))
                await pg.close()
            await b.close()
    asyncio.run(run())


def write_ico(path, pngs):
    """.ico con entradas PNG. pngs: [(lado_px, bytes_png)]"""
    import struct
    n = len(pngs)
    head = struct.pack("<HHH", 0, 1, n)
    off = 6 + 16 * n
    ent, data = b"", b""
    for s_, b_ in pngs:
        w = 0 if s_ >= 256 else s_
        ent += struct.pack("<BBBBHHII", w, w, 0, 0, 1, 32, len(b_), off + len(data))
        data += b_
    open(path, "wb").write(head + ent + data)
