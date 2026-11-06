# Decisiones — nortia. (empresa de ingresos extra; CEOs: Fer y Claude)

Formato: D-NN · título · fecha · quién · decisión · alternativas descartadas · riesgo · revisar cuando.

> **Fase 1 (D-ING-01…11) cerrada el 05/11/2026 a las 13:51** por Fer ("empecemos de cero con lo aprendido"). Se conserva abajo como histórico. Lecciones en `claude/cierre-y-aprendizajes-2026-11-05.md`. La fase 2 empieza en D-20.

---

# FASE 2 · nortia. y nortia. studio

## D-20 · Estructura: nortia. (madre) y subempresas
- 05/11/2026 · Fer. Fer y Claude son los dos CEOs. La marca madre es **nortia.** (siempre minúsculas y con punto). Las subempresas se nombran `nortia. <sub>`; la primera es **nortia. studio** (audio: apps para productores y artistas). Vendrán más. Ninguna persona visible en la marca.
- Método: ir más guiados que en fase 1; superpowers siempre (brainstorming → spec → plan → TDD → revisión → verificación); Claude valida cada etapa con Fer antes de producir.
- Inventario de skills en cada tarea: Claude se pregunta qué skill necesita, busca, y pide a Fer instalar lo que falte.
- 05/11/2026 14:26 · Fer: **Claude trabaja todo el día de forma continua como CEO** (latido programado cada hora); cuando Fer llega, Claude le cuenta y Fer revisa o vira. Las decisiones se toman con datos que validen lo más rentable. Fer prefiere decidir **una decisión cada vez, con las imágenes delante**.

## D-21 · Primer producto: app "master check" (antes "Master Out")
- 05/11/2026 · Fer. App sencilla para productores y artistas: entra el máster (WAV/AIFF/FLAC) y devuelve en una carpeta elegida todos los archivos listos para distribuir, nombrados por plataforma (Spotify, Apple Digital Masters, Amazon, Tidal, Deezer, YouTube, distribuidora estándar e hi-res, MP3 promo, portada, Canvas) más README, informe de loudness y ficha de lanzamiento. Spec aprobada: `nortia-studio/spec-master-out-2026-11-05.md` + adendas en el repo (la sección 5 "motor" y la 8 "tienda" quedan superadas por D-22/D-23).
- **Loudness: opción A** (medir y avisar, nunca tocar el máster).
- **Precio: 9,99 € pago único, siempre.** Sin suscripción, sin cuenta, sin red (salvo activar la licencia una vez).
- **Plataformas: Mac primero, después PC (Windows). Nada más** (iPhone/Android descartados por ahora).
- **Idioma: la app habla el idioma del sistema del usuario** (05/11 14:27, Fer). Mínimo español e inglés desde v1, textos externalizados; informes/README en ese idioma.
- **Público y tono de la interfaz (05/11 15:11–15:30, Fer):** es para el artista, no para el técnico: "rapidez, súper visual e interactivo, bonito; no le expliques nada, le da igual". Una sola pantalla; Artista/Título/Portada visibles; el resto bajo "Más datos"; lo técnico bajo "Ajustes avanzados"; loudness como pastilla de dos cifras; botón único "Crear carpeta de entrega". Sustituye a "tarjetas con semáforo y árbol de carpeta protagonista" de la spec §3.
- **Nombre definitivo: `master check`, siempre en minúsculas** (05/11 15:52; antes "Master Out", "Master Check", "Master Ready").

