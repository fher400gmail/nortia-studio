"""nortia. · plantillas: portadas, PDF A4, firma de correo, diapositiva."""
from core import *
from assets import D, ghost, title_of, bname, BRANDS

def R(x,y,w,h,fill,rx=0,stroke=None,sw=1):
    st=f' stroke="{stroke}" stroke-width="{sw}"' if stroke else ''
    return f'<rect x="{fmt(x)}" y="{fmt(y)}" width="{fmt(w)}" height="{fmt(h)}" rx="{fmt(rx)}" fill="{fill}"{st}/>'
def pill(x,y,text,fg,bg,size=13,h=None,padx=None,stroke=None,w_='m'):
    h=h or size*2.0; padx=padx if padx is not None else size*0.85
    w=tw(text,w_,size)+2*padx
    return R(x,y,w,h,bg,h/2,stroke)+txt(text,w_,size,x+padx,y+h/2+size*0.35,fg),w
def acc(b): return SUBS[b]['main'] if b else NAVY
def label(b): return 'nortia. studio' if b else 'nortia.'

# ---------- portadas de producto ----------
def cover(b,w,h):
    sq=(w==h); m=56 if sq else 80
    out=ghost(w*(0.80 if not sq else 0.86),h*0.5,h*(0.78 if not sq else 0.62))
    lg,lw,lh=place('horizontal',b,'reverse',m,m,height=48 if sq else 56); out+=lg
    ts=60 if sq else 80; ty=h*0.46 if not sq else h*0.43
    t,hh=para('Nombre del producto','s',ts,m,ty,w*(0.56 if not sq else 0.74),ts*1.1,WHITE,-0.025); out+=t
    sub='Una frase que diga qué hace, para quién y en qué sistemas funciona.'
    s2,h2=para(sub,'r',22 if sq else 28,m,ty+hh+ (14 if sq else 18),w*(0.5 if not sq else 0.66),(22 if sq else 28)*1.45,D_TEXT2); out+=s2
    y=h-m-34; x=m
    for tx in ('macOS','Windows'):
        p,pw=pill(x,y,tx,D_TEXT,'none',15,34,15,stroke='#3B4F73',w_='m'); out+=p; x+=pw+10
    p,_=pill(w-m-110,y,'Anunciado',ANN,ANN_BG,15,34,16) if False else ('',0)
    return svgdoc(w,h,out,'Portada de producto '+label(b),bg=NAVY)

# ---------- PDF A4 (595x842 pt) ----------
def a4(b):
    W,H=595,842; M=48; o=R(0,0,W,H,WHITE)
    lg,lw,lh=place('horizontal',b,'color',M,36,height=26); o+=lg
    o+=txt('Documento','m',10,W-M,54,INK2,anchor='end')+R(M,82,W-2*M,1,BORDER)
    o+=txt('Título del documento','s',28,M,150,NAVY,-0.02)
    o+=txt('Versión 1.0  ·  5 de noviembre de 2026','r',11,M,172,INK2)
    p1='Este documento es una plantilla de página con cabecera y pie. El cuerpo usa Inter Regular a 11 pt con interlínea de 17 pt, color de texto principal y líneas de unos 70 caracteres, para que se lea con comodidad en papel y en pantalla.'
    t,hh=para(p1,'r',11,M,212,W-2*M,17,NAVY); o+=t; y=212+hh+22
    o+=txt('Primer apartado','s',16,M,y,NAVY,-0.01); y+=24
    p2='Los apartados se separan con 24 pt de aire y un titular Inter SemiBold. Las cifras de las tablas usan numeración tabular y se alinean a la derecha; los estados van siempre con texto, nunca solo con color.'
    t,hh=para(p2,'r',11,M,y,W-2*M,17,NAVY); o+=t; y+=hh+20
    # tabla
    cw=[250,120,129]; xs=[M,M+cw[0],M+cw[0]+cw[1]]
    o+=R(M,y,W-2*M,28,PAPER,0)+''.join(txt(h_,'m',9.5,xs[i]+(0 if i<2 else 0)+10,y+18,INK2) for i,h_ in enumerate(('Elemento','Estado','Versión')))
    rows=[('Instalador para macOS','Vigente','1.0'),('Instalador para Windows','Anunciado','1.0'),('Guía de instalación','Vigente','1.0')]
    y+=28
    for r in rows:
        o+=txt(r[0],'r',11,xs[0]+10,y+22,NAVY)
        fg,bg=(OK,OK_BG) if r[1]=='Vigente' else (ANN,ANN_BG)
        p,_=pill(xs[1]+10,y+7,r[1],fg,bg,9.5,18,8); o+=p
        o+=txt(r[2],'r',11,xs[2]+10,y+22,NAVY)+R(M,y+35,W-2*M,1,BORDER); y+=36
    y+=26
    # llamada
    o+=R(M,y,W-2*M,64,PAPER,6)+R(M,y,3,64,acc(b),0)
    t,_=para('Nota: indica aquí versión, fecha y lo que el documento no cubre. Es la forma de la marca de decir las cosas con claridad.','r',10.5,M+18,y+24,W-2*M-36,16,NAVY); o+=t
    # pie
    o+=R(M,H-62,W-2*M,1,BORDER)
    o+=txt(label(b)+'  ·  Documento de ejemplo','r',9,M,H-40,INK2)+txt('Página 1 de 1','r',9,W-M,H-40,INK2,anchor='end')
    return svgdoc(W,H,o,'Plantilla A4 '+label(b),bg=WHITE)

