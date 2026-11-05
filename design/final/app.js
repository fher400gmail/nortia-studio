/* nortia. studio · master check · D5
   Sin dependencias. Un solo script clásico (funciona con file:// y con el protocolo de Tauri),
   organizado en módulos: Data · Icons · Wave · Fx · Store · Views · Create · Events · Boot.
   Querystring: ?theme=light|dark &state=empty|normal|creating|done &os=mac|win &still=1 &pop=adv|more|tree|detail &drag=1 */
'use strict';

/* ───────────── Data: lo que enviaría el motor (maqueta) ───────────── */
const Data = (() => {
  const ART = 'Vera Alcón', TIT = 'Luz de Noviembre';
  const ids = ['spotify', 'apple', 'amazon', 'tidal', 'deezer', 'ytm', 'dist', 'mp3', 'portada', 'canvas'];
  const name = { spotify: 'Spotify', apple: 'Apple Music', amazon: 'Amazon', tidal: 'Tidal', deezer: 'Deezer', ytm: 'YouTube', dist: 'Estándar', mp3: 'MP3', portada: 'Portada', canvas: 'Canvas' };
  const long = { spotify: 'Spotify', apple: 'Apple Music', amazon: 'Amazon Music', tidal: 'Tidal', deezer: 'Deezer', ytm: 'YouTube', dist: 'Distribuidora', mp3: 'Promo', portada: 'Portada', canvas: 'Canvas' };
  const brand = { spotify: '#1DB954', apple: '#FA243C', amazon: '#232F3E', tidal: '#000000', deezer: '#A238FF', ytm: '#FF0000' };
  const spec = { spotify: 'FLAC · 24 bit · 96 kHz', apple: 'WAV · 24 bit · 96 kHz', amazon: 'FLAC · 24 bit · 96 kHz', tidal: 'FLAC · 24 bit · 96 kHz', deezer: 'WAV · 16 bit · 44,1 kHz', ytm: 'WAV · 24 bit · 48 kHz', dist: 'WAV · 16 bit · 44,1 kHz', mp3: 'MP3 · 320 kbps', portada: 'JPG + PNG · 3000×3000', canvas: 'MP4 · 1080×1920 · 6 s' };
  const out = { spotify: ['01 Spotify', 74.1], apple: ['02 Apple Music', 133.6], amazon: ['03 Amazon Music', 74.1], tidal: ['04 Tidal', 74.1], deezer: ['05 Deezer', 40.9], ytm: ['06 YouTube', 66.8], dist: ['07 Distribuidora', 40.9], mp3: ['08 Promo', 9.3], portada: ['09 Imagen', 8.3], canvas: ['09 Imagen', 3.2] };
  const docs = [['README.txt', .003], ['informe-loudness.txt', .004], ['ficha-lanzamiento.csv', .001], ['ficha-lanzamiento.txt', .002]];
  const folders = ['Escritorio/Entregas', 'Documentos/Lanzamientos', 'Descargas'];
  const meta = { dur: '3:52', durS: 232, lufs: '−9,1', tp: '+1,2' };
  return { ART, TIT, ids, name, long, brand, spec, out, docs, folders, meta };
})();

