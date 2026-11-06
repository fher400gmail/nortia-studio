/* nortia. studio · master check · interfaz
   Módulo ES sin bundler. Los datos llegan del motor a través de `backend.js` (mock en el navegador,
   Tauri en la app) y los textos de i18n/{es,en}.json. Para abrirla en un navegador sirve la carpeta
   por http (p. ej. `python3 -m http.server`): los módulos ES y fetch no funcionan con file://.
   Querystring: ?backend=mock|tauri &lang=es|en &theme=light|dark &state=empty|normal|creating|done
                &os=mac|win &still=1 &fast=1 &pop=adv|more|tree|detail &drag=1 &sidecar=0
                (mock) &off=tidal,apple_motion → ids de preset con default_on:false (por defecto apple_motion)
                 &cover=1|rect &own=canvas,ytv,motion  → portada y material propio de ejemplo */
import { backend } from './backend.js';
import { loadLang, t, tn, getLang } from './i18n.js';
import { isCancelled, resolveDefaults, cancelOnFirstEvent, awaitingMaterial, finalNotes } from './flow.js';
import { fmtLufs, fmtDbtp, fmtDuration, fmtMB, fmtRate, fmtSpec, shortPath } from './format.js';

/* ───────────── Data: tabla de fichas de la interfaz ↔ destinos del motor ───────────── */
const Data = (() => {
  const ids = ['spotify', 'apple', 'amazon', 'tidal', 'deezer', 'ytm', 'dist', 'mp3', 'portada', 'canvas', 'ytv', 'motion'];
  /* ficha → preset del motor (presets.json) */
  const preset = { spotify: 'spotify', apple: 'apple_adm', amazon: 'amazon', tidal: 'tidal', deezer: 'deezer', ytm: 'youtube_music', dist: 'distributor_std', mp3: 'mp3_promo', portada: 'cover', canvas: 'canvas', ytv: 'youtube_video', motion: 'apple_motion' };
  /* fichas que aceptan material propio (imagen o vídeo arrastrado); la de portada es la portada de la entrega */
  const media = new Set(['portada', 'canvas', 'ytv', 'motion']);
  /* fichas que necesitan el sidecar de ffmpeg (MP3 y vídeo) */
  const needsSidecar = new Set(['mp3', 'canvas', 'ytv', 'motion']);
  /* fichas encendidas al empezar: `default_on` de presets.json, que el motor da en app_info
     (una sola fuente de verdad). Hasta que llega, y si no llega la lista, el conjunto seguro
     (todo salvo Apple Motion, como el prototipo aprobado) con un aviso en consola. */
  const presetIds = Object.values(preset);
  let onIds = resolveDefaults(undefined, presetIds);
  const setDefaults = list => { onIds = resolveDefaults(list, presetIds, m => console.warn(m)); };
  const defaultOn = id => onIds.has(preset[id]);
  const defaults = () => new Set(ids.filter(defaultOn));
  const byPreset = Object.fromEntries(Object.entries(preset).map(([k, v]) => [v, k]));
  const ext = { flac: '.flac', wav: '.wav', mp3: '.mp3', mp4: '.mp4', jpg: '.jpg', png: '.png' };
  const SAMPLE = 'Vera Alcón - Luz de Noviembre (master v3).wav';
  return { ids, preset, byPreset, ext, SAMPLE, media, needsSidecar, defaultOn, defaults, setDefaults };
})();

/* ───────────── Icons ───────────── */
const Icons = (() => {
  const s = (b, x = '') => `<svg class="ic" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" ${x}>${b}</svg>`;
  const g = (b, x = '') => `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round" ${x}>${b}</svg>`;
  /* glifos propios y neutros (formas genéricas, sin logos de terceros) */
  // TODO(brand): antes de la publicación, sustituir por los activos oficiales de cada plataforma, según sus guías de marca.
  const gly = {
    spotify: g('<path d="M5 9c4.8-1.5 9.6-1.1 14 1.4"/><path d="M6.2 12.8c4-1.1 7.6-.8 10.8 1"/><path d="M7.4 16.2c3-.8 5.6-.5 8 .9"/>', 'stroke-width="2.1"'),
    apple: g('<path d="M10 17.2V6.6l8-1.9v10.6"/><circle cx="7.9" cy="17.4" r="2.2" fill="currentColor"/><circle cx="15.9" cy="15.5" r="2.2" fill="currentColor"/>'),
    amazon: g('<text x="12" y="13" font-size="12.5" font-weight="700" text-anchor="middle" fill="currentColor" stroke="none" font-family="Hanken Grotesk,sans-serif">a</text><path class="ac" d="M5 15.6c4.4 3 9.6 3 14 0" stroke="currentColor" stroke-width="1.8"/><path class="ac" d="M16.6 15.1l2.6.4-.7 2.5" stroke="currentColor" stroke-width="1.8"/>'),
    tidal: g('<path d="M12 4.5l3 3-3 3-3-3z M6 10.5l3 3-3 3-3-3z M18 10.5l3 3-3 3-3-3z M12 13.5l3 3-3 3-3-3z" fill="currentColor" stroke="none"/>'),
    deezer: g('<path d="M4 18v-3M8 18v-6M12 18v-9M16 18v-6M20 18V6" stroke-width="3" stroke-linecap="butt"/>'),
    ytm: '<svg viewBox="0 0 24 24"><rect x="3" y="6" width="18" height="12" rx="3.6" fill="currentColor"/><path d="M10.2 9.3v5.4l4.6-2.7z" fill="var(--tc)"/></svg>',
    dist: g('<path d="M4 12l16-8-6 16-3-7z"/>'),
    mp3: g('<path d="M9 18V6l10-2v12"/><circle cx="6.5" cy="18" r="2.5"/><circle cx="16.5" cy="16" r="2.5"/>'),
    portada: g('<rect x="3.5" y="4.5" width="17" height="15" rx="2"/><circle cx="9" cy="10" r="1.6"/><path d="M4 17l5-4.5 4 3.5 3-2.5 4 3.5"/>'),
    canvas: g('<rect x="3.5" y="3.5" width="17" height="17" rx="2"/><path d="M10 8.5v7l5.5-3.5z"/>'),
    ytv: g('<rect x="3" y="4.5" width="18" height="11.5" rx="2"/><path d="M10.2 7.6v5.3l4.3-2.65z" fill="currentColor"/><path d="M6.5 19.5h11"/>'),
    motion: g('<rect x="8" y="8" width="12.5" height="12.5" rx="2"/><path d="M16 4H6.5A2.5 2.5 0 0 0 4 6.5V16"/>')
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
    x: s('<path d="M7 7l10 10M17 7L7 17"/>', 'stroke-width="2.4"'),
    moon: s('<path d="M20 14.5A8 8 0 0 1 9.5 4 8 8 0 1 0 20 14.5z"/>'),
    sun: s('<circle cx="12" cy="12" r="4"/><path d="M12 2.5v2.2M12 19.3v2.2M2.5 12h2.2M19.3 12h2.2M5.3 5.3l1.6 1.6M17.1 17.1l1.6 1.6M5.3 18.7l1.6-1.6M17.1 6.9l1.6-1.6"/>'),
    /* aviso de estado: triángulo con filete (el color lo pone el contexto; lleva siempre palabra) */
    warn: s('<path d="M12 4.5l9 15.5H3z"/><path d="M12 10v4.2M12 17.2v.1"/>', 'stroke-width="2"'),
    checkDraw: '<svg viewBox="0 0 24 24"><path pathLength="1" d="M5.5 12.5l4.2 4.2L18.5 8"/></svg>',
    /* símbolo barra de nortia. studio (punto estirado 2:1), en dos mitades para la entrada por polos */
    mark: (h, id = '') => `<svg width="${h / 2}" height="${h}" viewBox="0 0 28 56" aria-hidden="true"><path d="M0 14a14 14 0 0 1 28 0v14H0z" fill="var(--brand)"/><path d="M0 28h28v14a14 14 0 0 1-28 0z" fill="var(--brand)"/></svg>`,
    doneMark: `<svg class="markdone" viewBox="0 0 28 56" aria-hidden="true"><path class="t" d="M0 14a14 14 0 0 1 28 0v14H0z" fill="var(--brand)"/><path class="b" d="M0 28h28v14a14 14 0 0 1-28 0z" fill="var(--brand)"/></svg>`,
  };
})();
/* ───────────── util ───────────── */
const $ = (s, r = document) => r.querySelector(s);
const $$ = (s, r = document) => [...r.querySelectorAll(s)];
const esc = s => String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/"/g, '&quot;');
const Q = new URLSearchParams(location.search);
const STILL = Q.has('still');
const REDUCED = matchMedia('(prefers-reduced-motion: reduce)').matches;
const lg = () => getLang();
const baseName = p => String(p || '').split(/[\\/]/).filter(Boolean).pop() || '';
const errText = e => (e && (e.message || (e.key && t(e.key)))) || t('err.generic');
if (STILL) document.body.classList.add('still');

