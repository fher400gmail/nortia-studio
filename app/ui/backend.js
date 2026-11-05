/* nortia. studio · master check · adaptador de backend.
   mock  → navegador y prototipo (datos de ejemplo con la misma forma que los DTO del motor)
   tauri → app real (invoke/listen de Tauri 2 por window.__TAURI__, con withGlobalTauri)
   Los DTO son los del puente `mastercheck-bridge` (JSON en snake_case). */

export const PROGRESS_EVENT = 'mastercheck://progress';

/* ───────────── mock ───────────── */

/** Destinos del motor (presets.json) con lo justo para simular el plan. */
const PRESETS = [
  { id: 'spotify', sub: 'spotify', tag: 'Spotify', container: 'flac', bits: 'native', rate: 'native' },
  { id: 'apple_adm', sub: 'apple', tag: 'Apple Digital Masters', container: 'wav', bits: 'native', rate: 'native', minBits: 24, reason: 'apple_needs_24bit' },
  { id: 'amazon', sub: 'amazon', tag: 'Amazon Music', container: 'flac', bits: 'native', rate: 'native' },
  { id: 'tidal', sub: 'tidal', tag: 'Tidal', container: 'flac', bits: 'native', rate: 'native' },
  { id: 'deezer', sub: 'deezer', tag: 'Deezer', container: 'wav', bits: 16, rate: 44100 },
  { id: 'youtube_music', sub: 'youtube', tag: 'YouTube Music', container: 'wav', bits: 'native', rate: 48000, minBits: 16, reason: 'youtube_music_needs_16bit' },
  { id: 'youtube_video', sub: 'youtube', tag: 'YouTube', container: 'mp4', bits: 'native', rate: 48000, media: true, suffix: '1080p' },
  { id: 'distributor_std', sub: 'distributor', tag: 'Distribuidora', container: 'wav', bits: 16, rate: 44100 },
  { id: 'mp3_promo', sub: 'promo', tag: 'MP3 320', container: 'mp3', bits: 0, rate: 44100 },
  { id: 'cover', sub: 'artwork', tag: 'Portada', container: 'jpg', bits: 0, rate: 0, media: true, noSidecarOk: true, suffix: '3000x3000', extra: 'png' },
  { id: 'canvas', sub: 'artwork', tag: 'Canvas', container: 'mp4', bits: 0, rate: 0, media: true, suffix: '1080x1920' },
  { id: 'apple_motion', sub: 'apple', tag: 'Apple Motion', container: 'mp4', bits: 0, rate: 0, media: true, suffix: '3840x3840', extraSuffix: '2048x2732' }
];

/** PNG de 96 px (paisaje en gris pizarra, sin naranja) que el mock devuelve como miniatura. */
export const MOCK_THUMB = 'data:image/png;base64,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';

const FOLDERS = {
  es: { spotify: '01 Spotify', apple: '02 Apple Music', amazon: '03 Amazon Music', tidal: '04 Tidal', deezer: '05 Deezer', youtube: '06 YouTube', distributor: '07 Distribuidora', promo: '08 Promo', artwork: '09 Imagen' },
  en: { spotify: '01 Spotify', apple: '02 Apple Music', amazon: '03 Amazon Music', tidal: '04 Tidal', deezer: '05 Deezer', youtube: '06 YouTube', distributor: '07 Distributor', promo: '08 Promo', artwork: '09 Artwork' }
};

