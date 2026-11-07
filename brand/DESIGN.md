# ferboix. studio · DESIGN

## 1. Paleta y roles

Casi monocroma: tinta y papel; el color es una señal, no un decorado.

| Token | Claro | Oscuro | Rol |
|---|---|---|---|
| `--bg` | `#F4F3F0` papel | `#111111` tinta | fondo de ventana y página |
| `--bg-deep` | `#ECEBE7` | `#111111` | fondo hundido (en oscuro es plano, sin degradado) |
| `--card` | `#FFFFFF` | `#1A1A1A` | tarjeta, campo |
| `--fill` | `#ECEBE7` | `#222222` | reproductor, ficha en reposo, pastilla |
| `--line` | `#E2E0DB` | `#2B2B2B` | filetes de 1 px |
| `--ink` | `#111111` | `#F4F3F0` | texto principal, disco del símbolo, punto del wordmark |
| `--ink-2` | `#4A4843` | `#BDBBB6` | texto secundario |
| `--ink-3` | `#6A6863` | `#9A9893` | etiquetas y palabra `studio` |
| `--ctl` | `#85837E` | `#7A7873` | bordes de control y dropzone (≥3:1) |
| `--wv` / `--wv-on` | `#C9C7C2` / tinta | `#4A4945` / papel | onda sin reproducir / reproducida |
| `--btn-hi` / `--btn-lo` | `#111111` tinta | `#F4F3F0` papel | relleno del botón primario (plano: hi = lo) |
| `--on-btn` | `#F4F3F0` papel | `#111111` tinta | etiqueta del botón primario |
| `--brand` | `#0097B8` cian | `#0097B8` cian | marca del símbolo, play, lectura, selección, puntos de estado |
| `--brand-ink` | `#00728B` | `#0097B8` | enlaces, cifras en color y borde de foco (texto AA) |
| `--on-brand` | `#111111` | `#111111` | texto o check sobre un relleno cian |

**Acento único: cian LED `#0097B8`.**

- Relleno cian: 3,1:1 sobre papel y 5,5:1 sobre tinta (≥3:1 de gráfico en los dos modos con un solo valor).
- Texto-acento en claro **`#00728B`** (mismo tono, luminosidad bajada hasta ≥4,5:1 sobre bg, card y fill; 5,0:1 sobre papel). En oscuro el relleno ya es texto AA.
- Texto sobre relleno cian: tinta `#111111` (5,5:1); el blanco daría 3,4:1.
- **Botón primario = tinta** con texto papel en claro (17,0:1) y **papel con texto tinta** en oscuro (17,0:1). El cian no es relleno de botón.
- Estados: correcto `#1E7F4F`/`#5FCF96`, aviso `#E0A100` (texto `#7A5600`/`#F5C65A`), error `#B42318`/`#FF8A7D`. El cian es «activo/seleccionado/lectura», nunca ok ni error; todo estado lleva además palabra o icono.
- Sin degradados en iconos, botones ni fondos de marca (evita el bandeado). Los únicos degradados son las sombras suaves (blur) y los velos `--orb-*` al 3–5 %, en tinta.

### Contraste (generado por script, WCAG 2.x)

`python3 gen.py` aborta si algún par de **texto** baja de 4,5:1 (texto-acento, texto sobre botón, tinta y grises) o si el relleno cian baja de 3:1 sobre bg, card o el disco. Los pares **gráficos** se miden contra 3:1. El único que no llega es el relleno cian sobre `--fill` en claro (2,88:1): es el punto de una pastilla y siempre va acompañado de su palabra, por lo que no es único portador de significado.

