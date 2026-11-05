# Piezas compartidas por las tres direcciones (datos, tokens, onda, iconos).
import random

ARTIST = "Vera Alcón"
TITLE = "Luz de Noviembre"
PREFIX = f"{ARTIST} - {TITLE} "

BASE_CSS = r"""
*{box-sizing:border-box;margin:0;padding:0}
html,body{width:1440px;height:900px;overflow:hidden}
:root{
  --bg:#F7F8FA;--surf:#FFFFFF;--surf2:#EEF1F6;--chrome:#EDEFF4;--bd:#D5DBE5;--ctl:#808CA3;
  --tx:#0F1F3D;--tx2:#4A5568;--tx3:#5F6B7E;
  --ok:#146C43;--okbg:#E6F4EC;--wn:#8A5A00;--wnbg:#FBF1DC;--er:#B42318;
  --wvu:#B9C3D4;
}
:root[data-theme=dark]{
  --bg:#0A1429;--surf:#0F1F3D;--surf2:#152A52;--chrome:#0D1A33;--bd:#243659;--ctl:#6F7E9C;
  --tx:#E8ECF3;--tx2:#A3AEC2;--tx3:#8E9AB1;
  --ok:#6FD3A4;--okbg:rgba(111,211,164,.13);--wn:#E3B15C;--wnbg:rgba(227,177,92,.14);--er:#FF8A7E;
  --wvu:#3A4D75;
}
:root[data-studio=teal]{--st:#0D5C63;--stbg:#E3F0F0;--stbd:#8CBFC1;--onst:#FFFFFF}
:root[data-studio=teal][data-theme=dark]{--st:#4DB5B0;--stbg:rgba(77,181,176,.14);--stbd:#2C6B6E;--onst:#06202A}
:root[data-studio=graph]{--st:#2F343D;--stbg:#E9ECF1;--stbd:#9AA1AE;--onst:#FFFFFF}
:root[data-studio=graph][data-theme=dark]{--st:#D3D8E2;--stbg:rgba(211,216,226,.10);--stbd:#59627A;--onst:#0A1429}
body{font-family:'Inter',sans-serif;font-size:13px;line-height:1.35;color:var(--tx);background:var(--bg);
  font-variant-numeric:tabular-nums;-webkit-font-smoothing:antialiased}
.win{width:1440px;height:900px;display:flex;flex-direction:column;position:relative}
.tb{height:52px;flex:none;display:flex;align-items:center;gap:14px;padding:0 18px;background:var(--chrome);border-bottom:1px solid var(--bd)}
.lights{display:flex;gap:8px;margin-right:6px}.lights i{width:12px;height:12px;border-radius:50%;display:block}
.lights i:nth-child(1){background:#FF5F57}.lights i:nth-child(2){background:#FEBC2E}.lights i:nth-child(3){background:#28C840}
.brand{display:flex;align-items:center;gap:9px;font-size:14px;font-weight:600;letter-spacing:-.01em}
.brand .by{font-weight:400;color:var(--tx3);font-size:13px}
.sp1{flex:1}
.btn{height:32px;padding:0 14px;border-radius:6px;border:1px solid var(--ctl);background:var(--surf);color:var(--tx);
  font:500 13px 'Inter',sans-serif;display:inline-flex;align-items:center;gap:7px;white-space:nowrap}
.btn.pri{background:var(--st);border-color:var(--st);color:var(--onst);font-weight:600}
.btn.sm{height:28px;padding:0 10px;font-size:12px}
.pill{display:inline-flex;align-items:center;gap:6px;height:20px;padding:0 9px;border-radius:999px;font:500 11.5px 'Inter',sans-serif;white-space:nowrap}
.pill::before{content:'';width:6px;height:6px;border-radius:50%;background:currentColor}
.pill.ok{color:var(--ok);background:var(--okbg)}.pill.wn{color:var(--wn);background:var(--wnbg)}
.pill.nt{color:var(--tx2);background:var(--surf2)}.pill.nt::before{display:none}
.sw{width:30px;height:18px;border-radius:999px;background:var(--surf2);border:1px solid var(--ctl);position:relative;flex:none;display:inline-block}
.sw::after{content:'';position:absolute;top:2px;left:2px;width:12px;height:12px;border-radius:50%;background:var(--ctl)}
.sw.on{background:var(--st);border-color:var(--st)}.sw.on::after{left:14px;background:var(--onst)}
.mono{width:22px;height:22px;border-radius:5px;background:var(--stbg);color:var(--st);font:600 10.5px 'Inter',sans-serif;
  display:inline-flex;align-items:center;justify-content:center;flex:none;letter-spacing:-.02em}
.ic{width:16px;height:16px;flex:none;display:inline-block;vertical-align:middle}
.bar{height:5px;border-radius:99px;background:var(--surf2);overflow:hidden}
.bar i{display:block;height:100%;background:var(--st);border-radius:99px}
.bar.done i{background:var(--ok)}
.b{font-weight:600}.t2{color:var(--tx2)}.t3{color:var(--tx3)}
"""

