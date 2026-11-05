"""Láminas: kit.png (todo el sistema, estilo A.png) y familia.png (madre / studio / lab, claro y oscuro)."""
import math
from brand import *
from brand import _M
from draw import *
import tokens as TK


def ct(brand, mode):
    return TK.color_tokens(brand, mode)


def wm(who, mode, size, x, y_center):
    """wordmark de la familia centrado en vertical sobre x-height."""
    v = {("madre", "light"): "tinta", ("madre", "dark"): "papel"}.get((who, mode), "color" if mode == "light" else "color-oscuro")
    ink, dot, sfx, op = mark_colors(who, v)
    xh = font(SPEC["font"], **SPEC["axes"])["OS/2"].sxHeight / 1000 * size
    m = mark(who, size, x, y_center + xh / 2, ink, dot, sfx, op)
    return m


def icon_at(who, mode, size, x, y, shadow=True):
    inner, _ = icon_svg(who, mode, size, margin=True, shadow=shadow, uid=uid("i"))
    return f'<g transform="translate({x:.1f},{y:.1f})">{inner}</g>'


# ───────────────────────────── componentes ─────────────────────────────
def button(x, y, w, h, label, brand, mode, kind="primary", size=16):
    a = acc(brand, mode)
    th = THEME[mode]
    if kind == "primary":
        return (f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{RADIUS_B}" fill="{a["fill"]}"/>'
                + T(x + w / 2, y + h / 2 + size * 0.35, label, size, 600, a["on"], "middle"))
    return (f'<rect x="{x + 0.75}" y="{y + 0.75}" width="{w - 1.5}" height="{h - 1.5}" rx="{RADIUS_B}" fill="none" stroke="{th["ink"]}" stroke-opacity="0.85" stroke-width="1.5"/>'
            + T(x + w / 2, y + h / 2 + size * 0.35, label, size, 600, th["ink"], "middle"))


RADIUS_B = TK.RADIUS["button"]


def chip(x, y, label, brand, mode, w=140, h=32, checked=True):
    th, c = THEME[mode], ct(brand, mode)
    g = f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{h / 2}" fill="{c["on-bg"]}"/>'
    tx = x + 14
    if checked:
        g += ic_check(x + 20, y + h / 2, 12, th["ink"], 1.9)
        tx = x + 34
    return g + T(tx, y + h / 2 + 4.6, label, 13, 550, th["ink"])


def status(x, y, label, kind, brand, mode, w=120, h=30):
    th = THEME[mode]
    col = {"ok": SEM[mode]["ok"], "aviso": SEM[mode]["amber"], "err": SEM[mode]["err"], "info": acc(brand, mode)["fill"]}[kind]
    return (f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{h / 2}" fill="{th["fill"]}"/>'
            f'<circle cx="{x + 16}" cy="{y + h / 2}" r="4.5" fill="{col}"/>' + T(x + 30, y + h / 2 + 4.6, label, 13, 550, th["ink"]))


def field(x, y, w, label, value, brand, mode, focus=False, h=48):
    th = THEME[mode]
    a = acc(brand, mode)
    ring = f'<rect x="{x - 3}" y="{y - 3}" width="{w + 6}" height="{h + 6}" rx="{TK.RADIUS["field"] + 3}" fill="none" stroke="{a["text"]}" stroke-opacity="0.55" stroke-width="2"/>' if focus else ""
    return (ring + f'<rect x="{x + 0.5}" y="{y + 0.5}" width="{w - 1}" height="{h - 1}" rx="{TK.RADIUS["field"]}" fill="{th["card"]}" stroke="{a["text"] if focus else th["line"]}" stroke-width="{1.5 if focus else 1}"/>'
            + T(x + 16, y + 16, label, 11, 500, th["ink3"]) + T(x + 16, y + 37, value, 15.5, 500, th["ink"]))


def card(x, y, w, h, title, body, mode, brand):
    th = THEME[mode]
    return (f'<rect x="{x + 0.5}" y="{y + 0.5}" width="{w - 1}" height="{h - 1}" rx="{TK.RADIUS["card"]}" fill="{th["card"]}" stroke="{th["line"]}"/>'
            + f'<circle cx="{x + 26}" cy="{y + 22}" r="5" fill="{acc(brand, mode)["fill"]}"/>'
            + T(x + 42, y + 27, title, 15, 600, th["ink"]) + T(x + 24, y + 47, body, 13, 400, th["ink3"]))


