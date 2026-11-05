#!/usr/bin/env python3
"""nortia. · kit definitivo. `python3 gen.py` regenera TODO en esta carpeta (requiere fonttools, Pillow y playwright con chromium).
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
    assert not missing, f"faltan variables del prototipo: {missing}"
    rows, fails = TK.contrast_table()
    assert not fails, f"pares de texto que no llegan a AA: {fails}"
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
    leg = A.legibility_measure()
    assert all(r["ok"] for r in leg), "icono ilegible a alguno de los tamaños: " + str([r for r in leg if not r["ok"]])
    # 3 · docs
    brand_md, design_md = D.fill(leg, TK.contrast_md(rows), len(names))
    open(os.path.join(OUT, "BRAND.md"), "w", encoding="utf-8").write(brand_md)
    open(os.path.join(OUT, "DESIGN.md"), "w", encoding="utf-8").write(design_md)
    shutil.rmtree(A.BUILD, ignore_errors=True)
    print("ok ·", len(jobs), "PNG,", len(icos), ".ico,", len(htmljobs), "A4")


if __name__ == "__main__":
    main()