/* ───────────── Wave: onda real (waveform.bins); la barra bajo el puntero se realza ───────────── */
const Wave = (() => {
  const NS = 'http://www.w3.org/2000/svg';
  let host = null, svg = null, bars = [], prog = null, ro = null, raf = 0, mx = null, onSeek = null, bins = [], peak = 1, dur = 0;

  /* pico máximo de los bins que caen en cada barra, normalizado al más alto */
  const level = (i, n) => {
    const a = Math.floor(i * bins.length / n), b = Math.max(a + 1, Math.floor((i + 1) * bins.length / n));
    let m = 0; for (let k = a; k < b && k < bins.length; k++) if (bins[k] > m) m = bins[k];
    return Math.min(1, m / peak);
  };
  function draw() {
    if (!host) return;
    const w = host.clientWidth, h = host.clientHeight;
    if (!w || !h) return;
    const step = w < 520 ? 8 : 9, bw = step - 3.5, n = Math.max(24, Math.floor(w / step));
    const real = n * step - 3.5, x0 = (w - real) / 2;
    svg.setAttribute('viewBox', `0 0 ${w} ${h}`);
    let o = '';
    for (let i = 0; i < n; i++) {
      const bh = Math.max(4, level(i, n) * (h - 10));
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
    if (!host) return;
    const w = host.clientWidth;
    bars.forEach(r => {
      const cx = +r.getAttribute('x') + +r.getAttribute('width') / 2;
      const k = mx == null ? 0 : Math.exp(-(((cx - mx) / 46) ** 2));
      r.style.setProperty('--k', k < .02 ? 0 : k.toFixed(3));
    });
    const sc = $('.scrub', host);
    if (sc && mx != null) {
      sc.style.left = mx + 'px';
      $('b', sc).textContent = fmtDuration(Math.max(0, Math.min(1, mx / w)) * dur);
    }
  }
  const queue = () => { if (!raf) raf = requestAnimationFrame(swell); };

  function mount(el, { waveform = [], duration = 0, progress = null, interactive = false, fresh = false, seek = null } = {}) {
    unmount();
    host = el; prog = progress; onSeek = seek; bins = waveform; dur = duration;
    peak = bins.reduce((m, v) => (v > m ? v : m), 0) || 1;
    host.innerHTML = '<svg class="wv" aria-hidden="true" preserveAspectRatio="none"></svg>' + (interactive ? '<span class="scrub" aria-hidden="true"><i></i><b>0:00</b></span>' : '');
    svg = host.firstElementChild;
    if (fresh && !STILL) host.dataset.fresh = '';
    ro = new ResizeObserver(draw); ro.observe(host);
    if (interactive) {
      host.addEventListener('pointermove', e => { mx = e.clientX - host.getBoundingClientRect().left; queue(); });
      if (!REDUCED) host.addEventListener('pointerleave', () => { mx = null; queue(); });
      host.addEventListener('click', e => { const r = host.getBoundingClientRect(); onSeek && onSeek(Math.max(0, Math.min(1, (e.clientX - r.left) / r.width))); });
    }
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
    mode: 'idle', loaded: false, busy: false, artist: '', title: '', m: {}, cover: null, coverPath: null,
    err: '', banner: '', pp: 0, pt: 0, fresh: true, flash: 0, over: false,
    master: null, waveform: [], plan: null, planSeq: 0, outRoot: '', info: { sidecar_available: true, os: 'mac' },
    media: {}, notes: [], vnotes: [], prep: null,
    on: Data.defaults(), rows: [], delivery: null, cancelled: false, canceling: false
  };
  if (Q.has('drag')) st.over = true;

  const job = id => (st.plan ? st.plan.jobs.find(j => j.preset_id === Data.preset[id]) : null);
  /* null = disponible; si no, el motivo (tooltip) */
  const disabledReason = id => {
    const j = job(id);
    if (j && !j.enabled) return j.disabled_reason || t('err.generic');
    if (Data.needsSidecar.has(id) && st.info && st.info.sidecar_available === false) return t('sidecar.missing');
    return null;
  };
  /* material propio de una ficha: { name, thumb } o null. La portada es la de la entrega. */
  const own = id => id === 'portada' ? (st.cover ? { name: baseName(st.coverPath) || t('cover.alt'), thumb: st.cover } : null) : (st.media[id] || null);
  /* motor: ruta del material propio por destino (la portada viaja en `meta.cover_path`) */
  const mediaInputs = () => Object.fromEntries(Object.entries(st.media).map(([id, m]) => [Data.preset[id], m.path]));
  /* la ficha espera material (portada o el suyo) y acepta que se lo suelten: se insinúa con borde discontinuo */
  const awaiting = id => awaitingMaterial({ media: Data.media, id, disabled: !!disabledReason(id), own: !!own(id), cover: !!st.cover, sidecar: st.info && st.info.sidecar_available });
  const plannedVerdicts = () => Object.fromEntries((st.plan ? st.plan.jobs : []).map(j => [j.preset_id, j.verdict_text || '']));
  const notesOf = jobs => [...new Set(jobs.flatMap(j => j.warnings || []))];
  const selected = () => Data.ids.filter(id => st.on.has(id) && !disabledReason(id));
  const selectedJobs = () => selected().map(id => job(id)).filter(j => j && j.enabled);
  const allOn = () => Data.ids.every(id => disabledReason(id) || st.on.has(id) === Data.defaultOn(id));
  const docs = () => [t('doc.readme'), t('doc.loudness_report'), t('doc.sheet_csv'), t('doc.sheet_txt')];
  const stats = () => { const js = selectedJobs(); return { n: js.reduce((a, j) => a + 1 + (j.extra_files || []).length, 4), bytes: js.reduce((a, j) => a + j.estimated_bytes, 0) + 10000 }; };
  const meta = () => {
    const nz = v => { v = (v || '').trim(); return v ? v : null; };
    return {
      artist: st.artist.trim(), title: st.title.trim(), version: nz(st.m.ver), featuring: nz(st.m.feat), album: null,
      year: nz(st.m.year), isrc: nz(st.m.isrc), composer: nz(st.m.comp), label: nz(st.m.label), copyright: null, cover_path: st.coverPath
    };
  };
  return { st, job, disabledReason, own, awaiting, plannedVerdicts, mediaInputs, notesOf, selected, selectedJobs, allOn, docs, stats, meta };
})();
const S = Store.st;

/* ───────────── Views ───────────── */
const Views = (() => {
  /* tooltip de una ficha: el motivo si está deshabilitada; si no, el material propio o la pista de soltar */
  const chipTitle = id => {
    const why = Store.disabledReason(id), own = Store.own(id);
    if (why) return why;
    const j = Store.job(id), notes = j && j.warnings && j.warnings.length ? ' ' + j.warnings.join(' ') : '';
    if (own) return t('media.own', { name: own.name }) + notes;
    return (Data.media.has(id) ? t('media.hint.' + id) : '') + notes;
  };
  /* nombre de un destino del motor; «documents» (documentos de la entrega) y los desconocidos tienen el suyo */
  const destName = id => { const k = 'chip.long.' + (Data.byPreset[id] || id), s = t(k); return s === k ? id : s; };
  const dest = id => {
    const why = Store.disabledReason(id), on = S.on.has(id) && !why, own = Store.own(id), title = chipTitle(id), acc = Store.awaiting(id);
    const mono = own && own.thumb ? `<img class="mthumb" src="${esc(own.thumb)}" alt="">` : Icons.gly[id];
    const btn = `<button class="chip glass ${on ? 'on' : ''} ${why ? 'dis' : ''} ${own ? 'has-media' : ''} ${acc ? 'acc' : ''}" role="switch" aria-checked="${on}" ${why ? 'aria-disabled="true"' : ''} ${title ? `title="${esc(title)}"` : ''} data-act="toggle" data-id="${id}"><span class="mono">${mono}</span><span class="cn">${esc(t('chip.' + id))}</span>${acc ? `<small class="dh">${esc(t('accept.drop'))}</small>` : ''}<i class="ck">${Icons.checkDraw}</i></button>`;
    if (!Data.media.has(id)) return btn;
    const clear = own ? `<span class="mx" role="button" tabindex="0" data-act="media-clear" data-mid="${id}" aria-label="${esc(t('media.clear', { name: own.name }))}" title="${esc(t('media.clear', { name: own.name }))}">${Icons.x}</span>` : '';
    const pick = `<span class="mp" role="button" tabindex="0" data-act="media-pick" data-mid="${id}" aria-label="${esc(t('media.pick'))}" title="${esc(t('media.pick'))}">${Icons.plus}</span>`;
    return `<div class="chipw" data-drop="${id}">${btn}${clear}${pick}</div>`;
  };
  const dur = () => (S.master ? S.master.duration_secs : 0);
  const timeTxt = () => S.pp > 0 ? fmtDuration(S.pp * dur()) + ' / ' + fmtDuration(dur()) : fmtDuration(dur());
  const format = m => t('fmt.wav', { depth: m.is_float ? t('fmt.float32') : t('fmt.bits', { n: m.bits }), rate: fmtRate(m.sample_rate, lg()) });
  const lpill = () => {
    const m = S.master, over = m.true_peak_dbtp > 0;
    return `<span class="loudw"><button class="lpill glass" data-act="pop" data-p="detail" aria-expanded="false" aria-controls="pop-detail"><i></i>${fmtLufs(m.integrated_lufs, lg())} · ${t(over ? 'loud.over' : 'loud.ok')}</button>
    <span class="pop" id="pop-detail" role="dialog" aria-label="${esc(t('loud.pop.title'))}" style="top:calc(100% + 8px);right:0"><p>${t('loud.pop.note')}</p>
    <span class="kv"><span>${t('loud.k.vol')}</span><b>${fmtLufs(m.integrated_lufs, lg())}</b></span><span class="kv"><span>${t('loud.k.peak')}</span><b>${fmtDbtp(m.true_peak_dbtp, lg())}</b></span><span class="kv"><span>${t('loud.k.over')}</span><b>${m.overs_count ? tn('loud.over', m.overs_count) : t('loud.never')}</b></span><span class="kv"><span>${t('loud.k.format')}</span><b>${format(m)}</b></span></span></span>`;
  };
  const heroBar = () => S.mode === 'idle'
    ? `<div class="hbar"><button class="play" data-act="play" data-playing="${!!S.pt}" aria-label="${t(S.pt ? 'pause' : 'play')}">${S.pt ? Icons.pause : Icons.play}</button><span class="tm" data-time>${timeTxt()}</span><span class="sp"></span>${lpill()}<button class="ib" data-act="song" title="${esc(t('song.change'))}" aria-label="${esc(t('song.change'))}">${Icons.reset}</button></div>`
    : `<div class="hbar"><span class="ttl"><b>${esc(S.title)}</b>${esc(S.artist)}</span><span class="sp"></span>${S.mode === 'creating' ? `<span class="pct" data-pc>${t('pct', { n: 0 })}</span>` : `<span class="pct" role="img" aria-label="${esc(t('done.completed'))}">${Icons.check}</span>`}</div>`;
  const hero = () => S.loaded
    ? `<div class="hero glass dz ${S.mode === 'done' ? 'fin' : S.mode === 'creating' ? 'making' : ''}" id="hero">${heroBar()}<div class="wvw" id="wave"></div></div>`
    : `<div class="hero glass dz empty ${S.over ? 'over' : ''}" id="hero"><button class="dzone" data-act="song"><span class="mk">${Icons.mark(64)}</span><b>${t('drop.title')}</b>${S.err ? `<small class="er" role="alert">${esc(S.err)}</small>` : ''}</button></div>`;
  const cover = () => S.cover
    ? `<div class="cov has" data-act="cover" role="button" tabindex="0" title="${esc(t('cover.change'))}" aria-label="${esc(t('cover.change'))}"><img src="${esc(S.cover)}" alt="${esc(t('cover.alt'))}" style="width:100%;height:100%;object-fit:cover"></div>`
    : `<div class="cov empty" data-act="cover" role="button" tabindex="0" aria-label="${esc(t('cover.drag').replace('\n', ' '))}"><span class="cov-lbl">${esc(t('cover.drag')).replace('\n', '<br>')}</span></div>`;
  const more = () => { const f = (l, k, i, ph = '') => `<div class="f" style="--i:${i}"><label for="m-${k}">${esc(l)}</label><input id="m-${k}" data-m="${k}" value="${esc(S.m[k] || '')}" placeholder="${esc(ph)}" autocomplete="off"></div>`;
    return `<div class="pop" id="pop-more" role="dialog" aria-label="${esc(t('more'))}">${f(t('more.feat'), 'feat', 0)}${f(t('more.ver'), 'ver', 1, t('more.ver.ph'))}${f(t('more.year'), 'year', 2, String(new Date().getFullYear()))}${f(t('more.isrc'), 'isrc', 3, 'CC-XXX-YY-NNNNN')}${f(t('more.comp'), 'comp', 4)}${f(t('more.label'), 'label', 5)}</div>`; };
  const sum = () => `<span class="sumw"><button class="sum" data-act="pop" data-p="tree" aria-expanded="false" aria-controls="pop-tree"><span data-sum></span>${Icons.down}</button><div class="pop" id="pop-tree" role="dialog" aria-label="${esc(t('tree.title'))}"><div class="tree" id="tree"></div></div></span>`;
  const preset = () => `<button class="preset glass" data-act="preset" data-preset title="${esc(t('preset.reset'))}"><i>${Icons.check}</i><span>${t('preset.default')}</span></button>`;
  const folderLine = () => `<div class="fline">${Icons.folder}<b data-folder></b><button class="lnk link-b" data-act="folder" style="padding:1px 6px">${t('folder.change')}</button></div>`;
  /* x: { name, ext, state: 'run'|'done'|'fail', pct, title } */
  const row = (x, done) => `<div class="fr ${done ? '' : 'in'}" ${x.title ? `title="${esc(x.title)}"` : ''}>${Icons.file}<span class="fn">${esc(x.name)}<em>${esc(x.ext)}</em></span><span class="r ${x.state === 'done' || done ? 'ok' : ''}" ${x.state === 'fail' ? 'style="color:var(--err)"' : ''}>${x.state === 'done' || done ? Icons.check : ''}</span></div>`;

  function html() {
    if (S.mode === 'creating') return `${hero()}<div class="flist glass" data-flist></div><div class="go busy" role="status"><i class="spin"></i><span data-busy>${t('creating')}</span><button class="cx" data-act="cancel"${S.canceling ? ' disabled' : ''}>${t('cancel')}</button></div>`;
    if (S.mode === 'done') {
      const f = S.delivery ? S.delivery.failed : [];
      return `${hero()}<div class="donebox">${Icons.doneMark}<h1>${t('done.title', { n: '<span data-n></span>', name: '<span data-fname></span>' })}</h1>
      ${f.length ? `<p class="warnline" role="alert"><b>${t('done.failed')}</b> ${f.map(([id, msg]) => esc(destName(id) + ': ' + msg)).join(' · ')}</p>` : ''}
      ${S.openErr ? `<p class="warnline" role="alert">${Icons.x} ${esc(t('open.failed'))}</p>` : ''}
      ${S.vnotes.length ? `<p class="statusline" role="status">${Icons.warn}<span><b>${t('done.verdicts')}</b> ${S.vnotes.map(n => esc(n.name + ': ' + n.text)).join(' · ')}</span></p>` : ''}
      ${S.notes.length ? `<p class="noteline"><b>${t('done.notes')}</b> ${S.notes.map(esc).join(' ')}</p>` : ''}
      <div class="acts"><button class="bb pri" data-act="open">${Icons.folder}<span>${t('open')}</span></button><button class="bb glass" data-act="again">${Icons.plus}<span>${t('again')}</span></button></div>${sum()}</div>`;
    }
    return `${hero()}
      <div class="meta">${cover()}<div class="mf"><div class="two"><input class="fi glass" data-f="artist" placeholder="${esc(t('field.artist'))}" aria-label="${esc(t('field.artist'))}" autocomplete="off"><input class="fi glass" data-f="title" placeholder="${esc(t('field.title'))}" aria-label="${esc(t('field.title'))}" autocomplete="off"></div>
        <button class="lnk" data-act="pop" data-p="more" aria-expanded="false" aria-controls="pop-more">${t('more')}${Icons.down}</button>${more()}</div></div>
      <div class="dests"><div><div class="gh"><span>${t('group.platforms')}</span><span class="sp"></span>${preset()}${sum()}</div>
        <div class="tiles">${['spotify', 'apple', 'amazon', 'tidal', 'deezer', 'ytm'].map(dest).join('')}</div></div>
        <div class="row2"><div class="d1"><div class="gh">${t('group.dist')}</div><div class="tiles" style="--n:1">${dest('dist')}</div></div>
        <div class="d3"><div class="gh">${t('group.extras')}</div><div class="tiles" style="--n:5">${['mp3', 'portada', 'canvas', 'ytv', 'motion'].map(dest).join('')}</div></div></div></div>
      ${S.banner ? `<p class="banner" role="alert">${esc(S.banner)}</p>` : ''}
      <button class="go" data-act="create">${Icons.folderPlus}${t('create')}</button>${folderLine()}`;
  }
  return { html, row, timeTxt, dest, chipTitle, destName };
})();

/* ───────────── Plan: el motor decide qué destinos hay, nombres y tamaños ───────────── */
const Plan = (() => {
  let timer = 0;
  const presetIds = () => Data.ids.map(id => Data.preset[id]);
  async function run() {
    if (!S.loaded || !S.master) return;
    const seq = ++S.planSeq;
    try {
      const plan = await backend.plan(S.master, Store.meta(), presetIds(), Store.mediaInputs());
      if (seq !== S.planSeq) return;
      S.plan = plan;
    } catch (e) { console.warn('plan', e); }
  }
  /* los cambios de artista/título/versión recalculan nombres con una pequeña espera */
  function schedule() { clearTimeout(timer); timer = setTimeout(async () => { await run(); if (S.mode === 'idle') { App.syncChips(); App.live(); } }, 160); }
  return { run, schedule };
})();

/* ───────────── render / estado vivo ───────────── */
const App = (() => {
  function treeEntries() {
    if (S.mode === 'done' && S.delivery) {
      const root = S.delivery.folder.replace(/[\\/]+$/, '');
      return S.delivery.files.map(([p, bytes]) => {
        const rel = p.startsWith(root) ? p.slice(root.length).replace(/^[\\/]+/, '') : baseName(p);
        const parts = rel.split(/[\\/]/);
        return { fo: parts.length > 1 ? parts.slice(0, -1).join('/') : '', name: parts[parts.length - 1], bytes };
      });
    }
    const f = Store.selectedJobs().flatMap(j => [{ fo: j.subfolder, name: j.file_name, bytes: j.estimated_bytes }, ...(j.extra_files || []).map(name => ({ fo: j.subfolder, name, bytes: 0 }))]);
    Store.docs().forEach(d => f.push({ fo: '', name: d, bytes: 0 }));
    return f;
  }
  const folderName = () => (S.mode === 'done' && S.delivery) ? baseName(S.delivery.folder) : (S.plan ? S.plan.delivery_folder : `${S.artist} - ${S.title}`);

  function live() {
    const entries = treeEntries(), bytes = entries.reduce((a, x) => a + x.bytes, 0) + (S.mode === 'done' ? 0 : 10000);
    $$('[data-sum]').forEach(e => e.textContent = t('sum', { n: entries.length, size: fmtMB(bytes, lg(), 0) }));
    $$('[data-n]').forEach(e => e.textContent = entries.length);
    const def = Store.allOn();
    $$('[data-preset]').forEach(e => { e.classList.toggle('cus', !def); $('span', e).textContent = t(def ? 'preset.default' : 'preset.custom'); $('i', e).innerHTML = def ? Icons.check : Icons.reset; });
    $$('[data-fname]').forEach(e => e.textContent = folderName());
    $$('[data-folder]').forEach(e => e.textContent = shortPath(S.outRoot));
    const tr = $('#tree');
    if (tr) {
      let last = null;
      let h = `<div class="tr dir">${Icons.folder}<div class="nm">${esc(folderName())}/</div></div>`;
      /* agrupadas por carpeta (varios destinos comparten carpeta) */
      entries.filter(x => x.fo).sort((a, b) => (a.fo < b.fo ? -1 : a.fo > b.fo ? 1 : 0)).forEach(x => { if (x.fo !== last) { last = x.fo; h += `<div class="tr dir" style="padding-left:16px">${Icons.folder}<div class="nm">${esc(x.fo)}/</div></div>`; } h += `<div class="tr" style="padding-left:32px">${Icons.file}<div class="nm">${esc(x.name)}</div><span class="sz">${fmtMB(x.bytes, lg())}</span></div>`; });
      entries.filter(x => !x.fo).forEach(x => h += `<div class="tr" style="padding-left:18px">${Icons.file}<div class="nm">${esc(x.name)}</div><span class="sz">${x.bytes < 1e4 ? esc(t('size.lt_1kb')) : fmtMB(x.bytes, lg())}</span></div>`);
      tr.innerHTML = h;
    }
    const adv = $('#pop-adv');
    if (adv) adv.innerHTML = `<h4>${t('adv.title')}</h4>${Data.ids.map(id => { const j = Store.job(id), why = Store.disabledReason(id); return `<div class="kv"><span>${esc(t('chip.long.' + id))}</span><b>${j ? esc(why ? t('adv.off') : fmtSpec(j, lg())) : ''}</b></div>`; }).join('')}`;
  }
  /* Acerca de: versión, componente (release del sidecar) y licencias; se pinta una vez, al arrancar */
  function paintAbout() {
    const el = $('#pop-about'); if (!el) return;
    const i = S.info || {};
    el.innerHTML = `<h4>${esc(t('about.heading'))}</h4>
      <div class="kv"><span>${esc(t('about.version'))}</span><b>${esc(i.version || '')}</b></div>
      <div class="kv"><span>${esc(t('about.component'))}</span>${i.sidecar_release ? `<b class="mono">${esc(i.sidecar_release)}</b>` : `<b>${esc(t('about.component.none'))}</b>`}</div>
      <p>${esc(t('about.licenses'))}</p><p>${esc(t('about.others'))}</p>
      <button class="lnk link-b" data-act="legal">${esc(t('about.legal'))}</button>
      ${S.aboutErr ? `<p class="warnline" role="alert">${Icons.x} ${esc(t('about.legal_failed'))}</p>` : ''}
      <p class="abt-mk">${esc(t('about.maker'))}</p>`;
  }
  /* actualiza una ficha sin recrearla (conserva la animación) */
  function setDest(id, bump) {
    $$(`[data-id="${id}"]`).forEach(b => {
      const why = Store.disabledReason(id), on = S.on.has(id) && !why;
      b.classList.toggle('on', on); b.classList.toggle('dis', !!why); b.setAttribute('aria-checked', on);
      const acc = Store.awaiting(id), hint = $('.dh', b);
      b.classList.toggle('acc', acc);
      if (acc && !hint) b.insertAdjacentHTML('beforeend', `<small class="dh">${esc(t('accept.drop'))}</small>`);
      else if (!acc && hint) hint.remove();
      const tt = Views.chipTitle(id);
      if (why) b.setAttribute('aria-disabled', 'true'); else b.removeAttribute('aria-disabled');
      if (tt) b.title = tt; else b.removeAttribute('title');
      if (bump && on) { b.classList.remove('bump'); void b.offsetWidth; b.classList.add('bump'); }
    });
  }
  const syncChips = () => Data.ids.forEach(id => setDest(id, false));
  function render() {
    Create.stop();
    $('#view').innerHTML = Views.html();
    $$('input[data-f]').forEach(i => { i.value = S[i.dataset.f]; if (S.flash && i.value) i.classList.add('filled'); }); S.flash = 0;
    const w = $('#wave');
    if (w) Wave.mount(w, { waveform: S.waveform, duration: S.master ? S.master.duration_secs : 0, progress: S.mode === 'done' ? 1 : S.mode === 'creating' ? 0 : (S.pp > 0 ? S.pp : null), interactive: S.mode === 'idle', fresh: S.fresh, seek: p => { S.pp = p; Wave.setProgress(p); Player.seek(p); paintPlay(); } });
    else Wave.unmount();
    live(); S.fresh = false;
    const p = Q.get('pop');
    if (p && !App.popDone) { App.popDone = 1; requestAnimationFrame(() => { const e = $('#pop-' + p); if (e) { e.classList.add('open'); const b = $(`[data-p="${p}"]`); b && b.setAttribute('aria-expanded', 'true'); } }); }
    if (S.mode === 'creating') Create.refresh();
    if (S.mode === 'done' && !STILL) setTimeout(Wave.ripple, 60);
  }
  function closePops(except) { $$('.pop.open').forEach(p => { if (p !== except) p.classList.remove('open'); }); $$('[aria-controls][aria-expanded=true]').forEach(b => { if (!except || b.getAttribute('aria-controls') !== except.id) b.setAttribute('aria-expanded', 'false'); }); }
  function togglePop(name) { const el = $('#pop-' + name), was = el.classList.contains('open'); closePops(); if (!was) { el.classList.add('open'); const b = $(`[data-p="${name}"]`); b && b.setAttribute('aria-expanded', 'true'); const f = $('input', el); if (f && name === 'more') setTimeout(() => f.focus({ preventScroll: true }), 120); } }
  function paintPlay() {
    $$('[data-time]').forEach(e => e.textContent = Views.timeTxt());
    $$('[data-act=play]').forEach(b => { b.innerHTML = S.pt ? Icons.pause : Icons.play; b.dataset.playing = !!S.pt; b.setAttribute('aria-label', t(S.pt ? 'pause' : 'play')); });
  }
  return { live, setDest, syncChips, render, closePops, togglePop, paintAbout, paintPlay, popDone: 0 };
})();

/* ───────────── Player: audio real en la app (asset protocol), simulado en el mock ───────────── */
const Player = (() => {
  let audio = null, iv = 0;
  const real = () => backend.kind === 'tauri';
  function stop() {
    clearInterval(iv); iv = 0;
    if (audio) { audio.pause(); audio = null; }
    S.pt = 0; S.pp = 0;
  }
  function toggle() {
    if (real()) {
      if (!audio) {
        audio = new Audio(backend.convertFileSrc(S.master.path));
        audio.addEventListener('timeupdate', () => { S.pp = audio.duration ? audio.currentTime / audio.duration : 0; Wave.setProgress(S.pp > 0 ? S.pp : null); App.paintPlay(); });
        audio.addEventListener('ended', () => { S.pt = 0; S.pp = 0; Wave.setProgress(null); App.paintPlay(); });
        audio.addEventListener('error', () => { S.pt = 0; App.paintPlay(); });
      }
      if (S.pt) { audio.pause(); S.pt = 0; } else { S.pt = 1; audio.play().catch(() => { S.pt = 0; App.paintPlay(); }); }
    } else if (S.pt) { clearInterval(iv); iv = 0; S.pt = 0; }
    else { S.pt = 1; iv = setInterval(() => { S.pp += .02; if (S.pp >= 1) { S.pp = 0; clearInterval(iv); iv = 0; S.pt = 0; } Wave.setProgress(S.pp > 0 ? S.pp : null); App.paintPlay(); }, 100); }
    App.paintPlay();
  }
  function seek(p) { if (audio && audio.duration) audio.currentTime = p * audio.duration; }
  return { toggle, stop, seek };
})();

/* ───────────── Create: progreso real (ProgressDto); la barra es la propia onda ───────────── */
const Create = (() => {
  let hold = 0;
  const stop = () => { clearTimeout(hold); hold = 0; };
  const split = n => { const i = n.lastIndexOf('.'); return i > 0 ? [n.slice(0, i), n.slice(i)] : [n, '']; };
  const pct = () => { const js = S.rows.filter(r => r.kind === 'job'); if (!js.length) return 0; return js.reduce((a, r) => a + (r.state === 'done' || r.state === 'fail' ? 1 : r.state === 'run' ? r.frac : 0), 0) / js.length; };

  function refresh() {
    const list = $('[data-flist]'); if (!list) return;
    const shown = S.rows.filter(r => r.state !== 'wait');
    while (list.children.length < shown.length) list.insertAdjacentHTML('beforeend', Views.row(shown[list.children.length], STILL));
    shown.forEach((r, i) => {
      const c = list.children[i].querySelector('.r');
      c.classList.toggle('ok', r.state === 'done');
      c.style.color = r.state === 'fail' ? 'var(--err)' : '';
      if (r.state === 'done') c.innerHTML = Icons.check;
      else if (r.state === 'fail') { c.innerHTML = Icons.x; c.setAttribute('role', 'img'); c.setAttribute('aria-label', t('row.failed')); }
      else c.textContent = t('pct', { n: Math.round((r.frac || 0) * 100) });
    });
    list.scrollTop = list.scrollHeight;
    const pr = pct(); Wave.setProgress(pr); $$('[data-pc]').forEach(e => e.textContent = t('pct', { n: Math.round(pr * 100) }));
    /* mientras el motor prepara las muestras (antes del primer destino) la barra lo dice con su fracción */
    const busy = $('[data-busy]');
    if (busy) busy.textContent = S.prep != null && S.rows.every(r => r.state === 'wait') ? t('preparing') + ' ' + t('pct', { n: Math.round(S.prep * 100) }) : t('creating');
  }
  const docRows = () => Store.docs().map(d => { const [name, ext] = split(d); return { kind: 'doc', name, ext, state: 'done' }; });
  const jobRows = jobs => jobs.map(j => { const ui = Data.byPreset[j.preset_id]; return { kind: 'job', preset_id: j.preset_id, name: t('chip.long.' + ui), ext: Data.ext[j.container] || '', title: j.file_name, state: 'wait', frac: 0 }; });

  function onEvent(ev) {
    const row = S.rows.find(r => r.preset_id === ev.preset_id);
    if (ev.kind === 'preparing') S.prep = ev.fraction || 0;
    else if (ev.kind === 'job_started' && row) { row.state = 'run'; row.frac = 0; }
    else if (ev.kind === 'job_progress' && row) { row.state = 'run'; row.frac = ev.fraction || 0; }
    else if (ev.kind === 'job_done' && row) row.state = 'done';
    else if (ev.kind === 'job_failed' && row) { row.state = 'fail'; row.error = ev.error; }
    else if (ev.kind === 'cancelled') S.cancelled = true;
    refresh();
  }

  async function start() {
    const jobs = Store.selectedJobs();
    Player.stop();
    S.rows = jobRows(jobs); S.mode = 'creating'; S.delivery = null; S.cancelled = false; S.canceling = false; S.banner = ''; S.prep = null; S.vnotes = [];
    S.notes = Store.notesOf(jobs);
    App.render();
    if (STILL) return still();
    const req = { master_path: S.master.path, out_root: S.outRoot, meta: Store.meta(), active_ids: jobs.map(j => j.preset_id), media_inputs: Store.mediaInputs() };
    try {
      /* cancelar antes de que el motor registre la exportación no llega a nada: se reenvía con el primer evento */
      const forward = cancelOnFirstEvent(() => S.canceling, () => backend.cancelExport());
      const d = await backend.export(req, ev => { forward(ev); onEvent(ev); });
      if (isCancelled(S.cancelled, d)) { S.mode = 'idle'; S.rows = []; App.render(); return; }
      S.rows.filter(r => r.kind === 'job' && r.state !== 'fail').forEach(r => r.state = 'done');
      S.rows.push(...docRows());
      S.delivery = d; S.vnotes = finalNotes(d.verdicts, Data.byPreset, t, Store.plannedVerdicts()); refresh();
      hold = setTimeout(() => { S.mode = 'done'; App.render(); }, 380);
    } catch (e) { S.mode = 'idle'; S.rows = []; S.banner = errText(e); App.render(); }
  }
  /* fotograma fijo para capturas: 4 destinos hechos y uno al 45 % */
  function still() {
    const k = Math.max(1, Math.ceil(S.rows.length * .6));
    S.rows.forEach((r, i) => { if (i < k - 1) r.state = 'done'; else if (i === k - 1) { r.state = 'run'; r.frac = .45; } });
    refresh();
  }
  async function cancel() { S.canceling = true; const b = $('[data-act=cancel]'); if (b) b.disabled = true; try { await backend.cancelExport(); } catch (e) { console.warn(e); } }
  return { start, stop, refresh, cancel };
})();

/* ───────────── Events ───────────── */
const Events = (() => {
  async function loadSong(path) {
    if (S.busy) return;
    S.busy = true;
    try {
      const { master, waveform } = await backend.analyze(path, 600);
      Player.stop();
      S.master = master; S.waveform = waveform.bins; S.plan = null;
      S.err = ''; S.banner = ''; S.loaded = true; S.fresh = true; S.flash = 1; S.over = false;
      S.artist = master.suggested_artist || S.artist; S.title = master.suggested_title || '';
      S.on = Data.defaults();
      await Plan.run();
    } catch (e) {
      const msg = errText(e);
      S.over = false;
      if (S.loaded) S.banner = msg; else S.err = msg;
      S.busy = false; App.render();
      setTimeout(() => { if (S.err === msg || S.banner === msg) { S.err = ''; S.banner = ''; if (S.mode === 'idle') App.render(); } }, 3200);
      return;
    }
    S.busy = false; App.render();
  }
  /* mock/navegador: File (sin ruta real) · Tauri: ruta */
  /* aviso breve en la zona de soltar (o bajo los botones si ya hay canción) */
  function hint(msg) {
    if (S.loaded) S.banner = msg; else S.err = msg;
    App.render();
    setTimeout(() => { if (S.err === msg || S.banner === msg) { S.err = ''; S.banner = ''; if (S.mode === 'idle') App.render(); } }, 3200);
  }
  async function loadCover(src) {
    if (!src) return;
    if (!S.loaded) return hint(t('drop.need_song'));
    if (typeof src === 'string') {
      if (!/\.(png|jpe?g)$/i.test(src)) return hint(t('cover.bad_type'));
      await backend.allowAsset(src); S.coverPath = src; S.cover = backend.convertFileSrc(src);
    } else { if (!/^image\/(png|jpeg)$/.test(src.type)) return hint(t('cover.bad_type')); S.coverPath = src.name; S.cover = URL.createObjectURL(src); }
    await Plan.run(); App.render();
  }
  /* material propio de una ficha (imagen o vídeo): ruta (app) o File (navegador). La ficha de portada es la portada de la entrega. */
  const MEDIA_RE = /\.(jpe?g|png|webp|mp4|mov|m4v|mkv|webm)$/i;
  async function loadMedia(id, src) {
    if (!src || !Data.media.has(id)) return;
    if (!S.loaded) return hint(t('drop.need_song'));
    if (id === 'portada') return loadCover(src);
    const file = typeof src !== 'string';
    const name = file ? src.name : baseName(src);
    if (!(MEDIA_RE.test(name) || (file && /^(image|video)\//.test(src.type)))) return hint(t('media.bad_type'));
    let thumb;
    try {
      if (file) thumb = src.type.startsWith('image/') ? URL.createObjectURL(src) : await backend.mediaThumbnail(name);
      else { await backend.allowAsset(src); thumb = await backend.mediaThumbnail(src); }
    } catch (e) { return hint(errText(e)); }
    S.media[id] = { path: file ? name : src, name, thumb };
    await Plan.run(); App.render();
  }
  async function clearMedia(id) {
    if (id === 'portada') { S.cover = null; S.coverPath = null; } else delete S.media[id];
    await Plan.run(); App.render();
  }
  let pickId = null;
  async function pickMedia(id) {
    if (backend.kind === 'tauri') { const p = await backend.pickFile(id === 'portada' ? 'image' : 'media'); if (p) await loadMedia(id, p); }
    else { pickId = id; const f = $('#fmedia'); f.accept = id === 'portada' ? 'image/png,image/jpeg' : 'image/*,video/*'; f.click(); }
  }
  const missing = () => !S.loaded ? 'song' : !S.artist.trim() ? 'artist' : !S.title.trim() ? 'title' : !Store.selected().length ? 'dest' : null;
  function nudge(k) {
    let el = k === 'song' ? $('#hero') : k === 'dest' ? $('.dests') : $(`input[data-f=${k}]`); if (!el) return;
    if (k === 'artist' || k === 'title') { el.focus(); el.classList.add('in-err'); setTimeout(() => el.classList.remove('in-err'), 900); }
    el.classList.remove('nudge'); void el.offsetWidth; el.classList.add('nudge'); setTimeout(() => el.classList.remove('nudge'), 700);
  }
  function winCtl(a) { const w = window.__TAURI__ && window.__TAURI__.window && window.__TAURI__.window.getCurrentWindow && window.__TAURI__.window.getCurrentWindow(); if (!w) return; if (a === 'min') w.minimize(); if (a === 'max') w.toggleMaximize(); if (a === 'close') w.close(); }

  async function pickSong() {
    if (backend.kind === 'tauri') { const p = await backend.pickFile('audio'); if (p) loadSong(p); }
    else $('#fsong').click();
  }
  async function pickCover() {
    if (backend.kind === 'tauri') { const p = await backend.pickFile('image'); if (p) await loadCover(p); }
    else $('#fcov').click();
  }
  async function create() {
    const m = missing(); if (m) return nudge(m);
    if (!S.outRoot) { const p = await backend.pickFolder(); if (!p) { S.banner = t('err.no_folder'); return App.render(); } S.outRoot = p; }
    Create.start();
  }
  async function open() {
    const a = $('[data-act=open]'); if (!a || !S.delivery) return;
    const s = $('span', a); s.textContent = t('opening'); setTimeout(() => s.textContent = t('open'), 1200);
    try { S.openErr = false; await backend.openPath(S.delivery.folder); } catch (e) { console.warn(e); S.openErr = true; App.render(); }
  }
  function again() {
    Player.stop();
    Object.assign(S, { mode: 'idle', loaded: false, cover: null, coverPath: null, artist: '', title: '', m: {}, master: null, waveform: [], plan: null, delivery: null, rows: [], banner: '', err: '', media: {}, notes: [], vnotes: [], prep: null, openErr: false });
    S.on = Data.defaults();
    App.render();
  }
  function tauriDrop(p) {
    const at = p.position ? document.elementFromPoint(p.position.x / devicePixelRatio, p.position.y / devicePixelRatio) : null;
    const clear = () => $$('.over').forEach(z => z.classList.remove('over'));
    if (p.type === 'leave') return clear();
    if (S.mode !== 'idle') return;
    if (p.type === 'enter' || p.type === 'over') { clear(); const z = at && at.closest('.dz,.cov,[data-drop]'); if (z) z.classList.add('over'); return; }
    if (p.type === 'drop') {
      clear();
      const f = p.paths && p.paths[0]; if (!f) return;
      const chip = at && at.closest('[data-drop]');
      const run = chip ? loadMedia(chip.dataset.drop, f) : /\.(png|jpe?g)$/i.test(f) ? loadCover(f) : (at && at.closest('.cov')) ? null : loadSong(f);
      Promise.resolve(run).catch(e => console.warn('drop', e));
    }
  }
  function init() {
    document.addEventListener('click', e => {
      const wb = e.target.closest('[data-win]'); if (wb) return winCtl(wb.dataset.win);
      const a = e.target.closest('[data-act]');
      if (!e.target.closest('.pop') && !(a && a.dataset.act === 'pop')) App.closePops();
      if (!a) return; const act = a.dataset.act;
      if (act === 'pop') return App.togglePop(a.dataset.p);
      if (act === 'toggle') { if (S.mode !== 'idle' || Store.disabledReason(a.dataset.id)) return; S.on.has(a.dataset.id) ? S.on.delete(a.dataset.id) : S.on.add(a.dataset.id); App.setDest(a.dataset.id, 1); App.live(); }
      else if (act === 'preset') { Data.ids.forEach(id => { if (Data.defaultOn(id)) S.on.add(id); else S.on.delete(id); App.setDest(id, Data.defaultOn(id) && !Store.disabledReason(id)); }); App.live(); }
      else if (act === 'legal') { S.aboutErr = false; backend.openLegal().catch(err => { console.warn(err); S.aboutErr = true; App.paintAbout(); }).then(() => { if (!S.aboutErr) App.paintAbout(); }); }
      else if (act === 'song') pickSong();
      else if (act === 'cover') pickCover();
      else if (act === 'media-pick') pickMedia(a.dataset.mid);
      else if (act === 'media-clear') clearMedia(a.dataset.mid);
      else if (act === 'play') Player.toggle();
      else if (act === 'folder') backend.pickFolder().then(p => { if (p) { S.outRoot = p; App.live(); } });
      else if (act === 'create') create();
      else if (act === 'cancel') Create.cancel();
      else if (act === 'again') again();
      else if (act === 'open') open();
    });
    document.addEventListener('keydown', e => { if (e.key === 'Escape') { const o = $('.pop.open'); const tr = o && $(`[aria-controls="${o.id}"]`); App.closePops(); if (tr) tr.focus(); } if ((e.key === 'Enter' || e.key === ' ') && e.target.matches('.cov,.mx,.mp')) { e.preventDefault(); e.target.click(); } });
    document.addEventListener('input', e => {
      const i = e.target.closest('input[data-f]'); if (i) { S[i.dataset.f] = i.value; App.live(); Plan.schedule(); }
      const m = e.target.closest('input[data-m]'); if (m) { S.m[m.dataset.m] = m.value; if (m.dataset.m === 'ver') Plan.schedule(); }
    });
    document.addEventListener('change', e => { if (e.target.id === 'fsong') { const f = e.target.files[0]; if (f) loadSong(f.name); e.target.value = ''; } if (e.target.id === 'fcov') { loadCover(e.target.files[0]).catch(err => console.warn(err)); e.target.value = ''; } if (e.target.id === 'fmedia') { const f = e.target.files[0], id = pickId; e.target.value = ''; if (f && id) loadMedia(id, f).catch(err => console.warn(err)); } });
    ['dragenter', 'dragover'].forEach(n => document.addEventListener(n, e => { e.preventDefault(); const z = e.target.closest('.dz,.cov,[data-drop]'); if (z) z.classList.add('over'); }));
    ['dragleave', 'drop'].forEach(n => document.addEventListener(n, e => { e.preventDefault(); $$('.over').forEach(z => z.classList.remove('over')); }));
    document.addEventListener('drop', e => { if (S.mode !== 'idle') return; const f = e.dataTransfer.files[0]; if (!f) return; const chip = e.target.closest('[data-drop]'); const run = chip ? loadMedia(chip.dataset.drop, f) : e.target.closest('.cov') ? loadCover(f) : e.target.closest('.dz') ? (f.type.startsWith('image/') ? loadCover(f) : loadSong(f.name)) : null; Promise.resolve(run).catch(err => console.warn('drop', err)); });
    if (backend.kind === 'tauri') backend.onFileDrop(tauriDrop);
  }
  return { init, loadSong, loadMedia };
})();

/* ───────────── Boot ───────────── */
function applyStaticText() {
  document.title = t('app.title');
  $$('[data-i18n]').forEach(e => e.textContent = t(e.dataset.i18n));
  $$('[data-i18n-aria]').forEach(e => e.setAttribute('aria-label', t(e.dataset.i18nAria)));
  $$('[data-i18n-title]').forEach(e => e.title = t(e.dataset.i18nTitle));
}

/* mock/capturas: ?cover=1|rect pone una portada de ejemplo y ?own=canvas,ytv,motion material propio en esas fichas */
async function mockMedia() {
  const cover = Q.get('cover'), own = (Q.get('own') || '').split(',').filter(id => Data.media.has(id) && id !== 'portada');
  const names = { canvas: 'canvas-bucle.mp4', ytv: 'video-letra.mp4', motion: 'animacion-portada.mov' };
  if (cover) {
    S.coverPath = cover === 'rect' ? '/mock/portada-rect.jpg' : '/mock/portada.jpg';
    S.cover = await backend.mediaThumbnail(S.coverPath);
  }
  for (const id of own) S.media[id] = { path: '/mock/' + names[id], name: names[id], thumb: await backend.mediaThumbnail(names[id]) };
  if (cover || own.length) { await Plan.run(); App.render(); }
}

async function boot() {
  const d = document.documentElement;
  const forced = Q.get('lang');
  const lang = forced === 'es' || forced === 'en' ? forced : await backend.systemLang().catch(() => 'en');
  await loadLang(lang);
  d.lang = getLang();
  await backend.setLang(getLang()).catch(() => {});
  applyStaticText();
  S.info = await backend.appInfo().catch(() => S.info);
  Data.setDefaults(S.info.default_on);
  S.on = Data.defaults();
  S.outRoot = S.info.default_out_root || '';
  /* tema */
  const tb = $('#themeBtn'), paintTheme = () => tb.innerHTML = d.dataset.theme === 'dark' ? Icons.sun.replace('class="ic"', 'class="ic turn"') : Icons.moon.replace('class="ic"', 'class="ic turn"');
  paintTheme();
  tb.addEventListener('click', () => { d.dataset.theme = d.dataset.theme === 'dark' ? 'light' : 'dark'; paintTheme(); const u = new URL(location); u.searchParams.set('theme', d.dataset.theme); history.replaceState(0, '', u); });
  $('#pop-adv').classList.add('glass-pop'); $('#pop-about').classList.add('glass-pop'); App.paintAbout();
  Fx.init(); Events.init();
  /* estado inicial: la app real arranca vacía; el mock del prototipo, con la canción de ejemplo */
  const state = Q.get('state') || (backend.kind === 'mock' ? 'normal' : 'empty');
  if (state === 'empty') App.render();
  else {
    await Events.loadSong(Data.SAMPLE);
    if (backend.kind === 'mock') await mockMedia();
    if (state === 'creating') await Create.start();
    else if (state === 'done') {
      const jobs = Store.selectedJobs();
      S.delivery = await backend.export({ master_path: S.master.path, out_root: S.outRoot, meta: Store.meta(), active_ids: jobs.map(j => j.preset_id), media_inputs: Store.mediaInputs() }, () => {});
      S.notes = Store.notesOf(jobs); S.vnotes = finalNotes(S.delivery.verdicts, Data.byPreset, t, Store.plannedVerdicts());
      S.mode = 'done'; App.render();
    }
  }
}

boot().then(() => { document.documentElement.dataset.ready = '1'; return backend.ready(); })
  .catch(e => { console.error(e); document.documentElement.dataset.ready = 'error'; try { backend.ready(); } catch (_) { /* sin ventana que mostrar */ } });
