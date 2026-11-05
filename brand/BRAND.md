# nortia. · BRAND

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
