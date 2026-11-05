# Master Out · tres direcciones de interfaz

nortia. studio · 05/11/2026 · Maquetas HTML/CSS 1440×900 pensadas como base real de la interfaz Tauri 2. Datos de ejemplo: "Luz de Noviembre", Vera Alcón, 24 bit / 96 kHz, −9,1 LUFS, +1,2 dBTP (rojo en Spotify y Apple). Las cifras por destino son de ejemplo, no presets verificados.

## Qué hay

| Dirección | Archivo | Estados (PNG) |
|---|---|---|
| D1 Mesa de trabajo | `d1.html` | `d1_claro.png`, `d1_oscuro.png`, `d1_claro_exporting.png`, `d1_claro_terminado.png` |
| D2 Un solo flujo | `d2.html` | `d2_claro.png` (paso 3), `d2_oscuro.png`, `d2_claro_exporting.png`, `d2_claro_paso1.png`, `d2_claro_paso2.png`, `d2_claro_paso4.png` |
| D3 Tarjetas vivas | `d3.html` | `d3_claro.png`, `d3_oscuro.png`, `d3_claro_exporting.png`, `d3_claro_terminado.png` |
| Lámina única | `comparativa_direcciones.png` | las tres en claro, etiquetadas |

Querystring (todas): `?theme=dark|light` y `?state=exporting|done`; además `?step=1..4` en D2, `?open=<destino>` y `?drawer=1` en D3, `?still=1` desactiva animaciones (se usa para los PNG). Hay un botón de tema en la barra de título. Sin querystring son interactivas de verdad: los interruptores actualizan el árbol en vivo (las filas nuevas entran con animación), Exportar lanza una exportación simulada de ~8 s con progreso por archivo y termina con la celebración sobria (el norte marcado se ensambla y caen pocos rombos en ámbar, azul noche y verde azulado). La fuente de las páginas está en `_src/` (`common.css`, `common.js`, `dN.src.html`, `build.py`); `python3 _src/build.py` regenera los HTML.

Decisiones comunes: el semáforo nunca depende solo del color (forma + texto: círculo con check, triángulo, círculo con cruz); el ámbar de la marca solo aparece en el símbolo y en los avisos; el verde azulado #0E5E63 (provisional) es el acento de studio en interruptores, onda reproducida y progreso; el botón primario es azul noche (en oscuro se invierte a claro). Las marcas de destino son monogramas neutros, no logos de terceros. Texto de ámbar sobre claro usa #855600 para cumplir contraste.

## Tabla comparativa

| Criterio | D1 Mesa de trabajo | D2 Un solo flujo | D3 Tarjetas vivas |
|---|---|---|---|
| Velocidad para el experto | Muy alta: todo visible, 2 clics de principio a fin | Media: 4 pantallas y "Siguiente" aunque no haga falta | Alta: onda y destinos juntos; la tarjeta expandida añade un clic opcional |
| Claridad para el novato | Media: mucha información a la vez | Muy alta: una decisión por pantalla, textos que guían | Alta: la onda y el rojo/ámbar/verde explican solos; el cajón de carpeta se descubre menos |
| Esfuerzo en Tauri | Bajo: CSS grid fijo, un solo estado | Medio: máquina de pasos, transiciones, estado entre pasos | Medio-alto: grid denso con tarjetas que cambian de tamaño (View Transitions con respaldo sin animar), cajón y filtros |
| Riesgo de parecer genérica | Medio-alto: es el patrón clásico de tres paneles; se salva con medidores y árbol | Medio: un asistente por pasos es común; lo distingue el marcador del norte marcado | Bajo: cabecera tipo instrumento en azul noche, onda grande con picos en rojo, tarjetas con borde discontinuo para "apagado" |
| Escala a más destinos | Regular: cada destino nuevo suma altura; a partir de ~16 hay que hacer scroll en la columna central | Buena: las pestañas por grupo absorben cualquier cantidad | Buena: la rejilla crece por filas y ya hay filtros por grupo; con >20 pide scroll |
| Escala a Windows | Buena: sin dependencias de chrome nativo | Buena | Buena; el botón Exportar vive en la barra de título, que en Windows tendría que ir en una barra propia bajo el marco |

