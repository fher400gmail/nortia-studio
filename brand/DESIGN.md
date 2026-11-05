# nortia. · DESIGN

## 1. Paleta y roles

Casi monocroma: tinta y papel; el color es una señal, no un decorado.

| Token | Claro | Oscuro | Rol |
|---|---|---|---|
| `--bg` | `#F4F3F0` papel | `#111111` tinta | fondo de ventana y página |
| `--bg-deep` | `#ECEBE7` | `#111111` | fondo hundido (en oscuro es plano, sin degradado) |
| `--card` | `#FFFFFF` | `#1A1A1A` | tarjeta, campo |
| `--fill` | `#ECEBE7` | `#222222` | reproductor, ficha en reposo, pastilla |
| `--line` | `#E2E0DB` | `#2B2B2B` | filetes de 1 px |
| `--ink` | `#111111` | `#F4F3F0` | texto principal, botón primario de la madre |
| `--ink-2` | `#4A4843` | `#BDBBB6` | texto secundario |
| `--ink-3` | `#6A6863` | `#9A9893` | etiquetas, palabra de división |
| `--ctl` | `#85837E` | `#7A7873` | bordes de control y dropzone (≥3:1) |
| `--wv` / `--wv-on` | `#C9C7C2` / tinta | `#4A4945` / papel | onda sin reproducir / reproducida |
| `--brand` | acento (relleno) | acento | punto, botón primario, check, estados |
| `--brand-ink` | acento-texto AA | acento | enlaces y cifras en color |
| `--on-btn` | texto sobre `--brand` | idem | etiqueta del botón primario |

**Acentos** (solo punto, estados y botón primario):

| Marca | Relleno claro | Relleno oscuro | Texto claro (AA) | Texto oscuro | Texto sobre botón |
|---|---|---|---|---|---|
| studio (naranja) | `#FF5A00` | `#FF5A00` | `#B84100` | `#FF5A00` | `#111111` / `#111111` |
| lab (azul) | `#2B3BFF` | `#7483FF` | `#2B3BFF` | `#7483FF` | `#FFFFFF` / `#111111` |
| teal (teal) | `#00968B` | `#1FD1C2` | `#00756D` | `#1FD1C2` | `#111111` / `#111111` |
| magenta (magenta) | `#D6137A` | `#FF52A2` | `#C91273` | `#FF52A2` | `#FFFFFF` / `#111111` |

- **studio `#FF5A00`**: el naranja no llega a AA como texto sobre papel (2,82:1). **Naranja-texto oscurecido: `#B84100`** (mismo tono, luminosidad bajada por script hasta ≥4,6:1 sobre bg, card y fill). Úsalo para enlaces, cifras y el borde de foco. El relleno `#FF5A00` queda para punto, botón, barras y badges. Sobre botón naranja el texto es **tinta `#111111`** (6,04:1); el blanco daría 3,13:1 y no pasa.
- **lab `#2B3BFF`** es AA como texto en claro (6,0:1); en oscuro sube a `#7483FF`.
- **Reservas propuestas, compatibles con naranja y azul:** **teal `#00968B`** (tono 175°) y **magenta `#D6137A`** (tono 330°). Quedan a ≥55° de naranja y azul. Magenta está a unos 35° del rojo de error: **nunca** se usa para errores y todo estado lleva icono y palabra, no solo color.
- Estados: correcto `#1E7F4F`/`#5FCF96`, aviso `#E0A100` (texto `#7A5600`/`#F5C65A`), error `#B42318`/`#FF8A7D`. El aviso es ocre para no confundirse con el naranja de studio.
- Sin degradados en iconos, botones ni fondos de marca (evita el bandeado). Los únicos degradados son las sombras suaves (blur) y los velos `--orb-*` al 3–12 %.

### Contraste (generado por script, WCAG 2.x)

`python3 gen.py` aborta si algún par de **texto** baja de 4,5:1. Los pares **gráficos** se miden contra 3:1. El único que no llega es el relleno naranja sobre papel (2,82:1): es el color de la marca (el logo está exento) y nunca es el único portador de significado; el foco usa naranja-texto.

