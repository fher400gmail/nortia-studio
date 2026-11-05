"""nortia. · núcleo: colores, tipografía (Inter a trazados con kerning GPOS), símbolo y logos."""
import os, base64
from fontTools.ttLib import TTFont
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen

ROOT=os.path.dirname(os.path.abspath(__file__))
FD='/usr/share/fonts/opentype/inter/'
# ---- paleta -----------------------------------------------------------------
NAVY='#0F1F3D'; NAVY_HOVER='#1B2F57'; NIGHT='#0A1429'; AMBER='#BF7F12'; AMBER_TEXT='#8A5A00'
PAPER='#F7F8FA'; WHITE='#FFFFFF'; BLACK='#000000'
INK2='#4A5568'; INK3='#5F6B7E'; BORDER='#D5DBE5'; CONTROL='#808CA3'
D_TEXT='#E8ECF3'; D_TEXT2='#A3AEC2'; D_BORDER='#26385A'
OK='#146C43'; OK_BG='#E6F4EC'; ANN='#1F4E8C'; ANN_BG='#E6EEF9'; WARN=AMBER_TEXT; WARN_BG='#FBF1DC'; ERR='#B42318'; ERR_BG='#FDECEA'
# submarcas: color sobre claro, variante para fondo oscuro, tinte de superficie
SUBS={
 'verde':  dict(name='studio',label='Verde azulado',main='#0E5E63',light='#5BB8BC',tint='#E3F1F1',hover='#0B4A4E'),
 'grafito':dict(name='studio',label='Grafito cálido',main='#45413C',light='#CFC8BE',tint='#ECEAE6',hover='#33302C'),
}
DEFAULT_SUB='verde'

def fmt(v): return ('%.2f'%v).rstrip('0').rstrip('.')
def poly(pts): return 'M'+' L'.join(f'{fmt(x)} {fmt(y)}' for x,y in pts)+' Z'

# ---- tipografía ---------------------------------------------------------------
WF={'r':'Inter-Regular.otf','m':'Inter-Medium.otf','s':'Inter-SemiBold.otf','b':'Inter-Bold.otf'}
_F={}
def _font(w):
    fn=WF[w]
    if fn not in _F:
        f=TTFont(FD+fn); g=f['GPOS'].table; idx=set()
        for fr in g.FeatureList.FeatureRecord:
            if fr.FeatureTag=='kern': idx.update(fr.Feature.LookupListIndex)
        subs=[]
        for i in sorted(idx):
            lk=g.LookupList.Lookup[i]
            for st in lk.SubTable:
                if lk.LookupType==9: st=st.ExtSubTable
                if st.__class__.__name__=='PairPos': subs.append(st)
        _F[fn]=(f.getGlyphSet(),f.getBestCmap(),f['head'].unitsPerEm,subs)
    return _F[fn]
def kern(w,a,b):
    gs,cm,upm,subs=_font(w); ga,gb=cm[ord(a)],cm[ord(b)]
    for st in subs:
        cov=st.Coverage.glyphs
        if ga not in cov: continue
        if st.Format==1:
            for pv in st.PairSet[cov.index(ga)].PairValueRecord:
                if pv.SecondGlyph==gb: return getattr(pv.Value1,'XAdvance',0) or 0
        else:
            c1=st.ClassDef1.classDefs.get(ga,0); c2=st.ClassDef2.classDefs.get(gb,0)
            v=st.Class1Record[c1].Class2Record[c2].Value1
            val=getattr(v,'XAdvance',0) if v else 0
            if val: return val
    return 0
def _glyph(w,ch,size,x,y):
    gs,cm,upm,_=_font(w); sc=size/upm; gn=cm[ord(ch)]
    pen=SVGPathPen(gs,fmt); gs[gn].draw(TransformPen(pen,(sc,0,0,-sc,x,y)))
    return pen.getCommands(),gs[gn].width*sc
