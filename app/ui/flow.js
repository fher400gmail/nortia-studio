/* nortia. studio · master check · reglas de flujo puras (sin DOM), probadas con node --test. */

/** Una exportación está cancelada si lo dijo el evento `cancelled` O lo dice la propia entrega
    (el evento puede llegar después de que la promesa se resuelva). */
export const isCancelled = (flag, delivery) => !!(flag || (delivery && delivery.cancelled));

/** Fichas que arrancan apagadas si el motor no dice cuáles son (`app_info.default_on` ausente):
    las que presets.json trae con `default_on: false`. Hoy solo portada animada (el HEVC pesa mucho). */
export const SAFE_OFF = ['apple_motion'];

/** Conjunto de ids de preset encendidos al empezar. La lista del motor manda; si no llega una
    lista, se usa el conjunto seguro (todo salvo `SAFE_OFF`) y se avisa con `warn`. */
export function resolveDefaults(list, allPresetIds, warn = () => {}) {
  if (Array.isArray(list)) return new Set(list);
  warn('app_info sin default_on: se usa el conjunto seguro');
  return new Set(allPresetIds.filter(id => !SAFE_OFF.includes(id)));
}

/** Gancho de eventos de progreso que reenvía la cancelación con el PRIMER evento si el usuario ya
    la había pedido: pulsada antes de que el motor registre la exportación, `cancel_export` no
    encuentra nada que cancelar y la exportación acabaría como «listo». */
export function cancelOnFirstEvent(isCanceling, cancel) {
  let seen = false;
  return () => {
    if (seen) return;
    seen = true;
    if (isCanceling()) { try { Promise.resolve(cancel()).catch(() => {}); } catch { /* sin motor al que avisar */ } }
  };
}

/** ¿La ficha está apagada solo por falta de material y acepta que se lo suelten? (pista visual) */
export function awaitingMaterial({ media, id, disabled, own, cover, sidecar }) {
  if (!media.has(id) || !disabled || own || cover) return false;
  return id === 'portada' || sidecar !== false;
}

/** Avisos que nacen al exportar: destinos cuyo veredicto final (tras codificar, p. ej. el AAC de
    la comprobación con las herramientas de Apple) es peor que el que se anunció en el plan. Lo que ya se sabía antes de
    exportar (el pico real de la canción) no se repite. `planned`: preset_id -> texto del veredicto del plan (se compara el texto, no el nivel). */
export function finalNotes(verdicts, byPreset, t, planned = {}) {
  return (verdicts || [])
    .filter(v => v.verdict !== 'ok' && v.verdict_text && planned[v.preset_id] !== v.verdict_text)
    .map(v => {
      const id = byPreset[v.preset_id] || v.preset_id;
      return { id, preset_id: v.preset_id, name: t('chip.long.' + id), text: v.verdict_text };
    });
}
