"""Láminas: kit.png (todo el sistema) y familia.png (ferboix. studio + master check + hueco para apps futuras, claro y oscuro)."""
import math
from brand import *
from brand import _M
from draw import *
import tokens as TK


def ct(mode):
    return TK.color_tokens(mode)


def wm(mode, size, x, y_center, suffix=SUFFIX):
    """wordmark centrado en vertical sobre x-height."""
    ink, dot, sfx, op = mark_colors("claro" if mode == "light" else "oscuro")
    xh = font(SPEC["font"], **SPEC["axes"])["OS/2"].sxHeight / 1000 * size
    return mark(size, x, y_center + xh / 2, ink, dot, sfx, op, suffix=suffix)


def icon_at(mode, size, x, y, shadow=True):
    inner, _ = icon_svg(mode, size, margin=True, shadow=shadow, uid=uid("i"))
    return f'<g transform="translate({x:.1f},{y:.1f})">{inner}</g>'


def sym_at(mode, T_, cx, cy):
    return glyph_mode(cx, cy, T_, mode)


def app_header(mode, x, y_center, size, app="master check", sep_h=None):
    """Cabecera «símbolo + ferboix. studio | app». Devuelve (svg, ancho)."""
    th = THEME[mode]
    T_ = size * 1.5
    g = [sym_at(mode, T_, x + T_ * 0.34, y_center)]
    hx = x + T_ * 0.68 + size * 0.38
    m = wm(mode, size, hx, y_center)
    g.append(m.svg)
    dx = hx + m.w + size * 0.7
    sh = sep_h or size * 0.72
    g.append(f'<path d="M{dx:.1f},{y_center - sh / 2:.1f} V{y_center + sh / 2:.1f}" stroke="{th["line"]}" stroke-width="1.2"/>')
    g.append(T(dx + size * 0.64, y_center + size * 0.17, app, size * 0.66, 400, th["ink2"]))
    return "".join(g), dx + size * 0.64 - x + size * 0.66 * 0.54 * len(app)


# ───────────────────────────── componentes ─────────────────────────────
def button(x, y, w, h, label, mode, kind="primary", size=16):
    th = THEME[mode]
    b = btn_colors(mode)
    if kind == "primary":
        return (f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{RADIUS_B}" fill="{b["fill"]}"/>'
                + T(x + w / 2, y + h / 2 + size * 0.35, label, size, 600, b["on"], "middle"))
    return (f'<rect x="{x + 0.75}" y="{y + 0.75}" width="{w - 1.5}" height="{h - 1.5}" rx="{RADIUS_B}" fill="none" stroke="{th["ink"]}" stroke-opacity="0.85" stroke-width="1.5"/>'
            + T(x + w / 2, y + h / 2 + size * 0.35, label, size, 600, th["ink"], "middle"))


RADIUS_B = TK.RADIUS["button"]


def chip(x, y, label, mode, w=140, h=32, checked=True):
    th, c = THEME[mode], ct(mode)
    g = f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{h / 2}" fill="{c["on-bg"]}"/>'
    tx = x + 14
    if checked:
        g += ic_check(x + 20, y + h / 2, 12, th["ink"], 1.9)
        tx = x + 34
    return g + T(tx, y + h / 2 + 4.6, label, 13, 550, th["ink"])


def status(x, y, label, kind, mode, w=120, h=30):
    th = THEME[mode]
    col = {"ok": SEM[mode]["ok"], "aviso": SEM[mode]["amber"], "err": SEM[mode]["err"], "info": acc(mode)["fill"]}[kind]
    return (f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{h / 2}" fill="{th["fill"]}"/>'
            f'<circle cx="{x + 16}" cy="{y + h / 2}" r="4.5" fill="{col}"/>' + T(x + 30, y + h / 2 + 4.6, label, 13, 550, th["ink"]))


def field(x, y, w, label, value, mode, focus=False, h=48):
    th = THEME[mode]
    a = acc(mode)
    ring = f'<rect x="{x - 3}" y="{y - 3}" width="{w + 6}" height="{h + 6}" rx="{TK.RADIUS["field"] + 3}" fill="none" stroke="{a["text"]}" stroke-opacity="0.55" stroke-width="2"/>' if focus else ""
    return (ring + f'<rect x="{x + 0.5}" y="{y + 0.5}" width="{w - 1}" height="{h - 1}" rx="{TK.RADIUS["field"]}" fill="{th["card"]}" stroke="{a["text"] if focus else th["line"]}" stroke-width="{1.5 if focus else 1}"/>'
            + T(x + 16, y + 16, label, 11, 500, th["ink3"]) + T(x + 16, y + 37, value, 15.5, 500, th["ink"]))


