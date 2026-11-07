"""BRAND.md y DESIGN.md (las tablas de contraste, tokens e iconos se generan desde el código)."""
from brand import *
import tokens as TK

BRAND = """# ferboix. studio · BRAND

Sistema cerrado. Opción C «Instrumento» (decisión D-37). Todo lo de esta carpeta sale de `python3 gen.py`.

## 1. Propósito y personalidad

1. **Quién:** ferboix. studio es la empresa de software personal de Fer. Hace herramientas pequeñas, exactas y tranquilas para quien hace música, para que termine la canción y no pelee con el software.
2. **Confianza:** tu archivo sale como lo dejaste, sin sorpresas.
3. **Precisión:** números exactos (LUFS, kHz, bits, formatos); ningún adorno que no informe.
4. **Calma:** el producto se aparta; la marca es tinta y papel y solo se enciende donde hay significado.
5. **Carácter:** de aparato de estudio llevado al mínimo: un potenciómetro, un piloto LED. No de corporación.

## 2. Tono de voz (3 reglas)

1. **Dato antes que adjetivo.** Di qué pasa y con qué cifra; no cómo de bueno es.
2. **Frase corta, verbo directo, tú.** Una idea por frase; sin exclamaciones, sin jerga de marketing, sin «solución», «potente» ni «revolucionario».
3. **Calma en los errores.** Di qué ha pasado, qué está a salvo y qué hacer; nunca culpes ni alarmes.

| Bien | Mal |
|---|---|
| «Tu máster está a −9,1 LUFS y pasa de 0 dB. Bájalo 1 dB y listo.» | «¡Atención! Tu máster tiene un nivel demasiado alto y podría sonar fatal.» |
| «No hemos podido guardar la carpeta. Tus archivos siguen intactos. Elige otra ubicación.» | «¡Error crítico! Algo ha fallado.» |
| «Prepara tu lanzamiento en un paso.» | «La solución todo en uno más potente para llevar tu música al siguiente nivel.» |

Nombres de producto y de app: **siempre en minúsculas** (`master check`), sin punto propio. El punto es de la marca. Excepción heredada: la app anterior se llama **«ferboix. Facturación»** y conserva ese nombre.

## 3. Arquitectura: una marca y nombres de app

Una sola marca: **ferboix. studio**. No hay divisiones, ni marca madre, ni acentos por producto.

- **Wordmark:** `ferboix.` en tinta (punto incluido) + `studio` en gris, un espacio de 0,30 em detrás del punto.
- **Símbolo:** el potenciómetro (§4). Es el icono de las apps y el avatar de la marca.
- **Apps:** se presentan como `ferboix. studio | nombre de la app`: símbolo, wordmark, un filete vertical y el nombre en minúsculas (`master check`). Todas comparten marca, símbolo y acento; lo único que cambia es el nombre.
- **Familia:** `master check` (app actual), `ferboix. Facturación` (app anterior, misma familia) y el hueco para apps futuras (`familia.png`).

**Cómo se nombra una app nueva**

1. Una o dos palabras, minúsculas, que digan el **oficio** (master check), no la tecnología.
2. Se escribe `ferboix. studio | nombre`. Nunca guion, nunca mayúscula, nunca punto propio.
3. Usa el mismo icono (el potenciómetro). Si una app necesita distinguirse en el Dock, se distingue por su nombre, no por un color nuevo.
4. El acento sigue siendo el cian: ninguna app estrena color.

## 4. Reglas del punto y del potenciómetro

**El punto**

1. El punto del wordmark es **circular, dibujado a medida**, nunca el glifo de la fuente: diámetro = grosor de asta × 1,22, separado 0,075 em de la «x» y apoyado en la línea de base con 0,6 % de sobreimpresión.
2. El punto es **del color de la tinta** (tinta sobre papel, papel sobre tinta), igual que las letras. **No va en el acento.**
3. El punto no se estira, no se anima en bucle y no se usa de viñeta decorativa. Cierra la marca y los titulares.
4. El punto final de un titular (`plataforma.`) es el mismo círculo, en tinta.
5. Zona de respeto: 1 diámetro de punto alrededor del wordmark. Tamaño mínimo: wordmark 64 px de ancho (pantalla) / 18 mm (impresión); símbolo 16 px.

**El potenciómetro (símbolo)**

1. Es un **disco visto desde arriba con su marca de posición**. Disco: tinta sobre papel (claro) y papel sobre tinta (oscuro). Marca: **cian `#0097B8`**, trazo redondeado, girada hacia la una (35° desde la vertical).
2. Geometría (T = lado del tile): radio del disco 0,34 T; marca de grosor 0,36 R que va de 0,12 R a 0,93 R del centro; extremos redondos. Todo plano: sin degradados, sin sombras, sin brillos, sin biseles.
3. En el icono macOS el símbolo va centrado en el cuerpo de 824; el icono lleva solo la sombra suave de la plantilla de macOS (no es del símbolo).
4. La marca es **el único cian del icono**. Nunca se gira a otro ángulo, nunca se añaden segundas marcas, escalas ni graduaciones.
5. Tamaños pequeños: a 16 px el disco mide 9 px y la marca se mantiene (se mide en `python3 gen.py`).

**El acento (cian)**

1. Un solo acento: `#0097B8`. Como relleno vale 3,1:1 sobre papel y 5,5:1 sobre tinta. Como **texto** en claro se usa `#00728B` (5,0:1); en oscuro `#0097B8` ya vale (5,5:1).
2. Dónde vive: la marca del potenciómetro, los estados (lectura, selección, play, punto de las pastillas y fichas), el foco y los enlaces.
3. Dónde no: el punto del wordmark, el relleno del botón primario, titulares ni letras.
4. Los estados correcto, aviso y error siguen siendo verde, ocre y rojo (DESIGN §1): el cian nunca significa «bien» ni «mal», solo «esto está activo o seleccionado».

## 5. Usos prohibidos

- Cambiar la fuente, el peso (590), el interletraje o el punto del wordmark; recomponerlo con texto vivo.
- Poner el punto en el acento, colorear letras o la palabra `studio` con el acento; dos acentos en un mismo logo.
- Degradados, brillos, biseles, sombras en el wordmark o en el símbolo; contornos y efectos 3D.
- Convertir el símbolo en otra cosa: perilla con graduaciones, volumen con ondas, disco con aguja, marca en otro ángulo.
- Usar el cian de relleno como **texto** sobre papel (3,1:1): para texto usa `#00728B`.
- Botón primario en cian (el primario es tinta/papel).
- Estirar o deformar el símbolo; rotar el símbolo entero (se gira la marca, y solo a 35°).
- Poner el logo sobre fotos sin contraste.
- Logos o marcas de terceros dentro de los iconos y portadas; nombres de personas en plantillas.
- Mayúsculas en `ferboix.` o en nombres de app (`Master Check`, `FERBOIX`), salvo la excepción heredada de `ferboix. Facturación`.
- Usar el cian como color de error o el rojo como acento.

## 6. Archivos

`logos/` (SVG + PNG 512/2048 transparentes: wordmark claro/oscuro/blanco/negro, símbolo, lockup vertical) · `avatares/` · `favicon/` · `banners/` · `iconos/macos` (1024, iconset con todos los tamaños y `.icns`) y `iconos/windows` · `plantillas/` · `kit.png` y `familia.png` · `tokens.json` / `tokens.css` · `DESIGN.md`.
"""