## D-22 · Canal de venta: SOLO canal propio (sin App Store por ahora)
- 05/11/2026 14:25 · Fer: "no vendemos en App Store aún, solo nosotros". Venta desde la web de nortia. con pasarela merchant of record (liquida IVA, entrega archivo y clave de licencia; candidatas Lemon Squeezy / Polar, ambas 5 % + 0,50 USD con API de licencias; Paddle sin API de claves confirmada). Mac: build notarizada con Developer ID. Windows: firma por decidir (Azure Artifact Signing 9,99 USD/mes, OV/EV, o sin firmar con aviso SmartScreen).
- **Apple Developer Program: Fer lo da por hecho (05/11 14:45, "ya está lo de apple").** Se usará para Developer ID + notarización, no para la App Store.
- Descartado por ahora: Mac App Store (revisar cuando haya ventas propias estables).
- Margen estimado por venta a 9,99 € IVA incl.: ~7,3 € con pasarela MoR al 5 % + 0,50.

## D-23 · Stack y fábrica: Rust + Tauri, compilado en GitHub Actions
- 05/11/2026 · Fer no puede instalar Xcode y quiere Mac y luego Windows. Claude decide: **motor en Rust** (WAV/FLAC/MP3, loudness BS.1770-4, nombres, presets, metadatos, informes) + **interfaz Tauri 2** (HTML/CSS/JS en webview nativo; un código para Mac y Windows). Las piezas solo-Mac (afconvert/afclip) se activan solo en macOS.
- Descartado: SwiftUI + Core Audio; Electron; Flutter; alquilar Mac en la nube.
- **Fábrica: GitHub Actions.** Repo privado **`fher400gmail/nortia-studio`** creado por Fer y enlazado a la sesión el 05/11 14:45 (hubo que instalar la GitHub App "Claude" en su cuenta). Rama de trabajo del motor: `hito1-motor`; `main` recibe specs, planes, diseño y marca.
- **Rust local:** Fer permitió `static.crates.io`, `index.crates.io` y `static.rust-lang.org` en claude.ai → Capacidades (05/11 14:31); cargo compila en la sesión a través del proxy (`CARGO_HTTP_PROXY=$HTTPS_PROXY`, el puerto cambia entre reinicios). Tests del motor también en Actions (Linux, macOS, Windows).
- **Másters reales de prueba:** localizados en el Dropbox de Fer (`SEND MIXES BORR/plugin mastering/pendientes_verificar/…/master/*(MASTERED DK)*.wav`) y copiados a `/home/claude/testdata/masters/` (m96_24, m48_24, m44_24, m44_24_adm). Referencias ffmpeg: −6,2…−9,1 LUFS, true peak hasta +1,2 dBTP; dos con chunk `JUNK`. No se suben al repo ni se publican.
- Mínimo macOS 14 (Sonoma): TelemetryDeck 02/11/2026: ~87 % en macOS 26/27. Windows 10/11 (WebView2).
- Plan del hito 1 (motor, 11 tareas con tests): `docs/superpowers/plans/2026-11-05-hito1-motor-rust.md`; ejecución por trabajadores Sonnet con revisión por tarea (subagent-driven-development).

## D-24 · Departamentos en paralelo
- 05/11/2026 · Fer. Diseño de la app (opciones y caminos) y marca corporativa trabajan en paralelo con ingeniería. Política de modelos (hereda D-ING-09): Fable decide/habla/revisa; Sonnet produce; máximo 3 subagentes en paralelo.
- Incidencia: los dos departamentos se colgaron en el reinicio de contexto de las 14:13 y se relanzaron a las 14:55 (lección: tras una compactación, comprobar la hora de última actividad de cada agente).

## D-25 · Skills propias para nortia. studio
- 05/11/2026 · Claude. Se crean `audio-engine-desktop` y `desktop-release` con el método TDD de writing-skills. Líneas base sin skill y verificación con fuentes en `nortia-studio/docs/skills-baseline/` (repo). Pendiente: redactar las skills cuando el hito 1 enseñe qué se repite. **Hecha ya: `nortia-brand`** (ver D-28).

## D-26 · [SUPERADA por D-28] Marca v1.0 y color verde azulado
- 05/11/2026 15:10 · Fer eligió verde azulado #0E5E63 para studio sobre el símbolo "norte marcado" (rombo ámbar/azul noche). A las 15:42 rechazó toda la marca v1 ("parece una empresa que hace medicamentos"). Queda como histórico; el kit v1 se eliminó del repo.