THEME_JS = "<script>(function(){var q=new URLSearchParams(location.search).get('theme');document.documentElement.dataset.theme=q||(matchMedia('(prefers-color-scheme:dark)').matches?'dark':'light')})()</script>"

ICON = {
 "folder": '<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linejoin="round"><path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/></svg>',
 "check": '<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12.5l4.5 4.5L19 7.5"/></svg>',
 "play": '<svg class="ic" viewBox="0 0 24 24" fill="currentColor"><path d="M7 4.5v15l12-7.5z"/></svg>',
 "file": '<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linejoin="round"><path d="M6 3h8l5 5v13H6z"/><path d="M14 3v5h5"/></svg>',
 "chev": '<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 6l6 6-6 6"/></svg>',
 "down": '<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 9l6 6 6-6"/></svg>',
 "plus": '<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M12 5v14M5 12h14"/></svg>',
 "wave": '<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M4 10v4M8 6v12M12 3v18M16 7v10M20 10v4"/></svg>',
}

def sym(size=22):
    # Símbolo N2 de nortia.: mitad norte ámbar (única aparición del ámbar), mitad sur en el color de studio.
    return (f'<svg width="{size}" height="{size}" viewBox="0 0 64 64" aria-hidden="true">'
            f'<path d="M32 2L42 31H22Z" fill="#BF7F12"/><path d="M22 33H42L32 62Z" fill="var(--st)"/></svg>')

def titlebar(center="", right=""):
    return (f'<div class="tb"><div class="lights"><i></i><i></i><i></i></div>'
            f'<div class="brand">{sym(20)}<span>Master Out</span><span class="by">nortia. studio</span></div>'
            f'<div class="sp1">{center}</div>{right}</div>')

# ---------- destinos ----------
# (clave, nombre, mono, archivo, activo, estado, motivo)
DEST = [
 ("spotify","Spotify","Sp","FLAC 24 bit · 96 kHz",True,"wn","TP −0,3 dBTP; Spotify recomienda ≤ −1"),
 ("apple","Apple Music","Ap","WAV 24 bit · 96 kHz · Digital Masters",True,"ok","0 muestras con clipping (afclip)"),
 ("amazon","Amazon Music","Am","FLAC 24 bit · 96 kHz",False,None,None),
 ("tidal","Tidal","Ti","FLAC 24 bit · 96 kHz",True,"ok","Cumple · resolución nativa"),
 ("deezer","Deezer","De","WAV 16 bit · 44,1 kHz",True,"ok","Cumple · dither TPDF al bajar a 16 bit"),
 ("ytm","YouTube Music","YM","WAV 24 bit · 48 kHz",False,None,None),
 ("dist","Distribuidora estándar","Di","WAV 16 bit · 44,1 kHz",True,"ok","Cumple · dither TPDF al bajar a 16 bit"),
 ("disthr","Distribuidora Hi-Res","DH","WAV 24 bit · 96 kHz",False,None,None),
 ("tunecore","TuneCore","Tc","WAV 24 bit · 192 kHz",False,None,None),
 ("mp3","MP3 320","MP","MP3 · 320 kbps CBR",False,None,None),
 ("ytv","YouTube vídeo","YV","MP4 · 1080p · imagen fija",False,None,None),
 ("canvas","Spotify Canvas","Ca","MP4 · 1080×1920 · 6 s",False,None,None),
 ("portada","Portada","Po","JPG + PNG · 3000×3000",False,None,None),
]
DMAP = {d[0]: d for d in DEST}
GROUPS = [("Streaming",["spotify","apple","amazon","tidal","deezer","ytm"]),
          ("Distribuidoras",["dist","disthr","tunecore"]),
          ("Promo",["mp3"]),
          ("Vídeo e imagen",["ytv","canvas","portada"])]

