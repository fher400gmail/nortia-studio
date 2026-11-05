"""nortia. · sistema de marca (definición única). Todo lo demás (logos, iconos, láminas, tokens, docs) sale de aquí."""
import colorsys
from core import *

# ───────────────────────────── color ─────────────────────────────
INK, PAPER = "#111111", "#F4F3F0"

THEME = {
    "light": dict(bg=PAPER, deep="#ECEBE7", card="#FFFFFF", fill="#ECEBE7", line="#E2E0DB",
                  ink=INK, ink2="#4A4843", ink3="#6A6863", ctl="#85837E", wv="#C9C7C2"),
    "dark":  dict(bg=INK, deep=INK, card="#1A1A1A", fill="#222222", line="#2B2B2B",
                  ink=PAPER, ink2="#BDBBB6", ink3="#9A9893", ctl="#7A7873", wv="#4A4945"),
}
SEM = {  # estados (fill, text)
    "light": dict(ok="#1E7F4F", err="#B42318", amber="#E0A100", amber_ink="#7A5600"),
    "dark":  dict(ok="#5FCF96", err="#FF8A7D", amber="#F2B93B", amber_ink="#F5C65A"),
}
# acentos: (claro, oscuro). studio y lab están fijados por el CEO; teal y magenta son la propuesta de reserva.
ACCENTS = {
    "studio":  dict(label="naranja",  light="#FF5A00", dark="#FF5A00"),
    "lab":     dict(label="azul",     light="#2B3BFF", dark="#7483FF"),
    "teal":    dict(label="teal",     light="#00968B", dark="#1FD1C2"),
    "magenta": dict(label="magenta",  light="#D6137A", dark="#FF52A2"),
}
BRANDS = ["madre", "studio", "lab", "teal", "magenta"]


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
    """Color de TEXTO derivado del acento: mismo tono/saturación, luminosidad ajustada hasta AA (4,5) sobre todas las superficies."""
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


def acc(brand, mode):
    """Conjunto de acento de una marca en un modo: fill, text (AA), on (texto sobre fill)."""
    th = THEME[mode]
    if brand == "madre":
        fill = th["ink"]
        return dict(fill=fill, text=fill, on=th["bg"])
    fill = ACCENTS[brand][mode]
    surfaces = [th["bg"], th["card"], th["fill"]]
    text = fill if all(ratio(fill, s) >= 4.6 for s in surfaces) else text_variant(fill, surfaces, mode)
    on = max((INK, "#FFFFFF"), key=lambda c: ratio(c, fill))
    return dict(fill=fill, text=text, on=on)


# ───────────────────────────── wordmark ─────────────────────────────
SPEC = dict(font="HankenGrotesk.ttf", axes=dict(wght=590), track=-0.032, dot_k=1.22, dot_gap=0.075,
            kern={"or": 0.004, "rt": 0.012, "ti": 0.006, "ia": 0.002, "no": 0.002},
            suffix_axes=dict(wght=400), suffix_gap=0.30)
WORD = {"studio": "studio", "lab": "lab", "teal": "field", "magenta": "live"}   # palabras de ejemplo para las reservas


def _wm_metrics():
    f = font(SPEC["font"], **SPEC["axes"])
    f4 = font(SPEC["font"], wght=400)
    upm = f["head"].unitsPerEm
    nb = bounds(f, gname(f, "n"))
    top = max(bounds(f, gname(f, c))[3] for c in "nortia")
    top4 = max(bounds(f4, gname(f4, c))[3] for c in "studiolabfeldv")
    bot = -min(bounds(f, gname(f, c))[1] for c in "nortia")
    return dict(lsb=nb[0] / upm, top=max(top, top4) / upm, bot=bot / upm)


_M = _wm_metrics()