| Modo | Par | Texto/forma | Fondo | Ratio | Mín. | Resultado |
|---|---|---|---|---|---|---|
| claro | tinta sobre bg | `#111111` | `#F4F3F0` | 17.02 | 4.5 | AA |
| claro | texto 2 sobre bg | `#4A4843` | `#F4F3F0` | 8.23 | 4.5 | AA |
| claro | texto 3 / palabra de división sobre bg | `#6A6863` | `#F4F3F0` | 5.02 | 4.5 | AA |
| claro | tinta sobre card | `#111111` | `#FFFFFF` | 18.88 | 4.5 | AA |
| claro | texto 2 sobre card | `#4A4843` | `#FFFFFF` | 9.13 | 4.5 | AA |
| claro | texto 3 / palabra de división sobre card | `#6A6863` | `#FFFFFF` | 5.57 | 4.5 | AA |
| claro | tinta sobre fill | `#111111` | `#ECEBE7` | 15.83 | 4.5 | AA |
| claro | texto 2 sobre fill | `#4A4843` | `#ECEBE7` | 7.66 | 4.5 | AA |
| claro | texto 3 / palabra de división sobre fill | `#6A6863` | `#ECEBE7` | 4.67 | 4.5 | AA |
| claro | ok sobre bg | `#1E7F4F` | `#F4F3F0` | 4.50 | 4.5 | AA |
| claro | ok sobre card | `#1E7F4F` | `#FFFFFF` | 5.00 | 4.5 | AA |
| claro | error sobre bg | `#B42318` | `#F4F3F0` | 5.92 | 4.5 | AA |
| claro | error sobre card | `#B42318` | `#FFFFFF` | 6.57 | 4.5 | AA |
| claro | aviso (texto) sobre bg | `#7A5600` | `#F4F3F0` | 5.99 | 4.5 | AA |
| claro | aviso (texto) sobre card | `#7A5600` | `#FFFFFF` | 6.65 | 4.5 | AA |
| claro | control (ctl) sobre bg | `#85837E` | `#F4F3F0` | 3.41 | 3.0 | AA |
| claro | madre · texto-acento (--brand-ink) sobre bg | `#111111` | `#F4F3F0` | 17.02 | 4.5 | AA |
| claro | madre · texto-acento (--brand-ink) sobre card | `#111111` | `#FFFFFF` | 18.88 | 4.5 | AA |
| claro | madre · texto-acento (--brand-ink) sobre fill | `#111111` | `#ECEBE7` | 15.83 | 4.5 | AA |
| claro | madre · texto del botón (--on-btn) sobre relleno (--brand) | `#F4F3F0` | `#111111` | 17.02 | 4.5 | AA |
| claro | madre · relleno del acento (--brand) sobre bg | `#111111` | `#F4F3F0` | 17.02 | 3.0 | AA |
| claro | studio (naranja) · texto-acento (--brand-ink) sobre bg | `#B84100` | `#F4F3F0` | 4.99 | 4.5 | AA |
| claro | studio (naranja) · texto-acento (--brand-ink) sobre card | `#B84100` | `#FFFFFF` | 5.53 | 4.5 | AA |
| claro | studio (naranja) · texto-acento (--brand-ink) sobre fill | `#B84100` | `#ECEBE7` | 4.64 | 4.5 | AA |
| claro | studio (naranja) · texto del botón (--on-btn) sobre relleno (--brand) | `#111111` | `#FF5A00` | 6.04 | 4.5 | AA |
| claro | studio (naranja) · relleno del acento (--brand) sobre bg | `#FF5A00` | `#F4F3F0` | 2.82 | 3.0 | no llega (solo decorativo/logo; nunca único portador de significado) |
| claro | lab (azul) · texto-acento (--brand-ink) sobre bg | `#2B3BFF` | `#F4F3F0` | 5.97 | 4.5 | AA |
| claro | lab (azul) · texto-acento (--brand-ink) sobre card | `#2B3BFF` | `#FFFFFF` | 6.62 | 4.5 | AA |
| claro | lab (azul) · texto-acento (--brand-ink) sobre fill | `#2B3BFF` | `#ECEBE7` | 5.55 | 4.5 | AA |
| claro | lab (azul) · texto del botón (--on-btn) sobre relleno (--brand) | `#FFFFFF` | `#2B3BFF` | 6.62 | 4.5 | AA |
| claro | lab (azul) · relleno del acento (--brand) sobre bg | `#2B3BFF` | `#F4F3F0` | 5.97 | 3.0 | AA |
| claro | teal (teal) · texto-acento (--brand-ink) sobre bg | `#00756D` | `#F4F3F0` | 5.03 | 4.5 | AA |
| claro | teal (teal) · texto-acento (--brand-ink) sobre card | `#00756D` | `#FFFFFF` | 5.58 | 4.5 | AA |
| claro | teal (teal) · texto-acento (--brand-ink) sobre fill | `#00756D` | `#ECEBE7` | 4.68 | 4.5 | AA |
| claro | teal (teal) · texto del botón (--on-btn) sobre relleno (--brand) | `#111111` | `#00968B` | 5.16 | 4.5 | AA |
| claro | teal (teal) · relleno del acento (--brand) sobre bg | `#00968B` | `#F4F3F0` | 3.30 | 3.0 | AA |
| claro | magenta (magenta) · texto-acento (--brand-ink) sobre bg | `#C91273` | `#F4F3F0` | 4.96 | 4.5 | AA |
| claro | magenta (magenta) · texto-acento (--brand-ink) sobre card | `#C91273` | `#FFFFFF` | 5.50 | 4.5 | AA |
| claro | magenta (magenta) · texto-acento (--brand-ink) sobre fill | `#C91273` | `#ECEBE7` | 4.61 | 4.5 | AA |
| claro | magenta (magenta) · texto del botón (--on-btn) sobre relleno (--brand) | `#FFFFFF` | `#D6137A` | 4.96 | 4.5 | AA |
| claro | magenta (magenta) · relleno del acento (--brand) sobre bg | `#D6137A` | `#F4F3F0` | 4.47 | 3.0 | AA |
| oscuro | tinta sobre bg | `#F4F3F0` | `#111111` | 17.02 | 4.5 | AA |
| oscuro | texto 2 sobre bg | `#BDBBB6` | `#111111` | 9.84 | 4.5 | AA |
| oscuro | texto 3 / palabra de división sobre bg | `#9A9893` | `#111111` | 6.55 | 4.5 | AA |
| oscuro | tinta sobre card | `#F4F3F0` | `#1A1A1A` | 15.68 | 4.5 | AA |
| oscuro | texto 2 sobre card | `#BDBBB6` | `#1A1A1A` | 9.07 | 4.5 | AA |
| oscuro | texto 3 / palabra de división sobre card | `#9A9893` | `#1A1A1A` | 6.04 | 4.5 | AA |
| oscuro | tinta sobre fill | `#F4F3F0` | `#222222` | 14.34 | 4.5 | AA |
| oscuro | texto 2 sobre fill | `#BDBBB6` | `#222222` | 8.29 | 4.5 | AA |
| oscuro | texto 3 / palabra de división sobre fill | `#9A9893` | `#222222` | 5.52 | 4.5 | AA |
| oscuro | ok sobre bg | `#5FCF96` | `#111111` | 9.76 | 4.5 | AA |
| oscuro | ok sobre card | `#5FCF96` | `#1A1A1A` | 8.99 | 4.5 | AA |
| oscuro | error sobre bg | `#FF8A7D` | `#111111` | 8.26 | 4.5 | AA |
| oscuro | error sobre card | `#FF8A7D` | `#1A1A1A` | 7.61 | 4.5 | AA |
| oscuro | aviso (texto) sobre bg | `#F5C65A` | `#111111` | 11.79 | 4.5 | AA |
| oscuro | aviso (texto) sobre card | `#F5C65A` | `#1A1A1A` | 10.86 | 4.5 | AA |
| oscuro | control (ctl) sobre bg | `#7A7873` | `#111111` | 4.28 | 3.0 | AA |
| oscuro | madre · texto-acento (--brand-ink) sobre bg | `#F4F3F0` | `#111111` | 17.02 | 4.5 | AA |
| oscuro | madre · texto-acento (--brand-ink) sobre card | `#F4F3F0` | `#1A1A1A` | 15.68 | 4.5 | AA |
| oscuro | madre · texto-acento (--brand-ink) sobre fill | `#F4F3F0` | `#222222` | 14.34 | 4.5 | AA |
| oscuro | madre · texto del botón (--on-btn) sobre relleno (--brand) | `#111111` | `#F4F3F0` | 17.02 | 4.5 | AA |
| oscuro | madre · relleno del acento (--brand) sobre bg | `#F4F3F0` | `#111111` | 17.02 | 3.0 | AA |
| oscuro | studio (naranja) · texto-acento (--brand-ink) sobre bg | `#FF5A00` | `#111111` | 6.04 | 4.5 | AA |
| oscuro | studio (naranja) · texto-acento (--brand-ink) sobre card | `#FF5A00` | `#1A1A1A` | 5.56 | 4.5 | AA |
| oscuro | studio (naranja) · texto-acento (--brand-ink) sobre fill | `#FF5A00` | `#222222` | 5.09 | 4.5 | AA |
| oscuro | studio (naranja) · texto del botón (--on-btn) sobre relleno (--brand) | `#111111` | `#FF5A00` | 6.04 | 4.5 | AA |
| oscuro | studio (naranja) · relleno del acento (--brand) sobre bg | `#FF5A00` | `#111111` | 6.04 | 3.0 | AA |
| oscuro | lab (azul) · texto-acento (--brand-ink) sobre bg | `#7483FF` | `#111111` | 5.78 | 4.5 | AA |
| oscuro | lab (azul) · texto-acento (--brand-ink) sobre card | `#7483FF` | `#1A1A1A` | 5.33 | 4.5 | AA |
| oscuro | lab (azul) · texto-acento (--brand-ink) sobre fill | `#7483FF` | `#222222` | 4.87 | 4.5 | AA |
| oscuro | lab (azul) · texto del botón (--on-btn) sobre relleno (--brand) | `#111111` | `#7483FF` | 5.78 | 4.5 | AA |
| oscuro | lab (azul) · relleno del acento (--brand) sobre bg | `#7483FF` | `#111111` | 5.78 | 3.0 | AA |
| oscuro | teal (teal) · texto-acento (--brand-ink) sobre bg | `#1FD1C2` | `#111111` | 9.85 | 4.5 | AA |
| oscuro | teal (teal) · texto-acento (--brand-ink) sobre card | `#1FD1C2` | `#1A1A1A` | 9.08 | 4.5 | AA |
| oscuro | teal (teal) · texto-acento (--brand-ink) sobre fill | `#1FD1C2` | `#222222` | 8.30 | 4.5 | AA |
| oscuro | teal (teal) · texto del botón (--on-btn) sobre relleno (--brand) | `#111111` | `#1FD1C2` | 9.85 | 4.5 | AA |
| oscuro | teal (teal) · relleno del acento (--brand) sobre bg | `#1FD1C2` | `#111111` | 9.85 | 3.0 | AA |
| oscuro | magenta (magenta) · texto-acento (--brand-ink) sobre bg | `#FF52A2` | `#111111` | 6.28 | 4.5 | AA |
| oscuro | magenta (magenta) · texto-acento (--brand-ink) sobre card | `#FF52A2` | `#1A1A1A` | 5.79 | 4.5 | AA |
| oscuro | magenta (magenta) · texto-acento (--brand-ink) sobre fill | `#FF52A2` | `#222222` | 5.29 | 4.5 | AA |
| oscuro | magenta (magenta) · texto del botón (--on-btn) sobre relleno (--brand) | `#111111` | `#FF52A2` | 6.28 | 4.5 | AA |
| oscuro | magenta (magenta) · relleno del acento (--brand) sobre bg | `#FF52A2` | `#111111` | 6.28 | 3.0 | AA |