def layout(runs,size,x,y,track=0.0):
    """runs=[(texto,peso,color[,hueco_em_antes])] -> (svg, ancho). Kerning continuo entre runs sin hueco."""
    out=[]; cx=x; n=len(runs)
    for ri,run in enumerate(runs):
        t,w,col=run[:3]; gap=run[3] if len(run)>3 else 0
        cx+=gap*size; d=[]
        for i,ch in enumerate(t):
            if ch==' ':
                cx+=_font(w)[2] and (0.27*size)+track*size; continue
            c,adv=_glyph(w,ch,size,cx,y); d.append(c); cx+=adv+track*size
            nxt=t[i+1] if i+1<len(t) else (runs[ri+1][0][0] if ri+1<n and not (len(runs[ri+1])>3 and runs[ri+1][3]) else None)
            if nxt and nxt!=' ': cx+=kern(w,ch,nxt)*size/_font(w)[2]
        if d: out.append(f'<path d="{" ".join(d)}" fill="{col}"/>')
    return ''.join(out),cx-x
def tw(s,w,size,track=0.0): return layout([(s,w,'#000')],size,0,0,track)[1]
def txt(s,w,size,x,y,fill,track=0.0,anchor='start',op=None):
    width=tw(s,w,size,track)
    if anchor=='middle': x-=width/2
    elif anchor=='end': x-=width
    svg,_=layout([(s,w,fill)],size,x,y,track)
    if op: svg=svg.replace('/>',f' fill-opacity="{op}"/>')
    return svg
def para(s,w,size,x,y,maxw,lh,fill,track=0.0):
    words=s.split(); lines=[]; cur=''
    for wd in words:
        t=(cur+' '+wd).strip()
        if tw(t,w,size,track)<=maxw or not cur: cur=t
        else: lines.append(cur); cur=wd
    if cur: lines.append(cur)
    return ''.join(txt(l,w,size,x,y+i*lh,fill,track) for i,l in enumerate(lines)),len(lines)*lh

# ---- símbolo N2 «norte marcado» (viewBox 64) -------------------------------------
def sym_parts(hw=10):
    wd=lambda y: hw*(y-2)/30 if y<=32 else hw*(62-y)/30
    tip=[(32,2),(32+wd(31),31),(32-wd(31),31)]
    body=[(32-wd(33),33),(32+wd(33),33),(32,62)]
    return body,tip
def symbol(body_col,tip_col,hw=10):
    b,t=sym_parts(hw)
    return f'<path d="{poly(b)}" fill="{body_col}"/><path d="{poly(t)}" fill="{tip_col}"/>'

def cols(tone,brand):
    """colores por tono: name (nortia), dot, sub (studio), body, tip"""
    S=SUBS.get(brand) if brand else None
    if tone=='color':  return dict(name=NAVY,dot=AMBER,sub=S['main'] if S else None,body=S['main'] if S else NAVY,tip=AMBER)
    if tone=='reverse':return dict(name=WHITE,dot=AMBER,sub=S['light'] if S else None,body=S['light'] if S else PAPER,tip=AMBER)
    c=BLACK if tone=='negro' else WHITE
    return dict(name=c,dot=c,sub=c,body=c,tip=c)

# ---- logos --------------------------------------------------------------------
TRACK=-0.015; SUBGAP=0.2
def wm_runs(brand,tone):
    c=cols(tone,brand); runs=[('nortia','s',c['name']),('.','s',c['dot'])]
    if brand: runs.append((SUBS[brand]['name'],'r',c['sub'],SUBGAP))
    return runs
XH=0.546  # altura de x de Inter (em)
def svgdoc(w,h,body,title='nortia.',bg=None):
    b=f'<rect width="{fmt(w)}" height="{fmt(h)}" fill="{bg}"/>' if bg else ''
    return f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {fmt(w)} {fmt(h)}" width="{fmt(w)}" height="{fmt(h)}" role="img" aria-label="{title}"><title>{title}</title>{b}{body}</svg>\n'