def components(x, y, w, h, mode, brand="studio"):
    th = THEME[mode]
    g = [panel(x, y, w, h, th["bg"])]
    px = x + 28
    g.append(button(px, y + 30, 210, 52, "Crear carpeta de entrega", brand, mode, "primary", 15))
    g.append(button(px + 226, y + 30, 140, 52, "Cancelar", brand, mode, "secondary", 15))
    g.append(chip(px + 388, y + 40, "Lo necesario", brand, mode, 140))
    g.append(chip(px + 540, y + 40, "Opcional", brand, mode, 108, checked=False))
    g.append(field(px, y + 112, 300, "TÍTULO", "Luz de noviembre", brand, mode, focus=True))
    g.append(status(px + 330, y + 118, "Correcto", "ok", brand, mode, 108))
    g.append(status(px + 448, y + 118, "Revisar", "aviso", brand, mode, 100))
    g.append(status(px + 558, y + 118, "Error", "err", brand, mode, 90))
    g.append(card(px, y + 180, w - 56, 62, "−9,1 LUFS · pasa de 0 dB", "Tarjeta: ficha con borde de 1 px, radio 24 y sin sombra.", mode, brand))
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


def window(ox, oy, scale, mode="light", brand="studio"):
    th = THEME[mode]
    a = acc(brand, mode)
    c = ct(brand, mode)
    ink, mute, line, card_, fill = th["ink"], th["ink3"], th["line"], th["card"], th["fill"]
    ww, wh = 760, 770
    g = [f'<defs><filter id="wsh" x="-10%" y="-10%" width="120%" height="130%"><feDropShadow dx="0" dy="24" stdDeviation="28" flood-color="#000" flood-opacity="{0.22 if mode == "light" else 0.6}"/></filter></defs>',
         f'<rect width="{ww}" height="{wh}" rx="22" fill="{th["bg"]}" filter="url(#wsh)"/>',
         f'<rect x="0.5" y="0.5" width="{ww - 1}" height="{wh - 1}" rx="22" fill="none" stroke="{line}"/>',
         f'<path d="M0,56 H{ww}" stroke="{line}"/>']
    for k, col in enumerate(("#FF5F57", "#FEBC2E", "#28C840")):
        g.append(f'<circle cx="{26 + k * 20}" cy="28" r="6.2" fill="{col}"/>')
    m = wm(brand, mode, 21, 104, 28)
    g.append(m.svg)
    xd = 104 + m.w + 18
    g.append(f'<path d="M{xd},19 V37" stroke="{line}"/>')
    g.append(T(xd + 16, 33, "master check", 15, 500, mute))
    g.append(T(ww - 70, 33, "Ajustes avanzados", 14, 400, mute, "end"))
    cx0, cy0, cw, chh = 38, 80, 684, 650
    g.append(f'<rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="30" fill="{card_}" stroke="{line}"/>')
    px = cx0 + 22
    iw = cw - 44
    y = cy0 + 22
    g.append(f'<rect x="{px}" y="{y}" width="{iw}" height="150" rx="22" fill="{fill}"/>')
    g.append(f'<circle cx="{px + 36}" cy="{y + 34}" r="20" fill="{ink}"/>')
    g.append(f'<path d="M{px + 31},{y + 25.5} L{px + 44},{y + 34} L{px + 31},{y + 42.5}Z" fill="{card_}"/>')
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
    g.append(f'<rect x="{px}" y="{by}" width="{iw}" height="54" rx="16" fill="{a["fill"]}"/>')
    g.append(T(px + iw / 2, by + 33, "Crear carpeta de entrega", 17, 600, a["on"], "middle"))
    g.append(T(px + iw / 2 - 40, by + 54 + 33, "Escritorio/Entregas", 13.5, 400, mute, "middle"))
    g.append(T(px + iw / 2 + 62, by + 54 + 33, "cambiar", 13.5, 550, a["text"]))
    return f'<g transform="translate({ox},{oy}) scale({scale})">{"".join(g)}</g>'


