#!/usr/bin/env python3
"""ferboix. studio · kit definitivo (opción C «Instrumento»). `python3 gen.py` regenera TODO en esta carpeta
(requiere fonttools, Pillow y playwright con chromium).
Orden: tokens y contraste (aborta si un texto no llega a AA) -> logos/avatares/favicons/iconos/banners/plantillas -> láminas -> BRAND.md y DESIGN.md."""
import os, sys, json, shutil
from core import *
import tokens as TK
import assets as A
import sheets as S
import docs as D

OUT = A.OUT


def main():
    ensure_fonts()
    # 1 · tokens + contraste (puerta AA)
    t = TK.build_tokens()
    names, missing = TK.verify_against_prototype(t)
    if missing:
        print("aviso: variables del prototipo no cubiertas (ignorado en la propuesta):", missing)
    rows, fails = TK.contrast_table()
    assert not fails, f"pares de texto que no llegan a AA: {fails}"
    grafico = [r for r in rows if r["tipo"] == "gráfico" and not r["ok"]]
    # el relleno cian debe llegar a 3:1 sobre bg, card y sobre el disco; solo se tolera sobre `fill` (el punto de la pastilla lleva palabra)
    assert all("sobre fill" in r["par"] for r in grafico), f"relleno de acento por debajo de 3:1: {grafico}"
    json.dump(t, open(os.path.join(OUT, "tokens.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    open(os.path.join(OUT, "tokens.css"), "w", encoding="utf-8").write(TK.build_css(t))
    json.dump(rows, open(os.path.join(OUT, "contraste.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"tokens ok · {len(names)} variables del prototipo cubiertas · {len(rows)} pares de contraste, 0 fallos de texto")
    # 2 · archivos
    jobs, icos, htmljobs, logo_names = A.build_all()
    k, kw, kh = S.kit()
    f, fw, fh = S.familia()
    jobs += [(k, kw, kh, os.path.join(OUT, "kit.png"), False), (f, fw, fh, os.path.join(OUT, "familia.png"), False)]
    render_svgs(jobs)
    render_html(htmljobs)
    A.pack_icos(icos)
    A.pack_icns()
    leg = A.legibility_measure()
    assert all(r["ok"] for r in leg), "icono ilegible a alguno de los tamaños: " + str([r for r in leg if not r["ok"]])
    # 3 · docs
    brand_md, design_md = D.fill(leg, TK.contrast_md(rows), len(names) - len(missing))
    open(os.path.join(OUT, "BRAND.md"), "w", encoding="utf-8").write(brand_md)
    open(os.path.join(OUT, "DESIGN.md"), "w", encoding="utf-8").write(design_md)
    shutil.rmtree(A.BUILD, ignore_errors=True)
    print("ok ·", len(jobs), "PNG,", len(icos), ".ico, 2 .icns,", len(htmljobs), "A4;", sum(1 for r in leg if r["ok"]), "/", len(leg), "medidas de legibilidad ok")


if __name__ == "__main__":
    main()