def card(x, y, w, h, title, body, mode):
    th = THEME[mode]
    return (f'<rect x="{x + 0.5}" y="{y + 0.5}" width="{w - 1}" height="{h - 1}" rx="{TK.RADIUS["card"]}" fill="{th["card"]}" stroke="{th["line"]}"/>'
            + f'<circle cx="{x + 26}" cy="{y + 22}" r="5" fill="{acc(mode)["fill"]}"/>'
            + T(x + 42, y + 27, title, 15, 600, th["ink"]) + T(x + 24, y + 47, body, 13, 400, th["ink3"]))


def components(x, y, w, h, mode):
    th = THEME[mode]
    g = [panel(x, y, w, h, th["bg"])]
    px = x + 28
    g.append(button(px, y + 30, 210, 52, "Crear carpeta de entrega", mode, "primary", 15))
    g.append(button(px + 226, y + 30, 140, 52, "Cancelar", mode, "secondary", 15))
    g.append(chip(px + 388, y + 40, "Lo necesario", mode, 140))
    g.append(chip(px + 540, y + 40, "Opcional", mode, 108, checked=False))
    g.append(field(px, y + 112, 300, "TÍTULO", "Luz de noviembre", mode, focus=True))
    g.append(status(px + 330, y + 118, "Correcto", "ok", mode, 108))
    g.append(status(px + 448, y + 118, "Revisar", "aviso", mode, 100))
    g.append(status(px + 558, y + 118, "Error", "err", mode, 90))
    g.append(card(px, y + 180, w - 56, 62, "−9,1 LUFS · pasa de 0 dB", "Tarjeta: ficha con borde de 1 px, radio 24 y sin sombra.", mode))
    return "".join(g)


# ───────────────────────────── ventana de la app ─────────────────────────────
def shape_icon(k, cx, cy, col):
    if k == 0:
        return f'<circle cx="{cx}" cy="{cy}" r="9" fill="{col}"/>'
    if k == 1:
        return f'<rect x="{cx - 8}" y="{cy - 8}" width="16" height="16" rx="4" fill="{col}"/>'
    if k == 2:
        return f'<path d="M{cx - 9},{cy + 7} L{cx},{cy - 9} L{cx + 9},{cy + 7}Z" fill="{col}"/>'
    if k == 3:
        return f'<rect x="{cx - 4.5}" y="{cy - 10}" width="9" height="20" rx="4.5" fill="{col}"/>'
    if k == 4:
        return f'<circle cx="{cx}" cy="{cy}" r="8" fill="none" stroke="{col}" stroke-width="3"/>'
    return f'<path d="M{cx},{cy - 10} L{cx + 10},{cy} L{cx},{cy + 10} L{cx - 10},{cy}Z" fill="{col}"/>'