/* ───────────── Icons ───────────── */
const Icons = (() => {
  const s = (b, x = '') => `<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" ${x}>${b}</svg>`;
  const g = (b, x = '') => `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round" ${x}>${b}</svg>`;
  /* glifos propios evocadores; sustituir por los activos oficiales de cada marca antes de publicar */
  const gly = {
    spotify: g('<path d="M5 9c4.8-1.5 9.6-1.1 14 1.4"/><path d="M6.2 12.8c4-1.1 7.6-.8 10.8 1"/><path d="M7.4 16.2c3-.8 5.6-.5 8 .9"/>', 'stroke-width="2.1"'),
    apple: g('<path d="M10 17.2V6.6l8-1.9v10.6"/><circle cx="7.9" cy="17.4" r="2.2" fill="currentColor"/><circle cx="15.9" cy="15.5" r="2.2" fill="currentColor"/>'),
    amazon: g('<text x="12" y="13" font-size="12.5" font-weight="700" text-anchor="middle" fill="currentColor" stroke="none" font-family="Inter,sans-serif">a</text><path class="ac" d="M5 15.6c4.4 3 9.6 3 14 0" stroke="#FF9900" stroke-width="1.8"/><path class="ac" d="M16.6 15.1l2.6.4-.7 2.5" stroke="#FF9900" stroke-width="1.8"/>'),
    tidal: g('<path d="M12 4.5l3 3-3 3-3-3z M6 10.5l3 3-3 3-3-3z M18 10.5l3 3-3 3-3-3z M12 13.5l3 3-3 3-3-3z" fill="currentColor" stroke="none"/>'),
    deezer: g('<path d="M4 18v-3M8 18v-6M12 18v-9M16 18v-6M20 18V6" stroke-width="3" stroke-linecap="butt"/>'),
    ytm: '<svg viewBox="0 0 24 24"><rect x="3" y="6" width="18" height="12" rx="3.6" fill="currentColor"/><path d="M10.2 9.3v5.4l4.6-2.7z" fill="var(--tc)"/></svg>',
    dist: g('<path d="M4 12l16-8-6 16-3-7z"/>'),
    mp3: g('<path d="M9 18V6l10-2v12"/><circle cx="6.5" cy="18" r="2.5"/><circle cx="16.5" cy="16" r="2.5"/>'),
    portada: g('<rect x="3.5" y="4.5" width="17" height="15" rx="2"/><circle cx="9" cy="10" r="1.6"/><path d="M4 17l5-4.5 4 3.5 3-2.5 4 3.5"/>'),
    canvas: g('<rect x="3.5" y="3.5" width="17" height="17" rx="2"/><path d="M10 8.5v7l5.5-3.5z"/>')
  };
  return {
    gly,
    folder: s('<path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>'),
    folderPlus: s('<path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><path d="M12 10.5v5M9.5 13h5"/>'),
    file: '<svg class="ic f" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linejoin="round"><path d="M6 3h8l5 5v13H6z"/><path d="M14 3v5h5"/></svg>',
    check: s('<path d="M5 12.5l4.5 4.5L19 7.5"/>', 'stroke-width="2.6"'),
    play: '<svg class="ic" viewBox="0 0 24 24" fill="currentColor"><path d="M7 4.5v15l12-7.5z"/></svg>',
    pause: '<svg class="ic" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="5" width="4" height="14" rx="1"/><rect x="14" y="5" width="4" height="14" rx="1"/></svg>',
    reset: s('<path d="M4 12a8 8 0 1 0 2.6-5.9"/><path d="M4 4v5h5"/>'),
    img: s('<rect x="3.5" y="4.5" width="17" height="15" rx="2"/><circle cx="9" cy="10" r="1.6"/><path d="M4 17l5-4.5 4 3.5 3-2.5 4 3.5"/>'),
    down: s('<path d="M6 9l6 6 6-6"/>', 'stroke-width="2"'),
    plus: s('<path d="M12 5v14M5 12h14"/>', 'stroke-width="2"'),
    moon: s('<path d="M20 14.5A8 8 0 0 1 9.5 4 8 8 0 1 0 20 14.5z"/>'),
    sun: s('<circle cx="12" cy="12" r="4"/><path d="M12 2.5v2.2M12 19.3v2.2M2.5 12h2.2M19.3 12h2.2M5.3 5.3l1.6 1.6M17.1 17.1l1.6 1.6M5.3 18.7l1.6-1.6M17.1 6.9l1.6-1.6"/>'),
    checkDraw: '<svg viewBox="0 0 24 24"><path pathLength="1" d="M5.5 12.5l4.2 4.2L18.5 8"/></svg>',
    mark: (h, id = '') => `<svg width="${h * 20 / 64}" height="${h}" viewBox="22 2 20 60" aria-hidden="true"><path d="M32 2L42 31H22Z" fill="#BF7F12"/><path d="M22 33H42L32 62Z" fill="var(--brand)"/></svg>`,
    doneMark: `<svg class="markdone" viewBox="22 2 20 60" aria-hidden="true"><path class="t" d="M32 2L42 31H22Z" fill="#BF7F12"/><path class="b" d="M22 33H42L32 62Z" fill="var(--brand)"/></svg>`,
    cover: s => `<svg class="art" width="${s}" height="${s}" viewBox="0 0 100 100" role="img" aria-label="Portada"><rect width="100" height="100" fill="#1B3A4D"/><rect y="60" width="100" height="40" fill="#12293A"/><circle cx="62" cy="42" r="19" fill="#EBDDC3"/><rect y="58" width="100" height="2.4" fill="#EBDDC3" opacity=".6"/><rect x="12" y="68" width="52" height="2.2" fill="#EBDDC3" opacity=".35"/><rect x="24" y="75" width="38" height="2.2" fill="#EBDDC3" opacity=".25"/><rect x="34" y="82" width="22" height="2.2" fill="#EBDDC3" opacity=".18"/></svg>`
  };
})();

