import sys,asyncio,pathlib
from playwright.async_api import async_playwright
O=pathlib.Path(__file__).parent.parent
SZ={'1440':(1440,900),'1100':(1100,700)}
async def main(ds,sizes,states):
    async with async_playwright() as p:
        b=await p.chromium.launch()
        for d in ds:
          for sz in sizes:
            w,h=SZ[sz]
            for theme,state,extra in states:
                pg=await b.new_page(viewport={'width':w,'height':h},device_scale_factor=2)
                errs=[];pg.on('pageerror',lambda e:errs.append(str(e)));pg.on('console',lambda m:m.type=='error' and errs.append(m.text))
                await pg.route('**/fonts.g*/**',lambda r:r.abort())
                q=f'?theme={theme}&still=1'+(f'&state={state}' if state else '')+extra
                await pg.goto(f'file://{O}/{d}.html{q}');await pg.wait_for_timeout(400)
                name=f'{d}_{"oscuro" if theme=="dark" else "claro"}'+(f'_{state}' if state else '')+(f'_{extra.split("=")[1]}' if extra else '')+f'_{sz}.png'
                ov=await pg.evaluate('()=>{const s=document.scrollingElement;const c=document.querySelector(".card,.cols");const r=c.getBoundingClientRect();return [s.scrollHeight>innerHeight,s.scrollWidth>innerWidth,Math.round(r.top),Math.round(innerHeight-r.bottom)]}')
                await pg.screenshot(path=str(O/name));print(name,'overflow',ov,errs)
                await pg.close()
        await b.close()
ds=sys.argv[1].split(',');sizes=sys.argv[2].split(',')
st=[('light',None,''),('dark',None,''),('light','creating',''),('dark','creating',''),('light','done',''),('dark','done',''),('light','empty','')]
if len(sys.argv)>3: st=[s for s in st if (s[1] or 'normal') in sys.argv[3].split(',')]
asyncio.run(main(ds,sizes,st))