# ---------- diapositiva 16:9 ----------
def slide(b):
    W,H=1920,1080; o=R(0,0,W,H,PAPER)
    lg,lw,lh=place('horizontal',b,'color',96,72,height=52); o+=lg
    o+=txt('Título de la diapositiva','s',84,96,330,NAVY,-0.025)
    t,_=para('Una frase de apoyo que resume el mensaje en una o dos líneas, con voz de producto y sin superlativos.','r',32,96,402,1180,46,INK2); o+=t
    cw=(W-192-2*40)/3; y=560
    data=[('Etiqueta','00','Descripción breve del dato, en una línea.'),('Etiqueta','00','Descripción breve del dato, en una línea.'),('Etiqueta','00','Descripción breve del dato, en una línea.')]
    for i,(a,v,d) in enumerate(data):
        x=96+i*(cw+40)
        o+=R(x,y,cw,300,WHITE,8,BORDER,2)+R(x,y,cw,6,acc(b) if i==0 else BORDER,0)
        o+=txt(a,'m',22,x+32,y+62,INK2)+txt(v,'s',88,x+32,y+168,NAVY,-0.025)
        t,_=para(d,'r',22,x+32,y+232,cw-64,32,INK2); o+=t
    o+=R(96,990,W-192,2,BORDER)+txt(label(b)+'  ·  Presentación  ·  noviembre de 2026','r',20,96,1038,INK2)+txt('1','m',20,W-96,1038,INK2,anchor='end')
    return svgdoc(W,H,o,'Diapositiva '+label(b),bg=PAPER)

# ---------- firma de correo ----------
def signature(b):
    c=acc(b); nm=label(b); logo='firma-logo-'+bname(b)+'.png'
    sub=f' <span style="color:{c};font-weight:400">studio</span>' if b else ''
    return f'''<!doctype html>
<html lang="es"><head><meta charset="utf-8"><title>Firma de correo · {nm}</title></head>
<body style="margin:0;padding:24px;background:#ffffff">
<!-- Firma de correo {nm}. Sube {logo} (600 px de ancho) a un alojamiento propio y cambia la ruta del src.
     Tabla con estilos en línea: es lo único que respetan Gmail, Outlook y Apple Mail. Sin nombres de personas: rellena los corchetes. -->
<table cellpadding="0" cellspacing="0" border="0" role="presentation" style="font-family:Inter,'Helvetica Neue',Arial,sans-serif;color:#0F1F3D;font-size:14px;line-height:20px">
 <tr>
  <td style="padding:0 18px 0 0;vertical-align:top"><img src="{logo}" width="150" alt="{nm}" style="display:block;border:0"></td>
  <td style="border-left:2px solid {c};padding:0 0 0 18px;vertical-align:top">
   <div style="font-weight:600;font-size:15px;color:#0F1F3D">[Equipo o cargo]</div>
   <div style="color:#4A5568;font-size:13px">nortia<span style="color:#BF7F12">.</span>{sub}</div>
   <div style="padding-top:8px;font-size:13px;color:#4A5568">[correo]  ·  [teléfono]</div>
   <div style="font-size:13px"><a href="https://[dominio]" style="color:#0F1F3D;text-decoration:underline">[dominio]</a></div>
  </td>
 </tr>
 <tr><td colspan="2" style="padding-top:14px;font-size:11px;line-height:16px;color:#5F6B7E">Este mensaje y sus adjuntos son confidenciales. Si lo has recibido por error, avísanos y elimínalo.</td></tr>
</table>
</body></html>
'''

def build_templates():
    d=D('plantillas')
    for b in (None,DEFAULT_SUB):
        n='madre' if b is None else 'studio'
        if b:
            for w,h,nm in ((1280,720,'portada-producto-1280x720'),(600,600,'portada-producto-600x600')):
                s=cover(b,w,h); open(f'{d}/{n}-{nm}.svg','w').write(s); png(s,f'{d}/{n}-{nm}.png',w=w)
        s=a4(b); open(f'{d}/{n}-a4.svg','w').write(s); pdf(s,f'{d}/{n}-a4.pdf'); png(s,f'{d}/{n}-a4-preview.png',w=1190)
        s=slide(b); open(f'{d}/{n}-diapositiva-16x9.svg','w').write(s); png(s,f'{d}/{n}-diapositiva-16x9.png',w=1920)
        open(f'{d}/{n}-firma-correo.html','w').write(signature(b))
        png(logo_svg('horizontal',b,'color'),f'{d}/firma-logo-{bname(b)}.png',w=600)
