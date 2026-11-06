# Marcas de plataformas en master check · investigación y recomendación

Fecha de la investigación: 2026-11-06. Estado: **investigación técnica, no es asesoramiento legal**; confirmar con el asesor antes de vender. Solo se leyeron páginas públicas; no se descargó ningún activo.

## 0. Alcance y límites de la fuente

Cada plataforma publica guías para cosas distintas (desarrolladores de API, artistas que promocionan su música, anunciantes, mastering houses). **Ninguna tiene una guía pública pensada para «una herramienta de terceros que prepara archivos para esa plataforma»**; hay que inferir por analogía. Lo que no se pudo leer queda marcado como `NO VERIFICADO`.

| Fuente | Estado |
|---|---|
| Spotify Design & Branding Guidelines | Leída |
| Apple Digital Masters (página del badge) | Leída |
| Apple Marketing Resources and Identity Guidelines (App Store; reglas generales de marca de Apple) | Leída |
| Apple Music Identity Guidelines (`marketing.services.apple/apple-music-identity-guidelines`, antes `apple.com/itunes/marketing-on-music/identity-guidelines.html`) | **No cargó**: la herramienta no obtuvo permiso para esa URL. Solo se conoce por el resumen de `itunespartner.apple.com/music/support/5237-promote-your-release` |
| `apple.com/itunes/link/` | No cargó (mismo motivo) |
| YouTube API Services Branding Guidelines | Leída |
| `brand.youtube` y `youtube.com/yt/brand/using-logo.html` (tamaños mínimos, espacio) | `brand.youtube` devuelve solo metadatos (sitio dinámico). **Tamaños y espacio de YouTube: NO VERIFICADO** |
| Amazon Music | **No existe guía pública de marca de Amazon Music que se haya encontrado.** Se leyeron las guías de marca Amazon para anuncios y para afiliados (aplican a la marca Amazon, no específicamente a Music). `developer.amazon.com/support/legal/wwa-trademark-usage-guidelines`: sin permiso para leerla |
| TIDAL | **No se encontró brand kit público.** Solo los Términos de uso (`tidal.com/terms`) |
| Deezer | **No se encontró brand kit público.** `business.deezer.com/partner-distribution/` no muestra reglas de logo |
| TuneCore | **No se encontró guía de marca.** Solo registros de marca (Justia / USPTO) |

Marcas no verificadas se tratan como **«exigen permiso»** (hipótesis conservadora).

## 1. Hallazgo transversal

1. **Nombre en texto frente a logo.** Todas las guías encontradas separan dos cosas: *mencionar* la marca como texto (uso referencial, permitido con la ortografía correcta) y *mostrar su logo* (licencia, condiciones o permiso). Apple lo dice explícito: usar «works with» / «compatible with» en vez de prefijos o posesivos.
2. **Ninguna autoriza imitar** el aspecto del logo. Spotify: el logo de la app «no debe incluir ni parecerse al logo de Spotify ni a ninguno de sus elementos de marca». YouTube prohíbe marcas o diseños «confusamente similares» e imitar su *trade dress*. Es justo el problema de los glifos actuales (ondas tipo Spotify, «a» con sonrisa tipo Amazon, rombos tipo Tidal, play tipo YouTube): **el glifo imitativo no está cubierto por ningún permiso y es el peor caso**.
3. **El riesgo real no es el logo, es el aval.** Todas prohíben sugerir patrocinio, asociación o aprobación.

## 2. Plataforma por plataforma

Formato: (1) ¿sin acuerdo? · (2) forma · (3) color · (4) tamaño/espacio · (5) texto · (6) botón/icono de app · (7) descarga · (8) riesgo de aval.

### 2.1 Spotify

Fuente: <https://developer.spotify.com/documentation/design>