## 2. Tipografía

- **Familia única: Hanken Grotesk** (OFL, variable). Alternativa del sistema: `system-ui, -apple-system, "Segoe UI", sans-serif`.
- **Pesos:** 400 texto y palabra de división · 500 etiquetas y UI · 600 botones y titulares · **590 solo el wordmark**.
- **Escala** (tamaño/interlínea · interletraje): display 64/1,04 · −3,5 % · h1 40/1,10 · −2,5 % · h2 28/1,15 · −2 % · h3 20/1,25 · −1 % · cuerpo 16/1,5 · 0 · UI 13–15/1,4 · 0 · pie 12/1,4 · +1 % · etiqueta 11 en mayúsculas, 500, +6 %.
- **Cifras tabulares** (`font-variant-numeric: tabular-nums`) en medidores y tablas; el signo menos es `−` (U+2212).
- **Interletraje del wordmark** (en em): tracking global −0,032; kerning manual par a par: `or +0,004`, `rt +0,012`, `ti +0,006`, `ia +0,002`, `no +0,002`. Punto: Ø = 1,22 × grosor de asta, separación 0,075, sobreimpresión 0,006. Palabra de división: 400, separación 0,30 em, mismo tamaño, tracking −0,032. Sin GPOS: todo son contornos.

## 3. Espaciado, radios y sombras

