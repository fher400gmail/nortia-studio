"""nortia. · generación de logos, avatares, favicons, banners e iconos de app."""
import math
from core import *

def D(*p):
    d=os.path.join(ROOT,*p); os.makedirs(d,exist_ok=True); return d
BRANDS=[None]+list(SUBS)           # None = nortia. madre
def bname(b): return 'nortia' if b is None else f'studio-{b}'

# ---------- logos ----------
def build_logos():
    sv=D('logos','svg'); pn=D('logos','png')
    done=[]
    for b in BRANDS:
        forms=['simbolo','wordmark','horizontal','vertical'] if b is None else ['simbolo','horizontal','vertical']
        for f in forms:
            for tone in ('color','negro','blanco','reverse'):
                if b and tone in('negro','blanco') and b!=list(SUBS)[0]: continue   # mono idéntico en las dos opciones
                nm=(bname(b) if not(b and tone in('negro','blanco')) else 'studio')+f'-{f}-{tone}'
                s=logo_svg(f,b,tone); open(f'{sv}/{nm}.svg','w').write(s)
                fr,w,h=logo(f,b,tone)
                for n in (512,2048):
                    if w>=h: png(s,f'{pn}/{nm}-{n}.png',w=n)
                    else: png(s,f'{pn}/{nm}-{n}.png',h=n)
                done.append(nm)
    return done

# ---------- avatares, favicon, apple-touch ----------
def build_avatars():
    d=D('avatares')
    for b in BRANDS:
        s=avatar_svg(b); open(f'{d}/{bname(b)}-avatar.svg','w').write(s)
        for n in (1000,400): png(s,f'{d}/{bname(b)}-avatar-{n}.png',w=n)
    f=D('favicon')
    for b in BRANDS:
        ents=[]
        for n in (16,32,48):
            fr=0.88 if n<=32 else 0.82
            s=svgdoc(n,n,avatar_frag(b,n,0.22,True,fr,13 if n<=32 else 12))
            tmp=f'{f}/_t{n}.png'; png(s,tmp,w=n); ents.append((n,open(tmp,'rb').read())); os.remove(tmp)
        write_ico(f'{f}/{"favicon" if b is None else "favicon-"+bname(b)}.ico',ents)
        ap=svgdoc(180,180,avatar_frag(b,180,0,False,0.64,12))      # iOS aplica su máscara: cuadrado completo, sin transparencia
        png(ap,f'{f}/{"apple-touch-icon" if b is None else "apple-touch-icon-"+bname(b)}.png',w=180)

# ---------- fondo hero (aguja fantasma) ----------
GHOST_B='#14274A'; GHOST_T='#182D55'
def ghost(cx,cy,h,hw=10):
    s=h/60; return f'<g transform="translate({fmt(cx-32*s)} {fmt(cy-32*s)}) scale({s:.4f})">{symbol(GHOST_B,GHOST_T,hw)}</g>'
TAG={None:'Software sobrio y fiable.','studio':'Apps para productores y artistas.'}
def tag(b): return TAG[None] if b is None else TAG['studio']

def hero(w,h,b,lh,x,tagsize,ghost_cx):
    block=lh+tagsize*1.9; y=(h-block)/2
    body=ghost(ghost_cx,h/2,h*0.84)
    lg,lw,lhh=place('horizontal',b,'reverse',x,y,height=lh)
    t=txt(tag(b),'r',tagsize,x,y+lhh+tagsize*1.7,D_TEXT2)
    return svgdoc(w,h,body+lg+t,title_of(b),bg=NAVY)
def build_banners():
    d=D('banners')
    for b in BRANDS:
        n=bname(b)
        s=hero(1500,500,b,120,250,34,1230); open(f'{d}/banner-x-{n}.svg','w').write(s); png(s,f'{d}/banner-x-{n}.png',w=1500)
        s=hero(1280,640,b,120,110,34,1020); open(f'{d}/social-preview-{n}.svg','w').write(s); png(s,f'{d}/social-preview-{n}.png',w=1280)

# ---------- iconos de app ----------
def squircle(cx,cy,half,n=5.0,N=720):
    pts=[]
    for i in range(N):
        t=2*math.pi*i/N; c,s=math.cos(t),math.sin(t)
        pts.append((cx+half*math.copysign(abs(c)**(2/n),c),cy+half*math.copysign(abs(s)**(2/n),s)))
    return poly(pts)
def macos_icon(b,mode):
    S=SUBS[b]; light=(mode=='claro')
    bg=PAPER if light else NAVY
    body=S['main'] if light else S['light']
    k=0.62*824/60                                   # aguja = 62 % del lado de la plancha de 824
    sym=f'<g transform="translate({fmt(512-32*k)} {fmt(512-32*k)}) scale({k:.4f})">{symbol(body,AMBER)}</g>'
    sq=squircle(512,512,412)
    edge=f'<path d="{squircle(512,512,411)}" fill="none" stroke="{BORDER if light else "#26385A"}" stroke-width="2"/>'
    shadow=(f'<defs><filter id="bl" x="-20%" y="-20%" width="140%" height="150%"><feGaussianBlur stdDeviation="14"/></filter></defs>'
            f'<path d="{squircle(512,524,412)}" fill="#000" fill-opacity="0.28" filter="url(#bl)"/>')
    return svgdoc(1024,1024,shadow+f'<path d="{sq}" fill="{bg}"/>{edge}{sym}','nortia. studio icono macOS')
def win_icon(b,size):
    S=SUBS[b]; fr=0.80 if size<=32 else 0.72
    return svgdoc(size,size,avatar_frag(b,size,0.18,True,fr,13 if size<=32 else 12),'nortia. studio icono Windows')
def build_icons():
    m=D('iconos','macos'); w=D('iconos','windows')
    for b in SUBS:
        for mode in ('claro','oscuro'):
            s=macos_icon(b,mode); nm=f'studio-{b}-{mode}'
            open(f'{m}/{nm}.svg','w').write(s); png(s,f'{m}/{nm}-1024.png',w=1024)
        ents=[]
        for n in (16,24,32,48,64,128,256):
            tmp=f'{w}/_t.png'; png(win_icon(b,n),tmp,w=n); ents.append((n,open(tmp,'rb').read())); os.remove(tmp)
        write_ico(f'{w}/studio-{b}.ico',ents)
        png(win_icon(b,256),f'{w}/studio-{b}-256.png',w=256)
        open(f'{w}/studio-{b}.svg','w').write(win_icon(b,256))