def window(ox, oy, scale, mode="light"):
    th = THEME[mode]
    a = acc(mode)
    bt = btn_colors(mode)
    c = ct(mode)
    ink, mute, line, card_, fill = th["ink"], th["ink3"], th["line"], th["card"], th["fill"]
    ww, wh = 760, 770
    g = [f'<defs><filter id="wsh" x="-10%" y="-10%" width="120%" height="130%"><feDropShadow dx="0" dy="24" stdDeviation="28" flood-color="#000" flood-opacity="{0.22 if mode == "light" else 0.6}"/></filter></defs>',
         f'<rect width="{ww}" height="{wh}" rx="22" fill="{th["bg"]}" filter="url(#wsh)"/>',
         f'<rect x="0.5" y="0.5" width="{ww - 1}" height="{wh - 1}" rx="22" fill="none" stroke="{line}"/>',
         f'<path d="M0,56 H{ww}" stroke="{line}"/>']
    for k, col in enumerate(("#FF5F57", "#FEBC2E", "#28C840")):
        g.append(f'<circle cx="{26 + k * 20}" cy="28" r="6.2" fill="{col}"/>')
    g.append(sym_at(mode, 34, 112, 28))
    m = wm(mode, 21, 134, 28)
    g.append(m.svg)
    xd = 134 + m.w + 18
    g.append(f'<path d="M{xd},19 V37" stroke="{line}"/>')
    g.append(T(xd + 16, 33, "master check", 15, 500, mute))
    g.append(T(ww - 70, 33, "Ajustes avanzados", 14, 400, mute, "end"))
    cx0, cy0, cw, chh = 38, 80, 684, 650
    g.append(f'<rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="30" fill="{card_}" stroke="{line}"/>')
    px = cx0 + 22
    iw = cw - 44
    y = cy0 + 22
    g.append(f'<rect x="{px}" y="{y}" width="{iw}" height="150" rx="22" fill="{fill}"/>')
    g.append(f'<circle cx="{px + 36}" cy="{y + 34}" r="20" fill="{a["fill"]}"/>')          # play = acento
    g.append(f'<path d="M{px + 31},{y + 25.5} L{px + 44},{y + 34} L{px + 31},{y + 42.5}Z" fill="{a["on"]}"/>')
    g.append(T(px + 68, y + 39, "3:52", 15, 500, mute))
    chip_w = 232
    g.append(f'<rect x="{px + iw - chip_w - 44}" y="{y + 18}" width="{chip_w}" height="32" rx="16" fill="{c["on-bg"]}"/>')
    g.append(f'<circle cx="{px + iw - chip_w - 44 + 16}" cy="{y + 34}" r="4.2" fill="{a["fill"]}"/>')
    g.append(T(px + iw - chip_w - 44 + 30, y + 39, "−9,1 LUFS · pasa de 0 dB", 13.5, 550, ink))
    nb, bw = 62, 5.0
    gap = (iw - 40 - bw * nb) / (nb - 1)
    base_y = y + 100
    played = int(nb * 0.36)
    for k in range(nb):
        t = k / (nb - 1)
        env = 0.28 + 0.72 * (math.sin(math.pi * t) ** 0.8)
        n_ = (math.sin(k * 1.7) * 0.5 + math.sin(k * 0.63 + 1.1) * 0.5) * 0.28 + 0.78
        hgt = max(6, 66 * env * n_)
        g.append(f'<rect x="{px + 20 + k * (bw + gap):.1f}" y="{base_y - hgt / 2:.1f}" width="{bw}" height="{hgt:.1f}" rx="{bw / 2}" fill="{ink if k < played else th["wv"]}"/>')
    hx = px + 20 + played * (bw + gap) - gap / 2
    g.append(f'<circle cx="{hx:.1f}" cy="{y + 140}" r="4.2" fill="{a["fill"]}"/>')
    y2 = y + 170
    g.append(clip(px, y2, 76, 76, 14, f'<rect x="{px}" y="{y2}" width="76" height="76" fill="#161616"/><circle cx="{px + 46}" cy="{y2 + 32}" r="12" fill="#EDE6D6"/>'
                  f'<rect x="{px}" y="{y2 + 46}" width="76" height="1.5" fill="#EDE6D6" fill-opacity="0.35"/>'))
    fw = (iw - 76 - 16 - 10) / 2
    for k, (lab, txt) in enumerate((("ARTISTA", "Nombre del artista"), ("TÍTULO", "Título de la canción"))):
        fx = px + 92 + k * (fw + 10)
        g.append(f'<rect x="{fx + 0.5}" y="{y2 + 0.5}" width="{fw - 1}" height="41" rx="12" fill="{card_}" stroke="{line}"/>')
        g.append(T(fx + 16, y2 + 27, txt, 15, 450, ink))
    g.append(T(px + 94, y2 + 66, "Más datos", 13.5, 400, mute))
    y3 = y2 + 104
    g.append(T(px, y3 + 5, "Plataformas", 14, 500, mute))
    g.append(f'<rect x="{px + iw - 250 - 90}" y="{y3 - 14}" width="124" height="30" rx="15" fill="{c["on-bg"]}"/>')
    g.append(ic_check(px + iw - 250 - 90 + 18, y3 + 1, 12, ink, 1.8))
    g.append(T(px + iw - 250 - 90 + 32, y3 + 6, "Lo necesario", 13, 550, ink))
    g.append(T(px + iw - 18, y3 + 5, "15 archivos · 525 MB", 13, 400, mute, "end"))
    tw, th_ = (iw - 50) / 6, 80
    ty = y3 + 24
    names = ["Streaming", "Vídeo", "Radio", "Social", "Tienda", "Descarga"]

    def tile(tx, ty_, name, k, sel=True):
        o = [f'<rect x="{tx:.1f}" y="{ty_}" width="{tw:.1f}" height="{th_}" rx="17" fill="{fill}" stroke="{ink if sel else line}" stroke-opacity="{0.85 if sel else 1}" stroke-width="{1.3 if sel else 1}"/>']
        gx = tx + tw / 2
        o.append(f'<rect x="{gx - 17:.1f}" y="{ty_ + 11}" width="34" height="34" rx="10" fill="{ink}"/>')
        o.append(shape_icon(k, round(gx, 1), ty_ + 28, card_))
        o.append(T(gx, ty_ + 66, name, 12.5, 500, ink, "middle"))
        if sel:
            o.append(f'<circle cx="{tx + tw - 12:.1f}" cy="{ty_ + 12}" r="8" fill="{a["fill"]}"/>')
            o.append(ic_check(tx + tw - 12, ty_ + 12, 9.5, a["on"], 1.7))
        return "".join(o)
    for k, n_ in enumerate(names):
        g.append(tile(px + k * (tw + 10), ty, n_, k))
    y4 = ty + th_ + 24
    g.append(T(px, y4, "Distribuidora", 14, 500, mute))
    g.append(T(px + 2 * (tw + 10), y4, "Extras", 14, 500, mute))
    ty2 = y4 + 12
    g.append(tile(px, ty2, "Estándar", 3))
    for k, n_ in enumerate(("MP3", "Portada", "Canvas")):
        g.append(tile(px + (2 + k) * (tw + 10), ty2, n_, 4 + (k % 2), sel=(k != 2)))
    by = ty2 + th_ + 20
    g.append(f'<rect x="{px}" y="{by}" width="{iw}" height="54" rx="16" fill="{bt["fill"]}"/>')          # botón primario = tinta
    g.append(T(px + iw / 2, by + 33, "Crear carpeta de entrega", 17, 600, bt["on"], "middle"))
    g.append(T(px + iw / 2 - 40, by + 54 + 33, "Escritorio/Entregas", 13.5, 400, mute, "middle"))
    g.append(T(px + iw / 2 + 62, by + 54 + 33, "cambiar", 13.5, 550, a["text"]))
    return f'<g transform="translate({ox},{oy}) scale({scale})">{"".join(g)}</g>'