1. **Condicionado.** No hay programa de partners ni aprobación aparte; el uso se rige por los Developer Terms: «By using these resources, you accept our Developer Terms of Service» (la página no hace un resumen literal; síntesis de la lectura). Pero las reglas están escritas para **apps que usan la API/metadatos de Spotify**. master check no usa API de Spotify, así que no hay vínculo contractual que cubra el logo. Paráfrasis de la guía: una app puede decir que es «para Spotify», pero no puede sugerir aval.
2. **Logo completo (icono + wordmark)** en integraciones de partner; el icono solo cuando falta espacio o la marca ya está establecida. «The wordmark should never exist without the icon».
3. Verde solo sobre fondo negro o blanco; en cualquier otro fondo, logo monocromo (negro sobre claro, blanco sobre oscuro). «Don't introduce new colors outside our brand palette».
4. Logo ≥ 70 px (20 mm); icono ≥ 21 px (6 mm). Espacio de respeto = mitad de la altura del icono.
5. Texto: obligatorio atribuir con el logo cuando se muestra contenido/metadatos de Spotify (no es nuestro caso). Textos de enlace permitidos: «PLAY ON SPOTIFY», «LISTEN ON SPOTIFY», «OPEN SPOTIFY», «GET SPOTIFY FREE». Prohibido: nombre de app que incluya «Spotify» o suene parecido; implicar respaldo.
6. Prohibido «usar el logo para hacer nuevos objetos o formas», rotarlo o deformarlo, ponerlo sobre portadas, o combinarlo con otra marca (co-branding). Solo permitido el icono como icono de app **de Spotify** en dispositivo. Un botón con el logo no se menciona; el icono de app propio «no debe parecerse».
7. Descarga: `https://developer.spotify.com/images/guidelines/design/2024-spotify-full-logo.zip` y `.../2024-spotify-logo-icon.zip` (ZIP; SVG/PNG, no verificado el contenido exacto).
8. **Riesgo medio.** «Spotify ready», «certified for Spotify» o «optimized for Spotify» no figuran como permitidos y rozan el aval. Texto seguro: «Archivos para Spotify» / «Spotify: −14 LUFS» (referencia factual).

### 2.2 Apple Music / Apple Digital Masters

Fuentes: <https://apple.com/apple-music/mastered> · <https://developer.apple.com/app-store/marketing/guidelines> · <https://itunespartner.apple.com/music/support/5237-promote-your-release> · (bloqueada) <https://marketing.services.apple/apple-music-identity-guidelines>

**Apple Digital Masters (badge):**
1. **No.** El badge es para «approved mastering houses or other third parties expressly authorized by Apple» (paráfrasis de la página). Descargarlo exige aceptar una licencia y confirmar que eres representante autorizado de una mastering house aprobada. Exige contacto con `appledigitalmasters@apple.com` para otros usos. master check **no puede** usar el badge.
2–4. (Solo informativo) Badge sin alterar, sin recolorear, sin animar, sin añadir texto; altura mínima en impresión 8 mm; espacio de respeto = 1/4 de su altura; un badge por página; más pequeño que el nombre de la empresa. Apple puede retirar el permiso en cualquier momento (aviso de 10 días; 30 para retirarlo).
5–6. Licencia: no puede usarse de forma que sugiera que Apple te respalda; no se pueden registrar marcas confusamente similares.
7. ZIP ~1,5 MB tras aceptar licencia (localizado). No se descarga.
8. **Riesgo alto.** «Apple Digital Masters» es un programa con requisitos y una marca protegida. La app **no emite** ADM ni «certifica» nada: hace una pre-comprobación local con las herramientas de Apple (`afconvert` + `afclip`).

**Apple Music (logo, icono, badge «Listen on Apple Music»):**
1. **Condicionado / probablemente solo para quien promociona su propia música.** `itunespartner` ofrece badges y el icono de Apple Music dentro de «Apple Music Marketing Tools» para «promover tus lanzamientos»; la página no define quién es elegible (resumen de la herramienta). Las reglas generales de Apple (App Store): licencia «limited, non-exclusive, non-transferable, royalty-free» ligada al Apple Developer Program; «Use only the badge artwork provided»; «Don't use icons, logos, graphics, or images from www.apple.com to promote your app». Guía completa de Apple Music **no leída**: tratar como que **exige** aceptar la licencia de los marketing tools y que no cubre a una herramienta de terceros.
2–4. `NO VERIFICADO` para Apple Music. Referencia general de Apple (badges): altura mínima 40 px en pantalla; espacio = 1/4 de la altura; subordinado al mensaje principal; no modificar, inclinar ni animar.
5. Apple recomienda forma referencial: «*app name* for iPhone» o «works with» / «compatible with»; prohíbe el logo de Apple en lugar de la palabra Apple y escribir marcas en mayúsculas; mantener la ortografía («Apple Music»).
6. Prohíbe usos que sugieran asociación con Apple. Ejemplo de la guía: «Apple Vision Pro» no puede estar en el nombre de una app.
7. Marketing tools de Apple Music (artistas) y `marketing.services.apple` (bloqueado). Formatos: no verificados.
8. **Riesgo alto** con «Apple Digital Masters» como nombre de ficha, de archivo (`[Apple Digital Masters] 24bit-96kHz.wav`) o con «validado/certificado». Ver §5.