## D-27 · Interfaz de master check: D4a "una tarjeta" + Liquid Glass; la ventana es la tarjeta
- 05/11/2026 15:11 · Fer rechazó D1 (mesa de trabajo), D2 (flujo por pasos) y D3 (tarjetas vivas): "muy complejo… que se vea más simple y más esquemático; a nadie le importa lo que hace cada cosa".
- 15:25 · Fer eligió **D4a "Una tarjeta"** (vertical centrada: onda → artista/título/portada → fichas de plataforma con logo, todo activado por defecto → botón "Crear carpeta de entrega"; progreso rellenando la propia onda) frente a D4b (tres pasos en fila).
- 15:29–15:30 · Fer: **detalles de Liquid Glass (macOS 26), estética súper pro, muy interactivo al pasar por encima de botones, y que la ventana de la app se ajuste a la tarjeta central** (sin fondo vacío; barra de título integrada; crece fluida al agrandar). En producción como `design/final/` (index.html, styles.css con tokens, app.js, tokens.json, DISEÑO-FINAL.md): es el frontend real de Tauri. Publicado como artefacto interactivo para Fer.
- Todas las direcciones están en el repo `design/exploracion/` (D1–D4) por si se recuperan ideas (p. ej. texto guía del primer arranque de D2).

## D-28 · Marca v2 definitiva, nombre de la app y manual (05/11/2026 15:42–16:20)
- 15:42 · Fer rechazó la marca v1 (rombo ámbar/azul noche): "no me gusta nada… parece una empresa que hace medicamentos"; pidió "más Apple, más sencillo" y buscar skills de diseño. Se comprobó el catálogo: brand-studio y Design ya instalados; no hay "skill de superdiseñador"; la diferencia la pone el brief (referentes Apple/Linear/Raycast/Ableton/Teenage Engineering/Braun, mundo música, prohibido lo corporativo) y la crítica a ciegas.
- 16:12 · Fer aprobó la recomendación: **dirección A "Tipográfica pura" + gesto del icono de C para studio**. Sistema: wordmark `nortia.` en Hanken Grotesk 590 con punto circular a medida; tinta #111111, papel #F4F3F0; madre sin acento; **nortia. studio = naranja #FF5A00**, icono = punto estirado a barra vertical 2:1; ejemplo nortia. lab azul #2B3BFF; reservas teal #00968B y magenta #D6137A. Texto-acento #B84100 en claro (el naranja no cumple AA como texto sobre papel); texto sobre botón naranja = tinta. Kit completo en el repo `brand/` (regenerable con `python3 gen.py`, 82 pares de contraste comprobados) y copias de BRAND.md / DESIGN.md en el proyecto (`nortia/`).
- 15:45 · Fer: **nortia. es la empresa madre; nortia. studio es su división de audio; vendrán más** con el mismo patrón (`nortia. <oficio>`), cada una con su acento y su gesto del punto.
- Nombre de la app: 15:50 "Master Check" → 15:51 "Master Ready" → **15:52 definitivo: `master check`, siempre en minúsculas** (D-21 actualizado). Pendiente: dominio y conflicto de marca.
- 15:49 · Fer: **Portada y Canvas (y vídeo de YouTube) son zonas de arrastre**: el usuario suelta su material y la app lo conforma a la norma de cada plataforma; si no suelta nada, se fabrica desde la portada. 15:53: la portada vacía se muestra en negro con "Arrastrar portada" y muestra la imagen soltada.
- 16:12 · Fer: "hazte un manual para todo lo que desarrollemos de la marca". Hecho como **skill `nortia-brand`** (guardada por Fer el 05/11 ~16:30): se carga sola en cualquier tarea visible de nortia. y resume arquitectura, color, tipografía, componentes, tono de voz, prohibiciones y lista de comprobación.
- Prototipo `design/final/` re-tematizado con la marca v2 (fichas activas neutras con solo el check naranja; botón naranja; Hanken Grotesk); publicado como artefacto interactivo para Fer. Pendiente: logos oficiales de plataformas antes de publicar.

