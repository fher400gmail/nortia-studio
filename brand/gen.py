"""nortia. · genera TODA la entrega desde cero:  python3 gen.py
Módulos: core.py (paleta, Inter a trazados, símbolo, logos) · assets.py (logos, avatares, favicon, banners, iconos de app)
templates.py (portadas, A4, diapositiva, firma) · boards.py (kit.png, submarcas.png) · contrast.py (WCAG; inyecta la tabla en DESIGN.md).
Requisitos: Python 3 + fonttools, rsvg-convert (librsvg), Inter OTF en /usr/share/fonts/opentype/inter."""
import os,subprocess,sys
import assets,templates,boards
assets.build_logos(); assets.build_avatars(); assets.build_banners(); assets.build_icons()
templates.build_templates(); boards.build_boards()
d=os.path.dirname(os.path.abspath(__file__))
if os.path.exists(d+'/DESIGN.md'): subprocess.run([sys.executable,d+'/contrast.py','--inject',d+'/DESIGN.md'],cwd=d,check=True,stdout=subprocess.DEVNULL)
print('ok')