| Modo | Par | Texto/forma | Fondo | Ratio | Mín. | Resultado |
|---|---|---|---|---|---|---|
| claro | tinta sobre bg | `#111111` | `#F4F3F0` | 17.02 | 4.5 | AA |
| claro | texto 2 sobre bg | `#4A4843` | `#F4F3F0` | 8.23 | 4.5 | AA |
| claro | texto 3 / palabra «studio» sobre bg | `#6A6863` | `#F4F3F0` | 5.02 | 4.5 | AA |
| claro | tinta sobre card | `#111111` | `#FFFFFF` | 18.88 | 4.5 | AA |
| claro | texto 2 sobre card | `#4A4843` | `#FFFFFF` | 9.13 | 4.5 | AA |
| claro | texto 3 / palabra «studio» sobre card | `#6A6863` | `#FFFFFF` | 5.57 | 4.5 | AA |
| claro | tinta sobre fill | `#111111` | `#ECEBE7` | 15.83 | 4.5 | AA |
| claro | texto 2 sobre fill | `#4A4843` | `#ECEBE7` | 7.66 | 4.5 | AA |
| claro | texto 3 / palabra «studio» sobre fill | `#6A6863` | `#ECEBE7` | 4.67 | 4.5 | AA |
| claro | ok sobre bg | `#1E7F4F` | `#F4F3F0` | 4.50 | 4.5 | AA |
| claro | ok sobre card | `#1E7F4F` | `#FFFFFF` | 5.00 | 4.5 | AA |
| claro | error sobre bg | `#B42318` | `#F4F3F0` | 5.92 | 4.5 | AA |
| claro | error sobre card | `#B42318` | `#FFFFFF` | 6.57 | 4.5 | AA |
| claro | aviso (texto) sobre bg | `#7A5600` | `#F4F3F0` | 5.99 | 4.5 | AA |
| claro | aviso (texto) sobre card | `#7A5600` | `#FFFFFF` | 6.65 | 4.5 | AA |
| claro | control (ctl) sobre bg | `#85837E` | `#F4F3F0` | 3.41 | 3.0 | AA |
| claro | cian · texto-acento (--brand-ink) sobre bg | `#00728B` | `#F4F3F0` | 5.01 | 4.5 | AA |
| claro | cian · texto-acento (--brand-ink) sobre card | `#00728B` | `#FFFFFF` | 5.56 | 4.5 | AA |
| claro | cian · texto-acento (--brand-ink) sobre fill | `#00728B` | `#ECEBE7` | 4.66 | 4.5 | AA |
| claro | botón primario · texto (--on-btn) sobre relleno (--btn-hi) | `#F4F3F0` | `#111111` | 17.02 | 4.5 | AA |
| claro | cian · texto sobre relleno cian (check de ficha, --on-brand) | `#111111` | `#0097B8` | 5.50 | 4.5 | AA |
| claro | cian · relleno (--brand: marca, play, puntos) sobre bg | `#0097B8` | `#F4F3F0` | 3.09 | 3.0 | AA |
| claro | cian · relleno (--brand: marca, play, puntos) sobre card | `#0097B8` | `#FFFFFF` | 3.43 | 3.0 | AA |
| claro | cian · relleno (--brand) sobre fill (punto en pastilla; lleva palabra) | `#0097B8` | `#ECEBE7` | 2.88 | 3.0 | no llega (solo decorativo/logo; nunca único portador de significado) |
| claro | marca del símbolo (cian) sobre el disco | `#0097B8` | `#111111` | 5.50 | 3.0 | AA |
| oscuro | tinta sobre bg | `#F4F3F0` | `#111111` | 17.02 | 4.5 | AA |
| oscuro | texto 2 sobre bg | `#BDBBB6` | `#111111` | 9.84 | 4.5 | AA |
| oscuro | texto 3 / palabra «studio» sobre bg | `#9A9893` | `#111111` | 6.55 | 4.5 | AA |
| oscuro | tinta sobre card | `#F4F3F0` | `#1A1A1A` | 15.68 | 4.5 | AA |
| oscuro | texto 2 sobre card | `#BDBBB6` | `#1A1A1A` | 9.07 | 4.5 | AA |
| oscuro | texto 3 / palabra «studio» sobre card | `#9A9893` | `#1A1A1A` | 6.04 | 4.5 | AA |
| oscuro | tinta sobre fill | `#F4F3F0` | `#222222` | 14.34 | 4.5 | AA |
| oscuro | texto 2 sobre fill | `#BDBBB6` | `#222222` | 8.29 | 4.5 | AA |
| oscuro | texto 3 / palabra «studio» sobre fill | `#9A9893` | `#222222` | 5.52 | 4.5 | AA |
| oscuro | ok sobre bg | `#5FCF96` | `#111111` | 9.76 | 4.5 | AA |
| oscuro | ok sobre card | `#5FCF96` | `#1A1A1A` | 8.99 | 4.5 | AA |
| oscuro | error sobre bg | `#FF8A7D` | `#111111` | 8.26 | 4.5 | AA |
| oscuro | error sobre card | `#FF8A7D` | `#1A1A1A` | 7.61 | 4.5 | AA |
| oscuro | aviso (texto) sobre bg | `#F5C65A` | `#111111` | 11.79 | 4.5 | AA |
| oscuro | aviso (texto) sobre card | `#F5C65A` | `#1A1A1A` | 10.86 | 4.5 | AA |
| oscuro | control (ctl) sobre bg | `#7A7873` | `#111111` | 4.28 | 3.0 | AA |
| oscuro | cian · texto-acento (--brand-ink) sobre bg | `#0097B8` | `#111111` | 5.50 | 4.5 | AA |
| oscuro | cian · texto-acento (--brand-ink) sobre card | `#0097B8` | `#1A1A1A` | 5.07 | 4.5 | AA |
| oscuro | cian · texto-acento (--brand-ink) sobre fill | `#0097B8` | `#222222` | 4.64 | 4.5 | AA |
| oscuro | botón primario · texto (--on-btn) sobre relleno (--btn-hi) | `#111111` | `#F4F3F0` | 17.02 | 4.5 | AA |
| oscuro | cian · texto sobre relleno cian (check de ficha, --on-brand) | `#111111` | `#0097B8` | 5.50 | 4.5 | AA |
| oscuro | cian · relleno (--brand: marca, play, puntos) sobre bg | `#0097B8` | `#111111` | 5.50 | 3.0 | AA |
| oscuro | cian · relleno (--brand: marca, play, puntos) sobre card | `#0097B8` | `#1A1A1A` | 5.07 | 3.0 | AA |
| oscuro | cian · relleno (--brand) sobre fill (punto en pastilla; lleva palabra) | `#0097B8` | `#222222` | 4.64 | 3.0 | AA |
| oscuro | marca del símbolo (cian) sobre el disco | `#0097B8` | `#F4F3F0` | 3.09 | 3.0 | AA |