/* ───────────── util ───────────── */
const $ = (s, r = document) => r.querySelector(s);
const $$ = (s, r = document) => [...r.querySelectorAll(s)];
const esc = s => String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;');
const Q = new URLSearchParams(location.search);
const STILL = Q.has('still');
const REDUCED = matchMedia('(prefers-reduced-motion: reduce)').matches;
const nf = (n, d = 0) => n.toFixed(d).replace('.', ',');
if (STILL) document.body.classList.add('still');

/* ───────────── Wave: onda responsiva; la barra bajo el puntero se realza ───────────── */
const Wave = (() => {
  const NS = 'http://www.w3.org/2000/svg';
  const ENV = [[0, .1], [.05, .18], [.08, .5], [.28, .56], [.30, .88], [.5, .86], [.52, .5], [.63, .55], [.65, .94], [.85, .92], [.87, .5], [1, .08]];
  const env = t => { for (let i = 0; i < ENV.length - 1; i++) { const [a, x] = ENV[i], [b, y] = ENV[i + 1]; if (t >= a && t <= b) return x + (y - x) * (t - a) / (b - a); } return .1; };
  const hash = n => { const x = Math.sin(n * 127.1 + 311.7) * 43758.5453; return x - Math.floor(x); };
  let host = null, svg = null, bars = [], prog = null, ro = null, raf = 0, mx = null, onSeek = null;

  function draw() {
    if (!host) return;
    const w = host.clientWidth, h = host.clientHeight;
    if (!w || !h) return;
    const step = w < 520 ? 8 : 9, bw = step - 3.5, n = Math.max(24, Math.floor(w / step));
    const real = n * step - 3.5, x0 = (w - real) / 2;
    svg.setAttribute('viewBox', `0 0 ${w} ${h}`);
    let o = '';
    for (let i = 0; i < n; i++) {
      const t = (i + .5) / n, a = Math.min(1, env(t) * (.62 + .38 * hash(Math.round(t * 180))));
      const bh = Math.max(4, a * (h - 10));
      o += `<rect style="--i:${i}" x="${(x0 + i * step).toFixed(1)}" y="${((h - bh) / 2).toFixed(1)}" width="${bw}" height="${bh.toFixed(1)}" rx="${bw / 2}"/>`;
    }
    svg.innerHTML = o;
    bars = [...svg.children];
    setProgress(prog);
  }
  function setProgress(p) {
    prog = p;
    const n = bars.length;
    bars.forEach((r, i) => r.classList.toggle('p', p != null && (i + .5) / n <= p));
  }
  function swell() {
    raf = 0;
    const w = host.clientWidth;
    bars.forEach(r => {
      const cx = +r.getAttribute('x') + +r.getAttribute('width') / 2;
      const k = mx == null ? 0 : Math.exp(-(((cx - mx) / 46) ** 2));
      r.style.setProperty('--k', k < .02 ? 0 : k.toFixed(3));
    });
    const sc = $('.scrub', host);
    if (sc && mx != null) {
      sc.style.left = mx + 'px';
      const t = Math.max(0, Math.min(1, mx / w)) * Data.meta.durS;
      $('b', sc).textContent = Math.floor(t / 60) + ':' + String(Math.floor(t % 60)).padStart(2, '0');
    }
  }
  const queue = () => { if (!raf) raf = requestAnimationFrame(swell); };

  function mount(el, { progress = null, interactive = false, fresh = false, seek = null } = {}) {
    unmount();
    host = el; prog = progress; onSeek = seek;
    host.innerHTML = '<svg class="wv" aria-hidden="true" preserveAspectRatio="none"></svg>' + (interactive ? '<span class="scrub" aria-hidden="true"><i></i><b>0:00</b></span>' : '');
    svg = host.firstElementChild;
    if (fresh && !STILL) host.dataset.fresh = '';
    ro = new ResizeObserver(draw); ro.observe(host);
    if (interactive && !REDUCED) {
      host.addEventListener('pointermove', e => { mx = e.clientX - host.getBoundingClientRect().left; queue(); });
      host.addEventListener('pointerleave', () => { mx = null; queue(); });
    } else if (interactive) {
      host.addEventListener('pointermove', e => { mx = e.clientX - host.getBoundingClientRect().left; queue(); });
    }
    if (interactive) host.addEventListener('click', e => { const r = host.getBoundingClientRect(); onSeek && onSeek(Math.max(0, Math.min(1, (e.clientX - r.left) / r.width))); });
    draw();
  }
  function unmount() { if (ro) ro.disconnect(); ro = null; host = null; bars = []; mx = null; }
  function ripple() { host && host.closest('.hero').classList.add('finish'); }
  return { mount, unmount, setProgress, ripple };
})();