### 2.3 YouTube / YouTube Music

Fuentes: <https://developers.google.com/youtube/terms/branding-guidelines> · <https://brand.youtube> (sin detalle accesible)

1. **Condicionado.** «No special approval» para usar las APIs, pero **está pensado para apps que integran el API de YouTube**. Se requiere **autorización escrita** para cualquier uso que implique asociación o aprobación («written permission is required»). master check no integra YouTube.
2. Tres logos: «Developed with YouTube» (app depende de YouTube), logo YouTube (una función que usa/sube a YouTube) e icono YouTube (atribución en una bandeja de iconos sociales o con poco espacio). «Solo puedes usar los logos mostrados». Ninguno encaja con «preparado para YouTube».
3. **No se pueden cambiar los colores** de logos e iconos de YouTube; fondo sólido; contraste suficiente. Las versiones blanco y negro son para marca de agua. («Developed with YouTube» sí admite un solo color.)
4. Tamaños mínimos y espacio: `NO VERIFICADO` (la guía remite a `youtube.com/yt/brand/using-logo.html`). «No hagas el logo el elemento más destacado de la página».
5. Texto: no se exige. Prohibido: «YouTube», «YT» o variantes en el nombre de la app; no emparejar imágenes de marca con el nombre o descripción de la app.
6. El logo mostrado debe **enlazar a YouTube** o a un componente de YouTube en la app. No debe distorsionarse, cubrirse ni imitarse (trade dress).
7. <https://www.youtube.com/yt/brand/downloads.html> (PNG, AI, EPS; «Developed with YouTube» en claro y oscuro).
8. **Riesgo medio-alto.** YouTube dice que pueden pedirte que cambies la app o dejes de usar los logos. «Optimizado para YouTube» sugiere asociación. El triángulo de play genérico por sí solo es un símbolo común, pero junto a la palabra YouTube y en rojo/blanco es imitación.

### 2.4 Amazon Music

Fuentes: <https://advertising.amazon.com/resources/ad-policy/brand-usage> · <https://affiliate-program.amazon.com/help/operating/amazonmarks/> (guías de marca Amazon generales, **no** de Amazon Music).

1. **No / permiso escrito.** Para publicidad: «Other uses of the Amazon brand require review and written approval»; cualquier uso del logo de Amazon «must be approved and reviewed in all cases». Las marcas de afiliados se licencian «ONLY… to advertise availability of products on an Amazon site», revocable e intransferible. Nada cubre una herramienta de terceros. Guía propia de Amazon Music: `NO ENCONTRADA`.
2. `NO VERIFICADO` para Amazon Music.
3. Prohibido alterar proporciones, color o tipografía. No escribir «Amazon» todo en mayúsculas o minúsculas.
4. Espacio: «reasonable spacing»; fondo que no dañe la legibilidad. Tamaño: no verificado.
5. Prohibido: usar las marcas para implicar respaldo; registrar dominios, usuarios o nombres de app confusamente similares. Iconos del sitio (carrito, estrella, hoja) no sin aprobación.
6. Prohibido: botones con marca Amazon como CTA, salvo e-commerce; la «sonrisa» (flecha a-z) es elemento de marca → **la «a» con sonrisa actual es el elemento más delicado de todos**.
7. Aprobación vía representante de cuenta o equipo de marcas. Sin descarga pública para este uso.
8. **Riesgo alto** si se muestra algo que parezca el logo.

### 2.5 TIDAL

Fuente: <https://tidal.com/terms> (no hay brand kit público encontrado).

1. **No / permiso escrito.** Cita: «TIDAL's prior written permission specific for each such use». «All goodwill… inures to TIDAL's benefit». Los términos también piden consentimiento expreso para no implicar respaldo.
2–7. `NO ENCONTRADO`. Sin activo oficial descargable público.
8. **Riesgo alto** si se muestra el logo o algo parecido (rombos). Texto en palabras: solo uso referencial.

### 2.6 Deezer

Fuente: <https://business.deezer.com/partner-distribution/> (sin reglas de logo visibles).