## 2. Tipografía

- **Familia única: Hanken Grotesk** (OFL, variable). Alternativa del sistema: `system-ui, -apple-system, "Segoe UI", sans-serif`.
- **Pesos:** 400 texto y palabra `studio` · 500 etiquetas y UI · 600 botones y titulares · **590 solo el wordmark**.
- **Escala** (tamaño/interlínea · interletraje): display 64/1,04 · −3,5 % · h1 40/1,10 · −2,5 % · h2 28/1,15 · −2 % · h3 20/1,25 · −1 % · cuerpo 16/1,5 · 0 · UI 13–15/1,4 · 0 · pie 12/1,4 · +1 % · etiqueta 11 en mayúsculas, 500, +6 %.
- **Cifras tabulares** (`font-variant-numeric: tabular-nums`) en medidores y tablas; el signo menos es `−` (U+2212).
- **Wordmark** (en em): tracking global −0,032; kerning manual par a par: `rb +0,006`, `oi +0,004`, `ix +0,004`. Punto: Ø = 1,22 × grosor de asta, separación 0,075, sobreimpresión 0,006. Palabra `studio`: 400, separación 0,30 em, mismo tamaño, tracking −0,032. Sin GPOS: todo son contornos.

## 3. Espaciado, radios y sombras

- **Espaciado** (px): 4 · 8 · 12 · 16 · 24 · 32 · 48 · 64 · 96. Rejilla de 4.
- **Radios:** hero/tarjeta 24 · chip/botón/popover/portada 16 · campo 12 · pastilla 999 · ventana macOS 26, Windows 8. Icono macOS: squircle (superelipse n = 4,6) sobre cuerpo de 824 en lienzo de 1024.
- **Sombras:** tarjeta ninguna (filete de 1 px) · popover `0 10px 26px -14px rgba(17,17,17,.22)` · ventana `0 24px 56px -20px rgba(0,0,0,.28)` (oscuro .6) · icono `0 10 14 rgba(0,0,0,.22)` (solo plantilla macOS). **Wordmark y símbolo: ninguna.** Nada de brillos exteriores de color.

