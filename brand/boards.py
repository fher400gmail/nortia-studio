"""nortia. · láminas kit.png y submarcas.png (componen los activos ya generados)."""
from core import *
from assets import D, ghost, squircle, BRANDS, bname, title_of
from templates import R, pill

def img(path,x,y,w,h=None):
    h=h or w
    return f'<image href="{data_uri(path)}" x="{fmt(x)}" y="{fmt(y)}" width="{fmt(w)}" height="{fmt(h)}"/>'
def P(*a): return os.path.join(ROOT,*a)
def panel(x,y,w,h,label,fill=WHITE,lc=INK2):
    return R(x,y,w,h,fill,10,BORDER if fill==WHITE else None,1.5)+txt(label,'m',15,x+28,y+40,lc)
LBL={'verde':'Opción A · Verde azulado','grafito':'Opción B · Grafito cálido'}

def kit():
    W,H=2400,1800; o=''
    o+=place('horizontal',None,'color',80,64,height=48)[0]
    o+=txt('Kit de marca  ·  v1.0  ·  noviembre de 2026','r',17,W-80,100,INK2,anchor='end')
    # fila A
    y=170
    o+=panel(80,y,700,470,'nortia.  ·  logos')
    o+=place('horizontal',None,'color',112,y+84,height=92)[0]
    o+=place('vertical',None,'color',112,y+250,height=170)[0]
    o+=place('wordmark',None,'color',330,y+300,height=80)[0]
    o+=place('simbolo',None,'color',575,y+270,height=130)[0]
    o+=panel(812,y,760,470,'nortia. studio  ·  dos opciones de color')
    for i,b in enumerate(SUBS):
        yy=y+86+i*190
        o+=place('horizontal',b,'color',844,yy,height=78)[0]
        o+=txt(LBL[b]+'  '+SUBS[b]['main'],'m',15,844,yy+112,INK2)
        o+=place('vertical',b,'color',1300,yy-6,height=116)[0]
    o+=panel(1604,y,716,470,'Sobre azul noche',fill=NAVY,lc=D_TEXT2)
    for i,b in enumerate([None]+list(SUBS)):
        yy=y+86+i*128
        o+=place('horizontal',b,'reverse',1640,yy,height=66)[0]
    # fila B
    y=670
    o+=panel(80,y,1100,420,'Paleta')
    chips=[('Azul noche',NAVY,'texto y marca',WHITE),('Ámbar',AMBER,'solo punta y punto',NAVY),('Fondo',PAPER,'página',NAVY),('Texto secundario',INK2,'cuerpo secundario',WHITE),('Anunciado',ANN,'estado',WHITE),
           ('Verde azulado',SUBS['verde']['main'],'studio · A',WHITE),('Verde claro',SUBS['verde']['light'],'studio A en oscuro',NAVY),('Grafito cálido',SUBS['grafito']['main'],'studio · B',WHITE),('Grafito claro',SUBS['grafito']['light'],'studio B en oscuro',NAVY),('Noche',NIGHT,'fondo oscuro',WHITE)]
    for i,(n,h,r,fg) in enumerate(chips):
        cx=108+(i%5)*204; cy=y+66+(i//5)*168
        o+=R(cx,cy,188,150,h,8,BORDER,1.5)+txt(n,'s',16,cx+16,cy+98,fg)+txt(h,'r',14,cx+16,cy+120,fg,op=0.9 if False else None)+txt(r,'r',12.5,cx+16,cy+139,fg)
    o+=panel(1212,y,1108,420,'Tipografía  ·  Inter')
    o+=txt('Aa','s',230,1244,y+300,NAVY,-0.03)
    o+=txt('nortia. software','s',54,1580,y+150,NAVY,-0.025)
    o+=txt('Regular 400  ·  Medium 500  ·  SemiBold 600','r',20,1580,y+192,INK2)
    o+=txt('Software sobrio y fiable.','s',44,1580,y+262,NAVY,-0.025)
    t,_=para('Texto de cuerpo a 16/26: claro, concreto y sin superlativos. Las cifras usan numeración tabular: 1.234,56 €.','r',20,1580,y+310,700,30,INK2); o+=t
    o=o.replace('x="244"','x="1244"') if False else o
    # fila C
    y=1120
    o+=panel(80,y,620,600,'Avatar  ·  radio 22 %')
    for i,b in enumerate(BRANDS):
        o+=img(P('avatares',bname(b)+'-avatar-400.png'),108+i*188,y+70,164)
        o+=txt(['nortia.','studio A','studio B'][i],'m',14,108+i*188,y+262,INK2)
    o+=txt('Favicon 16 · 32 · 48 px  y  apple-touch 180 px','m',15,108,y+330,INK2)
    fx=108
    for n in (16,32,48):
        o+=img(P('avatares','nortia-avatar-400.png'),fx,y+360,n*2.4)+txt(str(n),'r',12,fx,y+360+n*2.4+22,INK2); fx+=n*2.4+22
    o+=img(P('favicon','apple-touch-icon.png'),fx+30,y+360,180)+txt('180','r',12,fx+30,y+568,INK2)
    o+=panel(732,y,700,600,'Icono de app  ·  nortia. studio')
    o+=img(P('iconos','macos','studio-verde-claro-1024.png'),748,y+60,330)
    o+=img(P('iconos','macos','studio-verde-oscuro-1024.png'),1090,y+60,330)
    o+=txt('macOS claro','m',14,830,y+420,INK2)+txt('macOS oscuro','m',14,1170,y+420,INK2)
    o+=img(P('iconos','windows','studio-verde-256.png'),780,y+450,110)
    o+=txt('Windows A (.ico 16–256)','m',14,908,y+510,INK2)
    o+=img(P('iconos','windows','studio-grafito-256.png'),1180,y+450,110)+txt('Windows B','m',14,1300,y+510,INK2)
    o+=panel(1464,y,856,600,'Banner  ·  1500×500')
    o+=img(P('banners','banner-x-nortia.png'),1492,y+60,768,256)
    o+=img(P('banners','banner-x-studio-verde.png'),1492,y+330,768,256)
    return svgdoc(W,H,o,'Kit de marca nortia.',bg=PAPER)

# ---------- submarcas ----------
def gen_icon(x,y,size,i):
    tones=['#E2E8F0','#CBD5E1','#B6C0CE','#94A3B8','#CFD6E0']; c=tones[i%5]; k=size/96; cx,cy=x+size/2,y+size/2
    sq=squircle(cx,cy,size/2*0.86); g=''
    if i%3==0: g=f'<circle cx="{cx}" cy="{cy}" r="{size*0.2}" fill="{WHITE}" fill-opacity=".85"/>'
    elif i%3==1: g=''.join(R(cx-size*0.22+j*size*0.16,cy-size*(0.12+0.05*(j%2)),size*0.1,size*(0.24+0.1*(j%2)),WHITE,3).replace('/>',' fill-opacity=".85"/>') for j in range(3))
    else: g=R(cx-size*0.2,cy-size*0.2,size*0.4,size*0.4,WHITE,6).replace('/>',' fill-opacity=".85"/>')
    return f'<path d="{sq}" fill="{c}"/>'+g
def dock(b,x,y,w=1080,h=260):
    o=R(x,y,w,h,'#2B3445',12)
    bw=860; bx=x+(w-bw)/2; by=y+60
    o+=R(bx,by,bw,124,WHITE,30).replace('fill="#FFFFFF"','fill="#FFFFFF" fill-opacity=".16" stroke="#FFFFFF" stroke-opacity=".28" stroke-width="1.5"')
    slots=[('g',0),('g',1),('g',2),('o','claro'),('o','oscuro'),('g',3),('g',4)]; sw=104; sx=bx+(bw-len(slots)*sw)/2
    for i,(k,v) in enumerate(slots):
        px=sx+i*sw
        if k=='g': o+=gen_icon(px+4,by+14,96,v)
        else:
            o+=img(P('iconos','macos',f'studio-{b}-{v}-1024.png'),px-10,by+4,sw+20)
            o+=txt(v,'m',14,px+sw/2,by+160,WHITE,anchor='middle')
    o+=f'<circle cx="{sx+3*sw+sw/2}" cy="{by+112}" r="3" fill="#fff"/>'
    o+=txt('Dock de macOS','m',14,x+24,y+34,'#C8D0DD')
    return o
def window(b,x,y,w=1000,h=470):
    S=SUBS[b]; c=S['main']
    o=f'<defs><filter id="sh" x="-5%" y="-5%" width="110%" height="120%"><feGaussianBlur stdDeviation="14"/></filter></defs>'
    o+=R(x,y+14,w,h,'#000',12).replace('/>',' fill-opacity=".18" filter="url(#sh)"/>')
    o+=R(x,y,w,h,WHITE,12,BORDER,1.5)
    o+=f'<path d="M{x} {y+44} V{y+12} A12 12 0 0 1 {x+12} {y} H{x+w-12} A12 12 0 0 1 {x+w} {y+12} V{y+44} Z" fill="#EEF0F4"/>'+R(x,y+44,w,1,BORDER)
    for i,col in enumerate(('#FF5F57','#FEBC2E','#28C840')): o+=f'<circle cx="{x+24+i*22}" cy="{y+22}" r="6.5" fill="{col}"/>'
    o+=txt('Proyecto sin título','m',14,x+w/2,y+27,NAVY,anchor='middle')
    sb=190; o+=R(x+1,y+45,sb,h-46-60,PAPER,0)+R(x+sb,y+45,1,h-46-60,BORDER)
    for i,n in enumerate(('Pistas','Mezclador','Biblioteca','Ajustes')):
        yy=y+66+i*40
        if i==0: o+=R(x+12,yy,sb-24,32,S['tint'],6)+txt(n,'s',14,x+28,yy+21,c)
        else: o+=txt(n,'m',14,x+28,yy+21,INK2)
    mx=x+sb+24; lanes=5; seed=7
    for l in range(lanes):
        ly=y+66+l*66
        o+=txt(f'Pista {l+1}','m',13,mx,ly+26,INK2)
        o+=R(mx+80,ly+34,w-sb-130,1,BORDER)
        cx0=mx+90+((l*97)%120); cw=190+((l*53)%160)
        for k in range(2 if l<3 else 1):
            xx=cx0+k*(cw+40)
            if xx+cw>x+w-40: break
            o+=R(xx,ly,cw,56,c if (l+k)%2==0 else S['main'],6).replace('/>',f' fill-opacity="{1 if (l+k)%2==0 else .72}"/>')
            for j in range(int(cw/6)-2):
                seed=(seed*1103515245+12345)&0x7fffffff; a=6+(seed%1000)/1000*34
                o+=R(xx+8+j*6,ly+28-a/2,3,a,WHITE,1.5).replace('/>',' fill-opacity=".85"/>')
    o+=R(mx+330,y+60,2,lanes*66,NAVY)
    ty=y+h-60; o+=R(x+1,ty,w-2,59,PAPER,0)+R(x+1,ty,w-2,1,BORDER)
    o+=f'<circle cx="{x+40}" cy="{ty+30}" r="16" fill="{c}"/><path d="M{x+35} {ty+22} L{x+35} {ty+38} L{x+48} {ty+30} Z" fill="#fff"/>'
    o+=txt('00:12.4','m',15,x+72,ty+36,NAVY)+R(x+190,ty+29,w-290,3,BORDER,1.5)+R(x+190,ty+29,(w-290)*0.38,3,c,1.5)
    return o
def web(b,x,y,w=1080,h=560):
    S=SUBS[b]; c=S['main']
    o=R(x,y,w,h,PAPER,12,BORDER,1.5)
    o+=f'<path d="M{x} {y+68} V{y+12} A12 12 0 0 1 {x+12} {y} H{x+w-12} A12 12 0 0 1 {x+w} {y+12} V{y+68} Z" fill="{WHITE}"/>'+R(x,y+68,w,1,BORDER)
    o+=place('horizontal',b,'color',x+32,y+16,height=36)[0]
    nx=x+w-32
    p=R(nx-120,y+14,120,40,c,6)+txt('Descargar','s',14,nx-60,y+39,WHITE,anchor='middle'); o+=p
    for i,n in enumerate(('Soporte','Productos')):
        wd=tw(n,'m',14); nx2=nx-150-i*(wd+36); o+=txt(n,'m',14,nx2,y+39,INK2,anchor='end')
    cards=[('Nombre del producto','Anunciado',ANN,ANN_BG),('Otra aplicación','Vigente',OK,OK_BG)]
    for i,(nm,st,fg,bg) in enumerate(cards):
        cx=x+36+i*(500+8); cy=y+104; cw=496; ch=424
        o+=R(cx,cy,cw,ch,WHITE,8,BORDER,1.5)
        o+=f'<path d="M{cx} {cy+190} V{cy+8} A8 8 0 0 1 {cx+8} {cy} H{cx+cw-8} A8 8 0 0 1 {cx+cw} {cy+8} V{cy+190} Z" fill="{NAVY}"/>'
        o+=ghost(cx+cw*0.82,cy+95,170)+place('horizontal',b,'reverse',cx+28,cy+26,height=34)[0]
        o+=txt('macOS  ·  Windows','m',13,cx+28,cy+166,D_TEXT2)
        o+=txt(nm,'s',22,cx+28,cy+238,NAVY,-0.015)
        t,_=para('Una frase que diga qué hace, para quién y en qué sistemas funciona.','r',15,cx+28,cy+266,cw-56,23,INK2); o+=t
        pp,pw=pill(cx+28,cy+312,st,fg,bg,13); o+=pp+txt('49 €','s',16,cx+cw-28,cy+331,NAVY,anchor='end')
        o+=R(cx+28,cy+358,190,40,c,6)+txt('Descargar para Mac','s',14,cx+28+95,cy+383,WHITE,anchor='middle')
        o+=R(cx+230,cy+358,108,40,WHITE,6,CONTROL,1.5)+txt('Windows','s',14,cx+284,cy+383,NAVY,anchor='middle')
    return o
def submarcas():
    W,H=2400,1760; o=''
    o+=place('horizontal',None,'color',80,60,height=48)[0]
    o+=txt('Submarcas  ·  nortia. studio en contexto','s',30,W-80,98,NAVY,-0.015,anchor='end')
    for ci,b in enumerate(SUBS):
        x=80+ci*1160; S=SUBS[b]
        o+=txt(LBL[b],'s',30,x,200,NAVY,-0.015)+txt(S['main']+'  ·  en oscuro '+S['light'],'r',18,x,232,INK2)
        o+=R(x+840,176,60,60,S['main'],8)+R(x+910,176,60,60,S['light'],8,BORDER,1)+R(x+980,176,60,60,S['tint'],8,BORDER,1)
        o+=dock(b,x,262)
        o+=f'<g>{window(b,x+40,562)}</g>'
        o+=web(b,x,1100,1080,560)
    return svgdoc(W,H,o,'Submarcas nortia. studio',bg=WHITE)

def build_boards():
    s=kit(); open(P('kit.svg'),'w').write(s); png(s,P('kit.png'),w=2400)
    s=submarcas(); open(P('submarcas.svg'),'w').write(s); png(s,P('submarcas.png'),w=2400)
