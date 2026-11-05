# nortia. · DESIGN

Todo se regenera con `python3 gen.py`. Los valores salen de `core.py` (paleta) y el contraste se verifica con `contrast.py`. Nota de método: no se pudo consultar el catálogo UXKIN; la dirección parte de la ronda anterior (/home/claude/marca/nortia-labs/) y de criterio propio; no se copia ninguna marca.

## 1. Símbolo y logo
- Símbolo **N2 «norte marcado»** (aprobado): rombo vertical 20 × 60 u en viewBox 64, mitad superior ámbar y mitad inferior en el color de la marca, **hueco de 2 u en la cintura** (y 31 a 33). Triángulos con vértices en (32,2), (22,31)/(42,31) y (22,33)/(42,33), (32,62).
- Wordmark: Inter SemiBold minúsculas «nortia» + «.» (ámbar), a trazados con kerning GPOS de Inter, tracking −0,015 em. Submarca: espacio 0,2 em + «studio» en Inter Regular.
- Lockup horizontal: símbolo de 64 u de alto, separación 14 u, cuerpo de texto 46 u (altura de x 25 u), centrado ópticamente con +0,5 u. Vertical: símbolo, 20 u de aire, wordmark de 30 u centrado.
- Zona de respeto 0,5 × altura del símbolo. Mínimos: horizontal 120 px, vertical 80 px, símbolo 16 px.
- Tonos: `color` (sobre #F7F8FA/blanco), `reverse` (sobre azul noche: texto blanco, cuerpo claro, ámbar intacto), `negro`, `blanco` (una tinta; el hueco se mantiene).
- Avatar: cuadrado azul noche, radio 22 % del lado, aguja al 80 % del lado, cuerpo en #F7F8FA (madre) o en la variante clara de la submarca, punta ámbar; trazo más grueso que el símbolo suelto (rombo de 24 u). Favicon: misma pieza, aguja al 88 % (16/32 px) y 82 % (48 px). Apple-touch: cuadrado completo sin radio (iOS aplica su máscara), aguja al 64 %.
- Icono macOS: lienzo 1024, plancha de 824 centrada (margen 100), superelipse n = 5 (aproximación de la esquina continua de las HIG), sombra suave de 12 px; claro (#F7F8FA, cuerpo del color de la submarca) y oscuro (azul noche, cuerpo en variante clara). Aguja = 62 % del lado de la plancha. Windows: cuadrado azul noche, esquinas 18 %, aguja al 72–80 %, `.ico` con 16, 24, 32, 48, 64, 128 y 256.
- Archivos: `logos/svg|png` (`nortia-{simbolo,wordmark,horizontal,vertical}-{color,negro,blanco,reverse}`, `studio-{verde,grafito}-{simbolo,horizontal,vertical}-{color,reverse}`, `studio-*-{negro,blanco}`; PNG a 512 y 2048 de lado mayor, fondo transparente), `avatares/`, `favicon/`, `banners/`, `iconos/macos|windows`, `plantillas/`.

## 2. Color
| Rol | Token | Hex | Uso |
|---|---|---|---|
| Marca / texto | `--navy` | #0F1F3D | logo, titulares, texto, botón primario de la madre, avatar |
| Primario hover | `--navy-hover` | #1B2F57 | hover del botón navy |
| Acento | `--amber` | #BF7F12 | **solo** punta del símbolo y punto del wordmark |
| Texto de ámbar | `--amber-text` | #8A5A00 | si un aviso necesita ámbar como texto |
| Fondo | `--paper` | #F7F8FA | página |
| Superficie | `--white` | #FFFFFF | tarjetas, inputs, tablas |
| Texto secundario | `--ink-2` | #4A5568 | cuerpo secundario, cabeceras de tabla |
| Texto terciario | `--ink-3` | #5F6B7E | notas, placeholders |
| Borde | `--border` | #D5DBE5 | divisores, tarjetas (decorativo) |
| Borde de control | `--control` | #808CA3 | inputs, botón secundario (≥ 3:1) |
| Anunciado | `--ann` / `--ann-bg` | #1F4E8C / #E6EEF9 | pastilla «Anunciado» |
| Vigente (éxito) | `--ok` / `--ok-bg` | #146C43 / #E6F4EC | estados correctos |
| Aviso | `--warn` / `--warn-bg` | #8A5A00 / #FBF1DC | advertencias |
| Error | `--err` / `--err-bg` | #B42318 / #FDECEA | errores |
| Fondo oscuro | `--night` | #0A1429 | modo oscuro (superficie elevada #0F1F3D) |
| Texto oscuro | `--d-text` / `--d-text-2` | #E8ECF3 / #A3AEC2 | principal / secundario en oscuro |
| Borde oscuro | `--d-border` | #26385A | divisores en oscuro |

Sin degradados. Los colores de estado nunca van solos: siempre con texto.

### Submarca nortia. studio: dos opciones
| | **A · Verde azulado** (recomendada) | **B · Grafito cálido** |
|---|---|---|
| Principal | #0E5E63 | #45413C |
| Claro (sobre oscuro) | #5BB8BC | #CFC8BE |
| Hover | #0B4A4E | #33302C |
| Tinte | #E3F1F1 | #ECEAE6 |
| Cuerpo del símbolo | principal / claro | principal / claro |

**Decisión sobre la punta: sigue ámbar en ambas.** Es la constante de familia (misma «aguja» en todas las submarcas), el color complementario del verde azulado y del grafito cálido, y mantiene la regla «ámbar = norte». Solo cambian el cuerpo del símbolo y la palabra «studio». La separación de 2 u evita que ámbar y color de submarca se toquen (ámbar sobre #0E5E63 = 2,24:1 si se tocaran).

**Recomendación: A (verde azulado).** (1) Se distingue de la madre por tono, no solo por luminosidad: grafito sobre azul noche es 1,62:1 y se lee «casi lo mismo»; (2) en oscuro sigue teniendo identidad (#5BB8BC) mientras el grafito claro se vuelve un beige neutro; (3) el audio profesional suele ir en grises oscuros (DAW); el verde azulado separa de la categoría sin perder seriedad; (4) 7,06:1 sobre el fondo. Riesgo de A: tono frecuente en tecnología sanitaria/fintech; B es la apuesta más discreta si se prioriza fundirse con el entorno de estudio. Ver `submarcas.png`.

### Contraste (script `contrast.py`, WCAG 2.1; texto ≥ 4,5, gráfico/UI ≥ 3)
<!--CONTRAST-->
| Grupo | Par | Ratio | Umbral | AA |
|---|---|---|---|---|
| Claro | Texto principal #0F1F3D sobre fondo (#0F1F3D / #F7F8FA) | 15.39 | 4.5 | sí |
| Claro | Texto principal sobre superficie blanca (#0F1F3D / #FFFFFF) | 16.36 | 4.5 | sí |
| Claro | Texto secundario sobre fondo (#4A5568 / #F7F8FA) | 7.08 | 4.5 | sí |
| Claro | Texto secundario sobre blanco (#4A5568 / #FFFFFF) | 7.53 | 4.5 | sí |
| Claro | Texto terciario sobre blanco (#5F6B7E / #FFFFFF) | 5.40 | 4.5 | sí |
| Claro | Botón primario: blanco sobre azul noche (#FFFFFF / #0F1F3D) | 16.36 | 4.5 | sí |
| Claro | Botón primario hover: blanco sobre #1B2F57 (#FFFFFF / #1B2F57) | 13.20 | 4.5 | sí |
| Claro | Ámbar #BF7F12 sobre blanco (gráfico: punta/punto) (#BF7F12 / #FFFFFF) | 3.35 | 3 | sí |
| Claro | Ámbar sobre fondo #F7F8FA (gráfico) (#BF7F12 / #F7F8FA) | 3.16 | 3 | sí |
| Claro | Ámbar sobre azul noche (gráfico, logo en negativo) (#BF7F12 / #0F1F3D) | 4.88 | 3 | sí |
| Claro | Ámbar como texto: #8A5A00 sobre blanco (#8A5A00 / #FFFFFF) | 5.93 | 4.5 | sí |
| Claro | Pastilla Anunciado (#1F4E8C / #E6EEF9) | 7.11 | 4.5 | sí |
| Claro | Pastilla Vigente / éxito (#146C43 / #E6F4EC) | 5.69 | 4.5 | sí |
| Claro | Pastilla Aviso (#8A5A00 / #FBF1DC) | 5.28 | 4.5 | sí |
| Claro | Error sobre blanco (#B42318 / #FFFFFF) | 6.57 | 4.5 | sí |
| Claro | Error sobre fondo de error #FDECEA (#B42318 / #FDECEA) | 5.75 | 4.5 | sí |
| Claro | Borde de control #808CA3 sobre blanco (inputs, botón secundario) (#808CA3 / #FFFFFF) | 3.39 | 3 | sí |
| Studio A | Verde azulado sobre fondo (texto «studio») (#0E5E63 / #F7F8FA) | 7.06 | 4.5 | sí |
| Studio A | Verde azulado sobre blanco (#0E5E63 / #FFFFFF) | 7.50 | 4.5 | sí |
| Studio A | Botón: blanco sobre verde azulado (#FFFFFF / #0E5E63) | 7.50 | 4.5 | sí |
| Studio A | Botón hover: blanco sobre #0B4A4E (#FFFFFF / #0B4A4E) | 9.98 | 4.5 | sí |
| Studio A | Verde azulado sobre tinte #E3F1F1 (#0E5E63 / #E3F1F1) | 6.47 | 4.5 | sí |
| Studio A | Verde claro #5BB8BC sobre azul noche (#5BB8BC / #0F1F3D) | 7.03 | 4.5 | sí |
| Studio A | Verde claro sobre fondo oscuro #0A1429 (#5BB8BC / #0A1429) | 7.88 | 4.5 | sí |
| Studio B | Grafito cálido sobre fondo (texto «studio») (#45413C / #F7F8FA) | 9.53 | 4.5 | sí |
| Studio B | Grafito cálido sobre blanco (#45413C / #FFFFFF) | 10.12 | 4.5 | sí |
| Studio B | Botón: blanco sobre grafito (#FFFFFF / #45413C) | 10.12 | 4.5 | sí |
| Studio B | Botón hover: blanco sobre #33302C (#FFFFFF / #33302C) | 13.13 | 4.5 | sí |
| Studio B | Grafito sobre tinte #ECEAE6 (#45413C / #ECEAE6) | 8.43 | 4.5 | sí |
| Studio B | Grafito claro #CFC8BE sobre azul noche (#CFC8BE / #0F1F3D) | 9.86 | 4.5 | sí |
| Studio B | Grafito claro sobre fondo oscuro #0A1429 (#CFC8BE / #0A1429) | 11.06 | 4.5 | sí |
| Oscuro | Texto principal #E8ECF3 sobre #0A1429 (#E8ECF3 / #0A1429) | 15.48 | 4.5 | sí |
| Oscuro | Texto secundario #A3AEC2 sobre #0A1429 (#A3AEC2 / #0A1429) | 8.20 | 4.5 | sí |
| Oscuro | Texto secundario sobre superficie #0F1F3D (#A3AEC2 / #0F1F3D) | 7.32 | 4.5 | sí |
| Oscuro | Botón primario: azul noche sobre blanco (#0F1F3D / #FFFFFF) | 16.36 | 4.5 | sí |
| Oscuro | Ámbar sobre #0A1429 (gráfico) (#BF7F12 / #0A1429) | 5.47 | 3 | sí |
| Oscuro | Borde de control #808CA3 sobre #0A1429 (UI) (#808CA3 / #0A1429) | 5.41 | 3 | sí |
| Oscuro | Éxito: #6FD3A0 sobre #0A1429 (#6FD3A0 / #0A1429) | 10.04 | 4.5 | sí |
| Oscuro | Error: #FF8A80 sobre #0A1429 (#FF8A80 / #0A1429) | 8.03 | 4.5 | sí |
<!--/CONTRAST-->

## 3. Tipografía
Inter (Google Fonts; OTF local para generar logos). Solo sans. Pesos: 400, 500, 600 (700 no se usa). Cifras tabulares (`font-variant-numeric: tabular-nums`) en precios y tablas. Sentence case; sin versalitas.

| Nivel | Tamaño/interlínea | Peso | Tracking |
|---|---|---|---|
| Display | 56/60 | 600 | −0,025 em |
| H1 | 40/46 | 600 | −0,02 em |
| H2 | 28/36 | 600 | −0,015 em |
| H3 | 20/28 | 600 | −0,01 em |
| Lead | 18/28 | 500 | 0 |
| Cuerpo | 16/26 | 400 | 0 |
| Secundario | 14/22 | 400 | 0 |
| Nota | 12/18 | 500 | 0 |

Líneas de 60–72 caracteres; cuerpo ≥ 16 px en web, 11 pt en PDF. **Wordmark:** tracking −0,015 em (≈ −0,7 u en cuerpo 46), kerning GPOS real de Inter, «nortia» 600 y «studio» 400.

## 4. Espaciado, radios, sombras
- Base 4: 4, 8, 12, 16, 24, 32, 48, 64, 96. Contenido máx. 1120 px; secciones 96 px (64 en móvil); gutter móvil 16 px.
- Radios: 4 (chips pequeños), 6 (botones, campos), 8 (tarjetas, ventanas de producto), 999 (pastillas). Avatar 22 % del lado; icono Windows 18 %.
- Sombras: ninguna por defecto; se separa con borde de 1 px. Menús flotantes: `0 4px 12px rgba(15,31,61,.08)`. Icono macOS: sombra del sistema (y +12, desenfoque 14, 28 %).

## 5. Componentes
- **Botón primario:** madre fondo #0F1F3D; submarca fondo = color principal (A #0E5E63, B #45413C); texto blanco 600 14 px; alto 40, radio 6; hover color de hover; foco: anillo 2 px del mismo color con separación 2 px; deshabilitado fondo #E9ECF2, texto #5F6B7E. Sobre oscuro: fondo blanco, texto #0F1F3D.
- **Botón secundario:** fondo blanco, borde 1,5 px #808CA3, texto #0F1F3D; hover fondo #F7F8FA.
- **Tarjeta:** blanco, borde 1,5 px #D5DBE5, radio 8, padding 20–28, sin sombra; portada de producto en azul noche con símbolo fantasma (#14274A), nunca ilustración ni foto de stock.
- **Tabla:** sin cebra; cabecera #F7F8FA 12 px #4A5568; filas con línea inferior #D5DBE5; cifras tabulares, a la derecha; celda de estado con pastilla.
- **Pastillas de estado:** radio 999, 12–13 px/500, padding 4×12. Vigente #146C43 sobre #E6F4EC; Anunciado #1F4E8C sobre #E6EEF9; Aviso #8A5A00 sobre #FBF1DC; Error #B42318 sobre #FDECEA. Siempre con texto.
- **Campos:** alto 40, borde 1,5 px #808CA3, radio 6, etiqueta 14/500 encima, ayuda 12 #5F6B7E; foco anillo como el botón; error: borde #B42318 + mensaje con texto.
- **Enlaces:** #0F1F3D subrayado (madre) o color de submarca subrayado; hover más oscuro y subrayado grueso.

## 6. Iconografía
Una sola familia, geometría de la aguja: cuadrícula 24 px, **trazo 1,5 px**, remates **rectos (butt)**, uniones **angulares (miter)**, esquinas con radio máximo 1 px, sin relleno salvo estados activos (relleno plano de la misma tinta). Color = texto (#0F1F3D / #4A5568); nunca ámbar. Sin emojis ni iconos de terceros mezclados. Tamaños 16, 20, 24; zona táctil 40 px.

## 7. Modo oscuro
- Fondo #0A1429; superficies #0F1F3D (borde #26385A); texto #E8ECF3 y #A3AEC2; bordes de control #808CA3.
- Logo: variante `reverse` (texto blanco, cuerpo #F7F8FA en madre, variante clara en submarcas, ámbar intacto).
- Botón primario: blanco con texto #0F1F3D; secundario: borde #808CA3, texto #E8ECF3. Color de submarca en texto/enlaces: su variante clara (#5BB8BC o #CFC8BE).
- Estados: éxito #6FD3A0 y error #FF8A80 como texto sobre #0A1429; las pastillas usan fondos del mismo tono al 16 % de opacidad.
- Sin sombras; la profundidad se da con bordes y superficies más claras.

## 8. Plantillas (en `plantillas/`)
Portada de producto 1280×720 y 600×600 (azul noche, título 80/60 px, pastillas de plataforma); A4 en PDF con cabecera (logo + «Documento»), filete, tabla, nota y pie con paginación (texto a trazados: es una plantilla de maquetación; reescribe el texto en tu editor con Inter); firma de correo HTML con tabla y estilos en línea (sube `firma-logo-*.png` y cambia el `src`; los corchetes se rellenan, sin nombres de personas); diapositiva 16:9 1920×1080. Versiones madre y studio (A) generadas; para B cambia `DEFAULT_SUB` en `core.py`.