/* ───────────── Fx: vidrio vivo (reflejos que siguen al puntero + paralaje del ambiente) ───────────── */
const Fx = (() => {
  let raf = 0, ev = null;
  const SEL = '.glass,.go,.play,.bb,.chip,.pop,.tbtn,.cov';
  function frame() {
    raf = 0; if (!ev) return;
    const t = ev.target.closest && ev.target.closest(SEL);
    if (t) { const r = t.getBoundingClientRect(); t.style.setProperty('--mx', (ev.clientX - r.left) + 'px'); t.style.setProperty('--my', (ev.clientY - r.top) + 'px'); }
    if (!REDUCED) {
      const w = $('#window').style;
      w.setProperty('--gx', (ev.clientX / innerWidth * 2 - 1).toFixed(3));
      w.setProperty('--gy', (ev.clientY / innerHeight * 2 - 1).toFixed(3));
    }
  }
  function init() { document.addEventListener('pointermove', e => { ev = e; if (!raf) raf = requestAnimationFrame(frame); }, { passive: true }); }
  return { init };
})();

/* ───────────── Store ───────────── */
const Store = (() => {
  const st = {
    mode: 'idle', loaded: true, artist: Data.ART, title: Data.TIT, cover: 'sample', err: '', pp: 0, pt: 0,
    folder: 0, on: new Set(Data.ids), fresh: true, flash: 0, over: false
  };
  const s = Q.get('state');
  if (s === 'empty') { st.loaded = false; st.cover = null; st.artist = ''; st.title = ''; }
  else if (s === 'creating' || s === 'done') st.mode = s;
  if (Q.has('drag')) st.over = true;
  const files = () => {
    const f = Data.ids.filter(id => st.on.has(id)).map(id => ({ id, fo: Data.out[id][0], mb: Data.out[id][1], name: Data.long[id], ext: id === 'mp3' ? '.mp3' : id === 'canvas' ? '.mp4' : id === 'portada' ? '.jpg' : id === 'spotify' || id === 'amazon' || id === 'tidal' ? '.flac' : '.wav' }));
    if (st.on.has('portada')) f.splice(f.findIndex(x => x.id === 'portada') + 1, 0, { id: 'portada', fo: Data.out.portada[0], mb: 6.4, name: 'Portada', ext: '.png' });
    Data.docs.forEach(d => f.push({ doc: d[0], mb: d[1], fo: '' }));
    return f;
  };
  const stats = () => { const f = files(); return { n: f.length, mb: f.reduce((a, x) => a + x.mb, 0) }; };
  return { st, files, stats };
})();
const S = Store.st;