DESIGN = """# ferboix. studio · DESIGN

## 1. Paleta y roles

Casi monocroma: tinta y papel; el color es una señal, no un decorado.

| Token | Claro | Oscuro | Rol |
|---|---|---|---|
| `--bg` | `{l_bg}` papel | `{d_bg}` tinta | fondo de ventana y página |
| `--bg-deep` | `{l_deep}` | `{d_deep}` | fondo hundido (en oscuro es plano, sin degradado) |
| `--card` | `{l_card}` | `{d_card}` | tarjeta, campo |
| `--fill` | `{l_fill}` | `{d_fill}` | reproductor, ficha en reposo, pastilla |
| `--line` | `{l_line}` | `{d_line}` | filetes de 1 px |
| `--ink` | `{l_ink}` | `{d_ink}` | texto principal, disco del símbolo, punto del wordmark |
| `--ink-2` | `{l_ink2}` | `{d_ink2}` | texto secundario |
| `--ink-3` | `{l_ink3}` | `{d_ink3}` | etiquetas y palabra `studio` |
| `--ctl` | `{l_ctl}` | `{d_ctl}` | bordes de control y dropzone (≥3:1) |
| `--wv` / `--wv-on` | `{l_wv}` / tinta | `{d_wv}` / papel | onda sin reproducir / reproducida |
| `--btn-hi` / `--btn-lo` | `{l_ink}` tinta | `{d_ink}` papel | relleno del botón primario (plano: hi = lo) |
| `--on-btn` | `{l_bg}` papel | `{d_bg}` tinta | etiqueta del botón primario |
| `--brand` | `{acc_l}` cian | `{acc_d}` cian | marca del símbolo, play, lectura, selección, puntos de estado |
| `--brand-ink` | `{acc_tl}` | `{acc_td}` | enlaces, cifras en color y borde de foco (texto AA) |
| `--on-brand` | `{on_l}` | `{on_d}` | texto o check sobre un relleno cian |

**Acento único: cian LED `#0097B8`.**

- Relleno cian: 3,1:1 sobre papel y 5,5:1 sobre tinta (≥3:1 de gráfico en los dos modos con un solo valor).
- Texto-acento en claro **`{acc_tl}`** (mismo tono, luminosidad bajada hasta ≥4,5:1 sobre bg, card y fill; 5,0:1 sobre papel). En oscuro el relleno ya es texto AA.
- Texto sobre relleno cian: tinta `#111111` (5,5:1); el blanco daría 3,4:1.
- **Botón primario = tinta** con texto papel en claro (17,0:1) y **papel con texto tinta** en oscuro (17,0:1). El cian no es relleno de botón.
- Estados: correcto `#1E7F4F`/`#5FCF96`, aviso `#E0A100` (texto `#7A5600`/`#F5C65A`), error `#B42318`/`#FF8A7D`. El cian es «activo/seleccionado/lectura», nunca ok ni error; todo estado lleva además palabra o icono.
- Sin degradados en iconos, botones ni fondos de marca (evita el bandeado). Los únicos degradados son las sombras suaves (blur) y los velos `--orb-*` al 3–5 %, en tinta.

### Contraste (generado por script, WCAG 2.x)

`python3 gen.py` aborta si algún par de **texto** baja de 4,5:1 (texto-acento, texto sobre botón, tinta y grises) o si el relleno cian baja de 3:1 sobre bg, card o el disco. Los pares **gráficos** se miden contra 3:1. El único que no llega es el relleno cian sobre `--fill` en claro (2,88:1): es el punto de una pastilla y siempre va acompañado de su palabra, por lo que no es único portador de significado.

{contrast}

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

{leg}

  El disco tiene 17,0:1 contra el cuerpo del icono; la marca cian tiene 5,5:1 contra el disco claro y 3,1:1 contra el disco papel del icono oscuro (gráfico ≥3).

## 6. Modo oscuro

Es un tema completo, no una inversión: `--bg` `#111111` plano; tarjetas `#1A1A1A`; filetes `#2B2B2B`; texto `#F4F3F0` con grises `#BDBBB6` y `#9A9893`. El cian se mantiene `#0097B8` (5,5:1 sobre tinta). El disco del símbolo pasa a papel y el botón primario a papel con texto tinta. Sombras más densas y filete interior de 1 px de luz. Icono oscuro: cuerpo `#111111` con filete `rgba(255,255,255,.16)`. Selector: `:root[data-theme=dark]`.

## 7. Tokens

Archivos: `tokens.json` (`color` = claro y oscuro con los mismos nombres de variable que `design/final/tokens.json`; `glass`, `layout`, `radius`, `space`) y `tokens.css` (`:root` = claro; `[data-theme=dark]`). La app los lleva en `app/ui/tokens.json` y `app/ui/styles.css`.

**Mapa de variables del prototipo** (las {n_proto} variables de color y cristal de `:root` y `[data-theme=dark]` de `design/final/styles.css` existen en los tokens: comprobado por script; las `--r-*` son radios y viven en `radius`):

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
"""


