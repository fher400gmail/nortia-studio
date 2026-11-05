/* nortia. studio · master check · formato de números por idioma (módulo puro, sin DOM).
   es: coma decimal y menos tipográfico (−9,1 LUFS) · en: punto decimal y guion (-9.1 LUFS). */

const MINUS = '−';

/** Número con `d` decimales. `signed` añade "+" a los positivos. Un valor que redondea a 0 no lleva signo. */
export function fmtNumber(v, d = 1, lang = 'en', { signed = false } = {}) {
  const r = Number(Math.abs(v).toFixed(d));
  let s = r.toFixed(d);
  if (lang === 'es') s = s.replace('.', ',');
  if (r === 0) return s;
  if (v < 0) return (lang === 'es' ? MINUS : '-') + s;
  return (signed ? '+' : '') + s;
}

export const fmtLufs = (v, lang = 'en') => `${fmtNumber(v, 1, lang)} LUFS`;
export const fmtDbtp = (v, lang = 'en', signed = true) => `${fmtNumber(v, 1, lang, { signed })} dBTP`;

/** 232 -> "3:52" */
export function fmtDuration(secs) {
  const s = Math.max(0, Math.round(secs));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, '0')}`;
}

/** 44100 -> "44,1 kHz" · 96000 -> "96 kHz" */
export function fmtRate(hz, lang = 'en') {
  const k = hz / 1000;
  return `${fmtNumber(k, Number.isInteger(k) ? 0 : 1, lang)} kHz`;
}

/** Tamaño en MB (1 MB = 10^6 bytes, como el Finder y el Explorador). */
export function fmtMB(bytes, lang = 'en', d = 1) {
  return `${fmtNumber(bytes / 1e6, d, lang)} MB`;
}

/** Tamaño legible: "<1 KB", "312 KB", "74,1 MB", "1,25 GB". */
export function fmtBytes(bytes, lang = 'en') {
  if (bytes < 1000) return '<1 KB';
  if (bytes < 1e6) return `${Math.round(bytes / 1e3)} KB`;
  if (bytes < 1e9) return fmtMB(bytes, lang, bytes < 1e8 ? 1 : 0);
  return `${fmtNumber(bytes / 1e9, 2, lang)} GB`;
}

const CONTAINER = { wav: 'WAV', flac: 'FLAC', mp3: 'MP3', mp4: 'MP4', jpg: 'JPG', png: 'PNG' };

/** Especificación corta de un destino: "FLAC · 24 bit · 96 kHz", "MP3 · 320 kbps", "JPG". */
export function fmtSpec(job, lang = 'en') {
  const c = CONTAINER[job.container] || String(job.container || '').toUpperCase();
  if (job.container === 'mp3') return `${c} · 320 kbps`;
  if (job.container === 'wav' || job.container === 'flac') {
    if (!job.bits) return c;
    return `${c} · ${job.bits} bit · ${fmtRate(job.sample_rate, lang)}`;
  }
  return c;
}

/** Ruta corta para mostrar: los dos últimos tramos ("Escritorio/Entregas"). */
export function shortPath(p) {
  const parts = String(p || '').split(/[\\/]+/).filter(Boolean);
  return parts.slice(-2).join('/');
}