def hero(x, y, w, h, mode="light"):
    th = THEME[mode]
    a = acc(mode)
    bt = btn_colors(mode)
    g = [f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="28" fill="{th["bg"]}"/>']
    inner = []
    inner.append(sym_at(mode, 40, x + 58, y + 55))
    m = wm(mode, 26, x + 90, y + 55)
    inner.append(m.svg)
    nx = x + w - 40
    inner.append(f'<rect x="{nx - 118}" y="{y + 36}" width="118" height="38" rx="19" fill="{bt["fill"]}"/>')
    inner.append(T(nx - 59, y + 60, "Descargar", 14.5, 600, bt["on"], "middle"))
    inner.append(T(nx - 150, y + 60, "Precios", 14.5, 450, th["ink3"], "end"))
    inner.append(T(nx - 222, y + 60, "master check", 14.5, 450, th["ink3"], "end"))
    # el gesto de la marca: el potenciómetro, a gran escala y recortado por el borde
    Rr = 205
    inner.append(glyph(x + w - 40, y + h - 20, Rr / KNOB["R"], th["ink"], a["fill"]))
    fs, ty = 66, y + 190
    for k, ln in enumerate(("Tu máster, listo", "para cada", "plataforma")):
        s, wd = head(x + 40, ty + k * fs * 1.04, ln, fs, th["ink"], dot=th["ink"] if k == 2 else None)
        inner.append(s)
    cy = ty + 2 * fs * 1.04 + 64
    inner.append(f'<rect x="{x + 40}" y="{cy}" width="196" height="48" rx="24" fill="{bt["fill"]}"/>')
    inner.append(T(x + 40 + 98, cy + 30, "Descargar para Mac", 15.5, 600, bt["on"], "middle"))
    inner.append(T(x + 262, cy + 30, "Windows", 15.5, 500, th["ink"]))
    g.append(clip(x, y, w, h, 28, "".join(inner)))
    return "".join(g)


# ───────────────────────────── kit.png ─────────────────────────────
W, H = 2400, 1500
CANVAS = "#D8D7D3"


def kit():
    g = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}">', f'<rect width="{W}" height="{H}" fill="{CANVAS}"/>']
    L, D_ = THEME["light"], THEME["dark"]
    c1x, c1w = 60, 740
    # columna 1: wordmark grande, wordmark + símbolo en claro y oscuro, paleta
    g.append(panel(c1x, 60, c1w, 460, L["bg"]))
    m0 = wm("light", 100, 0, 0)
    size = 100 * 600 / m0.w
    g.append(wm("light", size, c1x + (c1w - 600) / 2, 60 + 230).svg)
    for k, mode in enumerate(("light", "dark")):
        py = 540 + k * 250
        th = THEME[mode]
        g.append(panel(c1x, py, c1w, 230, th["bg"]))
        sz = 64
        m = wm(mode, sz, 0, 0)
        tot = 120 + 44 + m.w
        x = c1x + (c1w - tot) / 2
        g.append(sym_at(mode, 176, x + 60, py + 115))
        g.append(wm(mode, sz, x + 120 + 44, py + 115).svg)
    py, ph = 1060, 380
    g.append(panel(c1x, py, c1w, ph, CANVAS))
    chips = [(INK, "tinta"), (PAPER, "papel"), (ACCENT["light"], "cian · relleno"), (ACCENT_TEXT_LIGHT, "cian · texto"),
             (THEME["light"]["ink3"], "gris «studio»"), (SEM["light"]["ok"], "correcto")]
    cw = (c1w - 40 - 5 * 12) / 6
    for k, (col, nm) in enumerate(chips):
        cx = c1x + 20 + k * (cw + 12)
        g.append(f'<rect x="{cx:.1f}" y="{py + 20}" width="{cw:.1f}" height="{ph - 40}" rx="18" fill="{col}" stroke="{"rgba(0,0,0,.10)" if k == 1 else "none"}"/>')
        tc = "#111" if lum(col) > 0.25 else "#fff"
        g.append(T(cx + 12, py + ph - 56, nm, 12, 500, tc, op=0.85))
        g.append(T(cx + 12, py + ph - 36, col.upper(), 12.5, 500, tc, op=0.85))
    c2x, c2w = 820, 760
    g.append(panel(c2x, 60, c2w, 840, "#EEEDE9"))
    g.append(window(c2x + (c2w - 760 * 0.92) / 2, 60 + (840 - 770 * 0.92) / 2, 0.92))
    g.append(components(c2x, 920, c2w, 250, "light"))
    g.append(components(c2x, 1190, c2w, 250, "dark"))
    c3x, c3w = 1600, 740
    g.append(hero(c3x, 60, c3w, 580))
    for k, mode in enumerate(("light", "dark")):
        py = 660 + k * 240
        g.append(panel(c3x, py, c3w, 220, THEME[mode]["bg"] if mode == "light" else "#262626"))
        isz = 190
        for j, kind in enumerate(("light", "dark", "sym")):
            ix = c3x + (c3w - (3 * isz + 2 * 30)) / 2 + j * (isz + 30)
            if kind == "sym":
                g.append(sym_at(mode, 220, ix + isz / 2, py + 110))
            else:
                g.append(icon_at(kind, isz, ix, py + 15))
    for k, mode in enumerate(("light", "dark")):
        py = 1140 + k * 160
        g.append(panel(c3x, py, c3w, 140, THEME[mode]["bg"] if mode == "light" else "#262626"))
        sizes = [128, 64, 48, 32, 24, 16]
        tot = sum(sizes) + 5 * 34
        x = c3x + (c3w - tot) / 2
        for s in sizes:
            g.append(icon_at(mode, s, x, py + (140 - s) / 2, shadow=s >= 32))
            x += s + 34
    g.append("</svg>")
    return "".join(g), W, H