## 4. Componentes

| Componente | Especificación |
|---|---|
| **Botón primario** | Relleno `--btn-hi` (**tinta** en claro, **papel** en oscuro), texto `--on-btn` 600/16, alto 52 (compacto 38–44), radio 16, plano. Hover: mezcla 8 % con el color contrario. Pulsado: escala .98 en 90 ms. Foco: aro 2 px `--brand-ink` a 2 px. Deshabilitado: `--fill` con `--ink-3`. Un solo primario por vista. |
| **Botón secundario** | Sin relleno, borde 1,5 px `--ink` al 85 %, texto `--ink` 600. Mismas medidas. |
| **Campo** | Alto 48, radio 12, fondo `--card`, borde 1 px `--line`, etiqueta 11 px en mayúsculas `--ink-3` y valor 15,5/500. Foco: borde y aro 2 px en `--brand-ink`. Error: borde `--err` + mensaje con icono. |
| **Ficha / chip** | Pastilla alto 32, fondo `--on-bg` (cian al 13 %/20 %), check en tinta, texto 13/550. Reposo: sin check. Ficha de plataforma: radio 17, borde 1,3 px tinta al seleccionar + **badge de check en `--brand` (cian)** con check `--on-brand`. |
| **Pastilla de estado** | Alto 30, fondo `--fill`, punto de 9 px en el color del estado + palabra. Estados: correcto, aviso, error, lectura (cian). |
| **Play y lectura** | Botón de play: disco `--brand` con triángulo `--on-brand`. Posición de lectura: punto `--brand` bajo la onda. La onda reproducida va en `--wv-on` (tinta). |
| **Tarjeta** | Fondo `--card`, filete 1 px `--line`, radio 24, sin sombra. Título 15/600 con punto de estado opcional. |
| **Cabecera de app** | Símbolo (disco ≈ 23 px) + wordmark ≈ 21 px + filete + nombre de la app en `--ink-3`. Ejemplo: `ferboix. studio | master check`. |

Ejemplos: `kit.png` (componentes en claro y oscuro) y `familia.png` (marca, apps, estados y hueco para apps futuras).

## 5. Iconografía

- **Interfaz:** trazo 1,6–1,8 px, `stroke-linecap` y `linejoin` redondos, rejilla de 16/24, color `--ink-3` en reposo y `--ink` al activarse; el cian solo en estados. Sin logos de terceros; las plataformas se representan con formas neutras.
- **App:** el **potenciómetro** (disco Ø 68 % del cuerpo, tinta sobre papel en claro / papel sobre tinta en oscuro; marca cian a 35°). Colores planos. macOS: 1024 con cuerpo de 824, squircle y sombra suave de plantilla; `iconset` con 16, 32, 64, 128, 256, 512 y 1024 (@1x y @2x) y `.icns`; versión `fullbleed` sin máscara para Icon Composer. Windows: `.ico` 16, 24, 32, 48, 64, 128 y 256 con tile redondeado (r 22 %), símbolo ampliado ópticamente en 16–64 (×1,18 a 16 px).
- **Legibilidad medida** (icono macOS escalado; disco y marca en píxeles; criterio: disco ≥6 px en ambos ejes y marca ≥6 px de superficie visible):

  | Modo | px | Disco ancho | Disco alto | Marca (px) | Disco/cuerpo | Marca/disco | OK |
  |---|---|---|---|---|---|---|---|
  | claro | 16 | 8 | 8 | 15 | 17.02:1 | 5.5:1 | sí |
  | claro | 32 | 18 | 18 | 43 | 17.02:1 | 5.5:1 | sí |
  | claro | 128 | 70 | 70 | 529 | 17.02:1 | 5.5:1 | sí |
  | claro | 512 | 280 | 280 | 7892 | 17.02:1 | 5.5:1 | sí |
  | claro | 1024 | 560 | 560 | 31193 | 17.02:1 | 5.5:1 | sí |
  | oscuro | 16 | 8 | 8 | 15 | 17.02:1 | 3.09:1 | sí |
  | oscuro | 32 | 18 | 18 | 45 | 17.02:1 | 3.09:1 | sí |
  | oscuro | 128 | 70 | 70 | 530 | 17.02:1 | 3.09:1 | sí |
  | oscuro | 512 | 280 | 280 | 7878 | 17.02:1 | 3.09:1 | sí |
  | oscuro | 1024 | 560 | 560 | 31195 | 17.02:1 | 3.09:1 | sí |

  El disco tiene 17,0:1 contra el cuerpo del icono; la marca cian tiene 5,5:1 contra el disco claro y 3,1:1 contra el disco papel del icono oscuro (gráfico ≥3).