/* ───────────── Views ───────────── */
const Views = (() => {
  const dest = id => { const on = S.on.has(id); return `<button class="chip glass ${on ? 'on' : ''}" role="switch" aria-checked="${on}" data-act="toggle" data-id="${id}" style="--br:${Data.brand[id] || 'var(--brand)'}"><span class="mono">${Icons.gly[id]}</span><span class="cn">${Data.name[id]}</span><i class="ck">${Icons.checkDraw}</i></button>`; };
  const timeTxt = () => S.pp > 0 ? (s => Math.floor(s / 60) + ':' + String(s % 60).padStart(2, '0') + ' / ' + Data.meta.dur)(Math.round(S.pp * Data.meta.durS)) : Data.meta.dur;
  const lpill = () => `<span class="loudw"><button class="lpill glass" data-act="pop" data-p="detail" aria-expanded="false" aria-controls="pop-detail"><i></i>${Data.meta.lufs} LUFS · pasa de 0 dB</button>
    <span class="pop" id="pop-detail" role="dialog" aria-label="Detalle del volumen" style="top:calc(100% + 8px);right:0"><p>Las plataformas le bajarán el volumen. Tu archivo no se toca.</p>
    <span class="kv"><span>Volumen</span><b>${Data.meta.lufs} LUFS</b></span><span class="kv"><span>Pico más alto</span><b>${Data.meta.tp} dBTP</b></span><span class="kv"><span>Pasa de 0 dB</span><b>3 veces</b></span><span class="kv"><span>Formato</span><b>WAV 24 bit · 96 kHz</b></span></span></span>`;
  const heroBar = () => S.mode === 'idle'
    ? `<div class="hbar"><button class="play" data-act="play" data-playing="${!!S.pt}" aria-label="${S.pt ? 'Pausar' : 'Escuchar'}">${S.pt ? Icons.pause : Icons.play}</button><span class="tm" data-time>${timeTxt()}</span><span class="sp"></span>${lpill()}<button class="ib" data-act="song" title="Cambiar canción" aria-label="Cambiar canción">${Icons.reset}</button></div>`
    : `<div class="hbar"><span class="ttl"><b>${esc(S.title)}</b>${esc(S.artist)}</span><span class="sp"></span>${S.mode === 'creating' ? '<span class="pct" data-pc>0 %</span>' : `<span class="pct" aria-label="Completado">${Icons.check}</span>`}</div>`;
  const hero = () => S.loaded
    ? `<div class="hero glass dz ${S.mode === 'done' ? 'fin' : ''}" id="hero">${heroBar()}<div class="wvw" id="wave"></div></div>`
    : `<div class="hero glass dz empty ${S.over ? 'over' : ''}" id="hero"><button class="dzone" data-act="song"><span class="mk">${Icons.mark(64)}</span><b>Suelta tu canción</b>${S.err ? `<small class="er" role="alert">${esc(S.err)}</small>` : ''}</button></div>`;
  const cover = () => S.cover
    ? `<div class="cov glass has" data-act="cover" role="button" tabindex="0" title="Cambiar portada" aria-label="Cambiar portada">${S.cover === 'sample' ? Icons.cover(84) : `<img src="${S.cover}" alt="Portada" style="width:100%;height:100%;object-fit:cover">`}</div>`
    : `<div class="cov glass" data-act="cover" role="button" tabindex="0" aria-label="Añadir portada">${Icons.img}</div>`;
  const more = () => { const f = (l, k, i, v = '', p = '') => `<div class="f" style="--i:${i}"><label for="m-${k}">${l}</label><input id="m-${k}" data-m="${k}" value="${v}" placeholder="${p}" autocomplete="off"></div>`;
    return `<div class="pop" id="pop-more" role="dialog" aria-label="Más datos">${f('Con (feat.)', 'feat', 0)}${f('Versión', 'ver', 1, '', 'Radio Edit')}${f('Año', 'year', 2, '2026')}${f('ISRC', 'isrc', 3, '', 'CC-XXX-YY-NNNNN')}${f('Compositor/a', 'comp', 4)}${f('Sello', 'label', 5)}</div>`; };
  const sum = () => `<span class="sumw"><button class="sum" data-act="pop" data-p="tree" aria-expanded="false" aria-controls="pop-tree"><span data-sum></span>${Icons.down}</button><div class="pop" id="pop-tree" role="dialog" aria-label="Archivos que se crearán"><div class="tree" id="tree"></div></div></span>`;
  const preset = () => `<button class="preset glass" data-act="preset" data-preset title="Restablecer"><i>${Icons.check}</i><span>Lo necesario</span></button>`;
  const folderLine = () => `<div class="fline">${Icons.folder}<b data-folder>${Data.folders[S.folder]}</b><button class="lnk link-b" data-act="folder" style="padding:1px 6px">cambiar</button></div>`;
  const row = (x, done) => { const n = x.doc ? x.doc.slice(0, x.doc.lastIndexOf('.')) : x.name, e = x.doc ? x.doc.slice(x.doc.lastIndexOf('.')) : x.ext; return `<div class="fr ${done ? '' : 'in'}">${Icons.file}<span class="fn">${esc(n)}<em>${esc(e)}</em></span><span class="r ${done ? 'ok' : ''}">${done ? Icons.check : ''}</span></div>`; };

  function html() {
    if (S.mode === 'creating') return `${hero()}<div class="flist glass" data-flist></div><div class="go busy" role="status"><i class="spin"></i>Creando carpeta…</div>`;
    if (S.mode === 'done') return `${hero()}<div class="donebox">${Icons.doneMark}<h1>Listo: <span data-n></span> archivos en <span data-fname></span></h1>
      <div class="acts"><button class="bb pri" data-act="open">${Icons.folder}<span>Abrir carpeta</span></button><button class="bb glass" data-act="again">${Icons.plus}<span>Nueva canción</span></button></div>${sum()}</div>`;
    return `${hero()}
      <div class="meta">${cover()}<div class="mf"><div class="two"><input class="fi glass" data-f="artist" placeholder="Artista" aria-label="Artista" autocomplete="off"><input class="fi glass" data-f="title" placeholder="Título" aria-label="Título" autocomplete="off"></div>
        <button class="lnk" data-act="pop" data-p="more" aria-expanded="false" aria-controls="pop-more">Más datos${Icons.down}</button>${more()}</div></div>
      <div class="dests"><div><div class="gh"><span>Plataformas</span><span class="sp"></span>${preset()}${sum()}</div>
        <div class="tiles">${['spotify', 'apple', 'amazon', 'tidal', 'deezer', 'ytm'].map(dest).join('')}</div></div>
        <div class="row2"><div class="d1"><div class="gh">Distribuidora</div><div class="tiles" style="--n:1">${dest('dist')}</div></div>
        <div class="d3"><div class="gh">Extras</div><div class="tiles" style="--n:3">${['mp3', 'portada', 'canvas'].map(dest).join('')}</div></div></div></div>
      <button class="go" data-act="create">${Icons.folderPlus}Crear carpeta de entrega</button>${folderLine()}`;
  }
  return { html, row, timeTxt };
})();