def mark(who, size, x, y, ink, dot, sfx_ink=None, sfx_op=1.0):
    """wordmark de 'who' (madre sin 2ª palabra). x = borde de tinta izquierdo, y = línea de base."""
    sfx = None if who == "madre" else WORD[who]
    return wordmark(SPEC, size, x - _M["lsb"] * size, y, ink, dot, suffix=sfx, suffix_ink=sfx_ink, suffix_op=sfx_op)


def mark_colors(who, variant):
    """(ink, dot, sfx_ink, sfx_op) para cada variante de logo."""
    if variant in ("blanco", "negro"):
        c = "#FFFFFF" if variant == "blanco" else "#000000"
        return c, c, c, 0.6 if variant == "blanco" else 0.55
    if who == "madre":
        c = INK if variant == "tinta" else PAPER
        return c, c, None, 1.0
    if variant == "color":
        return INK, ACCENTS[who]["light"], THEME["light"]["ink3"], 1.0
    return PAPER, ACCENTS[who]["dark"], THEME["dark"]["ink3"], 1.0   # color-oscuro


def doc(inner, vx, vy, vw, vh, pw=None):
    pw = pw or vw
    ph = pw * vh / vw
    return (f'<svg xmlns="http://www.w3.org/2000/svg" width="{pw:.0f}" height="{ph:.0f}" '
            f'viewBox="{vx:.2f} {vy:.2f} {vw:.2f} {vh:.2f}">{inner}</svg>'), round(pw), round(ph)


def wordmark_svg(who, variant, pw=None, pad=0.30, size=100):
    ink, dot, sfx, op = mark_colors(who, variant)
    top = _M["top"] * size
    m = mark(who, size, 0, top, ink, dot, sfx, op)
    p = pad * size
    return doc(m.svg, -p, -p, m.w + 2 * p, top + _M["bot"] * size + 0.006 * size + 2 * p, pw)


# ───────────────────────────── símbolos / iconos ─────────────────────────────
def glyph(who, cx, cy, T, col):
    """Símbolo en un tile de lado T. madre = punto; studio = punto estirado a barra vertical (2:1); lab = dos puntos (ejemplo)."""
    if who == "madre":
        r = 0.17 * T
        return f'<circle cx="{cx:.2f}" cy="{cy:.2f}" r="{r:.2f}" fill="{col}"/>'
    if who == "lab":
        r = 0.115 * T
        d = 0.19 * T
        return (f'<circle cx="{cx:.2f}" cy="{cy - d:.2f}" r="{r:.2f}" fill="{col}"/>'
                f'<circle cx="{cx:.2f}" cy="{cy + d:.2f}" r="{r:.2f}" fill="{col}"/>')
    if who == "teal":    # reserva: punto estirado en horizontal
        w, h = 0.52 * T, 0.26 * T
        return f'<rect x="{cx - w / 2:.2f}" y="{cy - h / 2:.2f}" width="{w:.2f}" height="{h:.2f}" rx="{h / 2:.2f}" fill="{col}"/>'
    if who == "magenta":  # reserva: punto con anillo
        r = 0.12 * T
        return (f'<circle cx="{cx:.2f}" cy="{cy:.2f}" r="{r:.2f}" fill="{col}"/>'
                f'<circle cx="{cx:.2f}" cy="{cy:.2f}" r="{0.23 * T:.2f}" fill="none" stroke="{col}" stroke-width="{0.05 * T:.2f}"/>')
    w, h = 0.28 * T, 0.56 * T   # studio
    return f'<rect x="{cx - w / 2:.2f}" y="{cy - h / 2:.2f}" width="{w:.2f}" height="{h:.2f}" rx="{w / 2:.2f}" fill="{col}"/>'


def glyph_col(who, mode):
    return acc(who, mode)["fill"]


def tile_colors(mode):
    return (PAPER, "rgba(17,17,17,0.10)") if mode == "light" else (INK, "rgba(255,255,255,0.16)")


def symbol_svg(who, color, pw=None, size=100):
    """símbolo suelto (sin tile), lienzo cuadrado con aire."""
    T = size * 2.2
    inner = glyph(who, T / 2, T / 2, T, color)
    return doc(inner, 0, 0, T, T, pw)