# (nº carpeta, destino, nombre de carpeta, [marca], extensión, tamaño, clave, tamaño bytes estimado)
FILES = [
 ("01","Spotify","Spotify","[Spotify] 24bit-96kHz.flac","74,1 MB","spotify"),
 ("02","Apple Music","Apple Music","[Apple Digital Masters] 24bit-96kHz.wav","145,2 MB","apple"),
 ("04","Tidal","Tidal","[Tidal] 24bit-96kHz.flac","74,1 MB","tidal"),
 ("05","Deezer","Deezer","[Deezer] 16bit-44kHz.wav","44,5 MB","deezer"),
 ("07","Distribuidora","Distribuidora","[Distribuidora] 16bit-44kHz.wav","44,5 MB","dist"),
]
DOCS = [("README.txt","3 KB"),("informe-loudness.txt","4 KB"),("ficha-lanzamiento.csv","1 KB"),("ficha-lanzamiento.txt","2 KB")]
# progreso simulado de la exportación (D1, D3)
PROG = {"spotify":100,"apple":64,"tidal":88,"deezer":100,"dist":31}

def fname_html(rest):
    """Nombre completo con la marca de plataforma resaltada."""
    i = rest.index("]") + 1
    return f'{PREFIX}<b style="font-weight:600">{rest[:i]}</b>{rest[i:]}'

# ---------- forma de onda ----------
_ENV = [(0,.16),(.07,.22),(.09,.5),(.28,.56),(.30,.92),(.5,.9),(.52,.5),(.64,.57),(.66,.97),(.86,.95),(.88,.58),(1,.1)]
def _env(t):
    for (a,x),(b,y) in zip(_ENV,_ENV[1:]):
        if a <= t <= b:
            return x + (y-x)*(t-a)/(b-a)
    return .1

def wave(w, h, n, prog=None, seed=7, grow=None):
    """SVG de barras espejadas. prog: fracción 'reproducida/procesada' en color de studio."""
    rnd = random.Random(seed)
    bw = w / n; g = bw * .3
    out = []
    for i in range(n):
        t = (i+.5)/n
        a = min(1, _env(t) * (.6 + .4*rnd.random()))
        bh = max(3, a*h)
        cls = "p" if (prog is not None and t <= prog) else "u"
        out.append(f'<rect class="{cls}" x="{i*bw:.1f}" y="{(h-bh)/2:.1f}" width="{bw-g:.1f}" height="{bh:.1f}" rx="{(bw-g)/2:.1f}"/>')
    return (f'<svg class="wv" width="{w}" height="{h}" viewBox="0 0 {w} {h}" aria-hidden="true">'
            f'<style>.wv .p{{fill:var(--st)}}.wv .u{{fill:var(--wvu)}}</style>{"".join(out)}</svg>')

def cover(size=88):
    return (f'<svg width="{size}" height="{size}" viewBox="0 0 100 100" aria-label="Portada" style="border-radius:6px;display:block;flex:none">'
            '<rect width="100" height="100" fill="#1D3A4B"/><rect y="62" width="100" height="38" fill="#14293A"/>'
            '<circle cx="64" cy="40" r="19" fill="#EBDDC3"/><rect x="0" y="58" width="100" height="3" fill="#EBDDC3" opacity=".55"/>'
            '<rect x="14" y="68" width="48" height="2.5" fill="#EBDDC3" opacity=".35"/><rect x="26" y="75" width="36" height="2.5" fill="#EBDDC3" opacity=".25"/></svg>')

def page(title, studio, css, body):
    return (f'<!doctype html><html lang="es" data-studio="{studio}"><head><meta charset="utf-8">'
            f'<meta name="viewport" content="width=1440"><title>{title}</title>{THEME_JS}'
            f'<style>{BASE_CSS}{css}</style></head><body><div class="win">{body}</div></body></html>')
