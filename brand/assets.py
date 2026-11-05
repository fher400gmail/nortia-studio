"""Entregables de archivo: logos, avatares, favicons, banners, iconos macOS/Windows, plantillas."""
import os, io, json, shutil
from PIL import Image
from brand import *
from brand import _M
from draw import *
import tokens as TK

OUT = os.path.dirname(os.path.abspath(__file__))
BUILD = os.path.join(OUT, "_build")


def P(*a):
    p = os.path.join(OUT, *a)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    return p


def save_svg(path, svg):
    open(P(path), "w", encoding="utf-8").write('<?xml version="1.0" encoding="UTF-8"?>\n' + svg)


# ───────────────────────────── 2. logos ─────────────────────────────
def logos(jobs):
    items = []
    for v in ("tinta", "papel", "blanco", "negro"):
        items.append((f"nortia-wordmark-{v}", lambda pw=None, v=v: wordmark_svg("madre", v, pw)))
    for v in ("color", "color-oscuro", "blanco", "negro"):
        items.append((f"nortia-studio-wordmark-{v}", lambda pw=None, v=v: wordmark_svg("studio", v, pw)))
    for v in ("color", "color-oscuro"):
        items.append((f"nortia-lab-wordmark-{v}-ejemplo", lambda pw=None, v=v: wordmark_svg("lab", v, pw)))
    sym = [("nortia-simbolo-punto-tinta", "madre", INK), ("nortia-simbolo-punto-papel", "madre", PAPER),
           ("nortia-simbolo-punto-negro", "madre", "#000000"),
           ("nortia-studio-simbolo-barra-naranja", "studio", ACCENTS["studio"]["light"]),
           ("nortia-studio-simbolo-barra-blanco", "studio", "#FFFFFF"), ("nortia-studio-simbolo-barra-negro", "studio", "#000000"),
           ("nortia-lab-simbolo-ejemplo-azul", "lab", ACCENTS["lab"]["light"])]
    for name, who, col in sym:
        items.append((name, lambda pw=None, who=who, col=col: symbol_svg(who, col, pw)))
    for who, nm in (("madre", "nortia"), ("studio", "nortia-studio"), ("lab", "nortia-lab-ejemplo")):
        for mode, mn in (("light", "claro"), ("dark", "oscuro")):
            if who == "lab" and mode == "dark":
                pass
            items.append((f"{nm}-lockup-vertical-{mn}", lambda pw=None, who=who, mode=mode: lockup_svg(who, mode, pw)))
    for name, fn in items:
        svg, w, h = fn(None)
        save_svg(f"logos/svg/{name}.svg", svg)
        for px in (512, 2048):
            s, w, h = fn(px)
            jobs.append((s, w, h, P(f"logos/png/{name}-{px}.png"), True))
    return [n for n, _ in items]


# ───────────────────────────── 3. avatares / favicon / iconos ─────────────────────────────
def avatars(jobs):
    for who in ("madre", "studio"):
        for mode, mn in (("light", "claro"), ("dark", "oscuro")):
            for px in (1000, 400):
                inner, _ = icon_svg(who, mode, px, margin=False, shape="square", gscale=1.2)
                s, w, h = doc(inner, 0, 0, px, px)
                nm = "nortia" if who == "madre" else "nortia-studio"
                jobs.append((s, w, h, P(f"avatares/{nm}-avatar-{mn}-{px}.png"), False))


OPT = {16: 1.28, 24: 1.2, 32: 1.16, 48: 1.1, 64: 1.06}


def tile_png_job(jobs, path, who, mode, px, shape="round", pad=0.0, optical=True, transparent=True):
    g = OPT.get(px, 1.0) if optical else 1.0
    inner, _ = icon_svg(who, mode, px, margin=False, shape=shape, gscale=g, pad=pad)
    s, w, h = doc(inner, 0, 0, px, px)
    jobs.append((s, w, h, path, transparent))


def favicons(jobs, icos):
    for who in ("madre", "studio"):
        nm = "nortia" if who == "madre" else "nortia-studio"
        for px in (16, 32, 48):
            tile_png_job(jobs, os.path.join(BUILD, f"fav-{who}-{px}.png"), who, "light", px)
        icos.append((P(f"favicon/{nm}-favicon.ico"), [os.path.join(BUILD, f"fav-{who}-{px}.png") for px in (16, 32, 48)]))
        tile_png_job(jobs, P(f"favicon/{nm}-favicon-32.png"), who, "light", 32)
        inner, _ = icon_svg(who, "light", 180, margin=False, shape="square", gscale=1.0)
        s, w, h = doc(inner, 0, 0, 180, 180)
        jobs.append((s, w, h, P(f"favicon/{nm}-apple-touch-icon-180.png"), False))
        inner, _ = icon_svg(who, "light", 512, margin=False, shape="round", gscale=1.0)
        save_svg(f"favicon/{nm}-favicon.svg", doc(inner, 0, 0, 512, 512)[0])