1. **No verificado → tratar como permiso escrito.** Hay portales de partners; la página no recoge reglas de logo. Contacto: `support.deezer.business`.
2–7. `NO ENCONTRADO`.
8. Igual que TIDAL.

### 2.7 TuneCore

Fuente: <https://trademark.justia.com/owners/tunecore-inc-1675237> (marca registrada; no hay guía).

1. **No verificado → tratar como permiso escrito.** Marca registrada de TuneCore, Inc.
2–7. `NO ENCONTRADO`.
8. Distribuidora competidora de otras; implicar «TuneCore compatible» puede leerse como afiliación. Mención textual referencial.

### 2.8 Fichas sin marca de tercero

«Distribuidora estándar», MP3, Portada, Canvas (Spotify) y YouTube vídeo ya llevan nombre genérico o referencial. **Canvas** es un término de Spotify (como producto); usarlo en texto es referencial. **«Apple motion»**: «Apple Motion» es además un programa de Apple (editor de vídeo); el nombre de la ficha puede confundir y se escribe con minúscula. Mejor «Portada animada (Apple Music)».

## 3. Tabla resumen

| Plataforma | Permitido sin acuerdo | Forma | Color | Texto | Fuente |
|---|---|---|---|---|---|
| Spotify | **Condicionado** (reglas de developers; aval no permitido; sin vínculo para esta app) | Logo completo; icono solo si falta espacio | Verde solo sobre negro/blanco; si no, monocromo | Permitido «Listen/Play on Spotify»; nombre sin «Spotify»; sin aval | developer.spotify.com/documentation/design |
| Apple Music | **Condicionado/No** (marketing tools para artistas; guía no leída) | Badge/icono de Apple (no verificado) | No verificado | «works with»/«compatible with»; no logo Apple en lugar de la palabra | itunespartner.apple.com + developer.apple.com/app-store/marketing/guidelines |
| Apple Digital Masters | **No** (solo mastering houses aprobadas) | Badge de Apple | Sin recolorear | Sin añadir texto; sin aval | apple.com/apple-music/mastered |
| YouTube / YT Music | **Condicionado** (solo apps con API de YouTube; asociación = permiso escrito) | Uno de tres logos | No modificar colores | Sin «YouTube» en el nombre de la app | developers.google.com/youtube/terms/branding-guidelines |
| Amazon Music | **No** (aprobación escrita) | No verificado | No alterar | Sin aval; no imitar | advertising.amazon.com/resources/ad-policy/brand-usage |
| TIDAL | **No** (permiso escrito por uso) | No verificado | No verificado | Sin aval | tidal.com/terms |
| Deezer | **No** (no verificado, asumir permiso) | No verificado | No verificado | No verificado | business.deezer.com/partner-distribution |
| TuneCore | **No** (no verificado, asumir permiso) | No verificado | No verificado | No verificado | trademark.justia.com (sin guía) |

## 4. Qué puede y qué no puede decir la app

**Puede (uso referencial, ortografía oficial, hechos):**
- El **nombre en texto** de la plataforma como destino: «Spotify», «Apple Music», «Amazon Music», «TIDAL», «Deezer», «YouTube».
- Hechos verificables: «Objetivo: −14 LUFS», «Archivo WAV 24 bit», «Carpeta para subir a tu distribuidora».
- Frases tipo Apple: «para Apple Music», «compatible con».
- Resultados de una comprobación **propia y nombrada**: «Comprobación con las herramientas de Apple: 0 muestras con clipping en el AAC».
- Aviso de marcas: «Spotify, Apple Music, Amazon Music, TIDAL, Deezer, YouTube y TuneCore son marcas de sus propietarios. master check no está afiliado ni respaldado por ellos».

**No puede:**
- Mostrar un logo, badge o icono oficial sin licencia (Apple Digital Masters: nunca).
- Imitar el aspecto de un logo (glifos actuales).
- «Apple Digital Masters» como producto/ficha propia, ni «Apple Digital Masters certificado», «aprobado por Apple», «cumple ADM» sin matiz, «Spotify Ready», «Certified for…», «Optimizado para Spotify/Amazon/Tidal», «Official»/«oficial».
- Usar «Spotify», «YouTube», «Apple», etc. en el nombre de la app o de un archivo de marca propia.
- Sugerir que la plataforma «acepta» o «aprobará» el archivo («Apple lo aceptará», «pasará Spotify»). Decir «cumple la especificación publicada» está bien si es cierto y se cita.

