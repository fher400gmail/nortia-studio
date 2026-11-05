# master check · D5 (diseño final) · nortia. studio

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

## Tokens (`:root` / `[data-theme=dark]`; `tokens.json` es copia de `brand/tokens.json`)
Marca nortia. studio: naranja `#FF5A00` (relleno, punto, check, onda de "creando"); texto-acento `#B84100` en claro y `#FF5A00` en oscuro; papel `#F4F3F0` / `#ECEBE7` (oscuro `#111111` plano), tinta `#111111`. Ámbar ocre solo como estado de aviso. Superficies del kit: `--card #FFFFFF` / `#1A1A1A`, `--fill`, `--line`.
Tipografía: **Hanken Grotesk** variable (400/500/600, `fonts/HankenGrotesk.ttf`, `@font-face` con `font-weight:100 900`); escala intacta (13 base, 15,5 campos, 16 botón). La app no depende de Google Fonts.
Texto: `--ink #111111`, `--ink-2 #4A4843`, `--ink-3 #6A6863` (oscuro `#F4F3F0 / #BDBBB6 / #9A9893`).
Radios del sistema: hero 24, ficha/botón/portada/popover 16, campo 12, pastilla 999 (ventana macOS 26, Windows 8).
Vidrio (se mantiene): `--g-bg` blanco 62 % (oscuro 5,5 %), `--g-blur` 22 px (26 oscuro) con `saturate(1.55)`, borde especular de 1 px por máscara, reflejo `--g-spec` que sigue al puntero, sombras `--g-shadow` / `--g-shadow-lift`. Ambiente: focos neutros cálidos (`--orb-2` tinta 3 %) con un velo de naranja (`--orb-1` 12 % claro / 10 % oscuro) y paralaje de 14 px.
Botón primario: naranja plano (`--btn-hi = --btn-lo`), texto tinta `#111111`, relieve de cristal sutil (brillo superior inset, degradado suave `soft-light`, sombra `--btn-glow` a 40 %); hover = 8 % de tinta + reflejo cálido hacia el puntero.
Fichas de plataforma: activa = fondo `--card` neutro, borde tinta 1,3 px y badge de check naranja (único naranja; el badge entra con rebote 500 ms); apagada = borde `--line` y contenido al 45 %; hover = halo naranja suave (16 %) y escala 1,025. Pastilla "Lo necesario": tinta sobre `--fill` con el check naranja. Glifos neutros (tile en tinta), sin colores de terceros.
Onda: reposo `#C9C7C2` (oscuro `#4A4945`), hover/reproducción en tinta, **relleno naranja durante "creando"**, asentada en tinta al terminar. Pastilla de loudness: naranja solo como punto de 8 px.
Marca en la barra de título: wordmark "nortia. studio" del kit (SVG inline; "nortia" en `--ink`, punto en `--brand`, "studio" en `--ink-3`) + separador + "master check". Símbolo de la zona de soltar y de "listo": barra naranja 2:1 (sustituye al rombo).
Cambios de CSS de DESIGN.md aplicados: `:focus-visible` con `--brand-ink`; overlay de portada `rgba(17,17,17,.52)`; `--on-bg` sólido; `--btn-hi = --btn-lo` y `--bg-deep = --bg` en oscuro.

## Microinteracciones (curvas: `--ease` cubic-bezier(.2,.8,.2,1) · `--spring` cubic-bezier(.34,1.5,.64,1))
| Elemento | Hover / foco | Pulsación |
|---|---|---|
| Botón "Crear carpeta" | sube 2 px, sombra de color más larga, brillo radial hacia el puntero, icono gira −6°: 240 ms spring | baja a 0 y escala .992: 90 ms |
| Ficha de plataforma | escala 1,025 + sube 1 px, borde especular con destello en el puntero, tile 1,06: 240 ms spring | escala .975: 90 ms; al activar, tile "rebota" 420 ms y el check se dibuja (trazo 240 ms, retraso 90 ms) |
| Campos Artista/Título | fondo se aclara 200 ms; al foco sube 1 px, anillo 3,5 px + resplandor, placeholder se desplaza 3 px | n/a; al autorrellenar, pulso de color 1,1 s |
| Más datos / Ajustes avanzados | chevron gira 180° (spring); el panel se despliega desde su esquina (escala .94→1, −8 px→0, 240 ms spring, campos escalonados 28 ms) | Esc cierra y devuelve el foco |
| Zona de soltar (vacío) | "respira" 4 s (halo naranja + borde); hover: halo fijo, escala 1,004 | al arrastrar: borde sólido, halo 5 px + resplandor, símbolo sube 8 px y escala 1,16 (240 ms spring) |
| Onda | barras cercanas al puntero crecen hasta ×1,34 y pasan a tinta (gauss de 46 px, 180 ms spring); línea y marca de tiempo; clic = ir a ese punto | n/a |
| Play, pastillas, "cambiar", tema | elevan 1 px / fondo vidrio 200 ms; el icono de reiniciar gira −120° | 90 ms escala .92–.97 |
| Estados | creando: la onda se rellena como progreso y aparece la lista (fila 550 ms); listo: ondulación de la onda (750 ms, 7 ms por barra) y el símbolo entra por polos (700 ms) | |