/* ───────────── render / estado vivo ───────────── */
const App = (() => {
  function live() {
    const st = Store.stats();
    $$('[data-sum]').forEach(e => e.textContent = `${st.n} archivos · ${Math.round(st.mb)} MB`);
    $$('[data-n]').forEach(e => e.textContent = st.n);
    const def = S.on.size === Data.ids.length;
    $$('[data-preset]').forEach(e => { e.classList.toggle('cus', !def); $('span', e).textContent = def ? 'Lo necesario' : 'Personalizado'; $('i', e).innerHTML = def ? Icons.check : Icons.reset; });
    $$('[data-fname]').forEach(e => e.textContent = S.artist + ' - ' + S.title);
    $$('[data-folder]').forEach(e => e.textContent = Data.folders[S.folder]);
    const t = $('#tree');
    if (t) {
      const f = Store.files().filter(x => !x.doc); let last = null;
      let h = `<div class="tr dir">${Icons.folder}<div class="nm">${esc(S.artist + ' - ' + S.title)}/</div></div>`;
      f.forEach(x => { if (x.fo !== last) { last = x.fo; h += `<div class="tr dir" style="padding-left:16px">${Icons.folder}<div class="nm">${x.fo}/</div></div>`; } h += `<div class="tr" style="padding-left:32px">${Icons.file}<div class="nm">${esc(x.name)}${x.ext}</div><span class="sz">${nf(x.mb, 1)} MB</span></div>`; });
      Data.docs.forEach(d => h += `<div class="tr" style="padding-left:18px">${Icons.file}<div class="nm">${d[0]}</div><span class="sz">&lt;1 KB</span></div>`);
      t.innerHTML = h;
    }
  }
  function setDest(id, bump) {
    $$(`[data-id="${id}"]`).forEach(b => { const on = S.on.has(id); b.classList.toggle('on', on); b.setAttribute('aria-checked', on);
      if (bump && on) { b.classList.remove('bump'); void b.offsetWidth; b.classList.add('bump'); } });
  }
  function render() {
    Create.stop();
    $('#view').innerHTML = Views.html();
    $$('input[data-f]').forEach(i => { i.value = S[i.dataset.f]; if (S.flash && i.value) i.classList.add('filled'); }); S.flash = 0;
    const w = $('#wave');
    if (w) Wave.mount(w, { progress: S.mode === 'done' ? 1 : S.mode === 'creating' ? 0 : (S.pp > 0 ? S.pp : null), interactive: S.mode === 'idle', fresh: S.fresh, seek: p => { S.pp = p; Wave.setProgress(p); paintPlay(); } });
    else Wave.unmount();
    live(); S.fresh = false;
    const p = Q.get('pop');
    if (p && !App.popDone) { App.popDone = 1; requestAnimationFrame(() => { const e = $('#pop-' + p); if (e) { e.classList.add('open'); const b = $(`[data-p="${p}"]`); b && b.setAttribute('aria-expanded', 'true'); } }); }
    if (S.mode === 'creating') Create.start();
    if (S.mode === 'done') { const l = $('[data-flist]'); if (l) l.innerHTML = Store.files().map(x => Views.row(x, 1)).join(''); if (!STILL) setTimeout(Wave.ripple, 60); }
  }
  function closePops(except) { $$('.pop.open').forEach(p => { if (p !== except) p.classList.remove('open'); }); $$('[aria-controls][aria-expanded=true]').forEach(b => { if (!except || b.getAttribute('aria-controls') !== except.id) b.setAttribute('aria-expanded', 'false'); }); }
  function togglePop(name) { const el = $('#pop-' + name), was = el.classList.contains('open'); closePops(); if (!was) { el.classList.add('open'); const b = $(`[data-p="${name}"]`); b && b.setAttribute('aria-expanded', 'true'); const f = $('input', el); if (f && name === 'more') setTimeout(() => f.focus({ preventScroll: true }), 120); } }
  function paintPlay() {
    $$('[data-time]').forEach(e => e.textContent = Views.timeTxt());
    $$('[data-act=play]').forEach(b => { b.innerHTML = S.pt ? Icons.pause : Icons.play; b.dataset.playing = !!S.pt; b.setAttribute('aria-label', S.pt ? 'Pausar' : 'Escuchar'); });
  }
  function togglePlay() { if (S.pt) { clearInterval(S.pt); S.pt = 0; } else S.pt = setInterval(() => { S.pp += .02; if (S.pp >= 1) { S.pp = 0; clearInterval(S.pt); S.pt = 0; } Wave.setProgress(S.pp > 0 ? S.pp : null); paintPlay(); }, 100); paintPlay(); }
  return { live, setDest, render, closePops, togglePop, togglePlay, paintPlay, popDone: 0 };
})();

