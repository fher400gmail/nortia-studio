import sys,asyncio,pathlib
from playwright.async_api import async_playwright
O=pathlib.Path(__file__).parent.parent
async def main(items):
    async with async_playwright() as p:
        b=await p.chromium.launch()
        for d,theme,state,extra in items:
            pg=await b.new_page(viewport={'width':1440,'height':900},device_scale_factor=2)
            await pg.route('**/fonts.googleapis.com/**',lambda r:r.abort())
            await pg.route('**/fonts.gstatic.com/**',lambda r:r.abort())
            q=f'?theme={theme}&still=1'+(f'&state={state}' if state else '')+extra
            await pg.goto(f'file://{O}/{d}.html{q}')
            await pg.wait_for_timeout(500)
            name=f'{d}_{"oscuro" if theme=="dark" else "claro"}'+('_'+state if state else '')+'.png'
            await pg.screenshot(path=str(O/name));print(name)
            await pg.close()
        await b.close()
ds=sys.argv[1].split(',')
items=[]
for d in ds:
    items+= [(d,'light',None,''),(d,'dark',None,''),(d,'light','exporting','')]
asyncio.run(main(items))
