# master check · lista de pruebas del hito 2 (en tu Mac)

Para Fer. Son 12 comprobaciones, unos 20 minutos. Necesitas un Mac con macOS 14 o superior y un máster real en WAV (24 bit, 44,1 o 48 kHz, mejor de 3 a 5 minutos). Para la prueba 12 necesitas además un WAV a 192 kHz (si no tienes, expórtalo de tu DAW o pídemelo).

## Antes de empezar

1. Descarga el artefacto `master-check-macos` de la última ejecución verde de "app" en GitHub (Actions, rama `hito2-app`). Dentro hay `master check_0.1.0_universal.dmg` y `master-check-macos-app.zip`.
2. Abre el .dmg y arrastra `master check` a Aplicaciones (o descomprime el zip).
3. La app no está firmada: la primera vez macOS la bloquea. Eso se prueba en la comprobación 1.

Marca cada comprobación y, al final, responde con una línea por prueba: **funciona** o **falla: qué viste** (si puedes, adjunta captura).

---

## Comprobaciones

### 1. Abrir una app sin firmar
- Haz: en Aplicaciones, clic derecho (o Control + clic) sobre `master check` y elige **Abrir**. En el aviso, pulsa **Abrir**.
- Esperado: se abre una única ventana con una tarjeta (esquinas redondeadas) y el texto "Suelta tu canción". No aparece ningún error ni se cierra sola.
- Si no te deja: ejecuta en Terminal `xattr -dr com.apple.quarantine "/Applications/master check.app"` y vuelve a abrirla. Anótalo como "falla: necesitó xattr".

### 2. Soltar un máster real
- Haz: arrastra tu WAV encima de la ventana.
- Esperado: en menos de 5 segundos aparece el nombre de la canción, Artista y Título rellenados con lo que el archivo sugiera (o editables vacíos), y el botón **Escuchar**. Sin mensajes de error.

### 3. Onda y pastilla de volumen
- Haz: mira la onda y la pastilla de volumen; pulsa la pastilla.
- Esperado: la onda se dibuja entera, con la forma de tu canción (no una línea plana). La pastilla dice "bajo 0 dB" o "pasa de 0 dB". Al pulsarla se abre "Detalle del volumen" con Volumen, Pico más alto, Pasa de 0 dB y Formato (por ejemplo 24 bit · 48 kHz). Pulsa **Escuchar**: suena y se puede pausar.

### 4. Activar y desactivar plataformas
- Haz: apaga y enciende los chips de Plataformas (Spotify, Apple Music, Amazon, Tidal, Deezer, YouTube), Distribuidora y MP3. Pulsa **Restablecer**.
- Esperado: cada chip cambia de aspecto al pulsarlo; el resumen ("N archivos · tamaño") y el árbol "Archivos que se crearán" cambian al momento. El preajuste pasa a "Personalizado" y **Restablecer** lo devuelve a "Lo necesario". MP3 puede salir "No disponible" (todavía no llevamos lame): es lo esperado en este hito.

### 5. Cambiar la carpeta de entrega
- Haz: pulsa **cambiar** junto a la carpeta y elige una carpeta nueva (por ejemplo, Escritorio, o una con espacios y acentos en el nombre).
- Esperado: se abre el selector del sistema; al aceptar, la nueva ruta aparece en la ventana. Cancelar el selector deja la ruta anterior.

### 6. Crear la carpeta de entrega
- Haz: con Spotify y Apple Music activos, pulsa **Crear carpeta de entrega**.
- Esperado: aparece "Creando carpeta…" con progreso; termina con "Listo: N archivos en <nombre>". En la carpeta hay subcarpetas numeradas (`01 Spotify`, etc.), un FLAC por destino con nombre `Artista - Título [Spotify] 24bit-48kHz.flac` (o la frecuencia de tu máster), además de `README.txt` e `informe-loudness.txt`. Los FLAC se reproducen en un reproductor normal.

### 7. Abrir la carpeta
- Haz: pulsa **Abrir carpeta** en la pantalla final.
- Esperado: Finder se abre justo en la carpeta de entrega, con los archivos de la comprobación 6.

### 8. Cancelar a mitad
- Haz: pulsa **Nueva canción**, suelta un máster largo (cuanto más largo, mejor), activa todo y pulsa **Crear carpeta de entrega**; a los 2 o 3 segundos pulsa **Cancelar**.
- Esperado: se detiene en menos de unos segundos y muestra "Exportación cancelada." (o un resultado con solo los archivos completos). No quedan archivos a medias (sin FLAC cortados ni temporales `.part`). La app sigue respondiendo y puedes volver a crear.

### 9. Modo oscuro
- Haz: pulsa el botón claro/oscuro de la app; después cambia la apariencia del sistema (Ajustes del sistema, Apariencia) y reabre la app.
- Esperado: todos los textos se leen bien en ambos modos (sin texto oscuro sobre fondo oscuro), la onda y la pastilla mantienen contraste, la ventana conserva su forma de tarjeta sin bordes negros ni esquinas cuadradas. Con la app nueva sigue la apariencia del sistema.

### 10. Sistema en inglés
- Haz: Ajustes del sistema, General, Idioma y región: pon English como primer idioma (puedes revertirlo después), reabre la app.
- Esperado: toda la interfaz está en inglés (por ejemplo "Create delivery folder", "Open folder"), sin textos en español ni claves tipo `err.io` visibles. Los documentos de una entrega nueva salen en inglés (`README.txt` y el informe con nombre en inglés). Al volver a español, vuelve todo a español.

### 11. Tamaño mínimo de ventana
- Haz: arrastra una esquina para encoger la ventana al máximo; luego agrándala; prueba con un máster cargado y con el árbol de archivos abierto.
- Esperado: la ventana se detiene en un tamaño mínimo y no se puede reducir más. En ese tamaño no se solapan ni se cortan textos ni botones (el botón **Crear carpeta de entrega** sigue visible y pulsable). Arrastrar la ventana desde la barra superior la mueve; los botones rojo, amarillo y verde funcionan.

### 12. Un archivo de 192 kHz se rechaza
- Haz: suelta un WAV a 192 kHz sobre la ventana.
- Esperado: no se crea nada y aparece el mensaje exacto: "master check admite másters de hasta 96 kHz". La app no se bloquea y puedes soltar otro archivo sin reiniciarla. (Opcional: un archivo que no sea WAV debe mostrar "Este archivo no es un WAV válido. master check admite másters en formato WAV.")

---

## Para responder

Copia esto y complétalo:

```
1  funciona / falla: 
2  funciona / falla: 
3  funciona / falla: 
4  funciona / falla: 
5  funciona / falla: 
6  funciona / falla: 
7  funciona / falla: 
8  funciona / falla: 
9  funciona / falla: 
10 funciona / falla: 
11 funciona / falla: 
12 funciona / falla: 
Mac (chip y versión de macOS): 
```