def macos(jobs):
    for who in ("studio", "madre"):
        nm = "nortia-studio" if who == "studio" else "nortia"
        for mode, mn in (("light", "claro"), ("dark", "oscuro")):
            s, w, h = icon_doc(who, mode, 1024)
            jobs.append((s, w, h, P(f"iconos/macos/{nm}-{mn}-1024.png"), True))
            save_svg(f"iconos/macos/svg/{nm}-{mn}.svg", s)
            s2, w, h = doc(icon_svg(who, mode, 1024, margin=False, shape="square", gscale=824 / 1024)[0], 0, 0, 1024, 1024)
            jobs.append((s2, w, h, P(f"iconos/macos/fullbleed/{nm}-{mn}-1024-fullbleed.png"), False))


WIN_SIZES = (16, 24, 32, 48, 64, 128, 256)


def windows(jobs, icos):
    for who in ("studio", "madre"):
        nm = "nortia-studio" if who == "studio" else "nortia"
        for mode, mn in (("light", "claro"), ("dark", "oscuro")):
            files = []
            for px in WIN_SIZES:
                f = os.path.join(BUILD, f"win-{who}-{mode}-{px}.png")
                pad = 0.0 if px <= 48 else 0.03
                tile_png_job(jobs, f, who, mode, px, shape="round", pad=pad)
                files.append(f)
            icos.append((P(f"iconos/windows/{nm}-{mn}.ico"), files))
            tile_png_job(jobs, P(f"iconos/windows/png/{nm}-{mn}-256.png"), who, mode, 256, pad=0.03)


def pack_icos(icos):
    for path, files in icos:
        write_ico(path, [(Image.open(f).size[0], open(f, "rb").read()) for f in files])


# ───────────────────────────── legibilidad de iconos ─────────────────────────────
LEG_SIZES = (16, 32, 128, 512, 1024)


def legibility_jobs(jobs):
    for who in ("madre", "studio"):
        for mode in ("light", "dark"):
            for px in LEG_SIZES:
                s, w, h = icon_doc(who, mode, px)
                jobs.append((s, w, h, os.path.join(BUILD, f"leg-{who}-{mode}-{px}.png"), True))


def legibility_measure():
    res = []
    for who in ("madre", "studio"):
        for mode in ("light", "dark"):
            bgc = hex2rgb(tile_colors(mode)[0])
            gc = hex2rgb(glyph_col(who, mode))
            for px in LEG_SIZES:
                im = Image.open(os.path.join(BUILD, f"leg-{who}-{mode}-{px}.png")).convert("RGBA")
                cx = cy = px // 2

                def isg(p):
                    d = lambda a, b: sum((a[i] - b[i]) ** 2 for i in range(3))
                    return p[3] > 200 and d(p, gc) < d(p, bgc)
                row = sum(1 for x in range(px) if isg(im.getpixel((x, cy))))
                col = sum(1 for y in range(px) if isg(im.getpixel((cx, y))))
                res.append(dict(marca=who, modo=mode, px=px, ancho=row, alto=col, contraste=round(ratio(glyph_col(who, mode), tile_colors(mode)[0]), 2),
                                ok=row >= 3 and col >= 3))
    return res


# ───────────────────────────── banners ─────────────────────────────
TAG = {"madre": "Software tranquilo y exacto para quien hace música.",
       "studio": "Herramientas de audio para publicar sin sorpresas."}