# ───────────────────────────── familia.png ─────────────────────────────
FW, FH = 2400, 2150


def familia():
    g = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{FW}" height="{FH}" viewBox="0 0 {FW} {FH}">', f'<rect width="{FW}" height="{FH}" fill="{CANVAS}"/>']
    y0 = 60
    pw = 1110
    for k, mode in enumerate(("light", "dark")):
        th = THEME[mode]
        x = 60 + k * (pw + 60)
        g.append(panel(x, y0, pw, 960, th["bg"]))
        g.append(T(x + 60, y0 + 62, "claro" if mode == "light" else "oscuro", 15, 500, th["ink3"]))
        # marca
        g.append(wm(mode, 84, x + 60, y0 + 150).svg)
        g.append(T(x + pw - 60, y0 + 156, "marca · punto y letras en tinta", 14, 400, th["ink3"], "end"))
        # app actual
        svg, _ = app_header(mode, x + 60, y0 + 270, 46)
        g.append(svg)
        g.append(T(x + pw - 60, y0 + 276, "app actual", 14, 400, th["ink3"], "end"))
        # app anterior, misma familia
        svg, _ = app_header(mode, x + 60, y0 + 380, 46, app="Facturación")
        g.append(svg)
        g.append(T(x + pw - 60, y0 + 386, "app anterior · misma familia", 14, 400, th["ink3"], "end"))
        # hueco para apps futuras
        g.append(f'<rect x="{x + 60}" y="{y0 + 450}" width="{pw - 120}" height="76" rx="16" fill="none" stroke="{th["ctl"]}" stroke-width="1.5" stroke-dasharray="7 7"/>')
        g.append(T(x + 90, y0 + 495, "ferboix. studio  |  app futura", 21, 400, th["ink3"]))
        g.append(T(x + pw - 60, y0 + 576, "hueco para apps futuras", 14, 400, th["ink3"], "end"))
        # iconos
        if mode == "dark":
            g.append(panel(x + 20, y0 + 620, pw - 40, 320, "#262626", 24))
        ix = x + 60
        g.append(icon_at(mode, 230, ix - 10, y0 + 650))
        g.append(T(ix + 105, y0 + 912, "master check", 13, 400, th["ink3"], "middle"))
        for j in range(2):
            fx = ix + 250 + j * 250
            sq = squircle(fx + 105, y0 + 765, 190)
            g.append(f'<path d="{sq}" fill="none" stroke="{th["ctl"]}" stroke-width="1.5" stroke-dasharray="7 7"/>')
            g.append(T(fx + 105, y0 + 912, "app futura", 13, 400, th["ink3"], "middle"))
        g.append(sym_at(mode, 230 * 0.9, ix + 250 + 2 * 250 + 190, y0 + 780))
        g.append(T(ix + 250 + 2 * 250 + 190, y0 + 912, "símbolo", 13, 400, th["ink3"], "middle"))
    # estados y acento: botón tinta, secundario, ficha, estado y campo
    y1 = y0 + 1020
    for k, mode in enumerate(("light", "dark")):
        th = THEME[mode]
        a = acc(mode)
        x = 60 + k * (pw + 60)
        g.append(panel(x, y1, pw, 400, th["bg"]))
        cx = x + 40
        g.append(T(cx, y1 + 52, f"acento único · {ACCENT['label']} {a['fill']}", 15, 500, th["ink"]))
        g.append(T(cx, y1 + 76, "botón primario en tinta; el cian solo marca estado, selección, lectura y enlaces", 13, 400, th["ink3"]))
        g.append(button(cx, y1 + 110, 260, 52, "Primario", mode, "primary", 15))
        g.append(button(cx + 280, y1 + 110, 220, 52, "Secundario", mode, "secondary", 15))
        g.append(chip(cx + 530, y1 + 120, "Seleccionado", mode, 170))
        g.append(status(cx + 720, y1 + 121, "Lectura", "info", mode, 150))
        g.append(field(cx, y1 + 210, 400, "CAMPO", "En foco", mode, focus=True, h=48))
        g.append(T(cx + 440, y1 + 232, "enlace", 15, 500, a["text"]))
        g.append(T(cx + 440, y1 + 254, f"texto-acento {a['text']} · {ratio(a['text'], th['bg']):.1f}:1", 13, 400, th["ink3"]))
        g.append(status(cx, y1 + 300, "Correcto", "ok", mode, 130))
        g.append(status(cx + 150, y1 + 300, "Revisar", "aviso", mode, 120))
        g.append(status(cx + 290, y1 + 300, "Error", "err", mode, 110))
    # escala de iconos
    y2 = y1 + 460
    for k, mode in enumerate(("light", "dark")):
        th = THEME[mode]
        py = y2 + k * 320
        g.append(panel(60, py, FW - 120, 300, th["bg"] if mode == "light" else "#262626"))
        g.append(T(120, py + 48, "icono · " + ("claro" if mode == "light" else "oscuro") + " · 16 · 32 · 64 · 128 · 256 · 512 (mostrado a 256)", 15, 500, th["ink3"]))
        sizes = [16, 32, 64, 128, 256, 256]
        labs = ["16", "32", "64", "128", "256", "512"]
        tot = sum(sizes) + 5 * 60
        x = 60 + (FW - 120 - tot) / 2
        base = py + 270
        for s, lb in zip(sizes, labs):
            g.append(icon_at(mode, s, x, base - s - 20, shadow=s >= 32))
            g.append(T(x + s / 2, base, lb, 13, 400, th["ink3"], "middle"))
            x += s + 60
    g.append("</svg>")
    return "".join(g), FW, FH
