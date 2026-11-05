import pathlib,sys
S=pathlib.Path(__file__).parent;O=S.parent
css=(S/'common.css').read_text();js=(S/'common.js').read_text()
for n in ['d1','d2','d3']:
    f=S/f'{n}.src.html'
    if not f.exists():continue
    t=f.read_text().replace('/*__COMMON_CSS__*/',css).replace('/*__COMMON_JS__*/',js)
    (O/f'{n}.html').write_text(t)
    print('built',n)
