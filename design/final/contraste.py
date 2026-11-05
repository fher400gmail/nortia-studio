"""Auditoría AA con colores reales: color de texto (computado) sobre el fondo compuesto
(captura con el texto oculto, mediana de píxeles bajo cada caja). Uso: python3 contraste.py"""
import asyncio, io, json, pathlib, re, statistics
from playwright.async_api import async_playwright
from PIL import Image
F = pathlib.Path(__file__).parent.resolve()
def lin(c): c/=255; return c/12.92 if c<=.03928 else ((c+.055)/1.055)**2.4
def L(rgb): return .2126*lin(rgb[0])+.7152*lin(rgb[1])+.0722*lin(rgb[2])
def cr(a,b): la,lb=L(a),L(b); return (max(la,lb)+.05)/(min(la,lb)+.05)
def parse(s):
    m=re.match(r'rgba?\(([^)]+)\)',s)
    if m:
        v=[float(x) for x in re.split(r'[ ,/]+',m.group(1).strip()) if x]; return v[:3],(v[3] if len(v)>3 else 1)
    m=re.match(r'color\(srgb ([^)]+)\)',s)
    if m:
        v=[float(x) for x in re.split(r'[ /]+',m.group(1).strip())]; return [x*255 for x in v[:3]],(v[3] if len(v)>3 else 1)
    return None,1
JS='''()=>{const out=[];const w=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT);
while(w.nextNode()){const n=w.currentNode;if(!n.textContent.trim())continue;const el=n.parentElement;if(['SCRIPT','STYLE'].includes(el.tagName))continue;
 let vis=true,op=1;for(let e=el;e;e=e.parentElement){const s=getComputedStyle(e);if(s.visibility==='hidden'||s.display==='none'){vis=false;break}op*=parseFloat(s.opacity)}
 if(!vis||op<.99||el.closest('svg'))continue;const r=document.createRange();r.selectNodeContents(n);const b=r.getBoundingClientRect();if(b.width<2||b.height<2)continue;const top=document.elementFromPoint(b.x+b.width/2,b.y+b.height/2);if(!top||!(el===top||el.contains(top)||top.contains(el)))continue;
 const cs=getComputedStyle(el);out.push({t:n.textContent.trim().slice(0,28),c:cs.color,x:b.x,y:b.y,w:b.width,h:b.height,fs:parseFloat(cs.fontSize),fw:cs.fontWeight})}
 document.querySelectorAll('input').forEach(i=>{const r=i.getBoundingClientRect();if(!r.width||!i.placeholder)return;let e=i,op=1,vis=true;for(;e;e=e.parentElement){const s=getComputedStyle(e);if(s.visibility==='hidden')vis=false;op*=parseFloat(s.opacity)}if(!vis||op<.99)return;
  const ph=getComputedStyle(i,'::placeholder');if(!i.value)out.push({t:'['+i.placeholder+']',c:ph.color,o:parseFloat(ph.opacity),x:r.x+16,y:r.y+14,w:Math.min(r.width-32,60),h:r.height-28,fs:15.5,fw:'400'})});
 return out}'''
CASES=[('?state=empty',880,860),('?state=normal',880,860),('?state=normal',760,720),('?state=creating',880,860),('?state=done',880,860),('?state=normal&pop=more',880,860),('?state=normal&pop=adv',880,860),('?state=normal&pop=detail',880,860),('?state=normal&pop=tree',880,860),('?state=normal&os=win',880,860)]
async def main():
    res=[];worst={}
    async with async_playwright() as p:
        b=await p.chromium.launch()
        for theme in ('light','dark'):
            for q,w,h in CASES:
                pg=await b.new_page(viewport={'width':w,'height':h})
                await pg.goto(f'file://{F}/index.html{q}&theme={theme}&still=1'); await pg.wait_for_timeout(700)
                items=await pg.evaluate(JS)
                await pg.add_style_tag(content='*,*::placeholder{color:transparent!important;-webkit-text-fill-color:transparent!important;opacity:1}.fi::placeholder{opacity:0!important}.chip .mono,.ic,svg{visibility:hidden}')
                await pg.wait_for_timeout(250)
                im=Image.open(io.BytesIO(await pg.screenshot())).convert('RGB')
                for it in items:
                    x0,y0=int(max(0,it['x'])),int(max(0,it['y'])); x1,y1=int(min(w,it['x']+it['w'])),int(min(h,it['y']+it['h']))
                    if x1<=x0 or y1<=y0: continue
                    px=list(im.crop((x0,y0,x1,y1)).getdata())
                    # peor caso: el fondo más cercano al color del texto (percentil 5/95 según luminancia)
                    fg,al=parse(it['c']); al*= it.get('o',1) if 'o' in it else 1
                    bgs=sorted(px,key=L); med=bgs[len(bgs)//2]
                    cand=[bgs[0],bgs[-1],med]
                    fgc=[ (fg[i]*al+med[i]*(1-al)) for i in range(3)]
                    ratios=[cr([fg[i]*al+bg[i]*(1-al) for i in range(3)],bg) for bg in cand]
                    r=min(ratios); big=it['fs']>=18 or (it['fs']>=14 and int(it['fw'])>=600); need=3 if big else 4.5
                    res.append((theme,q+f' {w}x{h}',it['t'],round(r,2),need,r>=need))
                await pg.close()
        await b.close()
    bad=[r for r in res if not r[5]]
    print('textos medidos:',len(res),'· suspensos:',len(bad))
    for r in bad: print('FALLA',r)
    for th in ('light','dark'):
        rs=[r for r in res if r[0]==th]; m=min(rs,key=lambda r:r[3]); print(th,'mínimo',m)
    json.dump(res,open(F/'contraste.json','w'),ensure_ascii=False)
asyncio.run(main())
