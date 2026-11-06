"""Tokens (mismos nombres de variable que design/final/styles.css) + tabla de contraste AA. ferboix. studio, opción C «Instrumento»."""
import json, os, re
from brand import *

ROOT = os.path.dirname(os.path.abspath(__file__))
PROTO = os.path.join(os.path.dirname(ROOT), "design", "final")


def color_tokens(mode):
    th, sm = THEME[mode], SEM[mode]
    a = acc(mode)
    btn = btn_colors(mode)
    dark = mode == "dark"
    f, t, on = a["fill"], a["text"], a["on"]
    ink = th["ink"]
    return {
        "bg": th["bg"], "bg-deep": th["deep"],
        "ink": ink, "ink-2": th["ink2"], "ink-3": th["ink3"],
        "brand": f, "brand-ink": t,            # acento cian: estados (lectura, selección, play, puntos de ficha) y enlaces
        "amber": sm["amber"], "amber-ink": sm["amber_ink"],
        "ok": sm["ok"], "err": sm["err"],
        # botón primario = TINTA con texto papel (claro) / PAPEL con texto tinta (oscuro); plano (hi = lo)
        "btn-hi": btn["fill"], "btn-lo": btn["fill"], "on-btn": btn["on"], "btn-glow": rgba("#000000", 0.50 if dark else 0.30),
        "orb-1": rgba(ink, 0.05 if not dark else 0.04), "orb-2": rgba(ink, 0.03), "orb-3": "rgba(0,0,0,0)",
        "pop-bg": "rgba(26,26,26,.96)" if dark else "rgba(255,255,255,.96)",
        "on-bg": rgba(f, 0.20 if dark else 0.13), "on-line": rgba(f, 0.60),
        "on-glow": "0 10px 24px -14px " + rgba("#000000", 0.55 if dark else 0.35),
        "focus": rgba(f, 0.30), "ctl": th["ctl"], "wv": th["wv"], "wv-on": ink,
        "win-edge": "rgba(255,255,255,.14)" if dark else "rgba(17,17,17,.14)",
        # nuevos (no existen en el prototipo; opcionales)
        "card": th["card"], "fill": th["fill"], "line": th["line"], "on-brand": on,
    }


GLASS = {
    "light": {"g-bg": "rgba(255,255,255,.62)", "g-bg-hover": "rgba(255,255,255,.82)", "g-bg-strong": "rgba(255,255,255,.92)",
              "g-line": "rgba(17,17,17,.09)", "g-spec": "rgba(255,255,255,.75)", "g-ring-hi": "rgba(255,255,255,.98)", "g-ring-lo": "rgba(255,255,255,.40)",
              "g-shadow": "0 1px 1px rgba(17,17,17,.04),0 10px 26px -14px rgba(17,17,17,.22)",
              "g-shadow-lift": "0 1px 1px rgba(17,17,17,.05),0 16px 32px -14px rgba(17,17,17,.30)", "g-blur": "22px"},
    "dark": {"g-bg": "rgba(255,255,255,.055)", "g-bg-hover": "rgba(255,255,255,.095)", "g-bg-strong": "rgba(30,30,30,.90)",
             "g-line": "rgba(0,0,0,.45)", "g-spec": "rgba(255,255,255,.10)", "g-ring-hi": "rgba(255,255,255,.24)", "g-ring-lo": "rgba(255,255,255,.05)",
             "g-shadow": "0 1px 1px rgba(0,0,0,.30),0 14px 30px -14px rgba(0,0,0,.65)",
             "g-shadow-lift": "0 1px 1px rgba(0,0,0,.35),0 20px 38px -14px rgba(0,0,0,.75)", "g-blur": "26px"},
}
RADIUS = {"hero": 24, "chip": 16, "button": 16, "field": 12, "cover": 16, "pill": 999, "popover": 16, "card": 24}
SPACE = [4, 8, 12, 16, 24, 32, 48, 64, 96]


def build_tokens():
    color = {m: color_tokens(m) for m in ("light", "dark")}
    return {
        "$schema": "ferboix tokens v2",
        "fuente": "BRAND.md / DESIGN.md. Mismos nombres de variable que design/final/styles.css; 'color' = ferboix. studio (una sola marca, un solo acento: cian).",
        "font": {"family": "Hanken Grotesk", "fallback": "system-ui,-apple-system,'Segoe UI',sans-serif", "weights": [400, 500, 600],
                 "wordmark": 590, "base": "13px", "field": "15.5px", "button": "16px"},
        "window": {"default": {"width": 880, "height": 860}, "min": {"width": 760, "height": 720}, "unit": "pt (px lógicos)",
                   "radius": {"mac": 26, "win": 8}, "titlebar": {"mac": 52, "win": 36}},
        "motion": {"ease": "cubic-bezier(.2,.8,.2,1)", "spring": "cubic-bezier(.34,1.5,.64,1)", "fast": "150ms", "base": "200ms",
                   "spring_ms": "240ms", "press": "90ms"},
        "color": color,
        "glass": GLASS,
        "layout": {"pad": "clamp(22px,3.4vw,40px)", "gap": "clamp(12px,2.1vh,20px)", "tgap": "clamp(8px,1.2vw,12px)",
                   "chip-h": "clamp(68px,9.8vh,92px)", "go-h": "clamp(52px,6.6vh,60px)", "tb-h": "52px", "tb-pl": "20px"},
        "radius": RADIUS,
        "space": SPACE,
    }


