# Master Out · D5 (diseño final) · nortia. studio

Base: D4a "Una tarjeta". La ventana es la tarjeta. Archivos: `index.html`, `styles.css`, `app.js`, `tokens.json`, `fonts/`. Estados: `?theme=light|dark&state=empty|normal|creating|done&os=mac|win` (+ `still=1` congela animaciones, `pop=adv|more|tree|detail`, `drag=1`). Capturas en `capturas/` (`python3 capturas.py`), lámina `final.png`, auditoría `python3 contraste.py`.

## Ventana (pt = px lógicos)
| | Valor |
|---|---|
| Por defecto | **880 × 860** (contenido útil 820 pt de ancho; onda 820 × 293, ≈2,8:1) |
| Mínimo | **760 × 720** (onda 204 pt de alto; todo cabe sin scroll) |
| Máximo | libre: la onda recalcula barras (9 pt de paso) y las fichas se reparten 6 columnas; sin márgenes vacíos. Sin tope |
| Barra de título | macOS 52 pt (semáforos sobre la tarjeta, x=20, centrados); Windows 36 pt, controles 46 pt a la derecha |
| Esquinas | macOS 26 pt (las dibuja el sistema); Windows 8 pt (CSS) |
| Márgenes | `clamp(22px, 3.4vw, 40px)`; separación vertical `clamp(12px, 2.1vh, 20px)`; fichas `clamp(68px, 9.8vh, 92px)`; botón `clamp(52px, 6.6vh, 60px)` |

Reparto vertical a 860: barra 52 + onda flexible 293 + datos 84 + plataformas 28+84 + distribuidora/extras 26+84 + botón 57 + carpeta 20 + márgenes. Solo la onda absorbe el cambio de alto.

## Tokens (`:root` / `[data-theme=dark]`, espejo en `tokens.json`)
Marca: verde azulado `#0E5E63` (oscuro `#5BB8BC`), azul noche `#0F1F3D` (texto claro), ámbar `#BF7F12` solo en la punta del símbolo y en el punto de aviso del loudness, fondo `#F7F8FA` (oscuro `#0A1429`), Inter 400/500/600 incluida en `fonts/` (la app no depende de Google Fonts).
Vidrio: `--g-bg` blanco 58 % (oscuro 5,8 %), `--g-blur` 22 px (26 oscuro) con `saturate(1.55)`, borde especular de 1 px por máscara (`--g-ring-hi/lo`), reflejo `--g-spec` que sigue al puntero, sombras `--g-shadow` / `--g-shadow-lift`. Ambiente: tres focos de color difuso (`--orb-1..3`) detrás del vidrio, con paralaje de 14 px.
Botón primario: gradiente `--btn-hi → --btn-lo` (`#16787D → #0E5E63`; oscuro `#62C2C6 → #43A2A7`), brillo superior inset, sombra de color `--btn-glow`.
Texto: `--ink #0F1F3D`, `--ink-2 #44506A`, `--ink-3 #566279` (oscuro `#EAF0F8 / #B6C2D8 / #97A5C0`).

## Microinteracciones (curvas: `--ease` cubic-bezier(.2,.8,.2,1) · `--spring` cubic-bezier(.34,1.5,.64,1))
| Elemento | Hover / foco | Pulsación |
|---|---|---|
| Botón "Crear carpeta" | sube 2 px, sombra de color más larga, brillo radial hacia el puntero, icono gira −6°: 240 ms spring | baja a 0 y escala .992: 90 ms |
| Ficha de plataforma | escala 1,025 + sube 1 px, borde especular con destello en el puntero, tile 1,06: 240 ms spring | escala .975: 90 ms; al activar, tile "rebota" 420 ms y el check se dibuja (trazo 240 ms, retraso 90 ms) |
| Campos Artista/Título | fondo se aclara 200 ms; al foco sube 1 px, anillo 3,5 px + resplandor, placeholder se desplaza 3 px | n/a; al autorrellenar, pulso de color 1,1 s |
| Más datos / Ajustes avanzados | chevron gira 180° (spring); el panel se despliega desde su esquina (escala .94→1, −8 px→0, 240 ms spring, campos escalonados 28 ms) | Esc cierra y devuelve el foco |
| Zona de soltar (vacío) | "respira" 4 s (halo teal + borde); hover: halo fijo, escala 1,004 | al arrastrar: borde sólido, halo 5 px + resplandor, símbolo sube 8 px y escala 1,16 (240 ms spring) |
| Onda | barras cercanas al puntero crecen hasta ×1,34 y pasan a verde azulado (gauss de 46 px, 180 ms spring); línea y marca de tiempo; clic = ir a ese punto | n/a |
| Play, pastillas, "cambiar", tema | elevan 1 px / fondo vidrio 200 ms; el icono de reiniciar gira −120° | 90 ms escala .92–.97 |
| Estados | creando: la onda se rellena como progreso y aparece la lista (fila 550 ms); listo: ondulación de la onda (750 ms, 7 ms por barra) y el símbolo entra por polos (700 ms) | |