## Pros y contras

**D1 Mesa de trabajo**
- Pros: es la spec tal cual; el experto ve máster, destinos y carpeta a la vez; el árbol en vivo es el centro de la columna derecha; la más barata de construir.
- Contras: densa (la columna central roza el límite de 700 px de alto mínimo); el novato ve de golpe 13 tarjetas; es la que menos "personalidad" tiene sin los medidores.

**D2 Un solo flujo**
- Pros: la más amable; el aviso "2 destinos no cumplen" llega en el paso en que importa; el resumen final es muy tranquilizador; el marcador de progreso con el símbolo es un detalle de marca memorable.
- Contras: obliga a pasar por cuatro pantallas siempre, y el experto repite el mismo lote cada semana; por pestañas no se ven a la vez todos los destinos; el árbol solo aparece en dos pasos.

**D3 Tarjetas vivas**
- Pros: la mejor primera impresión y la más propia: la onda con los picos en rojo cuenta el problema antes de leer; la tarjeta expandida enseña archivo, motivo y medidor de true peak en el mismo sitio; el estado exportando se entiende de un vistazo (cada tarjeta es su propia barra).
- Contras: la carpeta queda fuera de vista en el estado normal (carril de 46 px); cabecera de 262 px que come altura en pantallas de 700; mayor esfuerzo y más casos límite (tarjeta expandida + cajón abierto).

## Crítica de diseño (resumen por dirección)

**D1.** Primera impresión: herramienta seria y precisa. Lo mejor: medidores con referencia (−14 LUFS, −1 dBTP) y la fila del árbol que se rellena al exportar. Hallazgos: 🟡 columna central al límite de altura (corregido compactando tarjetas; sin margen para más destinos); 🟡 nombres del árbol truncados por la izquierda ("Vera Alcón - Luz de…"), la plataforma se mantiene legible; 🟢 la columna izquierda deja aire vacío bajo los medidores. Contraste: texto secundario #4A5568 sobre #F7F8FA ≈ 7:1; objetivos táctiles de 27–32 px, correctos en escritorio.

**D2.** Primera impresión: tranquila y acompañada. Lo mejor: jerarquía muy clara (titular 28 px, una acción primaria por pantalla). Hallazgos: 🟡 el paso 3 con pestañas oculta los destinos de otros grupos (el contador 4/6 lo compensa); 🟡 los pasos 1 y 2 dejan la mitad inferior vacía, es deliberado pero se ve poco aprovechado en 1440×900; 🟢 el "Siguiente" repite el resumen del pie en todos los pasos.

**D3.** Primera impresión: la más atractiva; la cabecera oscura ancla la vista en la onda y los picos rojos. Hallazgos: 🟡 en modo exportando la rejilla pasa a 3 columnas y el motivo se recorta a una línea (se ve completo al terminar); 🟡 el texto del carril vertical es difícil de leer para quien no lo ha abierto antes; 🟢 el borde discontinuo para "desactivado" es nuevo y se entiende, pero conviene comprobarlo con usuarios daltónicos junto al texto "Aviso/Cumple". Contraste en cabecera: texto #E9EDF5 sobre #0F1F3D > 13:1.

## Recomendación

D3 como dirección principal, con una concesión de D1: dejar el cajón de carpeta abierto por defecto cuando la ventana mida 1280 px o más, de modo que el árbol en vivo (el diferenciador del producto según la spec) se vea siempre. D3 es la única que no parece un panel de ajustes: la onda con picos en rojo y las tarjetas que cambian de estado son la marca "software de confianza" hecha visible, y escala bien a más destinos con los filtros. D1 es el plan B si el coste de Tauri se dispara (es la más barata y ya es válida), y de D2 conviene tomar solo el texto guía del aviso para el primer arranque (un recorrido de un solo uso, no la estructura). Antes de construir: probar la rejilla en 1100×700 (la ventana mínima de la spec), donde la cabecera habrá que reducirla a unos 200 px, y confirmar el color de studio, que hoy solo afecta a interruptores, onda y progreso.
