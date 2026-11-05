"""BRAND.md y DESIGN.md (las tablas de contraste, tokens e iconos se generan desde el código)."""
from brand import *
import tokens as TK

BRAND = """# nortia. · BRAND

Sistema cerrado. Dirección A «Tipográfica pura» con el gesto del icono de C para studio. Todo lo de esta carpeta sale de `python3 gen.py`.

## 1. Propósito y personalidad

1. **Propósito:** hacer herramientas pequeñas, exactas y tranquilas para quien hace música, para que termine la canción y no pelee con el software.
2. **Confianza:** tu archivo sale como lo dejaste, sin sorpresas.
3. **Precisión:** números exactos (LUFS, kHz, bits, formatos); ningún adorno que no informe.
4. **Calma:** el producto se aparta; la marca es tinta y papel y solo se enciende donde hay significado.
5. **Carácter:** de estudio y de aparato (un piloto LED, una barra de nivel), no de corporación.

## 2. Tono de voz (3 reglas)

1. **Dato antes que adjetivo.** Di qué pasa y con qué cifra; no cómo de bueno es.
2. **Frase corta, verbo directo, tú.** Una idea por frase; sin exclamaciones, sin jerga de marketing, sin «solución», «potente» ni «revolucionario».
3. **Calma en los errores.** Di qué ha pasado, qué está a salvo y qué hacer; nunca culpes ni alarmes.

| Bien | Mal |
|---|---|
| «Tu máster está a −9,1 LUFS y pasa de 0 dB. Bájalo 1 dB y listo.» | «¡Atención! Tu máster tiene un nivel demasiado alto y podría sonar fatal.» |
| «No hemos podido guardar la carpeta. Tus archivos siguen intactos. Elige otra ubicación.» | «¡Error crítico! Algo ha fallado.» |
| «Prepara tu lanzamiento en un paso.» | «La solución todo en uno más potente para llevar tu música al siguiente nivel.» |

Nombres de producto y app: **siempre en minúsculas** (`master check`), sin punto propio. El punto es de la marca.

## 3. Arquitectura: madre + divisiones

- **nortia.** es la madre. Wordmark en tinta, punto en tinta. Sin segunda palabra y sin acento. Icono: un punto.
- **nortia. &lt;división&gt;** = el mismo wordmark + una segunda palabra en gris (Hanken Grotesk 400, `#6A6863` en claro / `#9A9893` en oscuro) + **acento propio solo en el punto**, en los estados y en el botón primario.
- Cada división tiene un color y una **transformación del punto** en su icono:

| División | Palabra | Acento (claro / oscuro) | Icono (gesto) | Estado |
|---|---|---|---|---|
| nortia. | — | tinta / papel | un punto | madre |
| nortia. studio | studio | `#FF5A00` / `#FF5A00` | punto estirado a barra vertical 2:1 («el punto se estira cuando suena») | fijado |
| nortia. lab | lab | `#2B3BFF` / `#7483FF` | dos puntos (ejemplo) | reservado, ejemplo |
| reserva 1 | p. ej. field | `#00968B` / `#1FD1C2` | punto estirado en horizontal | propuesta |
| reserva 2 | p. ej. live | `#D6137A` / `#FF52A2` | punto con anillo | propuesta |

**Cómo se nombra una división nueva**

1. Una sola palabra, minúscula, 3 a 8 letras, que diga el **oficio** (studio, lab, field), no la tecnología.
2. Se escribe `nortia. palabra`: punto, espacio, palabra. Nunca guion, nunca mayúscula, nunca dos palabras.
3. Coge el siguiente acento libre de la reserva (o propón uno nuevo y pásalo por `python3 gen.py`: el script exige AA en texto-acento, texto sobre botón y 3:1 del relleno sobre oscuro).
4. Define su gesto: una sola transformación del punto (estirar, dividir, anillar). Se dibuja con el mismo diámetro base y vive en `brand.glyph()`.
5. Añade la palabra a `WORD` y el color a `ACCENTS` en `brand.py`; el kit, los tokens y los iconos se regeneran solos.
6. Comprueba que a 16 px el icono se lee (el script lo mide) y que el acento no se confunde con los estados (error `#B42318`, aviso `#E0A100`, correcto `#1E7F4F`).

## 4. Reglas del punto

1. El punto del wordmark es **circular, dibujado a medida**, nunca el glifo de la fuente: diámetro = grosor de asta × 1,22, separado 0,075 em de la «a» y apoyado en la línea de base con 0,6 % de sobreimpresión.
2. En la madre el punto es del color del texto. En una división, es **el único sitio del logo donde vive el acento**.
3. El punto se estira solo en el **icono** de la división (studio = barra). En el wordmark no se estira nunca.
4. El punto final de un titular (`plataforma.`) es el mismo círculo y va en el acento de la división; un titular lleva un solo punto de acento.
5. El punto no se anima en bucle ni se usa como viñeta decorativa: marca estado (lectura, selección) o cierre de frase.
6. Zona de respeto: 1 diámetro de punto alrededor del wordmark. Tamaño mínimo: wordmark 64 px de ancho (pantalla) / 18 mm (impresión); símbolo 16 px.

## 5. Usos prohibidos

- Cambiar la fuente, el peso (590), el interletraje o el punto del wordmark; recomponerlo con texto vivo.
- Colorear letras o la segunda palabra con el acento; dos acentos en un mismo logo.
- Degradados, brillos, biseles, sombras en el wordmark o en el símbolo; contornos y efectos 3D.
- Poner el wordmark naranja sobre papel como **texto** (2,8:1): para texto usa tinta, y para enlaces el naranja-texto `#B84100`.
- Estirar o deformar el símbolo; rotar la barra; usar la barra en la madre o el punto simple en studio.
- Poner el logo sobre fotos sin contraste o sobre colores de otra división.
- Logos o marcas de terceros dentro de los iconos y portadas; nombres de personas en plantillas.
- Mayúsculas en `nortia.` o en nombres de app (`Master Check`, `NORTIA`).
- Usar el naranja de studio como color de error o el rojo como acento.

## 6. Archivos

`logos/` (SVG + PNG 512/2048 transparentes) · `avatares/` · `favicon/` · `banners/` · `iconos/macos` y `iconos/windows` · `plantillas/` · `kit.png` y `familia.png` · `tokens.json` / `tokens.css` · `DESIGN.md`.
"""