const TEXT = {
  es: {
    'err.not_wav': 'Este archivo no es un WAV válido. master check admite másters en formato WAV.',
    'err.rate_over_96k': 'master check admite másters de hasta 96 kHz',
    'err.cancelled': 'Exportación cancelada.',
    'verdict.ok': 'Correcto',
    'tp_over_zero': 'El pico real supera 0 dBTP',
    'reason.no_cover': 'Falta la portada: arrastra una imagen a la ficha de portada.',
    'reason.no_sidecar': 'Falta el componente de vídeo y MP3 (ffmpeg) de master check. Reinstala la aplicación.',
    'warn.cover_cropped': 'La portada no era cuadrada: se ha recortado desde el centro.',
    'warn.cover_upscaled': 'La portada es más pequeña que 3000×3000 y se ha ampliado. Para máxima calidad usa una de 3000×3000 o más.',
    'tag.cover': 'Portada',
    adm_clipping: 'pasa de 0 dB al codificar',
    apple_needs_24bit: 'Apple Digital Masters exige un máster de 24 bit',
    youtube_music_needs_16bit: 'Se necesita un máster de al menos 16 bit',
    'tag.distributor': 'Distribuidora'
  },
  en: {
    'err.not_wav': 'This file is not a valid WAV. master check supports WAV masters.',
    'err.rate_over_96k': 'master check supports masters up to 96 kHz',
    'err.cancelled': 'Export cancelled.',
    'verdict.ok': 'OK',
    'tp_over_zero': 'True peak is above 0 dBTP',
    'reason.no_cover': 'The cover is missing: drop an image on the cover chip.',
    'reason.no_sidecar': 'The video and MP3 component (ffmpeg) is missing from master check. Reinstall the app.',
    'warn.cover_cropped': 'The cover was not square: it was cropped from the center.',
    'warn.cover_upscaled': 'The cover is smaller than 3000×3000 and was enlarged. For best quality use one of 3000×3000 or more.',
    'tag.cover': 'Cover',
    adm_clipping: 'clips when encoded',
    apple_needs_24bit: 'Apple Digital Masters requires a 24-bit master',
    youtube_music_needs_16bit: 'A master of at least 16 bit is required',
    'tag.distributor': 'Distributor'
  }
};

const MOCK_FOLDERS = {
  es: ['~/Escritorio/Entregas', '~/Documentos/Lanzamientos', '~/Descargas'],
  en: ['~/Desktop/Deliveries', '~/Documents/Releases', '~/Downloads']
};

const DOCS = {
  es: ['README.txt', 'informe-loudness.txt', 'ficha-lanzamiento.csv', 'ficha-lanzamiento.txt'],
  en: ['README.txt', 'loudness-report.txt', 'release-sheet.csv', 'release-sheet.txt']
};

/** Envolvente de la onda de ejemplo (la del prototipo). */
const ENV = [[0, .1], [.05, .18], [.08, .5], [.28, .56], [.30, .88], [.5, .86], [.52, .5], [.63, .55], [.65, .94], [.85, .92], [.87, .5], [1, .08]];
const env = t => { for (let i = 0; i < ENV.length - 1; i++) { const [a, x] = ENV[i], [b, y] = ENV[i + 1]; if (t >= a && t <= b) return x + (y - x) * (t - a) / (b - a); } return .1; };
const hash = n => { const x = Math.sin(n * 127.1 + 311.7) * 43758.5453; return x - Math.floor(x); };

export function mockWaveform(bins) {
  const out = [];
  for (let i = 0; i < bins; i++) {
    const t = (i + .5) / bins;
    out.push(Math.min(1, env(t) * (.62 + .38 * hash(Math.round(t * 180)))));
  }
  return out;
}

const baseName = p => String(p).split(/[\\/]/).pop();

/** "Artista - Título (master v3).wav" -> { artist, title } (como el puente). */
export function suggestFromName(fileName) {
  let stem = baseName(fileName).replace(/\.[^.]+$/, '').replace(/_+/g, ' ').trim();
  for (;;) {
    const m = stem.match(/\s*\(([^)]*)\)\s*$/);
    if (m && /master|mix|v\d/i.test(m[1])) stem = stem.slice(0, m.index).trim(); else break;
  }
  const m = stem.match(/^(.+?)\s+[-–—]\s+(.+)$/);
  return m ? { artist: m[1].trim(), title: m[2].trim() } : { artist: null, title: stem || null };
}

function bridgeError(lang, key, detail = null) {
  return { key, message: (TEXT[lang] || TEXT.en)[key] || key, detail };
}

