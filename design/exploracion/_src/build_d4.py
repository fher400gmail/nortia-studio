import pathlib
S=pathlib.Path(__file__).parent;O=S.parent
r={'/*__COMMON_CSS__*/':(S/'common.css').read_text(),'/*__COMMON_JS__*/':(S/'common.js').read_text(),'/*__D4_CSS__*/':(S/'d4.css').read_text(),'/*__D4_JS__*/':(S/'d4.js').read_text()}
for n in ['d4a','d4b']:
    f=S/f'{n}.src.html'
    if not f.exists():continue
    t=f.read_text()
    for k,v in r.items():t=t.replace(k,v)
    (O/f'{n}.html').write_text(t);print('built',n)