## D-29 · Hito 1 del motor: estado al 05/11 17:35
- 10 de 11 tareas completas con revisión independiente por tarea (4 fix rounds; hallazgos reales: resample fallaba con múltiplos de 1024 muestras; subcarpetas fuera del contrato; panic con versión larga; etiqueta Apple "Apple Music" en vez de "Apple Digital Masters" oculta por tests que asertaban el valor erróneo). CI en GitHub Actions: Linux y Windows en verde (lame real en Windows); macOS pendiente por falta de capacidad de runners (se reintenta con macos-15 y timeout por job). Revisión final de rama en curso (modelo más capaz); después PR y fusión a `main`.
- Lección: los tests que copian el valor de la implementación en vez del contrato no protegen nada; el revisor debe comprobar los literales contra la spec.

---

# FASE 1 (histórico, cerrada)

## D-ING-01 · Vías no limitadas a música ni apps
- 05/11/2026 · Fer. Se busca la vía nueva que mejor funcione.

## D-ING-02 · Permiso de IA
- 05/11/2026 · Fer. No ir con lupa en plataformas que pagan por resultado (Fiverr, Upwork, Gumroad, bounties). Quedan fuera las de microtareas humanas que prohíben IA (MTurk, Clickworker, Appen, Outlier, DataAnnotation, Prolific, Chegg, Rev).

## D-ING-03 · Claude actúa como CEO de una empresa aparte
- 05/11/2026 · Fer. Claude decide y ejecuta; acumula las tareas que necesita de Fer y se las pide una vez al día (tarea programada 18:07). Fer factura como autónomo y hace lo que exige identidad, cuentas, publicar y cobrar.
- 05/11/2026 · Fer (precisión). La empresa es distinta de su gestoría y de su carrera musical: Claude no usa el apartado de gestoría como base de trabajo. La gestoría es un departamento al que se consulta si hace falta y al que se comunican los ingresos cuando los haya. Skill de la empresa: `ceo-ingresos-fer`.
- 05/11/2026 · Fer (mandato): "decide y maximiza siempre"; Claude crea los agentes y tareas que necesite, incluida la difusión, cuidando el límite semanal de Ultra.
- 05/11/2026 · Fer (método): usar superpowers (brainstorming antes de construir, planes, TDD, revisión, verificación) siempre que aplique, salvo que no tenga sentido. Las decisiones arquitecturales (marca, web, producto nuevo, pista nueva) pasan por brainstorming con Fer antes de producir.

## D-ING-04 · Tres pistas en paralelo
- 05/11/2026 · Claude. Pista 1 encargos (Fiverr + Upwork), pista 2 catálogo (Gumroad ahora; Polar + Cloudflare Pages cuando haya GitHub), pista 3 bounties (Algora, cuando haya GitHub). Descartadas: contenido/audiencia, KDP, stock, microtareas. Revisar: a los 90 días (3/2/2027) con ingresos reales.

## D-ING-05 · Línea 1 del catálogo: herramientas para autónomos en España
- 05/11/2026 · Claude. Producto 1: libro de ingresos y gastos 2026 con 130/303 orientativos (Excel), 19 € (14 € lanzamiento). Producto 2: guía Verifactu y factura electrónica (PDF, 9–12 €). Producto 3: calculadora web gratuita como gancho. Después: línea 2 y 3 según ranking.
- Por qué: hueco sin competencia de pago en español, demanda estacional, poca competencia. Riesgo: error normativo → doble revisión obligatoria; aviso "orientativo".
- 05/11/2026 · Fer (objeción) y Claude: la elección pudo estar sesgada. Productos 1 y 2 se publican igual (hechos a coste cero); producto 3 y siguientes CONGELADOS hasta la comparativa de nichos a ciegas (`productos/_investigacion/nichos_2026-11.md`) → D-ING-12.