def banner(who, mode, W, H):
    th = THEME[mode]
    a = acc("studio", mode)
    g = [f'<rect width="{W}" height="{H}" fill="{th["bg"]}"/>']
    variant = {("madre", "light"): "tinta", ("madre", "dark"): "papel", ("studio", "light"): "color", ("studio", "dark"): "color-oscuro"}[(who, mode)]
    ink, dot, sfx, op = mark_colors(who, variant)
    wide = W / H > 2.5
    size = H * (0.30 if who == "madre" else 0.25)
    if who == "studio":
        D = H * 0.062
        gap = D * 0.62
        mw = 0.30 * W
        n = int((mw + gap) / (D + gap))
        mw = n * D + (n - 1) * gap
        x0 = W * 0.94 - mw
        x = W * (0.16 if wide else 0.08)
        m0 = mark(who, size, 0, 0, ink, dot, sfx, op)
        size = min(size, size * (x0 - x - 0.04 * W) / m0.w)
    m0 = mark(who, size, 0, 0, ink, dot, sfx, op)
    if who == "madre":
        x = (W - m0.w) / 2
    cy = H * 0.50
    base = cy + (_M["top"] * size) / 2 - H * 0.02
    g.append(mark(who, size, x, base, ink, dot, sfx, op).svg)
    tg = H * 0.050
    ty = base + size * 0.50
    g.append(T(x if who == "studio" else W / 2, ty, TAG[who], tg, 400, th["ink3"], "start" if who == "studio" else "middle"))
    if who == "studio":
        g.append(pills(x0, H * 0.80 if wide else H * 0.84, n, D, gap, mode, a["fill"], n - 4, maxh=H * 0.42))
    return doc("".join(g), 0, 0, W, H)


def banners(jobs):
    for who in ("madre", "studio"):
        nm = "nortia" if who == "madre" else "nortia-studio"
        for mode, mn in (("light", "claro"), ("dark", "oscuro")):
            for tag, W, H in (("x-1500x500", 1500, 500), ("social-1280x640", 1280, 640)):
                s, w, h = banner(who, mode, W, H)
                jobs.append((s, w, h, P(f"banners/{nm}-banner-{tag}-{mn}.png"), False))


# ───────────────────────────── plantillas ─────────────────────────────
def cover(W, H, mode):
    th = THEME[mode]
    a = acc("studio", mode)
    sq = W == H
    pad = 0.075 * W
    g = [f'<rect width="{W}" height="{H}" fill="{th["bg"]}"/>']
    ink, dot, sfx, op = mark_colors("studio", "color" if mode == "light" else "color-oscuro")
    ws = 0.027 * W if not sq else 0.05 * W
    g.append(mark("studio", ws, pad, pad + _M["top"] * ws, ink, dot, sfx, op).svg)
    if not sq:
        s100, w100 = head(0, 0, "master check", 100, th["ink"])
        size = 0.76 * W / (w100 / 100)
        by = H * 0.50
        s, wd = head(pad, by, "master check", size, th["ink"])
        g.append(s)
        g.append(T(pad + 2, by + size * 0.50, "Tu máster, listo para cada plataforma.", 0.031 * W, 400, th["ink3"]))
        D, gap, n = 0.019 * W, 0.0105 * W, 40
        x0 = pad
        n = int((W - 2 * pad + gap) / (D + gap))
        g.append(pills(x0, H - pad * 0.72, n, D, gap, mode, a["fill"], int(n * 0.68), maxh=H * 0.17))
    else:
        size = 0.215 * W
        by = H * 0.43
        g.append(head(pad, by, "master", size, th["ink"])[0])
        g.append(head(pad, by + size * 0.98, "check", size, th["ink"])[0])
        g.append(T(pad + 2, by + size * 0.98 + size * 0.38, "Tu máster, listo para cada plataforma.", 0.04 * W, 400, th["ink3"]))
        D, gap = 0.028 * W, 0.016 * W
        n = int((W - 2 * pad + gap) / (D + gap))
        g.append(pills(pad, H - pad * 0.70, n, D, gap, mode, a["fill"], int(n * 0.68), maxh=H * 0.14))
    return doc("".join(g), 0, 0, W, H)


def covers(jobs):
    for mode, mn in (("light", "claro"), ("dark", "oscuro")):
        for W, H in ((1280, 720), (600, 600)):
            s, w, h = cover(W, H, mode)
            jobs.append((s, w, h, P(f"plantillas/portada-master-check-{W}x{H}-{mn}.png"), False))


