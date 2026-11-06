/* ferboix. studio · master check · textos de la interfaz (es/en). Módulo sin DOM. */

let dict = {};
let current = 'es';

export const getLang = () => current;

/** Carga i18n/<lang>.json. `loader(lang)` devuelve el objeto (en el navegador, fetch). */
export async function loadLang(lang, loader = defaultLoader) {
  current = lang === 'es' ? 'es' : 'en';
  dict = await loader(current);
  return current;
}

async function defaultLoader(lang) {
  const r = await fetch(new URL(`./i18n/${lang}.json`, import.meta.url));
  if (!r.ok) throw new Error(`i18n ${lang}: ${r.status}`);
  return r.json();
}

export function setDict(lang, d) { current = lang; dict = d; }

/** t('key', {n: 3}) — sustituye {n}. Si falta la clave devuelve la clave. */
export function t(key, vars = {}) {
  const s = dict[key] ?? key;
  return s.replace(/\{(\w+)\}/g, (m, k) => (k in vars ? vars[k] : m));
}

/** Plural simple: busca `key.one` cuando n === 1, si no `key.other`. */
export function tn(key, n, vars = {}) {
  return t(`${key}.${n === 1 ? 'one' : 'other'}`, { n, ...vars });
}