## 6. Modo oscuro

Es un tema completo, no una inversión: `--bg` `#111111` plano; tarjetas `#1A1A1A`; filetes `#2B2B2B`; texto `#F4F3F0` con grises `#BDBBB6` y `#9A9893`. El cian se mantiene `#0097B8` (5,5:1 sobre tinta). El disco del símbolo pasa a papel y el botón primario a papel con texto tinta. Sombras más densas y filete interior de 1 px de luz. Icono oscuro: cuerpo `#111111` con filete `rgba(255,255,255,.16)`. Selector: `:root[data-theme=dark]`.

## 7. Tokens

Archivos: `tokens.json` (`color` = claro y oscuro con los mismos nombres de variable que `design/final/tokens.json`; `glass`, `layout`, `radius`, `space`) y `tokens.css` (`:root` = claro; `[data-theme=dark]`). La app los lleva en `app/ui/tokens.json` y `app/ui/styles.css`.

**Mapa de variables del prototipo** (las 54 variables de color y cristal de `:root` y `[data-theme=dark]` de `design/final/styles.css` existen en los tokens: comprobado por script; las `--r-*` son radios y viven en `radius`):

| Variable | Valor ferboix. studio |
|---|---|
| `--brand` / `--brand-ink` | cian / cian-texto AA |
| `--btn-hi` / `--btn-lo` / `--on-btn` | iguales entre sí (botón plano) en tinta (claro) o papel (oscuro) / texto contrario |
| `--bg`, `--bg-deep`, `--ink`, `--ink-2`, `--ink-3` | papel/tinta y grises de la tabla 1 |
| `--amber`, `--amber-ink` | estado de aviso ocre (el punto del indicador de nivel) |
| `--ok`, `--err` | estados |
| `--on-bg`, `--on-line`, `--focus` | cian al 13–60 % |
| `--on-glow`, `--btn-glow` | sombra neutra (sin brillo de color) |
| `--orb-1..3` | velos tenues de tinta (3–5 %) |
| `--g-*`, `--pop-bg`, `--win-edge` | cristal neutro |
| `--ctl`, `--wv`, `--wv-on` | control gris, onda gris, onda reproducida en tinta |

**Cambios de CSS necesarios respecto al prototipo** (los tokens no pueden resolverlos):
1. `font-family`: `'Hanken Grotesk'` (`fonts/HankenGrotesk.ttf`).
2. `:focus-visible{outline:2px solid var(--brand-ink)}`: el relleno cian sobre papel es 3,1:1 y el aro es el texto-acento.
3. `.cov.has::before` usa `rgba(17,17,17,.52)` (tinta), no un azul fijo.
4. `--on-bg` es color sólido translúcido; `--btn-hi = --btn-lo` deja el botón plano; en oscuro `--bg-deep = --bg` (sin banda).
5. El botón primario usa `--btn-hi` / `--on-btn` (tinta/papel), no `--brand`.
