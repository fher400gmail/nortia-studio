"""ferboix. studio · sistema de marca (definición única, opción C «Instrumento», D-37).
Todo lo demás (logos, iconos, láminas, tokens, docs) sale de aquí."""
import colorsys
import math
from core import *

NAME = "ferboix"            # letras del wordmark
SUFFIX = "studio"           # segunda palabra (gris)

# ───────────────────────────── color ─────────────────────────────
INK, PAPER = "#111111", "#F4F3F0"

THEME = {
    "light": dict(bg=PAPER, deep="#ECEBE7", card="#FFFFFF", fill="#ECEBE7", line="#E2E0DB",
                  ink=INK, ink2="#4A4843", ink3="#6A6863", ctl="#85837E", wv="#C9C7C2"),
    "dark":  dict(bg=INK, deep=INK, card="#1A1A1A", fill="#222222", line="#2B2B2B",
                  ink=PAPER, ink2="#BDBBB6", ink3="#9A9893", ctl="#7A7873", wv="#4A4945"),
}
SEM = {  # estados (fill, text): se mantienen tal cual (BRAND §3)
    "light": dict(ok="#1E7F4F", err="#B42318", amber="#E0A100", amber_ink="#7A5600"),
    "dark":  dict(ok="#5FCF96", err="#FF8A7D", amber="#F2B93B", amber_ink="#F5C65A"),
}
# Un solo acento: cian LED. Relleno igual en los dos modos; el texto-acento se oscurece en claro hasta AA.
ACCENT = dict(label="cian", light="#0097B8", dark="#0097B8")
ACCENT_TEXT_LIGHT = "#00728B"   # texto sobre papel (5,0:1); en oscuro el relleno ya es AA como texto (5,5:1)
TICK_DEG = 35                   # marca de posición del potenciómetro, girada hacia la una


def _lin(c):
    c /= 255
    return c / 12.92 if c <= 0.03928 else ((c + 0.055) / 1.055) ** 2.4


def lum(h):
    h = h.lstrip("#")
    r, g, b = (int(h[i:i + 2], 16) for i in (0, 2, 4))
    return 0.2126 * _lin(r) + 0.7152 * _lin(g) + 0.0722 * _lin(b)


def ratio(a, b):
    la, lb = lum(a), lum(b)
    if la < lb:
        la, lb = lb, la
    return (la + 0.05) / (lb + 0.05)


def hex2rgb(h):
    h = h.lstrip("#")
    return tuple(int(h[i:i + 2], 16) for i in (0, 2, 4))


def rgba(h, a):
    r, g, b = hex2rgb(h)
    return f"rgba({r},{g},{b},{a})"


def mix(a, b, t):
    ca, cb = hex2rgb(a), hex2rgb(b)
    return "#" + "".join(f"{round(ca[i] + (cb[i] - ca[i]) * t):02X}" for i in range(3))


def text_variant(color, surfaces, mode, target=4.6):
    """Color de TEXTO derivado del acento: mismo tono/saturación, luminosidad ajustada hasta AA sobre todas las superficies."""
    r, g, b = (v / 255 for v in hex2rgb(color))
    h, l, s = colorsys.rgb_to_hls(r, g, b)
    step = -0.004 if mode == "light" else 0.004
    for _ in range(300):
        rr, gg, bb = colorsys.hls_to_rgb(h, l, s)
        hx = "#%02X%02X%02X" % (round(rr * 255), round(gg * 255), round(bb * 255))
        if all(ratio(hx, sf) >= target for sf in surfaces):
            return hx
        l += step
    return hx


def acc(mode):
    """Conjunto de acento en un modo: fill (cian), text (AA como texto), on (texto sobre cian, p. ej. el check de una ficha)."""
    th = THEME[mode]
    fill = ACCENT[mode]
    surfaces = [th["bg"], th["card"], th["fill"]]
    text = ACCENT_TEXT_LIGHT if mode == "light" and all(ratio(ACCENT_TEXT_LIGHT, s) >= 4.5 for s in surfaces) \
        else (fill if all(ratio(fill, s) >= 4.5 for s in surfaces) else text_variant(fill, surfaces, mode))
    on = max((INK, "#FFFFFF"), key=lambda c: ratio(c, fill))
    return dict(fill=fill, text=text, on=on)


def btn_colors(mode):
    """Botón primario: TINTA con texto papel (claro) / PAPEL con texto tinta (oscuro). El acento no se usa de relleno."""
    th = THEME[mode]
    return dict(fill=th["ink"], on=th["bg"])