`prefers-reduced-motion`: sin keyframes, transiciones ≈0, sin escalas/traslaciones/paralaje ni ondulación (los cambios de color se mantienen). `prefers-reduced-transparency` y falta de `backdrop-filter`: vidrio opaco (alfa .92–.95).

## Para ingeniería (Tauri 2)
```json
// tauri.conf.json → app.windows[0]
{ "title": "Master Out", "width": 880, "height": 860, "minWidth": 760, "minHeight": 720,
  "resizable": true, "center": true, "visible": false,
  "decorations": true, "titleBarStyle": "Overlay", "hiddenTitle": true,   // solo macOS
  "transparent": false }
```
- **macOS**: `decorations: true` + `titleBarStyle: "Overlay"` + `hiddenTitle: true`. El contenido llega hasta arriba y los semáforos nativos quedan sobre la tarjeta; el sistema dibuja esquinas, borde y sombra, por eso **no hace falta `transparent`** (evita `macOSPrivateApi`). La app detecta Tauri (`window.__TAURI_INTERNALS__`) y pone `html[data-native=true]`: oculta los semáforos de maqueta, deja `padding-left: 88 pt` en la barra y radio 0. Si la versión de Tauri soporta `trafficLightPosition`, usar `{x:20, y:20}`; si no, comprobar a ojo que los 3 botones no pisan el logotipo (empieza en x=88). No verificado en dispositivo.
- **Windows**: `decorations: false`, `transparent: true` (esquinas de 8 pt dibujadas en CSS; sin sombra nativa, añadir `shadow: true`). Los controles de la derecha son HTML (`[data-win=min|max|close]`) y llaman a `getCurrentWindow().minimize()/toggleMaximize()/close()`: requiere `withGlobalTauri: true` o sustituir `winCtl()` en `app.js` por el import de `@tauri-apps/api/window`. Capability `core:window:allow-minimize|toggle-maximize|close|start-dragging`. `os` se detecta por `navigator.platform`; forzable con `?os=`.
- Arrastre: `data-tauri-drag-region` ya está en la barra; los botones no lo llevan.
- Vidrio real del sistema (opcional): con `window-vibrancy` (mac `NSVisualEffectMaterial::Sidebar`, Win11 Mica/Acrylic) pasar `transparent: true` y bajar `--bg` alfa; la maqueta no lo necesita.
- Foco de teclado visible en todo, `role=switch` con `aria-checked` en las fichas, paneles con `aria-expanded/controls`. Los logos de plataforma son glifos propios evocadores: sustituir por activos oficiales.

## Revisión
**Contraste AA (colores reales compuestos, `contraste.py`)**: 588 textos medidos (claro/oscuro × vacío, normal, mínimo, creando, listo, Windows y paneles abiertos), con el peor píxel de fondo bajo cada caja. Estados principales: **0 fallos**; mínimos 4,74:1 (claro, "45 %" sobre fila verde) y 5,34:1 (oscuro). Botón primario: blanco sobre `#16787D` ≥ 5,2:1; oscuro `#05262A` sobre `#43A2A7` ≥ 5,0:1. Onda ≥ 3:1 (`#7D90B5`). 4 avisos, todos con un panel abierto: rótulos "Distribuidora/Extras" a 3,7:1 mientras el panel Más datos proyecta sombra sobre ellos, y 2 filas del árbol cortadas por el borde del panel (falsos positivos de muestreo en el borde).
**Pendiente / riesgos**: (1) el punto ámbar del loudness es la única excepción a "ámbar solo en el símbolo" (es semántico; el texto "pasa de 0 dB" ya lleva el aviso). (2) Objetivos pequeños: "Más datos", "cambiar" y botones de barra miden 24–30 pt de alto (válido en escritorio, WCAG 2.5.8; ampliar si hay pantalla táctil). (3) Las animaciones no se pueden comprobar en las PNG: revisar a mano `prefers-reduced-motion`. (4) Pruebas con VoiceOver/NVDA sin hacer. (5) El estado "listo" deja mucho aire bajo la onda a 860 pt; aceptable, se puede añadir la lista de archivos si se quiere más densidad.