## D-ING-06 · Gigs de la pista 1
- 05/11/2026 · Claude. Fiverr: automatización Excel/Sheets, limpieza de datos, scripts Python. Upwork: mismo posicionamiento, precio fijo 50–300 $. Propuestas las envía Fer.

## D-ING-07 · Orden de apertura de cuentas
- 05/11/2026 · Claude. Gumroad → Fiverr → Upwork → GitHub → Polar → Algora.
- Estado 05/11/2026 13:20: Gumroad abierto con pagos; Fiverr abierto (anti-bot bloquea el navegador integrado: gigs a mano desde Chrome de Fer); Upwork perfil PUBLICADO (sin comprar connects; decidir mañana con ofertas reales); GitHub conectado y repo creado (pendiente renombrar a la marca); X cuenta creada (alta a medio terminar; usuario y perfil pendientes de la marca).

## D-ING-08 · Relación con la gestoría de Fer
- 05/11/2026 · Fer. La empresa lleva su propio libro (`claude/libro-empresa-AAAA.csv`). Se consulta la gestoría solo para datos y se le comunican los ingresos por trimestre.

## D-ING-09 · Política de modelos y cupo
- 05/11/2026 · Fable solo para decidir, hablar con Fer, revisar normativa/dinero y bloqueos. Sonnet para producir y para las tareas programadas (diaria 18:07 y difusión lunes 10:19, ambas ya en Sonnet). Haiku para lo mecánico. Máximo 3 subagentes. Medir cupo los primeros 7 días e informar en el brief. Si aprieta: bajar modelos → pausar bounties → último recurso, créditos extra. Revisar 12/11/2026.

## D-ING-10 · Difusión (solo canales de la empresa)
- 05/11/2026 · Fer: NADA en sus redes personales ni profesionales. Canales propios de Nortia Labs, llevados por Claude: (1) web propia en Cloudflare Pages desde el repo (SEO, fichas, herramienta gratuita); (2) Gumroad Discover (fichas, etiquetas, categorías); (3) cuenta de X de la empresa (publica Claude con calendario; textos filtrados por product-legal); (4) comunidades de autónomos solo donde sus normas permitan compartir recursos, con cuenta de la empresa, respondiendo a preguntas reales. Descartado: anuncios de pago, newsletter, cualquier publicación en nombre de Fer. Tarea semanal (lunes 10:19, Sonnet) prepara y, cuando X esté operativa, publica en los canales propios. Prohibido comprar seguidores/reseñas, urgencia falsa, promesas. Revisar en semana 4 con datos de conversión por canal.

## D-ING-11 · Marca paraguas: Nortia Labs
- 05/11/2026 · Fer: quitar todo lo visible de su persona; marca genérica que englobe todos los trabajos. Claude propuso Nortia; "Nortia" a secas está cogido en Gumroad y existe Nortia Inteligencia de Negocio (empresa española de software) → se elige **Nortia Labs**, usuario `nortialabs` (libre en Gumroad, GitHub y X comprobado el 05/11/2026 13:22).
- Alcance: nombre visible y usuario en Gumroad, GitHub, X, web, portadas, PDFs, fichas, correo de la empresa (alias de correo de la empresa; no se publica en el repositorio). Upwork y Fiverr exigen nombre real: se mantiene el nombre, pero el perfil no menciona música ni discográficas. Los recibos de Gumroad llevan el nombre fiscal por ley (solo lo ve el comprador).
- Identidad visual: dirección B (software de confianza), símbolo A (norte), variante N2 aprobada. **Sustituida en fase 2 por "nortia." (D-20).**
- Hecho ya: sustituido "Fernando Boix" por "Nortia Labs" en la guía Verifactu (PDF regenerado), guía de uso y fuentes de portadas; el Excel no contenía el nombre.
