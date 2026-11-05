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
  { id: 'distributor_std', sub: 'distributor', tag: 'Distribuidora', container: 'wav', bits: 16, rate: 44100 },
  { id: 'mp3_promo', sub: 'promo', tag: 'MP3 320', container: 'mp3', bits: 0, rate: 44100 },
  { id: 'cover', sub: 'artwork', tag: 'Portada', container: 'jpg', bits: 0, rate: 0, off: true },
  { id: 'canvas', sub: 'artwork', tag: 'Canvas', container: 'mp4', bits: 0, rate: 0, off: true }
];

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
    not_in_this_build: 'Aún no disponible en esta versión',
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
    not_in_this_build: 'Not yet available in this version',
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
 * `lame` (false simula que no hay lame), `os`.
 */
export function createMock({ fast = false, lang = 'es', lame = true, os = 'mac', version = '0.1.0-mock' } = {}) {
  let curLang = lang;
  let cancelled = false;
  let folderIx = 0;
  const tx = k => (TEXT[curLang] || TEXT.en)[k] || k;

  const be = {
    kind: 'mock',
    async setLang(l) { curLang = l === 'es' ? 'es' : 'en'; },
    async systemLang() { return curLang; },
    async appInfo() { return { version, os, lame_available: lame, default_out_root: MOCK_FOLDERS[curLang][0] }; },

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

    async plan(master, meta, activeIds) {
      const lg = curLang;
      const artist = (meta.artist || '').trim(), title = (meta.title || '').trim() || (suggestFromName(master.path).title || '');
      const base = `${artist} - ${title}`;
      const ver = (meta.version || '').trim();
      const folder = ver ? `${base} (${ver})` : base;
      const nativeBits = Math.min(master.is_float ? 32 : master.bits, 24);
      const jobs = PRESETS.filter(p => activeIds.includes(p.id)).map(p => {
        const subfolder = FOLDERS[lg][p.sub];
        const container = p.container;
        if (p.off || (p.minBits && p.bits === 'native' && nativeBits < p.minBits)) {
          return {
            preset_id: p.id, subfolder, file_name: '', container, bits: 0, sample_rate: 0, verdict: 'ok', verdict_text: '',
            estimated_bytes: 0, enabled: false, disabled_reason: tx(p.off ? 'not_in_this_build' : p.reason)
          };
        }
        const bits = p.bits === 'native' ? nativeBits : p.bits;
        const rate = p.rate === 'native' ? master.sample_rate : p.rate;
        const khz = rate / 1000;
        const tag = p.id === 'distributor_std' ? tx('tag.distributor') : p.tag;
        const khzLabel = Number.isInteger(khz) ? `${khz}kHz` : `${khz.toFixed(1)}kHz`;
        const file_name = container === 'mp3' ? `${base} [${tag}].mp3` : `${base} [${tag}] ${bits}bit-${khzLabel}.${container}`;
        const wav = master.frames * master.channels * bits / 8;
        const estimated_bytes = container === 'flac' ? Math.round(wav * .62) : container === 'wav' ? wav : container === 'mp3' ? Math.round(master.frames * 40000 / rate) : 0;
        const bad = master.true_peak_dbtp > 0 && container !== 'wav';
        return {
          preset_id: p.id, subfolder, file_name, container, bits, sample_rate: rate,
          verdict: bad ? 'warn' : 'ok', verdict_text: bad ? tx('tp_over_zero') : tx('verdict.ok'),
          estimated_bytes, enabled: true, disabled_reason: null
        };
      });
      const on = jobs.filter(j => j.enabled);
      return { delivery_folder: folder, jobs, total_files: on.length + 4, total_bytes: on.reduce((a, j) => a + j.estimated_bytes, 0) };
    },

    /** Emite la misma secuencia de eventos que el puente y devuelve el DeliveryDto. */
    async export(req, onProgress = () => {}) {
      cancelled = false;
      const { master } = await be.analyze(req.master_path, 4);
      const plan = await be.plan(master, req.meta, req.active_ids);
      const jobs = plan.jobs.filter(j => j.enabled);
      const step = fast ? 2 : 1;
      const wait = ms => new Promise(r => setTimeout(r, fast ? 6 : ms));
      const root = `${req.out_root}/${plan.delivery_folder}`;
      const files = [];
      onProgress({ kind: 'analyzed' });
      onProgress({ kind: 'planned' });
      for (const j of jobs) {
        if (cancelled) { onProgress({ kind: 'cancelled' }); return { folder: root, files, failed: [], elapsed_ms: 0, cancelled: true }; }
        onProgress({ kind: 'job_started', preset_id: j.preset_id });
        const ms = 280 + Math.min(j.estimated_bytes / 1e6, 140) / 140 * 500;
        for (let f = .25; f < 1; f += .25 * step) { await wait(ms / 4); onProgress({ kind: 'job_progress', preset_id: j.preset_id, fraction: Math.min(.99, f) }); }
        await wait(ms / 4);
        const path = `${root}/${j.subfolder}/${j.file_name}`;
        files.push([path, j.estimated_bytes]);
        onProgress({ kind: 'job_done', preset_id: j.preset_id, path, bytes: j.estimated_bytes });
      }
      DOCS[curLang].forEach((d, i) => files.push([`${root}/${d}`, [3000, 4000, 1000, 2000][i]]));
      const delivery = { folder: root, files, failed: [], elapsed_ms: 1200 };
      onProgress({ kind: 'finished', delivery });
      return delivery;
    },

    async cancelExport() { cancelled = true; },
    async pickFolder() { folderIx = (folderIx + 1) % MOCK_FOLDERS[curLang].length; return MOCK_FOLDERS[curLang][folderIx]; },
    async pickFile() { return null; },
    async openPath() {},
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
    plan: (master, meta, activeIds) => invoke('plan', { master, meta, activeIds }),
    async export(req, onProgress = () => {}) {
      const un = await T().event.listen(PROGRESS_EVENT, e => onProgress(e.payload));
      try { return await invoke('export', { req }); } finally { un(); }
    },
    cancelExport: () => invoke('cancel_export'),
    pickFolder: () => invoke('pick_folder'),
    pickFile: kind => invoke('pick_file', { kind }),
    openPath: path => invoke('open_path', { path }),
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
  return createMock({ fast: q.has('fast') || q.has('still'), lang: q.get('lang') === 'en' ? 'en' : 'es', lame: q.get('lame') !== '0', os: q.get('os') === 'win' ? 'win' : 'mac' });
}

export const backend = detect();