def slide(kind, mode):
    W, H = 1920, 1080
    th = THEME[mode]
    a = acc("studio", mode)
    ink, dot, sfx, op = mark_colors("studio", "color" if mode == "light" else "color-oscuro")
    pad = 120
    g = [f'<rect width="{W}" height="{H}" fill="{th["bg"]}"/>']
    ws = 40
    g.append(mark("studio", ws, pad, pad * 0.8 + _M["top"] * ws, ink, dot, sfx, op).svg)
    if kind == "portada":
        by = 520
        g.append(head(pad, by, "Título de la", 150, th["ink"])[0])
        g.append(head(pad, by + 150, "presentación", 150, th["ink"], dot=a["fill"])[0])
        g.append(T(pad + 4, by + 150 + 96, "Subtítulo, autor y fecha", 40, 400, th["ink3"]))
        D, gap = 26, 16
        n = int((W - 2 * pad + gap) / (D + gap))
        g.append(pills(pad, H - 90, n, D, gap, mode, a["fill"], int(n * 0.7), maxh=150))
    else:
        g.append(head(pad, 330, "Un mensaje por diapositiva", 92, th["ink"], dot=a["fill"])[0])
        for k, txt in enumerate(("Primer punto, en una línea.", "Segundo punto, con un dato exacto.", "Tercer punto: la decisión.")):
            y = 520 + k * 120
            g.append(f'<circle cx="{pad + 14}" cy="{y - 14}" r="13" fill="{a["fill"]}"/>')
            g.append(T(pad + 60, y, txt, 44, 400, th["ink"]))
        g.append(T(W - pad, H - 90, "02", 28, 500, th["ink3"], "end"))
        g.append(T(pad, H - 90, "nortia. studio · confidencial", 28, 400, th["ink3"]))
    return doc("".join(g), 0, 0, W, H)


def slides(jobs):
    for kind, mode, mn in (("portada", "light", "claro"), ("contenido", "light", "claro"), ("portada", "dark", "oscuro"), ("contenido", "dark", "oscuro")):
        s, w, h = slide(kind, mode)
        jobs.append((s, w, h, P(f"plantillas/diapositiva-16x9-{kind}-{mn}.png"), False))


def wm_inline(who, variant, height_px):
    s, w, h = wordmark_svg(who, variant, pad=0.05)
    return s.replace(f'width="{w}" height="{h}"', f'height="{height_px}" style="display:block;width:auto"')


A4 = """<!doctype html>
<html lang="es" data-brand="{brand}">
<head>
<meta charset="utf-8">
<title>Plantilla A4 · {title}</title>
<link rel="stylesheet" href="../tokens.css">
<style>
@font-face{{font-family:'Hanken Grotesk';src:url('../fonts/HankenGrotesk.ttf');font-weight:100 900}}
@page{{size:A4;margin:0}}
*{{box-sizing:border-box;margin:0;padding:0}}
html,body{{background:var(--bg)}}
body{{width:210mm;height:297mm;font:400 10.5pt/1.55 var(--font);color:var(--ink);position:relative;overflow:hidden;-webkit-print-color-adjust:exact;print-color-adjust:exact}}
header{{position:absolute;left:20mm;right:20mm;top:16mm;display:flex;justify-content:space-between;align-items:center;padding-bottom:6mm;border-bottom:1px solid var(--line)}}
header .meta{{font-size:8.5pt;color:var(--ink-3);text-align:right;line-height:1.4}}
main{{position:absolute;left:20mm;right:20mm;top:46mm;bottom:30mm}}
h1{{font-weight:600;font-size:30pt;line-height:1.05;letter-spacing:-.035em;margin-bottom:6mm}}
h1 i{{display:inline-block;width:.2em;height:.2em;border-radius:50%;background:var(--brand);margin-left:.06em}}
.lead{{font-size:13pt;color:var(--ink-2);margin-bottom:9mm;max-width:150mm}}
h2{{font-weight:600;font-size:13pt;letter-spacing:-.01em;margin:8mm 0 2mm}}
p{{margin-bottom:3mm;max-width:155mm}}
table{{width:100%;border-collapse:collapse;margin-top:4mm;font-size:9.5pt}}
th{{text-align:left;font-weight:500;color:var(--ink-3);border-bottom:1px solid var(--ink);padding:2mm 0}}
td{{border-bottom:1px solid var(--line);padding:2.2mm 0}}
footer{{position:absolute;left:20mm;right:20mm;bottom:14mm;display:flex;justify-content:space-between;font-size:8pt;color:var(--ink-3);border-top:1px solid var(--line);padding-top:4mm}}
</style>
</head>
<body>
<header>
  <div>{logo}</div>
  <div class="meta">Documento · referencia 000<br>[Fecha]</div>
</header>
<main>
  <h1>Título del documento<i></i></h1>
  <div class="lead">Entradilla de una o dos líneas que resume qué dice el documento y qué se espera del lector.</div>
  <h2>Primer apartado</h2>
  <p>Cuerpo de texto en Hanken Grotesk 400, 10,5 pt sobre 1,55. Frases cortas, datos exactos y sin adornos. Los números llevan unidad (−14 LUFS, 48 kHz, 24 bit).</p>
  <p>Un solo mensaje por párrafo. El acento solo aparece en el punto del titular y en los estados; el resto es tinta y gris.</p>
  <h2>Datos</h2>
  <table>
    <tr><th>Concepto</th><th>Valor</th><th>Estado</th></tr>
    <tr><td>Nivel integrado</td><td>−14,0 LUFS</td><td>Correcto</td></tr>
    <tr><td>Pico real</td><td>−1,0 dBTP</td><td>Correcto</td></tr>
    <tr><td>Formato</td><td>WAV 24 bit · 48 kHz</td><td>Revisar</td></tr>
  </table>
</main>
<footer>
  <div>nortia.{sub} · [Dirección] · [Web] · [Correo]</div>
  <div>1 / 1</div>
</footer>
</body>
</html>
"""