# ───────────────────────────── wordmark ─────────────────────────────
SPEC = dict(letters=NAME, font="HankenGrotesk.ttf", axes=dict(wght=590), track=-0.032, dot_k=1.22, dot_gap=0.075,
            kern={"rb": 0.006, "oi": 0.004, "ix": 0.004},
            suffix_axes=dict(wght=400), suffix_gap=0.30)


def _wm_metrics():
    f = font(SPEC["font"], **SPEC["axes"])
    f4 = font(SPEC["font"], wght=400)
    upm = f["head"].unitsPerEm
    nb = bounds(f, gname(f, SPEC["letters"][0]))
    top = max(bounds(f, gname(f, c))[3] for c in SPEC["letters"])
    top4 = max(bounds(f4, gname(f4, c))[3] for c in SUFFIX + "Facturación"[0] + "ó")
    bot = -min(bounds(f, gname(f, c))[1] for c in SPEC["letters"])
    return dict(lsb=nb[0] / upm, top=max(top, top4) / upm, bot=bot / upm)


_M = _wm_metrics()


def mark(size, x, y, ink, dot=None, sfx_ink=None, sfx_op=1.0, suffix=SUFFIX):
    """wordmark «ferboix. studio». x = borde de tinta izquierdo, y = línea de base. El punto es del color de la tinta (dot=None)."""
    return wordmark(SPEC, size, x - _M["lsb"] * size, y, ink, dot or ink, suffix=suffix, suffix_ink=sfx_ink, suffix_op=sfx_op)


def mark_colors(variant):
    """(ink, dot, sfx_ink, sfx_op) para cada variante de logo. El punto SIEMPRE es del color de la tinta."""
    if variant in ("blanco", "negro"):
        c = "#FFFFFF" if variant == "blanco" else "#000000"
        return c, c, c, 0.6 if variant == "blanco" else 0.55
    if variant == "claro":
        return INK, INK, THEME["light"]["ink3"], 1.0
    return PAPER, PAPER, THEME["dark"]["ink3"], 1.0   # oscuro


def doc(inner, vx, vy, vw, vh, pw=None):
    pw = pw or vw
    ph = pw * vh / vw
    return (f'<svg xmlns="http://www.w3.org/2000/svg" width="{pw:.0f}" height="{ph:.0f}" '
            f'viewBox="{vx:.2f} {vy:.2f} {vw:.2f} {vh:.2f}">{inner}</svg>'), round(pw), round(ph)


def wordmark_svg(variant, pw=None, pad=0.30, size=100):
    ink, dot, sfx, op = mark_colors(variant)
    top = _M["top"] * size
    m = mark(size, 0, top, ink, dot, sfx, op)
    p = pad * size
    return doc(m.svg, -p, -p, m.w + 2 * p, top + _M["bot"] * size + 0.006 * size + 2 * p, pw)


# ───────────────────────────── símbolo: potenciómetro ─────────────────────────────
KNOB = dict(R=0.34, w=0.36, a=0.12, b=0.93)   # radio = 0,34 T; marca de trazo 0,36 R, de 0,12 R a 0,93 R, extremos redondos


def knob_tick(cx, cy, R, col, ang=TICK_DEG):
    a_ = math.radians(ang)
    x1, y1 = cx + math.sin(a_) * R * KNOB["a"], cy - math.cos(a_) * R * KNOB["a"]
    x2, y2 = cx + math.sin(a_) * R * KNOB["b"], cy - math.cos(a_) * R * KNOB["b"]
    return (f'<line x1="{x1:.2f}" y1="{y1:.2f}" x2="{x2:.2f}" y2="{y2:.2f}" stroke="{col}" '
            f'stroke-width="{KNOB["w"] * R:.2f}" stroke-linecap="round"/>')


def glyph(cx, cy, T, disc, tick):
    """Potenciómetro visto desde arriba en un tile de lado T: disco (tinta en claro, papel en oscuro) + marca de posición en el acento."""
    R = KNOB["R"] * T
    return f'<circle cx="{cx:.2f}" cy="{cy:.2f}" r="{R:.2f}" fill="{disc}"/>' + knob_tick(cx, cy, R, tick)


def glyph_mode(cx, cy, T, mode):
    return glyph(cx, cy, T, THEME[mode]["ink"], ACCENT[mode])