def fill(leg_rows, contrast_md, n_proto):
    L, D = THEME["light"], THEME["dark"]
    al, ad = acc("light"), acc("dark")
    leg = ["| Modo | px | Disco ancho | Disco alto | Marca (px) | Disco/cuerpo | Marca/disco | OK |", "|---|---|---|---|---|---|---|---|"]
    for r in leg_rows:
        leg.append(f"| {'claro' if r['modo']=='light' else 'oscuro'} | {r['px']} | {r['ancho']} | {r['alto']} | {r['marca_px']} | {r['contraste']}:1 | {r['contraste_marca']}:1 | {'sí' if r['ok'] else 'no'} |")
    d = DESIGN
    for k, v in dict(l_bg=L["bg"], d_bg=D["bg"], l_deep=L["deep"], d_deep=D["deep"], l_card=L["card"], d_card=D["card"], l_fill=L["fill"], d_fill=D["fill"],
                     l_line=L["line"], d_line=D["line"], l_ink=L["ink"], d_ink=D["ink"], l_ink2=L["ink2"], d_ink2=D["ink2"], l_ink3=L["ink3"], d_ink3=D["ink3"],
                     l_ctl=L["ctl"], d_ctl=D["ctl"], l_wv=L["wv"], d_wv=D["wv"],
                     acc_l=al["fill"], acc_d=ad["fill"], acc_tl=al["text"], acc_td=ad["text"], on_l=al["on"], on_d=ad["on"]).items():
        d = d.replace("{" + k + "}", v)
    d = d.replace("{contrast}", contrast_md).replace("{leg}", "\n".join("  " + x for x in leg)).replace("{n_proto}", str(n_proto))
    return BRAND, d