- **Espaciado** (px): 4 · 8 · 12 · 16 · 24 · 32 · 48 · 64 · 96. Rejilla de 4.
- **Radios:** hero/tarjeta 24 · chip/botón/popover/portada 16 · campo 12 · pastilla 999 · ventana macOS 26, Windows 8. Icono macOS: squircle (superelipse n = 4,6) sobre cuerpo de 824 en lienzo de 1024.
- **Sombras:** tarjeta ninguna (filete de 1 px) · popover `0 10px 26px -14px rgba(17,17,17,.22)` · ventana `0 24px 56px -20px rgba(0,0,0,.28)` (oscuro .6) · icono `0 10 14 rgba(0,0,0,.22)`. Nada de brillos exteriores de color.

## 4. Componentes

| Componente | Especificación |
|---|---|
| **Botón primario** | Relleno `--brand` (madre: `--ink`), texto `--on-btn` 600/16, alto 52 (compacto 38–44), radio 16, plano. Hover: mezcla 8 % con tinta. Pulsado: escala .98 en 90 ms. Foco: aro 2 px `--brand-ink` a 2 px. Deshabilitado: `--fill` con `--ink-3`. Un solo primario por vista. |
| **Botón secundario** | Sin relleno, borde 1,5 px `--ink` al 85 %, texto `--ink` 600. Mismas medidas. |
| **Campo** | Alto 48, radio 12, fondo `--card`, borde 1 px `--line`, etiqueta 11 px en mayúsculas `--ink-3` y valor 15,5/500. Foco: borde y aro 2 px en `--brand-ink`. Error: borde `--err` + mensaje con icono. |
| **Ficha / chip** | Pastilla alto 32, fondo `--on-bg` (acento al 13 %/20 %), check en tinta, texto 13/550. Reposo: sin check. Ficha de plataforma: radio 17, borde 1,3 px tinta al seleccionar + badge de check en `--brand`. |
| **Pastilla de estado** | Alto 30, fondo `--fill`, punto de 9 px en el color del estado + palabra. Estados: correcto, aviso, error, lectura (acento). |
| **Tarjeta** | Fondo `--card`, filete 1 px `--line`, radio 24, sin sombra. Título 15/600 con punto de acento opcional. |