/**
 * Backend simulado. Opciones: `fast` (progreso casi instantáneo), `lang` inicial,
 * `sidecar` (false simula que no hay ffmpeg: sin MP3 ni vídeo), `defaultOff` (ids de preset con
 * `default_on: false`, como presets.json), `os`, `adm` ('clip' simula que el AAC de Apple Digital
 * Masters pasa de 0 dB al codificar: el veredicto final de la ficha de Apple es un aviso).
 */
export function createMock({ fast = false, lang = 'es', sidecar = true, defaultOff = ['apple_motion'], os = 'mac', version = '0.1.0-mock', dropCancelEvent = false, adm = null } = {}) {
  let curLang = lang;
  let cancelled = false;
  let folderIx = 0;
  const tx = k => (TEXT[curLang] || TEXT.en)[k] || k;

  const be = {
    kind: 'mock',
    async setLang(l) { curLang = l === 'es' ? 'es' : 'en'; },
    async systemLang() { return curLang; },
    async appInfo() { return { version, os, sidecar_available: sidecar, sidecar_release: sidecar ? 'ffmpeg-sidecar-v7.1.5-3a21763' : '', default_on: PRESETS.map(p => p.id).filter(id => !defaultOff.includes(id)), default_out_root: MOCK_FOLDERS[curLang][0] }; },

    async analyze(path, bins = 600) {
      const file = baseName(path);
      if (!/\.wav$/i.test(file)) throw bridgeError(curLang, 'err.not_wav');
      if (/192k/i.test(file)) throw bridgeError(curLang, 'err.rate_over_96k');
      const low = /16bit/i.test(file);
      const s = suggestFromName(file);
      const rate = low ? 44100 : 96000, dur = 232;
      const master = {
        path: String(path), file_name: file, sample_rate: rate, bits: low ? 16 : 24, is_float: false, channels: 2,
        frames: rate * dur, duration_secs: dur, integrated_lufs: -9.1, true_peak_dbtp: 1.2, lra_lu: 5.8, overs_count: 3,
        suggested_artist: s.artist, suggested_title: s.title
      };
      return { master, waveform: { bins: mockWaveform(bins) } };
    },

    /** `mediaInputs`: { preset_id: ruta } de la imagen o el vídeo propio de cada ficha. */
    async plan(master, meta, activeIds, mediaInputs = {}) {
      const lg = curLang;
      const artist = (meta.artist || '').trim(), title = (meta.title || '').trim() || (suggestFromName(master.path).title || '');
      const base = `${artist} - ${title}`;
      const ver = (meta.version || '').trim();
      const folder = ver ? `${base} (${ver})` : base;
      const nativeBits = Math.min(master.is_float ? 32 : master.bits, 24);
      const cover = meta.cover_path || null;
      const jobs = PRESETS.filter(p => activeIds.includes(p.id)).map(p => {
        const subfolder = FOLDERS[lg][p.sub];
        const container = p.container;
        const off = (why) => ({
          preset_id: p.id, subfolder, file_name: '', container, bits: 0, sample_rate: 0, verdict: 'ok', verdict_text: '',
          estimated_bytes: 0, enabled: false, disabled_reason: tx(why), warnings: [], extra_files: []
        });
        if (p.minBits && p.bits === 'native' && nativeBits < p.minBits) return off(p.reason);
        if ((p.container === 'mp3' || (p.media && !p.noSidecarOk)) && !sidecar) return off('reason.no_sidecar');
        if (p.media && !(cover || mediaInputs[p.id])) return off('reason.no_cover');
        const bits = p.bits === 'native' ? nativeBits : p.bits;
        const rate = p.rate === 'native' ? master.sample_rate : p.rate;
        const khz = rate / 1000;
        const tag = p.id === 'distributor_std' ? tx('tag.distributor') : p.id === 'cover' ? tx('tag.cover') : p.tag;
        const khzLabel = Number.isInteger(khz) ? `${khz}kHz` : `${khz.toFixed(1)}kHz`;
        let file_name, extra_files = [], estimated_bytes, warnings = [];
        if (p.media) {
          file_name = `${base} [${tag}] ${p.suffix}.${container}`;
          if (p.extra) extra_files = [`${base} [${tag}] ${p.suffix}.${p.extra}`];
          if (p.extraSuffix) extra_files = [`${base} [${tag}] ${p.extraSuffix}.${container}`];
          estimated_bytes = { cover: 12e6, canvas: 4e6, apple_motion: 150e6 }[p.id] ?? Math.round(master.duration_secs * 1.05e6);
          /* una portada con "rect" en el nombre simula una no cuadrada y pequeña */
          if (p.id === 'cover' && /rect/i.test(cover || '')) warnings = [tx('warn.cover_cropped'), tx('warn.cover_upscaled')];
        } else {
          file_name = container === 'mp3' ? `${base} [${tag}].mp3` : `${base} [${tag}] ${bits}bit-${khzLabel}.${container}`;
          const wav = master.frames * master.channels * bits / 8;
          estimated_bytes = container === 'flac' ? Math.round(wav * .62) : container === 'wav' ? wav : container === 'mp3' ? Math.round(master.frames * 40000 / rate) : 0;
        }
        const bad = master.true_peak_dbtp > 0 && container !== 'wav' && !(p.media && p.id !== 'youtube_video');
        return {
          preset_id: p.id, subfolder, file_name, container, bits: p.media && p.id !== 'youtube_video' ? 0 : bits, sample_rate: p.media && p.id !== 'youtube_video' ? 0 : rate,
          verdict: bad ? 'warn' : 'ok', verdict_text: bad ? tx('tp_over_zero') : tx('verdict.ok'),
          estimated_bytes, enabled: true, disabled_reason: null, warnings, extra_files
        };
      });
      const on = jobs.filter(j => j.enabled);
      return { delivery_folder: folder, jobs, total_files: on.reduce((a, j) => a + 1 + j.extra_files.length, 0) + 4, total_bytes: on.reduce((a, j) => a + j.estimated_bytes, 0) };
    },

    /** Emite la misma secuencia de eventos que el puente y devuelve el DeliveryDto. */
    async export(req, onProgress = () => {}) {
      cancelled = false;
      const { master } = await be.analyze(req.master_path, 4);
      const plan = await be.plan(master, req.meta, req.active_ids, req.media_inputs || {});
      const jobs = plan.jobs.filter(j => j.enabled);
      const step = fast ? 2 : 1;
      const wait = ms => new Promise(r => setTimeout(r, fast ? 6 : ms));
      const root = `${req.out_root}/${plan.delivery_folder}`;
      const files = [];
      onProgress({ kind: 'analyzed' });
      onProgress({ kind: 'planned' });
      /* el motor prepara una vez las muestras de cada combinación (bits, frecuencia) antes del primer destino */
      const combos = [...new Set(jobs.filter(j => j.bits).map(j => `${j.bits}/${j.sample_rate}`))];
      for (let i = 0; i < combos.length; i++) { await wait(120); onProgress({ kind: 'preparing', fraction: (i + 1) / combos.length }); }
      for (const j of jobs) {
        if (cancelled) {
          if (!dropCancelEvent) onProgress({ kind: 'cancelled' });
          return { folder: root, cancelled: true, files, failed: [], failed_keys: [], failed_details: [], verdicts: [], elapsed_ms: 0 };
        }
        onProgress({ kind: 'job_started', preset_id: j.preset_id });
        const ms = 280 + Math.min(j.estimated_bytes / 1e6, 140) / 140 * 500;
        for (let f = .25; f < 1; f += .25 * step) { await wait(ms / 4); onProgress({ kind: 'job_progress', preset_id: j.preset_id, fraction: Math.min(.99, f) }); }
        await wait(ms / 4);
        const path = `${root}/${j.subfolder}/${j.file_name}`;
        files.push([path, j.estimated_bytes]);
        j.extra_files.forEach(x => files.push([`${root}/${j.subfolder}/${x}`, Math.round(j.estimated_bytes / 3)]));
        onProgress({ kind: 'job_done', preset_id: j.preset_id, path, bytes: j.estimated_bytes });
      }
      DOCS[curLang].forEach((d, i) => files.push([`${root}/${d}`, [3000, 4000, 1000, 2000][i]]));
      /* veredicto final por destino: el de ADM puede empeorar el de Apple tras codificar el AAC */
      const verdicts = jobs.map(j => (adm === 'clip' && j.preset_id === 'apple_adm' && j.verdict === 'ok')
        ? { preset_id: j.preset_id, verdict: 'warn', verdict_text: tx('adm_clipping') }
        : { preset_id: j.preset_id, verdict: j.verdict, verdict_text: j.verdict_text });
      const delivery = { folder: root, cancelled: false, files, failed: [], failed_keys: [], failed_details: [], verdicts, elapsed_ms: 1200 };
      onProgress({ kind: 'finished', delivery });
      return delivery;
    },

    async cancelExport() { cancelled = true; },
    async pickFolder() { folderIx = (folderIx + 1) % MOCK_FOLDERS[curLang].length; return MOCK_FOLDERS[curLang][folderIx]; },
    async pickFile() { return null; },
    async openPath() {},
    /** Abre el documento de licencias y fuentes (terceros) incluido en la app. */
    legalOpened: 0,
    async openLegal() { if (be.legalFails) throw new Error('err.internal'); be.legalOpened++; },
    async allowAsset() {},
    /** Miniatura de una imagen o un vídeo: el mock devuelve siempre el mismo PNG pequeño. */
    async mediaThumbnail() { return MOCK_THUMB; },
    async ready() {},
    convertFileSrc(p) { return p; },
    async onFileDrop() { return () => {}; }
  };
  return be;
}