/* ───────────── Create: simulación del proceso; la barra es la propia onda ───────────── */
const Create = (() => {
  let raf = 0;
  const stop = () => { cancelAnimationFrame(raf); raf = 0; };
  function start() {
    const F = Store.files(), du = F.map(x => x.doc ? .18 : .28 + Math.min(x.mb, 140) / 140 * .5), st = []; let c = 0; du.forEach(d => { st.push(c); c += d; }); const total = c;
    const paint = t => {
      const list = $('[data-flist]'); if (!list) return; let shown = 0; F.forEach((x, i) => { if (t >= st[i]) shown = i + 1; });
      while (list.children.length < shown) { list.insertAdjacentHTML('beforeend', Views.row(F[list.children.length], STILL)); list.scrollTop = list.scrollHeight; }
      F.forEach((x, i) => { if (i >= shown) return; const p = Math.min(1, (t - st[i]) / du[i]), r = list.children[i].querySelector('.r'); if (p >= 1) { if (!r.classList.contains('ok')) { r.classList.add('ok'); r.innerHTML = Icons.check; } } else r.textContent = Math.round(p * 100) + ' %'; });
      const pr = Math.min(1, t / total); Wave.setProgress(pr); $$('[data-pc]').forEach(e => e.textContent = Math.round(pr * 100) + ' %');
    };
    if (STILL) { paint(st[8] + du[8] * .45); return; }
    const t0 = performance.now();
    const tick = () => { const t = (performance.now() - t0) / 1000; paint(t); if (t >= total + .3) { S.mode = 'done'; App.render(); return; } raf = requestAnimationFrame(tick); };
    raf = requestAnimationFrame(tick);
  }
  return { start, stop };
})();