Ejemplos: `kit.png` (componentes en claro y oscuro) y `familia.png` (botón, chips, estado y campo en cada acento).

## 5. Iconografía

- **Interfaz:** trazo 1,6–1,8 px, `stroke-linecap` y `linejoin` redondos, rejilla de 16/24, color `--ink-3` en reposo y `--ink` al activarse; el acento solo en estados. Sin logos de terceros; las plataformas se representan con formas neutras.
- **App:** madre = punto (Ø 34 % del cuerpo, tinta sobre papel); **studio = el punto estirado a barra vertical 28 % × 56 % (2:1)** en naranja sobre papel (claro) y sobre `#111111` (oscuro). Colores planos. macOS: 1024 con cuerpo de 824, squircle y sombra suave; versión `fullbleed` sin máscara para Icon Composer. Windows: `.ico` 16, 24, 32, 48, 64, 128 y 256 con tile redondeado (r 22 %), símbolo ampliado ópticamente en 16–64 (×1,28 a 16 px).
- **Legibilidad medida** (icono macOS escalado; ancho y alto del símbolo en píxeles; contraste símbolo/cuerpo; criterio: ≥3 px en ambos ejes):

  | Marca | Modo | px | Ancho | Alto | Contraste | OK |
  |---|---|---|---|---|---|---|
  | madre | claro | 16 | 4 | 4 | 17.02:1 | sí |
  | madre | claro | 32 | 8 | 8 | 17.02:1 | sí |
  | madre | claro | 128 | 34 | 34 | 17.02:1 | sí |
  | madre | claro | 512 | 140 | 140 | 17.02:1 | sí |
  | madre | claro | 1024 | 280 | 280 | 17.02:1 | sí |
  | madre | oscuro | 16 | 4 | 4 | 17.02:1 | sí |
  | madre | oscuro | 32 | 8 | 8 | 17.02:1 | sí |
  | madre | oscuro | 128 | 34 | 34 | 17.02:1 | sí |
  | madre | oscuro | 512 | 140 | 140 | 17.02:1 | sí |
  | madre | oscuro | 1024 | 280 | 280 | 17.02:1 | sí |
  | studio | claro | 16 | 4 | 6 | 2.82:1 | sí |
  | studio | claro | 32 | 8 | 14 | 2.82:1 | sí |
  | studio | claro | 128 | 28 | 58 | 2.82:1 | sí |
  | studio | claro | 512 | 116 | 230 | 2.82:1 | sí |
  | studio | claro | 1024 | 230 | 462 | 2.82:1 | sí |
  | studio | oscuro | 16 | 4 | 6 | 6.04:1 | sí |
  | studio | oscuro | 32 | 8 | 14 | 6.04:1 | sí |
  | studio | oscuro | 128 | 28 | 58 | 6.04:1 | sí |
  | studio | oscuro | 512 | 116 | 230 | 6.04:1 | sí |
  | studio | oscuro | 1024 | 230 | 462 | 6.04:1 | sí |

  El naranja sobre papel queda en 2,82:1 como forma: el símbolo es logo (exento de 1.4.11) y a 16 px mantiene 4 × 6 px de superficie; en la versión oscura sube a 6,04:1.