def title_of(brand): return 'nortia. '+SUBS[brand]['name'] if brand else 'nortia.'
SIZE_H=46; GAP_H=14; RAISE=0.5; ASC=0.74   # ASC: altura del punto de la i en Inter (em)
def logo(form,brand,tone):
    """devuelve (fragmento svg, ancho, alto) en unidades donde el símbolo mide 64 de alto (agujas de 20 de ancho, recortadas)"""
    c=cols(tone,brand); sym=symbol(c['body'],c['tip'])
    if form=='simbolo': return sym,64,64
    runs=wm_runs(brand,tone)
    if form=='wordmark':
        size=SIZE_H; base=ASC*size+3
        wm,w=layout(runs,size,0,base,TRACK); return wm,w,base+3
    if form=='horizontal':
        base=32+XH*SIZE_H/2+RAISE
        wm,w=layout(runs,SIZE_H,20+GAP_H,base,TRACK)
        return f'<g transform="translate(-22 0)">{sym}</g>'+wm,20+GAP_H+w,64
    if form=='vertical':
        size=30; wmw=layout(runs,size,0,0,TRACK)[1]; vw=max(wmw,20); base=64+20+ASC*size
        wm,_=layout(runs,size,(vw-wmw)/2,base,TRACK)
        return f'<g transform="translate({fmt(vw/2-32)} 0)">{sym}</g>'+wm,vw,base+3
def logo_svg(form,brand,tone):
    f,w,h=logo(form,brand,tone); return svgdoc(w,h,f,title_of(brand))
def place(form,brand,tone,x,y,height=None,width=None):
    f,w,h=logo(form,brand,tone); s=(height/h) if height else (width/w)
    return f'<g transform="translate({fmt(x)} {fmt(y)}) scale({s:.5f})">{f}</g>',w*s,h*s

def avatar_frag(brand,size,radius=0.22,round_=True,frac=0.80,hw=12):
    """cuadrado navy con aguja al frac del lado; fragmento en coordenadas 0..size"""
    S=SUBS.get(brand) if brand else None
    body=S['light'] if S else PAPER
    sc=frac*size/64; off=(size-64*sc)/2   # la aguja ocupa 60/64*sc; se centra la caja de 64
    r=f' rx="{fmt(size*radius)}"' if round_ else ''
    return f'<rect width="{fmt(size)}" height="{fmt(size)}"{r} fill="{NAVY}"/><g transform="translate({fmt(off)} {fmt(off)}) scale({sc:.5f})">{symbol(body,AMBER,hw)}</g>'
def avatar_svg(brand,round_=True,frac=0.80):
    return svgdoc(64,64,avatar_frag(brand,64,0.22,round_,frac),title_of(brand)+' avatar')

# ---- raster / ico -------------------------------------------------------------
import subprocess, struct, io
def png(svg_text,out,w=None,h=None):
    tmp=out+'.tmp.svg'; open(tmp,'w').write(svg_text)
    a=['rsvg-convert','-o',out]+(['-w',str(w)] if w else [])+(['-h',str(h)] if h else [])+[tmp]
    subprocess.run(a,check=True); os.remove(tmp)
def pdf(svg_text,out):
    tmp=out+'.tmp.svg'; open(tmp,'w').write(svg_text)
    subprocess.run(['rsvg-convert','-f','pdf','-o',out,tmp],check=True); os.remove(tmp)
def data_uri(path):
    return 'data:image/png;base64,'+base64.b64encode(open(path,'rb').read()).decode()
def write_ico(out,entries):
    """entries=[(size,png_bytes)] PNG dentro de ICO"""
    n=len(entries); head=struct.pack('<HHH',0,1,n); off=6+16*n; dirs=b''; data=b''
    for s,b in entries:
        dirs+=struct.pack('<BBBBHHII',s%256,s%256,0,0,1,32,len(b),off+len(data)); data+=b
    open(out,'wb').write(head+dirs+data)