DESIGN = """# nortia. · DESIGN

## 1. Paleta y roles

Casi monocroma: tinta y papel; el color es una señal, no un decorado.

| Token | Claro | Oscuro | Rol |
|---|---|---|---|
| `--bg` | `{l_bg}` papel | `{d_bg}` tinta | fondo de ventana y página |
| `--bg-deep` | `{l_deep}` | `{d_deep}` | fondo hundido (en oscuro es plano, sin degradado) |
| `--card` | `{l_card}` | `{d_card}` | tarjeta, campo |
| `--fill` | `{l_fill}` | `{d_fill}` | reproductor, ficha en reposo, pastilla |
| `--line` | `{l_line}` | `{d_line}` | filetes de 1 px |
| `--ink` | `{l_ink}` | `{d_ink}` | texto principal, botón primario de la madre |
| `--ink-2` | `{l_ink2}` | `{d_ink2}` | texto secundario |
| `--ink-3` | `{l_ink3}` | `{d_ink3}` | etiquetas, palabra de división |
| `--ctl` | `{l_ctl}` | `{d_ctl}` | bordes de control y dropzone (≥3:1) |
| `--wv` / `--wv-on` | `{l_wv}` / tinta | `{d_wv}` / papel | onda sin reproducir / reproducida |
| `--brand` | acento (relleno) | acento | punto, botón primario, check, estados |
| `--brand-ink` | acento-texto AA | acento | enlaces y cifras en color |
| `--on-btn` | texto sobre `--brand` | idem | etiqueta del botón primario |

**Acentos** (solo punto, estados y botón primario):

| Marca | Relleno claro | Relleno oscuro | Texto claro (AA) | Texto oscuro | Texto sobre botón |
|---|---|---|---|---|---|
{acc_rows}

- **studio `#FF5A00`**: el naranja no llega a AA como texto sobre papel (2,82:1). **Naranja-texto oscurecido: `{orange_text}`** (mismo tono, luminosidad bajada por script hasta ≥4,6:1 sobre bg, card y fill). Úsalo para enlaces, cifras y el borde de foco. El relleno `#FF5A00` queda para punto, botón, barras y badges. Sobre botón naranja el texto es **tinta `#111111`** (6,04:1); el blanco daría 3,13:1 y no pasa.
- **lab `#2B3BFF`** es AA como texto en claro (6,0:1); en oscuro sube a `#7483FF`.
- **Reservas propuestas, compatibles con naranja y azul:** **teal `#00968B`** (tono 175°) y **magenta `#D6137A`** (tono 330°). Quedan a ≥55° de naranja y azul. Magenta está a unos 35° del rojo de error: **nunca** se usa para errores y todo estado lleva icono y palabra, no solo color.
- Estados: correcto `#1E7F4F`/`#5FCF96`, aviso `#E0A100` (texto `#7A5600`/`#F5C65A`), error `#B42318`/`#FF8A7D`. El aviso es ocre para no confundirse con el naranja de studio.
- Sin degradados en iconos, botones ni fondos de marca (evita el bandeado). Los únicos degradados son las sombras suaves (blur) y los velos `--orb-*` al 3–12 %.

### Contraste (generado por script, WCAG 2.x)

`python3 gen.py` aborta si algún par de **texto** baja de 4,5:1. Los pares **gráficos** se miden contra 3:1. El único que no llega es el relleno naranja sobre papel (2,82:1): es el color de la marca (el logo está exento) y nunca es el único portador de significado; el foco usa naranja-texto.

{contrast}

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

{leg}

  El naranja sobre papel queda en 2,82:1 como forma: el símbolo es logo (exento de 1.4.11) y a 16 px mantiene 4 × 6 px de superficie; en la versión oscura sube a 6,04:1.

## 6. Modo oscuro

Es un tema completo, no una inversión: `--bg` `#111111` plano; tarjetas `#1A1A1A`; filetes `#2B2B2B`; texto `#F4F3F0` con grises `#BDBBB6` y `#9A9893`. Los acentos se aclaran solo si hace falta (azul `#7483FF`, teal `#1FD1C2`, magenta `#FF52A2`); el naranja se mantiene. Texto sobre botón en tinta salvo magenta claro. Sombras más densas y filete interior de 1 px de luz. Iconos oscuros: cuerpo `#111111` con filete `rgba(255,255,255,.16)`. Selector: `:root[data-theme=dark]`.

## 7. Tokens

Archivos: `tokens.json` (`color` = studio claro y oscuro con los mismos nombres que `nortia-studio/design/final/tokens.json`; `brands` = madre, studio, lab, teal, magenta × claro/oscuro; `glass`, `layout`, `radius`, `space`) y `tokens.css` (`:root` = madre claro; `[data-theme=dark]`; `[data-brand=studio|lab|teal|magenta]`).

**Mapa de variables del prototipo** (las {n_proto} variables de `:root` y `[data-theme=dark]` de `styles.css` existen en los tokens: comprobado por script):

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
"""


