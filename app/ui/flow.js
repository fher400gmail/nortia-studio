/* nortia. studio · master check · reglas de flujo puras (sin DOM), probadas con node --test. */

/** Una exportación está cancelada si lo dijo el evento `cancelled` O lo dice la propia entrega
    (el evento puede llegar después de que la promesa se resuelva). */
export const isCancelled = (flag, delivery) => !!(flag || (delivery && delivery.cancelled));