def css_block(sel, d):
    return sel + "{\n" + "".join(f"  --{k}:{v};\n" for k, v in d.items()) + "}\n"


def build_css(t):
    out = ["/* ferboix. studio · tokens · generado por gen.py (no editar). data-theme = light (por defecto) | dark */\n"]
    out.append(css_block(":root", {**t["color"]["light"], **t["glass"]["light"], "win-r": "26px", "font": "'Hanken Grotesk',system-ui,-apple-system,'Segoe UI',sans-serif"}))
    out.append(css_block(":root[data-theme=dark]", {**t["color"]["dark"], **t["glass"]["dark"]}))
    return "\n".join(out)


def verify_against_prototype(t):
    """Todas las variables de color que el prototipo define en :root y [data-theme=dark] existen en los tokens."""
    css = open(os.path.join(PROTO, "styles.css"), encoding="utf-8").read()
    blocks = re.findall(r":root\{(.*?)\}|:root\[data-theme=dark\]\{(.*?)\}", css, flags=re.S)
    names = set()
    for a, b in blocks:
        names |= set(re.findall(r"--([a-z0-9-]+):", a or b))
    have = set(t["color"]["light"]) | set(t["glass"]["light"]) | {"win-r"} | {"pad", "gap", "tgap", "chip-h", "go-h", "tb-h", "tb-pl"} \
        | {"ease", "spring", "t-fast", "t-base", "t-spring"}
    missing = sorted(names - have)
    return sorted(names), missing


# ───────────────────────────── contraste ─────────────────────────────
def contrast_rows():
    """(modo, descripción, fg, bg, umbral, tipo). tipo: texto | gráfico."""
    rows = []
    for mode in ("light", "dark"):
        th, sm = THEME[mode], SEM[mode]
        for surf in ("bg", "card", "fill"):
            for k, nm in (("ink", "tinta"), ("ink2", "texto 2"), ("ink3", "texto 3 / palabra «studio»")):
                rows.append((mode, f"{nm} sobre {surf}", th[k], th[surf], 4.5, "texto"))
        for k, nm in (("ok", "ok"), ("err", "error"), ("amber_ink", "aviso (texto)")):
            for surf in ("bg", "card"):
                rows.append((mode, f"{nm} sobre {surf}", sm[k], th[surf], 4.5, "texto"))
        rows.append((mode, "control (ctl) sobre bg", th["ctl"], th["bg"], 3.0, "gráfico"))
        a, b = acc(mode), btn_colors(mode)
        for surf in ("bg", "card", "fill"):
            rows.append((mode, f"cian · texto-acento (--brand-ink) sobre {surf}", a["text"], th[surf], 4.5, "texto"))
        rows.append((mode, "botón primario · texto (--on-btn) sobre relleno (--btn-hi)", b["on"], b["fill"], 4.5, "texto"))
        rows.append((mode, "cian · texto sobre relleno cian (check de ficha, --on-brand)", a["on"], a["fill"], 4.5, "texto"))
        for surf in ("bg", "card"):
            rows.append((mode, f"cian · relleno (--brand: marca, play, puntos) sobre {surf}", a["fill"], th[surf], 3.0, "gráfico"))
        rows.append((mode, "cian · relleno (--brand) sobre fill (punto en pastilla; lleva palabra)", a["fill"], th["fill"], 3.0, "gráfico"))
        rows.append((mode, "marca del símbolo (cian) sobre el disco", a["fill"], th["ink"], 3.0, "gráfico"))
    return rows


def contrast_table():
    rows = []
    fails = []
    for mode, desc, fg, bg, thr, kind in contrast_rows():
        r = ratio(fg, bg)
        ok = r >= thr
        if kind == "texto" and not ok:
            fails.append((mode, desc, fg, bg, round(r, 2)))
        rows.append(dict(modo=mode, par=desc, fg=fg.upper(), bg=bg.upper(), ratio=round(r, 2), umbral=thr, tipo=kind, ok=ok))
    return rows, fails


def contrast_md(rows):
    out = ["| Modo | Par | Texto/forma | Fondo | Ratio | Mín. | Resultado |", "|---|---|---|---|---|---|---|"]
    for r in rows:
        res = "AA" if r["ok"] else ("no llega (solo decorativo/logo; nunca único portador de significado)" if r["tipo"] == "gráfico" else "FALLA")
        out.append(f"| {'claro' if r['modo']=='light' else 'oscuro'} | {r['par']} | `{r['fg']}` | `{r['bg']}` | {r['ratio']:.2f} | {r['umbral']} | {res} |")
    return "\n".join(out)


if __name__ == "__main__":
    t = build_tokens()
    names, missing = verify_against_prototype(t)
    print(len(names), "vars prototipo; faltan:", missing)
    rows, fails = contrast_table()
    print(len(rows), "pares; fallos de texto:", fails)
    for r in rows:
        if not r["ok"]:
            print(r)