def fill(leg_rows, contrast_md, n_proto):
    L, D = THEME["light"], THEME["dark"]
    rows = []
    for b in ("studio", "lab", "teal", "magenta"):
        a, d = acc(b, "light"), acc(b, "dark")
        rows.append(f"| {b} ({ACCENTS[b]['label']}) | `{a['fill']}` | `{d['fill']}` | `{a['text']}` | `{d['text']}` | `{a['on']}` / `{d['on']}` |")
    leg = ["| Marca | Modo | px | Ancho | Alto | Contraste | OK |", "|---|---|---|---|---|---|---|"]
    for r in leg_rows:
        leg.append(f"| {r['marca']} | {'claro' if r['modo']=='light' else 'oscuro'} | {r['px']} | {r['ancho']} | {r['alto']} | {r['contraste']}:1 | {'sí' if r['ok'] else 'no'} |")
    d = DESIGN
    for k, v in dict(l_bg=L["bg"], d_bg=D["bg"], l_deep=L["deep"], d_deep=D["deep"], l_card=L["card"], d_card=D["card"], l_fill=L["fill"], d_fill=D["fill"],
                     l_line=L["line"], d_line=D["line"], l_ink=L["ink"], d_ink=D["ink"], l_ink2=L["ink2"], d_ink2=D["ink2"], l_ink3=L["ink3"], d_ink3=D["ink3"],
                     l_ctl=L["ctl"], d_ctl=D["ctl"], l_wv=L["wv"], d_wv=D["wv"]).items():
        d = d.replace("{" + k + "}", v)
    d = d.replace("{acc_rows}", "\n".join(rows)).replace("{orange_text}", acc("studio", "light")["text"]).replace("{contrast}", contrast_md)
    d = d.replace("{leg}", "\n".join("  " + x for x in leg)).replace("{n_proto}", str(n_proto))
    return BRAND, d