def tile_colors(mode):
    return (PAPER, "rgba(17,17,17,0.10)") if mode == "light" else (INK, "rgba(255,255,255,0.16)")


def symbol_svg(variant, pw=None, size=100):
    """símbolo suelto (sin tile), lienzo cuadrado con aire. variant: claro | oscuro | negro | blanco."""
    T = size * 2.2
    if variant in ("claro", "oscuro"):
        mode = "light" if variant == "claro" else "dark"
        inner = glyph_mode(T / 2, T / 2, T, mode)
    elif variant == "negro":
        inner = glyph(T / 2, T / 2, T, "#000000", "#FFFFFF")
    else:
        inner = glyph(T / 2, T / 2, T, "#FFFFFF", "#000000")
    return doc(inner, 0, 0, T, T, pw)


def squircle(cx, cy, s, n=4.6):
    return squircle_path(cx, cy, s, n)


def icon_svg(mode, size=1024, margin=True, shadow=True, shape="squircle", gscale=1.0, pad=0.0, uid="i"):
    """Icono de app. margin=True: cuerpo de 824 en lienzo de 1024 (plantilla macOS) con la sombra suave de plantilla.
    Colores PLANOS (sin degradados). shape: squircle | round (r=22%) | square. La sombra es de la plantilla del icono, no del símbolo."""
    bg, rim = tile_colors(mode)
    s = size / 1024
    body = 824 if margin else 1024 * (1 - 2 * pad)
    c = 512
    if shape == "squircle":
        outline = squircle(c, c, body)
    elif shape == "round":
        rr = body * 0.22
        outline = f'M{c - body / 2 + rr:.2f},{c - body / 2:.2f} H{c + body / 2 - rr:.2f} A{rr:.2f},{rr:.2f} 0 0 1 {c + body / 2:.2f},{c - body / 2 + rr:.2f} V{c + body / 2 - rr:.2f} A{rr:.2f},{rr:.2f} 0 0 1 {c + body / 2 - rr:.2f},{c + body / 2:.2f} H{c - body / 2 + rr:.2f} A{rr:.2f},{rr:.2f} 0 0 1 {c - body / 2:.2f},{c + body / 2 - rr:.2f} V{c - body / 2 + rr:.2f} A{rr:.2f},{rr:.2f} 0 0 1 {c - body / 2 + rr:.2f},{c - body / 2:.2f}Z'
    else:
        outline = 'M0,0H1024V1024H0Z'
    sh_op = 0.22 if mode == "light" else 0.55
    defs = (f'<defs><filter id="{uid}s" x="-20%" y="-20%" width="140%" height="150%">'
            f'<feDropShadow dx="0" dy="10" stdDeviation="14" flood-color="#000" flood-opacity="{sh_op}"/></filter></defs>') if (shadow and margin) else ""
    fl = f' filter="url(#{uid}s)"' if (shadow and margin) else ""
    g = [defs, f'<path d="{outline}" fill="{bg}"{fl}/>']
    if shape != "square":
        g.append(f'<path d="{outline}" fill="none" stroke="{rim}" stroke-width="3"/>')
    g.append(glyph_mode(512, 512, body * gscale, mode))
    return f'<g transform="scale({s:.5f})">{"".join(g)}</g>', size


def icon_doc(mode, size=1024, **kw):
    inner, _ = icon_svg(mode, size, **kw)
    return doc(inner, 0, 0, size, size)


def lockup_svg(mode, pw=None):
    """lockup vertical: tile (icono plano) sobre el wordmark; fondo transparente."""
    ink, dot, sfx, op = mark_colors("claro" if mode == "light" else "oscuro")
    size = 100
    T = 210
    top = _M["top"] * size
    m0 = mark(size, 0, 0, ink, dot, sfx, op)
    W = max(m0.w, T)
    gap = 0.62 * size
    x = (W - m0.w) / 2
    ty = 0
    bg, rim = tile_colors(mode)
    sq = squircle(W / 2, ty + T / 2, T)
    tile = (f'<path d="{sq}" fill="{bg}"/><path d="{sq}" fill="none" stroke="{rim}" stroke-width="1.2"/>'
            + glyph_mode(W / 2, ty + T / 2, T, mode))
    base = ty + T + gap + top
    m = mark(size, x, base, ink, dot, sfx, op)
    p = 24
    return doc(tile + m.svg, -p, -p, W + 2 * p, base + _M["bot"] * size + 2 * p, pw)
