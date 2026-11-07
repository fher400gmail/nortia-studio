# ferboix. studio · BRAND

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