def hero(x, y, w, h, mode="light"):
    th = THEME[mode]
    a = acc("studio", mode)
    g = [f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="28" fill="{th["bg"]}"/>']
    inner = []
    m = wm("studio", mode, 26, x + 40, y + 55)
    inner.append(m.svg)
    nx = x + w - 40
    inner.append(f'<rect x="{nx - 118}" y="{y + 36}" width="118" height="38" rx="19" fill="{a["fill"]}"/>')
    inner.append(T(nx - 59, y + 60, "Descargar", 14.5, 600, a["on"], "middle"))
    inner.append(T(nx - 150, y + 60, "Precios", 14.5, 450, th["ink3"], "end"))
    inner.append(T(nx - 222, y + 60, "master check", 14.5, 450, th["ink3"], "end"))
    # el gesto de studio: un punto que se estira a barra, a gran escala
    bw = 170
    inner.append(f'<rect x="{x + w - bw - 70}" y="{y + h - 380}" width="{bw}" height="460" rx="{bw / 2}" fill="{a["fill"]}"/>')
    fs, ty = 66, y + 190
    for k, ln in enumerate(("Tu máster, listo", "para cada", "plataforma")):
        s, wd = head(x + 40, ty + k * fs * 1.04, ln, fs, th["ink"], dot=a["fill"] if k == 2 else None)
        inner.append(s)
    cy = ty + 2 * fs * 1.04 + 64
    inner.append(f'<rect x="{x + 40}" y="{cy}" width="196" height="48" rx="24" fill="{a["fill"]}"/>')
    inner.append(T(x + 40 + 98, cy + 30, "Descargar para Mac", 15.5, 600, a["on"], "middle"))
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
    g.append(panel(c1x, 60, c1w, 460, L["bg"]))
    m0 = mark("madre", 100, 0, 0, INK, INK)
    size = 100 * 560 / m0.w
    xh = font(SPEC["font"], **SPEC["axes"])["OS/2"].sxHeight / 1000 * size
    g.append(mark("madre", size, c1x + (c1w - 560) / 2, 60 + 230 + xh / 2, INK, INK).svg)
    for k, mode in enumerate(("light", "dark")):
        py = 540 + k * 250
        th = THEME[mode]
        g.append(panel(c1x, py, c1w, 230, th["bg"]))
        sz, gp = 58, 44
        ms = [wm(w_, mode, sz, 0, 0) for w_ in ("madre", "studio", "lab")]
        tot = sum(m.w for m in ms) + 2 * gp
        sz *= min(1.0, (c1w - 80) / tot)
        ms = [wm(w_, mode, sz, 0, 0) for w_ in ("madre", "studio", "lab")]
        tot = sum(m.w for m in ms) + 2 * gp
        x = c1x + (c1w - tot) / 2
        for i, w_ in enumerate(("madre", "studio", "lab")):
            m = wm(w_, mode, sz, x, py + 115)
            g.append(m.svg)
            if i == 0:
                g.append(f'<path d="M{x + m.w + gp / 2:.1f},{py + 80} V{py + 150}" stroke="{th["line"]}" stroke-width="1.5"/>')
            x += m.w + gp
    py, ph = 1060, 380
    g.append(panel(c1x, py, c1w, ph, CANVAS))
    chips = [(INK, "tinta"), (PAPER, "papel"), (ACCENTS["studio"]["light"], "studio"), (ACCENTS["lab"]["light"], "lab"),
             (ACCENTS["teal"]["light"], "reserva"), (ACCENTS["magenta"]["light"], "reserva")]
    cw = (c1w - 40 - 5 * 12) / 6
    for k, (col, nm) in enumerate(chips):
        cx = c1x + 20 + k * (cw + 12)
        g.append(f'<rect x="{cx:.1f}" y="{py + 20}" width="{cw:.1f}" height="{ph - 40}" rx="18" fill="{col}" stroke="{"rgba(0,0,0,.10)" if k == 1 else "none"}"/>')
        tc = "#111" if lum(col) > 0.25 else "#fff"
        g.append(T(cx + 12, py + ph - 56, nm, 13, 500, tc, op=0.85))
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
        for j, who in enumerate(("madre", "studio", "lab")):
            isz = 190
            ix = c3x + (c3w - (3 * isz + 2 * 30)) / 2 + j * (isz + 30)
            g.append(icon_at(who, mode, isz, ix, py + 15))
    for k, mode in enumerate(("light", "dark")):
        py = 1140 + k * 160
        g.append(panel(c3x, py, c3w, 140, THEME[mode]["bg"] if mode == "light" else "#262626"))
        sizes = [128, 64, 48, 32, 24, 16]
        tot = sum(sizes) + 5 * 34
        x = c3x + (c3w - tot) / 2
        for s in sizes:
            g.append(icon_at("studio", mode, s, x, py + (140 - s) / 2, shadow=s >= 32))
            x += s + 34
    g.append("</svg>")
    return "".join(g), W, H


# ───────────────────────────── familia.png ─────────────────────────────
FW, FH = 2400, 2760
RESERVA = {"teal": "reserva 1", "magenta": "reserva 2"}


def familia():
    g = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{FW}" height="{FH}" viewBox="0 0 {FW} {FH}">', f'<rect width="{FW}" height="{FH}" fill="{CANVAS}"/>']
    y0 = 60
    pw = 1140
    for k, mode in enumerate(("light", "dark")):
        th = THEME[mode]
        x = 60 + k * (pw + 60)
        g.append(panel(x, y0, pw, 740, th["bg"]))
        g.append(T(x + 60, y0 + 62, "claro" if mode == "light" else "oscuro", 15, 500, th["ink3"]))
        for i, (who, cap) in enumerate((("madre", "madre · sin acento"), ("studio", f"studio · {ACCENTS['studio']['light']}"), ("lab", f"lab · {ACCENTS['lab'][mode]} (ejemplo)"))):
            yc = y0 + 140 + i * 118
            g.append(wm(who, mode, 74, x + 60, yc).svg)
            g.append(T(x + pw - 60, yc + 6, cap, 14, 400, th["ink3"], "end"))
        if mode == "dark":
            g.append(panel(x + 20, y0 + 440, pw - 40, 250, "#262626", 24))
        for j, who in enumerate(("madre", "studio", "lab", "teal", "magenta")):
            ix = x + 60 + j * 208
            g.append(icon_at(who, mode, 190, ix - 10, y0 + 470))
            g.append(T(ix + 85, y0 + 712, who if who not in RESERVA else RESERVA[who], 13, 400, th["ink3"], "middle"))
    # acentos: botón, ficha, estado y campo en cada acento
    y1 = y0 + 800
    for k, mode in enumerate(("light", "dark")):
        th = THEME[mode]
        x = 60 + k * (pw + 60)
        g.append(panel(x, y1, pw, 560, th["bg"]))
        cw = (pw - 60) / 4
        for j, b in enumerate(("studio", "lab", "teal", "magenta")):
            a = acc(b, mode)
            cx = x + 30 + j * cw
            nm = f"{ACCENTS[b]['label']} · {a['fill']}" + (" (ejemplo)" if b == "lab" else "")
            g.append(T(cx + 4, y1 + 52, b if b not in RESERVA else RESERVA[b], 13, 400, th["ink3"]))
            g.append(T(cx + 4, y1 + 74, nm, 15, 500, th["ink"]))
            mm = wm("studio" if b == "studio" else b if b in ("lab",) else "studio", mode, 30, cx + 4, y1 + 130)
            ink_, dot_, sfx_, op_ = mark_colors("studio", "color" if mode == "light" else "color-oscuro")
            word = WORD[b]
            m2 = wordmark(SPEC, 30, cx + 4 - _M["lsb"] * 30, y1 + 138, ink_, a["fill"], suffix=word, suffix_ink=th["ink3"])
            g.append(m2.svg)
            g.append(button(cx, y1 + 170, cw - 30, 48, "Primario", b, mode, "primary", 15))
            g.append(button(cx, y1 + 232, cw - 30, 48, "Secundario", b, mode, "secondary", 15))
            g.append(chip(cx, y1 + 308, "Seleccionado", b, mode, cw - 30))
            g.append(status(cx, y1 + 358, "Lectura", "info", b, mode, cw - 30))
            g.append(field(cx, y1 + 410, cw - 30, "CAMPO", "En foco", b, mode, focus=True, h=48))
            g.append(T(cx + 4, y1 + 498, "texto-acento", 12, 400, th["ink3"]))
            g.append(T(cx + 4, y1 + 522, f"{acc(b, mode)['text']}  ·  {ratio(a['text'], th['bg']):.1f}:1", 14, 600, a["text"]))
    # escala de iconos
    y2 = y1 + 620
    for k, mode in enumerate(("light", "dark")):
        th = THEME[mode]
        py = y2 + k * 640
        g.append(panel(60, py, FW - 120, 600, th["bg"] if mode == "light" else "#262626"))
        g.append(T(120, py + 56, "icono studio · " + ("claro" if mode == "light" else "oscuro") + " · 16 · 32 · 64 · 128 · 256 · 512 · 1024 (mostrado a 512)", 15, 500, th["ink3"]))
        sizes = [16, 32, 64, 128, 256, 512, 512]
        labs = ["16", "32", "64", "128", "256", "512", "1024"]
        tot = sum(sizes) + 6 * 44
        x = 60 + (FW - 120 - tot) / 2
        base = py + 560 - 36
        for s, lb in zip(sizes, labs):
            g.append(icon_at("studio", mode, s, x, base - s, shadow=s >= 32))
            g.append(T(x + s / 2, base + 26, lb, 13, 400, th["ink3"], "middle"))
            x += s + 44
    g.append("</svg>")
    return "".join(g), FW, FH
