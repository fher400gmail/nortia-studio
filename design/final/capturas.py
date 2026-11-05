"""Capturas D5 con Playwright (deviceScaleFactor 2). Uso: python3 capturas.py  (desde design/final)"""
import asyncio, pathlib
from playwright.async_api import async_playwright
F = pathlib.Path(__file__).parent.resolve(); O = F / 'capturas'
async def shot(b, name, q, w=880, h=860, act=None):
    pg = await b.new_page(viewport={'width': w, 'height': h}, device_scale_factor=2)
    errs = []; pg.on('pageerror', lambda e: errs.append(str(e))); pg.on('console', lambda m: m.type == 'error' and errs.append(m.text))
    await pg.goto(f'file://{F}/index.html{q}&still=1'); await pg.wait_for_timeout(450)
    if act: await act(pg); await pg.wait_for_timeout(450)
    ov = await pg.evaluate('()=>[document.scrollingElement.scrollHeight>innerHeight,document.scrollingElement.scrollWidth>innerWidth,Math.round(document.querySelector("#hero").getBoundingClientRect().height)]')
    await pg.screenshot(path=str(O / name), omit_background=True); print(name, 'overflow/hero', ov, errs); await pg.close()
async def hov(sel, x=.5, y=.35):
    async def f(pg):
        el = pg.locator(sel).first; bb = await el.bounding_box()
        await pg.mouse.move(bb['x'] + bb['width'] * x, bb['y'] + bb['height'] * y); await pg.hover(sel, position={'x': bb['width'] * x, 'y': bb['height'] * y})
    return f
async def main():
    async with async_playwright() as p:
        b = await p.chromium.launch()
        await shot(b, '01_vacio_claro.png', '?theme=light&state=empty')
        await shot(b, '02_normal_claro.png', '?theme=light&state=normal')
        await shot(b, '03_normal_oscuro.png', '?theme=dark&state=normal')
        await shot(b, '04_hover_boton.png', '?theme=light&state=normal', act=await hov('.go', .72, .3))
        await shot(b, '05_hover_ficha.png', '?theme=light&state=normal', act=await hov('.chip[data-id=deezer]', .3, .25))
        await shot(b, '06_creando.png', '?theme=light&state=creating')
        await shot(b, '07_listo.png', '?theme=light&state=done')
        await shot(b, '08_minimo_760x720.png', '?theme=light&state=normal', 760, 720)
        # extras de revisión
        async def off(pg):
            for i in ('amazon', 'deezer', 'canvas'): await pg.click(f'.chip[data-id={i}]')
            await pg.mouse.move(5, 5)
        await shot(b, 'x_apagadas.png', '?theme=light&state=normal', act=off)
        async def foco(pg):
            await pg.click('input[data-f=artist]')
        await shot(b, 'x_foco_oscuro.png', '?theme=dark&state=normal', act=foco)
        await shot(b, 'x_hover_onda.png', '?theme=light&state=normal', act=await hov('#wave', .62, .5))
        await shot(b, 'x_vacio_oscuro.png', '?theme=dark&state=empty')
        await shot(b, 'x_arrastre.png', '?theme=light&state=empty&drag=1')
        await shot(b, 'x_creando_oscuro.png', '?theme=dark&state=creating')
        await shot(b, 'x_listo_oscuro.png', '?theme=dark&state=done')
        await shot(b, 'x_mas_datos.png', '?theme=light&state=normal&pop=more')
        await shot(b, 'x_ajustes.png', '?theme=dark&state=normal&pop=adv')
        await shot(b, 'x_win_claro.png', '?theme=light&state=normal&os=win')
        await shot(b, 'x_win_oscuro_min.png', '?theme=dark&state=normal&os=win', 760, 720)
        await shot(b, 'x_minimo_oscuro.png', '?theme=dark&state=empty', 760, 720)
        await shot(b, 'x_grande_1440x900.png', '?theme=light&state=normal', 1440, 900)
        await b.close()
asyncio.run(main())