/* ───────────── Events ───────────── */
const Events = (() => {
  const parseName = n => { let b = n.replace(/\.[^.]+$/, '').replace(/_+/g, ' '); b = b.replace(/\s*[\(\[-]?\s*(master|mastered|final|mix|v\d+)[^\)\]]*[\)\]]?\s*$/i, '').trim(); const m = b.match(/^(.+?)\s+-\s+(.+)$/); return m ? { artist: m[1].trim(), title: m[2].trim() } : { title: b }; };
  function loadSong(f) {
    const ext = (f && f.name || '').split('.').pop().toLowerCase();
    if (f && !['wav', 'aif', 'aiff', 'flac', 'caf', 'm4a'].includes(ext)) { S.err = ['mp3', 'aac', 'ogg'].includes(ext) ? 'Un MP3 no sirve como máster. Usa WAV o FLAC.' : 'Usa un archivo WAV, AIFF o FLAC.'; App.render(); setTimeout(() => { S.err = ''; if (!S.loaded && S.mode === 'idle') App.render(); }, 3200); return; }
    S.err = ''; S.loaded = true; S.fresh = true; S.pp = 0; S.flash = 1; S.over = false;
    if (f) { const p = parseName(f.name); S.artist = p.artist || S.artist; S.title = p.title; } else { S.artist = Data.ART; S.title = Data.TIT; if (!S.cover) S.cover = 'sample'; }
    App.render();
  }
  function loadCover(f) { if (!f || !f.type.startsWith('image/')) return; S.cover = URL.createObjectURL(f); App.render(); }
  const missing = () => !S.loaded ? 'song' : !S.artist.trim() ? 'artist' : !S.title.trim() ? 'title' : !S.on.size ? 'dest' : null;
  function nudge(k) {
    let el = k === 'song' ? $('#hero') : k === 'dest' ? $('.dests') : $(`input[data-f=${k}]`); if (!el) return;
    if (k === 'artist' || k === 'title') { el.focus(); el.classList.add('in-err'); setTimeout(() => el.classList.remove('in-err'), 900); }
    el.classList.remove('nudge'); void el.offsetWidth; el.classList.add('nudge'); setTimeout(() => el.classList.remove('nudge'), 700);
  }
  function winCtl(a) { const w = window.__TAURI__ && window.__TAURI__.window && window.__TAURI__.window.getCurrentWindow && window.__TAURI__.window.getCurrentWindow(); if (!w) return; if (a === 'min') w.minimize(); if (a === 'max') w.toggleMaximize(); if (a === 'close') w.close(); }
  function init() {
    document.addEventListener('click', e => {
      const wb = e.target.closest('[data-win]'); if (wb) return winCtl(wb.dataset.win);
      const a = e.target.closest('[data-act]');
      if (!e.target.closest('.pop') && !(a && a.dataset.act === 'pop')) App.closePops();
      if (!a) return; const act = a.dataset.act;
      if (act === 'pop') return App.togglePop(a.dataset.p);
      if (act === 'toggle') { if (S.mode !== 'idle') return; S.on.has(a.dataset.id) ? S.on.delete(a.dataset.id) : S.on.add(a.dataset.id); App.setDest(a.dataset.id, 1); App.live(); }
      else if (act === 'preset') { Data.ids.forEach(id => { S.on.add(id); App.setDest(id, 1); }); App.live(); }
      else if (act === 'song') $('#fsong').click();
      else if (act === 'cover') $('#fcov').click();
      else if (act === 'play') App.togglePlay();
      else if (act === 'folder') { S.folder = (S.folder + 1) % Data.folders.length; App.live(); }
      else if (act === 'create') { const m = missing(); if (m) return nudge(m); S.mode = 'creating'; App.render(); }
      else if (act === 'again') { S.mode = 'idle'; S.loaded = false; S.cover = null; S.artist = ''; S.title = ''; S.pp = 0; Data.ids.forEach(id => S.on.add(id)); App.render(); }
      else if (act === 'open') { const s = $('span', a); s.textContent = 'Abriendo…'; setTimeout(() => s.textContent = 'Abrir carpeta', 1200); }
    });
    document.addEventListener('keydown', e => { if (e.key === 'Escape') { const o = $('.pop.open'); const tr = o && $(`[aria-controls="${o.id}"]`); App.closePops(); if (tr) tr.focus(); } if ((e.key === 'Enter' || e.key === ' ') && e.target.matches('.cov')) { e.preventDefault(); e.target.click(); } });
    document.addEventListener('input', e => { const i = e.target.closest('input[data-f]'); if (i) { S[i.dataset.f] = i.value; App.live(); } });
    document.addEventListener('change', e => { if (e.target.id === 'fsong') { loadSong(e.target.files[0] || null); e.target.value = ''; } if (e.target.id === 'fcov') { loadCover(e.target.files[0]); e.target.value = ''; } });
    ['dragenter', 'dragover'].forEach(n => document.addEventListener(n, e => { e.preventDefault(); const z = e.target.closest('.dz,.cov'); if (z) z.classList.add('over'); }));
    ['dragleave', 'drop'].forEach(n => document.addEventListener(n, e => { e.preventDefault(); $$('.over').forEach(z => z.classList.remove('over')); }));
    document.addEventListener('drop', e => { if (S.mode !== 'idle') return; const f = e.dataTransfer.files[0]; if (!f) return; if (e.target.closest('.cov')) loadCover(f); else if (e.target.closest('.dz')) f.type.startsWith('image/') ? loadCover(f) : loadSong(f); });
  }
  return { init };
})();

/* ───────────── Boot ───────────── */
(() => {
  const d = document.documentElement;
  /* tema */
  const tb = $('#themeBtn'), paintTheme = () => tb.innerHTML = d.dataset.theme === 'dark' ? Icons.sun.replace('class="ic"', 'class="ic turn"') : Icons.moon.replace('class="ic"', 'class="ic turn"');
  paintTheme();
  tb.addEventListener('click', () => { d.dataset.theme = d.dataset.theme === 'dark' ? 'light' : 'dark'; paintTheme(); const u = new URL(location); u.searchParams.set('theme', d.dataset.theme); history.replaceState(0, '', u); });
  /* ajustes avanzados */
  $('#pop-adv').innerHTML = `<h4>Ajustes avanzados</h4>${Data.ids.map(id => `<div class="kv"><span>${Data.long[id]}</span><b>${Data.spec[id]}</b></div>`).join('')}`;
  $('#pop-adv').classList.add('glass-pop');
  Fx.init(); Events.init(); App.render();
})();
