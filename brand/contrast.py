"""nortia. · contraste WCAG 2.1 de todos los pares de la paleta. python3 contrast.py [--inject DESIGN.md]
Falla (exit 1) si algún par no cumple su umbral: texto 4.5, gráfico/UI 3."""
import sys,re
from core import *
def lum(h):
    h=h.lstrip('#'); r,g,b=[int(h[i:i+2],16)/255 for i in (0,2,4)]
    f=lambda c:c/12.92 if c<=.03928 else ((c+.055)/1.055)**2.4
    return .2126*f(r)+.7152*f(g)+.0722*f(b)
def cr(a,b):
    la,lb=sorted((lum(a),lum(b)),reverse=True); return (la+.05)/(lb+.05)
V,G=SUBS['verde'],SUBS['grafito']
T,U=4.5,3.0
PAIRS=[ # (grupo, descripción, fg, bg, umbral)
('Claro','Texto principal #0F1F3D sobre fondo',NAVY,PAPER,T),('Claro','Texto principal sobre superficie blanca',NAVY,WHITE,T),
('Claro','Texto secundario sobre fondo',INK2,PAPER,T),('Claro','Texto secundario sobre blanco',INK2,WHITE,T),
('Claro','Texto terciario sobre blanco',INK3,WHITE,T),
('Claro','Botón primario: blanco sobre azul noche',WHITE,NAVY,T),('Claro','Botón primario hover: blanco sobre #1B2F57',WHITE,NAVY_HOVER,T),
('Claro','Ámbar #BF7F12 sobre blanco (gráfico: punta/punto)',AMBER,WHITE,U),('Claro','Ámbar sobre fondo #F7F8FA (gráfico)',AMBER,PAPER,U),
('Claro','Ámbar sobre azul noche (gráfico, logo en negativo)',AMBER,NAVY,U),('Claro','Ámbar como texto: #8A5A00 sobre blanco',AMBER_TEXT,WHITE,T),
('Claro','Pastilla Anunciado',ANN,ANN_BG,T),('Claro','Pastilla Vigente / éxito',OK,OK_BG,T),('Claro','Pastilla Aviso',WARN,WARN_BG,T),
('Claro','Error sobre blanco',ERR,WHITE,T),('Claro','Error sobre fondo de error #FDECEA',ERR,ERR_BG,T),
('Claro','Borde de control #808CA3 sobre blanco (inputs, botón secundario)',CONTROL,WHITE,U),
('Studio A','Verde azulado sobre fondo (texto «studio»)',V['main'],PAPER,T),('Studio A','Verde azulado sobre blanco',V['main'],WHITE,T),
('Studio A','Botón: blanco sobre verde azulado',WHITE,V['main'],T),('Studio A','Botón hover: blanco sobre #0B4A4E',WHITE,V['hover'],T),
('Studio A','Verde azulado sobre tinte #E3F1F1',V['main'],V['tint'],T),
('Studio A','Verde claro #5BB8BC sobre azul noche',V['light'],NAVY,T),('Studio A','Verde claro sobre fondo oscuro #0A1429',V['light'],NIGHT,T),
('Studio B','Grafito cálido sobre fondo (texto «studio»)',G['main'],PAPER,T),('Studio B','Grafito cálido sobre blanco',G['main'],WHITE,T),
('Studio B','Botón: blanco sobre grafito',WHITE,G['main'],T),('Studio B','Botón hover: blanco sobre #33302C',WHITE,G['hover'],T),
('Studio B','Grafito sobre tinte #ECEAE6',G['main'],G['tint'],T),
('Studio B','Grafito claro #CFC8BE sobre azul noche',G['light'],NAVY,T),('Studio B','Grafito claro sobre fondo oscuro #0A1429',G['light'],NIGHT,T),
('Oscuro','Texto principal #E8ECF3 sobre #0A1429',D_TEXT,NIGHT,T),('Oscuro','Texto secundario #A3AEC2 sobre #0A1429',D_TEXT2,NIGHT,T),
('Oscuro','Texto secundario sobre superficie #0F1F3D',D_TEXT2,NAVY,T),('Oscuro','Botón primario: azul noche sobre blanco',NAVY,WHITE,T),
('Oscuro','Ámbar sobre #0A1429 (gráfico)',AMBER,NIGHT,U),('Oscuro','Borde de control #808CA3 sobre #0A1429 (UI)',CONTROL,NIGHT,U),
('Oscuro','Éxito: #6FD3A0 sobre #0A1429','#6FD3A0',NIGHT,T),('Oscuro','Error: #FF8A80 sobre #0A1429','#FF8A80',NIGHT,T),
]
def table():
    rows=['| Grupo | Par | Ratio | Umbral | AA |','|---|---|---|---|---|']; bad=[]
    for g,n,a,b,th in PAIRS:
        r=cr(a,b); ok=r>=th
        if not ok: bad.append((n,r))
        rows.append(f'| {g} | {n} ({a} / {b}) | {r:.2f} | {th:g} | {"sí" if ok else "NO"} |')
    return '\n'.join(rows),bad
if __name__=='__main__':
    t,bad=table(); print(t)
    if '--inject' in sys.argv:
        p=sys.argv[sys.argv.index('--inject')+1]; s=open(p).read()
        s=re.sub(r'(<!--CONTRAST-->).*?(<!--/CONTRAST-->)',lambda m:m.group(1)+'\n'+t+'\n'+m.group(2),s,flags=re.S); open(p,'w').write(s)
    if bad: print('FALLAN:',bad); sys.exit(1)