/* ───────────── tauri ───────────── */

export function createTauri(g = globalThis) {
  const T = () => g.__TAURI__;
  const invoke = (cmd, args) => T().core.invoke(cmd, args);
  return {
    kind: 'tauri',
    setLang: l => invoke('set_lang', { tag: l }),
    systemLang: () => invoke('system_lang'),
    appInfo: () => invoke('app_info'),
    analyze: (path, bins) => invoke('analyze', { path, bins }),
    plan: (master, meta, activeIds, mediaInputs = {}) => invoke('plan', { master, meta, activeIds, mediaInputs }),
    async export(req, onProgress = () => {}) {
      const un = await T().event.listen(PROGRESS_EVENT, e => onProgress(e.payload));
      try { return await invoke('export', { req }); } finally { un(); }
    },
    cancelExport: () => invoke('cancel_export'),
    pickFolder: () => invoke('pick_folder'),
    pickFile: kind => invoke('pick_file', { kind }),
    openPath: path => invoke('open_path', { path }),
    openLegal: () => invoke('open_legal'),
    /** Autoriza una imagen arrastrada para verla (asset protocol, ámbito vacío por defecto). */
    allowAsset: path => invoke('allow_asset', { path }),
    /** Miniatura (data URL de 160 px) de una imagen o del primer fotograma de un vídeo. */
    mediaThumbnail: path => invoke('media_thumbnail', { path }),
    ready: () => invoke('frontend_ready'),
    convertFileSrc: p => T().core.convertFileSrc(p),
    /** Soltar archivos del sistema: `cb({type:'enter'|'over'|'drop'|'leave', paths, position})`. */
    onFileDrop(cb) { return T().webview.getCurrentWebview().onDragDropEvent(e => cb(e.payload)); }
  };
}

/** `window.__TAURI_INTERNALS__` → tauri; forzable con `?backend=mock|tauri`. */
export function detect(g = globalThis) {
  const q = new URLSearchParams((g.location && g.location.search) || '');
  const forced = q.get('backend');
  const hasTauri = !!(g.__TAURI_INTERNALS__ || g.__TAURI__);
  if (forced === 'tauri' || (!forced && hasTauri)) return createTauri(g);
  return createMock({ fast: q.has('fast') || q.has('still'), lang: q.get('lang') === 'en' ? 'en' : 'es', sidecar: q.get('sidecar') !== '0', defaultOff: q.has('off') ? q.get('off').split(',') : undefined, os: q.get('os') === 'win' ? 'win' : 'mac', adm: q.get('adm') });
}

export const backend = detect();