`prefers-reduced-motion`: sin keyframes, transiciones ≈0, sin escalas/traslaciones/paralaje ni ondulación (los cambios de color se mantienen). `prefers-reduced-transparency` y falta de `backdrop-filter`: vidrio opaco (alfa .92–.95).

## Para ingeniería (Tauri 2)
```json
// tauri.conf.json → app.windows[0]
{ "title": "master check", "width": 880, "height": 860, "minWidth": 760, "minHeight": 720,
  "resizable": true, "center": true, "visible": false,
  "decorations": true, "titleBarStyle": "Overlay", "hiddenTitle": true,   // solo macOS
  "transparent": false }
```
- **macOS**: `decorations: true` + `titleBarStyle: "Overlay"` + `hiddenTitle: true`. El contenido llega hasta arriba y los semáforos nativos quedan sobre la tarjeta; el sistema dibuja esquinas, borde y sombra, por eso **no hace falta `transparent`** (evita `macOSPrivateApi`). La app detecta Tauri (`window.__TAURI_INTERNALS__`) y pone `html[data-native=true]`: oculta los semáforos de maqueta, deja `padding-left: 88 pt` en la barra y radio 0. Si la versión de Tauri soporta `trafficLightPosition`, usar `{x:20, y:20}`; si no, comprobar a ojo que los 3 botones no pisan el logotipo (empieza en x=88). No verificado en dispositivo.
- **Windows**: `decorations: false`, `transparent: true` (esquinas de 8 pt dibujadas en CSS; sin sombra nativa, añadir `shadow: true`). Los controles de la derecha son HTML (`[data-win=min|max|close]`) y llaman a `getCurrentWindow().minimize()/toggleMaximize()/close()`: requiere `withGlobalTauri: true` o sustituir `winCtl()` en `app.js` por el import de `@tauri-apps/api/window`. Capability `core:window:allow-minimize|toggle-maximize|close|start-dragging`. `os` se detecta por `navigator.platform`; forzable con `?os=`.
- Arrastre: `data-tauri-drag-region` ya está en la barra; los botones no lo llevan.
- Vidrio real del sistema (opcional): con `window-vibrancy` (mac `NSVisualEffectMaterial::Sidebar`, Win11 Mica/Acrylic) pasar `transparent: true` y bajar `--bg` alfa; la maqueta no lo necesita.
- Foco de teclado visible en todo, `role=switch` con `aria-checked` en las fichas, paneles con `aria-expanded/controls`. Los glifos de plataforma son formas neutras propias (kit: sin logos de terceros).

## Revisión
**Contraste AA (colores reales compuestos, `contraste.py`)**: 582 textos medidos (claro/oscuro × vacío, normal, mínimo, creando, listo, Windows y paneles abiertos). Estados principales (sin panel abierto): **0 fallos**; mínimos 4,68:1 (claro, "Más datos" a 760 × 720) y 5,15:1 (oscuro). Texto tinta sobre naranja `#FF5A00` ≈ 6,6:1; texto-acento `#B84100` sobre papel ≥ 4,6:1. El naranja de relleno sobre papel (2,82:1) solo se usa en formas (punto, check, onda), nunca en texto. 4 avisos, todos con un panel abierto y de muestreo (sombra del panel sobre los rótulos "Distribuidora/Extras", "Lo necesario" bajo el árbol, fila cortada por el borde del panel).
**Pendiente / riesgos**: (1) el punto del loudness pasa a naranja de marca (el aviso lo lleva el texto "pasa de 0 dB"); el ámbar queda reservado a estados de aviso. (2) Objetivos pequeños: "Más datos", "cambiar" y botones de barra miden 24–30 pt de alto (válido en escritorio, WCAG 2.5.8; ampliar si hay pantalla táctil). (3) Las animaciones no se pueden comprobar en las PNG: revisar a mano `prefers-reduced-motion`. (4) Pruebas con VoiceOver/NVDA sin hacer. (5) El estado "listo" deja mucho aire bajo la onda a 860 pt; aceptable, se puede añadir la lista de archivos si se quiere más densidad.