SIGN = """<!doctype html>
<html lang="es"><head><meta charset="utf-8"><title>Firma de correo · {title}</title></head>
<body style="margin:0;padding:24px;background:#ffffff">
<!-- Copiar y pegar la tabla en la configuración de firma. Subir {logo} a una URL pública y ajustar el src.
     Fuentes: los clientes de correo no cargan webfonts; se usa Arial/Helvetica como alternativa. -->
<table cellpadding="0" cellspacing="0" border="0" role="presentation" style="font-family:'Hanken Grotesk',Helvetica,Arial,sans-serif;font-size:13px;line-height:1.5;color:#111111">
  <tr><td style="padding:0 0 12px 0"><img src="{logo}" width="{w}" height="{h}" alt="{alt}" style="display:block;border:0;outline:none"></td></tr>
  <tr><td style="font-size:14px;font-weight:bold;color:#111111;padding:0">Nombre Apellido</td></tr>
  <tr><td style="color:{ink3};padding:0 0 10px 0">Cargo · {alt}</td></tr>
  <tr><td style="padding:0"><a href="https://[web]" style="color:{brandink};text-decoration:none;font-weight:bold">[web]</a><span style="color:{ink3}"> &nbsp;·&nbsp; </span><a href="mailto:[correo]" style="color:{brandink};text-decoration:none">[correo]</a></td></tr>
  <tr><td style="padding:14px 0 0 0"><table cellpadding="0" cellspacing="0" border="0" role="presentation" width="100%"><tr><td style="border-top:1px solid #E2E0DB;padding-top:10px;font-size:11px;line-height:1.45;color:{ink3}">Mensaje confidencial. Si lo has recibido por error, avísanos y bórralo.</td></tr></table></td></tr>
</table>
</body></html>
"""


def html_templates(htmljobs, jobs):
    for who, sub, title, variant in (("madre", "", "nortia.", "tinta"), ("studio", " studio", "nortia. studio", "color")):
        brand = "studio" if who == "studio" else "madre"
        html = A4.format(brand=brand, title=title, logo=wm_inline(who, variant, 30), sub=sub)
        nm = "nortia" if who == "madre" else "nortia-studio"
        hp = P(f"plantillas/a4-membrete-{nm}.html")
        open(hp, "w", encoding="utf-8").write(html)
        htmljobs.append((hp, 794, 1123, P(f"plantillas/a4-membrete-{nm}.png"), P(f"plantillas/a4-membrete-{nm}.pdf")))
        # firma
        s, w, h = wordmark_svg(who, variant, pw=300, pad=0.05)   # 150 px a 2x
        logo = f"firma-logo-{nm}@2x.png"
        jobs.append((s, w, h, P(f"plantillas/{logo}"), True))
        a = acc("studio" if who == "studio" else "madre", "light")
        html = SIGN.format(title=title, logo=logo, w=150, h=round(h / 2), alt=title,
                           ink3=THEME["light"]["ink3"], brandink=a["text"] if who == "studio" else INK)
        open(P(f"plantillas/firma-correo-{nm}.html"), "w", encoding="utf-8").write(html)


def build_all():
    shutil.rmtree(BUILD, ignore_errors=True)
    os.makedirs(BUILD)
    jobs, icos, htmljobs = [], [], []
    names = logos(jobs)
    avatars(jobs)
    favicons(jobs, icos)
    macos(jobs)
    windows(jobs, icos)
    legibility_jobs(jobs)
    banners(jobs)
    covers(jobs)
    slides(jobs)
    html_templates(htmljobs, jobs)
    return jobs, icos, htmljobs, names