## 6. Modo oscuro

Es un tema completo, no una inversión: `--bg` `#111111` plano; tarjetas `#1A1A1A`; filetes `#2B2B2B`; texto `#F4F3F0` con grises `#BDBBB6` y `#9A9893`. Los acentos se aclaran solo si hace falta (azul `#7483FF`, teal `#1FD1C2`, magenta `#FF52A2`); el naranja se mantiene. Texto sobre botón en tinta salvo magenta claro. Sombras más densas y filete interior de 1 px de luz. Iconos oscuros: cuerpo `#111111` con filete `rgba(255,255,255,.16)`. Selector: `:root[data-theme=dark]`.

## 7. Tokens

Archivos: `tokens.json` (`color` = studio claro y oscuro con los mismos nombres que `nortia-studio/design/final/tokens.json`; `brands` = madre, studio, lab, teal, magenta × claro/oscuro; `glass`, `layout`, `radius`, `space`) y `tokens.css` (`:root` = madre claro; `[data-theme=dark]`; `[data-brand=studio|lab|teal|magenta]`).

**Mapa de variables del prototipo** (las 50 variables de `:root` y `[data-theme=dark]` de `styles.css` existen en los tokens: comprobado por script):

| Variable | Valor nortia. |
|---|---|
| `--brand` / `--brand-ink` | acento / acento-texto AA (antes verde azulado) |
| `--btn-hi` / `--btn-lo` / `--on-btn` | iguales entre sí (botón plano) / texto sobre acento |
| `--bg`, `--bg-deep`, `--ink`, `--ink-2`, `--ink-3` | papel/tinta y grises de la tabla 1 (antes azul noche) |
| `--amber`, `--amber-ink` | estado de aviso ocre (el punto del indicador de nivel) |
| `--ok`, `--err` | estados |
| `--on-bg`, `--on-line`, `--on-glow`, `--focus`, `--btn-glow` | acento al 13–60 % (madre: tinta) |
| `--orb-1..3` | velos tenues de acento y tinta (3–12 %) |
| `--g-*`, `--pop-bg`, `--win-edge` | cristal neutro (sin tinte azul) |
| `--ctl`, `--wv`, `--wv-on` | control gris, onda gris, onda reproducida en tinta |

**Cambios de CSS necesarios en el prototipo** (los tokens no pueden resolverlos):
1. `font-family`: de Inter a `'Hanken Grotesk'` (`fonts/HankenGrotesk.ttf`).
2. `:focus-visible{outline:2px solid var(--brand)}` → `var(--brand-ink)`: el naranja de relleno sobre papel es 2,82:1.
3. `.cov.has::before` usa `rgba(15,31,61,.52)` fijo (azul noche): cámbialo a `rgba(17,17,17,.52)`.
4. `--on-bg` pasa de degradado a color sólido translúcido; `--btn-hi = --btn-lo` deja el botón plano; en oscuro `--bg-deep = --bg` (sin banda).
