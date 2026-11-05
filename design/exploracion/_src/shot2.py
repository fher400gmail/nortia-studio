import sys,asyncio,pathlib
from playwright.async_api import async_playwright
O=pathlib.Path(__file__).parent.parent
async def main(items):
    async with async_playwright() as p:
        b=await p.chromium.launch()
        for name,d,q in items:
            pg=await b.new_page(viewport={'width':1440,'height':900},device_scale_factor=2)
            await pg.route('**/fonts.g*/**',lambda r:r.abort())
            await pg.goto(f'file://{O}/{d}.html?still=1&{q}')
            await pg.wait_for_timeout(500)
            await pg.screenshot(path=str(O/name));print(name)
            await pg.close()
        await b.close()
d=sys.argv[1]
items=[(f'{d}_{sys.argv[2]}.png',d,sys.argv[3])]
asyncio.run(main(items))