## 5. Textos de la UI/repo que podrían implicar aval y deben cambiar

| Dónde | Texto actual | Problema | Cambio propuesto |
|---|---|---|---|
| `engine/masterout-core/presets.json` l.21, 23 | nombre «Apple Music (Digital Masters)», tag «Apple Digital Masters» | Presenta ADM como destino/programa propio; el tag va al nombre de archivo | nombre «Apple Music · 24 bit» ; tag «24bit-96kHz» sin la marca ADM |
| Nombre de archivo de salida `[Apple Digital Masters] 24bit-96kHz.wav` (tests/export.rs, plan.rs, design/exploracion) | Etiqueta el WAV como ADM | **Es la afirmación más fuerte de aval**: el archivo lo emite un tercero no autorizado | `[Apple Music] 24bit-96kHz.wav` |
| `app/ui/backend.js` l.13, 47, 63 | tag y «Apple Digital Masters exige un máster de 24 bit» | Atribuye una exigencia al programa; correcto como dato si se cita, pero con el nombre de ficha | «Apple Music pide un máster de 24 bit o más» (ES) / «Apple Music asks for a 24-bit master or higher» (EN) |
| `engine/.../reports.rs` y claves `report.adm_*` en es/en | «Apple Digital Masters: n muestras con clipping en el AAC» | Parece que Apple valida | «Comprobación con las herramientas de Apple: n muestras con clipping en el AAC» |
| `app/ui/flow.js` comentario l.38 | Solo comentario | Sin impacto visible | Opcional |
| `app/ui/i18n/*.json` `chip.motion` «Apple motion»; `chip.long.motion` «Apple Motion» | «Apple Motion» es una app de Apple | Confusión de marca | «Portada animada» |
| `chip.amazon` «Amazon» vs `chip.long.amazon` «Amazon Music» | Ortografía inconsistente | Nombre oficial es «Amazon Music» | Unificar |
| `chip.tidal` «Tidal» | La marca se escribe «TIDAL» | Ortografía | «TIDAL» |
| Glifos en `app/ui/app.js` (~l.41, imitativos) | Ondas, «a» con sonrisa, rombos, play | Imitación de marca | Retirar (ver recomendación) |
| README / web / descargas (aún no existen) | — | Futuro | No decir «certificado», «ready», «optimized for», «oficial». Incluir aviso de marcas |

## 6. Evaluación de las tres opciones

| Criterio | A: oficiales donde se pueda + glifo neutro + nombre | B: sin logos; nombre + glifo propio por grupo | C: oficiales en todas |
|---|---|---|---|
| Riesgo legal | **Medio.** Hay que decidir cuáles «se pueden»: solo Spotify y YouTube tienen reglas escritas, y no para esta app. Solución mixta: dos reglas de marca en la misma lista | **Bajo.** Solo uso referencial de nombres | **Alto.** Apple Digital Masters prohibido; Amazon/TIDAL/Deezer/TuneCore exigen permiso o sin guía |
| Esfuerzo | Medio-alto: descargar, revisar tamaños y colores, mantener una rama por plataforma y reglas de fondos | **Bajo**: tres glifos propios | Alto y bloqueado por permisos |
| Claridad para un artista con prisa | Alta en Spotify/YouTube; desigual en el resto (la lista queda mezclada) | Alta si el nombre es legible: el artista busca la palabra, no el logo | Máxima, pero ilegal en la mayoría |
| Coherencia con BRAND.md | Mala: logos de colores de terceros chocan con «tinta y papel», con «acento naranja solo en estado/CTA» y con la regla «Logos o marcas de terceros dentro de los iconos y portadas» (§5) | **Alta** | Muy mala |

### RECOMENDACIÓN: opción B

**Ningún logo de terceros. Nombre oficial en texto + glifo propio abstracto por grupo (streaming / distribuidora / extras).**