def squircle(cx, cy, s, n=4.6):
    return squircle_path(cx, cy, s, n)


def icon_svg(who, mode, size=1024, margin=True, shadow=True, shape="squircle", gscale=1.0, pad=0.0, uid="i"):
    """Icono de app. margin=True: cuerpo de 824 en lienzo de 1024 (plantilla macOS) con sombra suave.
    Colores PLANOS (sin degradados: evita el bandeado). shape: squircle | round (r=22%) | square."""
    th = THEME[mode]
    bg, rim = tile_colors(mode)
    col = glyph_col(who, mode)
    s = size / 1024
    body = 824 if margin else 1024 * (1 - 2 * pad)
    c = 512
    if shape == "squircle":
        outline = squircle(c, c, body)
    elif shape == "round":
        rr = body * 0.22
        outline = f'M{c - body / 2 + rr:.2f},{c - body / 2:.2f} H{c + body / 2 - rr:.2f} A{rr:.2f},{rr:.2f} 0 0 1 {c + body / 2:.2f},{c - body / 2 + rr:.2f} V{c + body / 2 - rr:.2f} A{rr:.2f},{rr:.2f} 0 0 1 {c + body / 2 - rr:.2f},{c + body / 2:.2f} H{c - body / 2 + rr:.2f} A{rr:.2f},{rr:.2f} 0 0 1 {c - body / 2:.2f},{c + body / 2 - rr:.2f} V{c - body / 2 + rr:.2f} A{rr:.2f},{rr:.2f} 0 0 1 {c - body / 2 + rr:.2f},{c - body / 2:.2f}Z'
    else:
        outline = f'M0,0H1024V1024H0Z'
    sh_op = 0.22 if mode == "light" else 0.55
    defs = (f'<defs><filter id="{uid}s" x="-20%" y="-20%" width="140%" height="150%">'
            f'<feDropShadow dx="0" dy="10" stdDeviation="14" flood-color="#000" flood-opacity="{sh_op}"/></filter></defs>') if (shadow and margin) else ""
    fl = f' filter="url(#{uid}s)"' if (shadow and margin) else ""
    g = [defs, f'<path d="{outline}" fill="{bg}"{fl}/>']
    if shape != "square":
        g.append(f'<path d="{outline}" fill="none" stroke="{rim}" stroke-width="3"/>')
    g.append(glyph(who, 512, 512, body * gscale, col))
    return f'<g transform="scale({s:.5f})">{"".join(g)}</g>', size


def icon_doc(who, mode, size=1024, **kw):
    inner, _ = icon_svg(who, mode, size, **kw)
    return doc(inner, 0, 0, size, size)


def lockup_svg(who, mode, pw=None):
    """lockup vertical: tile (icono plano) sobre el wordmark; fondo transparente."""
    variant = "color" if mode == "light" else "color-oscuro"
    if who == "madre":
        variant = "tinta" if mode == "light" else "papel"
    ink, dot, sfx, op = mark_colors(who, variant)
    size = 100
    T = 210
    top = _M["top"] * size
    m0 = mark(who, size, 0, 0, ink, dot, sfx, op)
    W = max(m0.w, T)
    gap = 0.62 * size
    x = (W - m0.w) / 2
    ty = 0
    bg, rim = tile_colors(mode)
    col = glyph_col(who, mode)
    sq = squircle(W / 2, ty + T / 2, T)
    tile = (f'<path d="{sq}" fill="{bg}"/><path d="{sq}" fill="none" stroke="{rim}" stroke-width="1.2"/>'
            + glyph(who, W / 2, ty + T / 2, T, col))
    base = ty + T + gap + top
    m = mark(who, size, x, base, ink, dot, sfx, op)
    p = 24
    return doc(tile + m.svg, -p, -p, W + 2 * p, base + _M["bot"] * size + 2 * p, pw)