Motivos:
1. **Legal.** La única vía sin riesgo y sin papeles. Spotify y YouTube solo dan reglas para apps que usan su API, y su regla de oro («no parecerse», «no implicar respaldo») la incumplimos hoy con los glifos. Apple Digital Masters está vedado. Amazon, TIDAL, Deezer y TuneCore exigen permiso escrito o no publican guía. Con B no hay nada que pedir ni que perder si una marca cambia sus reglas.
2. **Claridad.** Una ficha con el nombre en tipografía clara («Spotify», «Apple Music») es lo que un artista reconoce y busca; el logo no añade información. Los números (LUFS, bits, kHz) son lo que informa, y eso concuerda con «dato antes que adjetivo».
3. **Marca.** Cumple BRAND.md: tinta y papel, el acento naranja solo marca estado, y el punto como único gesto (§4–5). Los glifos de grupo pueden ser formas simples con el punto (p. ej. streaming = punto + barra de nivel; distribuidora = punto + línea; extras = punto sobre marco), sin reproducir formas de ninguna marca.
4. **Esfuerzo y mantenimiento.** Tres glifos, un SVG cada uno, una regla. No hay activos de terceros que actualizar.
5. **Salida abierta.** Si en el futuro Spotify o YouTube concedieran uso por escrito, o si nortia. firma acuerdos de partner, se pasa a A para esas fichas.

**Condiciones de B:**
- Los glifos de grupo **no** pueden reconocerse como los de ninguna plataforma (sin ondas, sonrisa, rombos, play).
- Ortografía oficial («Apple Music», «Amazon Music», «TIDAL», «YouTube», «Deezer», «TuneCore»).
- Aviso de marcas (§4) en «Acerca de» y en la web.
- Aplicar los cambios de §5, sobre todo el **nombre de archivo ADM** y el nombre de ficha.
- Antes de vender, preguntar por escrito a Apple (`appledigitalmasters@apple.com`) si una herramienta de terceros puede nombrar «Apple Digital Masters» como especificación que comprueba, y revisar la guía de Apple Music que aquí no se pudo cargar.

## 7. Pendiente y URLs que no cargaron

- <https://marketing.services.apple/apple-music-identity-guidelines> (permiso no concedido por la herramienta).
- <https://www.apple.com/itunes/link/> (idem).
- <https://developer.amazon.com/support/legal/wwa-trademark-usage-guidelines> (idem).
- <https://brand.youtube> y <https://www.youtube.com/yt/brand/using-logo.html> (contenido dinámico; tamaños mínimos y espacio sin verificar).
- No se encontraron brand kits públicos de Amazon Music, TIDAL, Deezer ni TuneCore. Podrían existir como kits de prensa bajo petición.
- Un abogado debe confirmar el uso referencial de nombres de marca en España/UE y EE. UU., y las implicaciones de la marca «Apple Digital Masters».

## 8. Decisión D-31 (06/11/2026): opción B

**Decidido por el CEO, pendiente de ratificar con Fer en la revisión:** opción B. master check no muestra ningún logo, badge ni glifo imitativo de terceros. Cada plataforma aparece con su **nombre oficial en texto** y el punto de estado de la marca (variante V1 «solo nombre», por defecto); la variante V2 «gesto por grupo» (`data-chip-style="gesto"`) queda en una rama de estilo para elegir viendo las capturas.

Aplicado en la tarea 8:
- Fuera los glifos de `app/ui/app.js`. Nombres: Amazon Music, TIDAL, YouTube Music, YouTube (vídeo), Portada animada / Motion artwork.
- El archivo de Apple pasa de `[Apple Digital Masters] 24bit-96kHz.wav` a `[Apple Music] 24bit-96kHz.wav` (contrato §4, con adenda en la spec). La subcarpeta `02 Apple Music/` no cambia.
- «Apple Digital Masters» desaparece como nombre de ficha, archivo, carpeta y texto visible; el informe dice «Comprobación con las herramientas de Apple (AAC 256)». Un test falla si reaparecen «certificado», «aprobado por», «Ready», «optimizado para», «Mastered for» o «Digital Masters».
- Aviso de marcas en «Acerca de» y al final del README de entrega: «Spotify, Apple Music, Amazon Music, TIDAL, Deezer, YouTube y TuneCore son marcas de sus titulares. master check no está afiliado ni avalado por ellos.»

**Salidas abiertas:**
1. **Uso por escrito de Spotify o YouTube** (permiso o acuerdo de partner): se pasa a la opción A **solo para esas fichas** (logo oficial según su guía, sobre fondo permitido, tamaños mínimos), sin tocar el resto.
2. **Consulta a Apple** (`appledigitalmasters@apple.com`): si una herramienta de terceros puede nombrar «Apple Digital Masters» como especificación que comprueba. Hasta tener respuesta por escrito, el nombre no se usa en nada visible.
3. Pendiente también: revisar la guía de Apple Music que no se pudo cargar (§7) y la confirmación del asesor antes de vender.
