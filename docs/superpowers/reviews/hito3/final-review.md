# Revisión final de rama · hito 3 (master check) · main..hito3-media (HEAD d7c77a0)

Revisor independiente; no participó en el hito. Solo lectura, salvo ejecutar las baterías (de una en una, `CARGO_BUILD_JOBS=2`, sin copias del repositorio). 67 commits, 97 archivos.

## 1. Resumen ejecutivo

1. El hito está casi completo: portada, Canvas, YouTube, portada animada, MP3 por ffmpeg, la comprobación de Apple con afclip, las fichas con material propio, el sidecar empaquetado con su humo, 13 de las 14 mejoras del hito 2 y la decisión D-31 tienen código y tests, y todas las baterías locales pasan (269 tests del motor, 45 de la UI, Playwright es/en, clippy nativo y cruzado).
2. **Hay un fallo crítico que nadie ha visto: el sidecar LGPL publicado (`ffmpeg-sidecar-v7.1.5-3a21763`) no incluye el filtro `setparams`.** Desde 6a6848a, todas las cadenas de vídeo lo usan, así que en la app distribuida, en Mac y en Windows, fallarían todos los Canvas, vídeos de YouTube y portadas animadas.
3. No lo detecta ningún test porque los tests de ejecución del motor usan el ffmpeg del sistema (GPL, con todos los filtros) y el único que usa el sidecar real, el humo de la app, no se ha ejecutado desde 9c214b9. Ese commit es anterior a `setparams`, y luego la CI se paró por facturación.
4. El resto son Important de cadena de suministro y robustez (discover cae a PATH en producción, tres acciones sin SHA) y Minor de documentación, marca y deuda legal previa a la venta (la oferta pública de fuentes LGPL, el texto de la OFL y los avisos MIT).
5. **Veredicto: Fusionar tras corregir C1** (sidecar nuevo con `setparams`, una prueba pura que ate los filtros del motor a la lista blanca del sidecar) **y con la CI en verde en las tres plataformas**, incluidos los humos de macOS (ADM=required) y de Windows.

## 2. Tests ejecutados (worktree hito3, HEAD d7c77a0, árbol limpio antes y después)

| Orden | Resultado |
|---|---|
| `app/`: `npm test` | 45/45 ok (backend.spec + marcas.spec) |
| `app/`: `npm run check` (Playwright) | ok es y en (vacío → soltar → crear → listo) |
| `python3 app/scripts/check-wiring.py` | RESULTADO OK (≈70 comprobaciones) |
| `bash app/scripts/tests/test-fetch-sidecar.sh` | todo ok, exit 0 (descarga mac/win, hash alterado, release manipulado, sin hash en el repo, placeholder…) |
| `engine/`: `cargo fmt --all --check` | ok |
| `engine/`: `cargo clippy --workspace --all-targets -- -D warnings` | ok |
| `engine/`: `cargo clippy … --target x86_64-pc-windows-gnu -- -D warnings` | ok |
| `engine/`: `MASTEROUT_FIXTURES=/home/claude/testdata/masters cargo test --workspace` | **269 passed, 0 failed, 0 ignored** (26 binarios; media_video 29, media_adm 21, export 26, plan 25, app_support 22…). Con **ffmpeg 6.1.1 del sistema (Ubuntu, GPL)**, no con el sidecar |
| `app/src-tauri`: `cargo clippy --no-default-features --all-targets -- -D warnings` | ok |
| `app/src-tauri`: `cargo fmt --check` / `cargo test --no-default-features` | ok / 0 tests |
| Release real del sidecar (REST, solo lectura): `configure_line-windows.txt` y `configure_line-macos.txt` de `ffmpeg-sidecar-v7.1.5-3a21763` | `--enable-filter=scale,crop,zoompan,gblur,overlay,format,fps,setsar,trim,atrim,aresample,aformat,anull,null,color,loop,pad,setpts,split,fade`: **sin `setparams`** (prueba de C1) |

Capturas leídas: `t8-captures/v1-claro-es-sin-portada`, `v1-oscuro-es-con-portada`, `v1-claro-en-con-portada`, `t7-captures/listo-adm-claro-es`.

## 3. Cobertura (A)

| Requisito del hito 3 | Implementación | Tests | Estado |
|---|---|---|---|
| Sidecar ffmpeg LGPL propio (CI, GPG, inventario, release) | `tools/ffmpeg/*`, `.github/workflows/ffmpeg-sidecar.yml` | `accept.sh` (inventario contra lista blanca, aceptación h264/hevc/aac/mp3, sin elst) | ✅ salvo **C1** (la lista blanca no cubre lo que usa el motor) |
| Portada conformada 3000² sRGB JPG+PNG | `media/image.rs` | `tests/media_image.rs` (9: recorte, EXIF, pequeño, enorme antes de decodificar, alfa, ICC, retrato, 16 bit) | ✅ |
| Spotify Canvas (portada o material propio) | `media/video.rs:255-299`, `export.rs:582-600` | goldens + ejecución (`canvas_from_user_video_is_conformed`, bucle < 3 s) | ✅ en el motor; ❌ con el sidecar real (C1) |
| YouTube 1080p (fondo gblur prerenderizado, AAC 384k/48k, sin elst, faststart) | `video.rs:306-414`, `export.rs:633-657` | goldens mac/win/linux, `youtube_output_matches_spec`, recorrido de cajas sin `elst`, BT.709 real | ✅ / ❌ sidecar (C1) |
| Portada animada (HEVC 3840², H.264 2048×2732, omitida sin HEVC) | `video.rs:420-468`, `plan.rs` (`hevc_skipped`) | goldens, `apple_motion_*`, `capabilities_detect_missing_hevc…` | ✅ / ❌ sidecar (C1). `hevc_mf` nunca codificó de verdad (runner sin extensión): diferido a hito 4, anotado en progress.md |
| MP3 320 por ffmpeg + ID3v2.3 con APIC | `media/mp3.rs` | `tests/media_mp3.rs` (4), e2e | ✅ (sin TCON/género, que la spec §5 pide: Minor) |
| Comprobación con afclip (Mac) | `media/adm.rs`, `reports.rs`, veredicto final en la UI | `tests/media_adm.rs` (21, con herramientas falsas y `real_tools` en macOS) | ✅. Real en CI hasta 74d507f; texto crudo de afclip nunca leído (artefacto no descargable) |
| Fichas arrastrables (miniatura + ×, halo, validación de rutas) | `app.js:193-240, 527-540`, `plan.rs:resolve_media`, `thumbnail.rs` | node + Playwright (soltar en Canvas, quitar), `own_media_paths_are_validated_in_the_plan`, webp con tope | ✅ |
| externalBin + humo mac/win | `tauri.conf.json:47-57`, `fetch-sidecar.sh`, `smoke-*.{sh,ps1}`, `smoke_verify.py` | `check-wiring.py`, `test-fetch-sidecar.sh`, humo en CI | ✅ (último humo real en 9c214b9; ver §5) |
| 14 mejoras del hito 2 | T7 (ca91c64…5b1e414) | ver task-7-review/rereview | 13 ✅; #7 (humo a través de Tauri) diferido con motivo en `docs/pruebas/huecos.md` ✅; Node 20 en Actions diferido, **solo anotado en `.superpowers/`** (Minor G3) |
| Marcas de terceros (D-31) | UI V1 sin glifos, `[Apple Music]`, `[TIDAL]`, `[Portada animada]`, aviso de marcas | `marcas.spec.mjs`, `brand_text.rs`, `reports.rs::readme_ends_with_the_trademark_notice`, `ui-check` | ✅. D-31 pendiente de ratificar por Fer y **sin entrada en `docs/decisiones.md`** (G1) |

Diferidos justificados y anotados: humo a través de Tauri (huecos.md, hito 4), tests `.cmd` de plazo/kill en Windows (huecos.md), `hevc_mf` real (progress.md:85), firma y notarización (hito 4), fuentes LGPL públicas (terceros.md §3).

## 4. Hallazgos

### Critical

**C1. El sidecar distribuido no tiene `setparams`: en la app real fallan todos los vídeos (Canvas, YouTube y portada animada) en Mac y en Windows.**
- Dónde: `engine/masterout-core/src/media/video.rs:217` (`BT709 = "scale=…,format=yuv420p,setparams=colorspace=bt709:…"`). Se usa en `zoom_chain` (:243), en Canvas desde vídeo (:286), en las dos ramas de YouTube (:354-361, :374) y en la portada animada (:440). Llegó en 6a6848a (arreglo de color en Windows), rebasado sobre 9c214b9.
- La lista blanca del sidecar no lo incluye (`tools/ffmpeg/common.sh:30`), y `accept.sh:80` (`IMPLICIT_FILTERS`) tampoco lo admite como implícito. El inventario falla si sobra un filtro, así que el binario aprobado no lo contiene. Las `configure_line-{macos,windows}.txt` del release `ffmpeg-sidecar-v7.1.5-3a21763`, leídas aquí, lo confirman.
- Por qué nadie lo vio:
  - La sonda de capacidades (`media/mod.rs:111`) solo usa `format=yuv420p`, así que da `h264=true` y las fichas salen habilitadas.
  - Los tests de ejecución del motor usan el ffmpeg del sistema: apt (`engine-tests.yml:41`), brew (`:70`) y choco (`:92`), todos con todos los filtros.
  - El único que prueba el sidecar real es el humo de `app`, y su último run (9c214b9) es anterior a 6a6848a. Después la CI quedó bloqueada.
- Efecto: cada job de vídeo terminaría en `err.ffmpeg_failed` («No such filter: 'setparams'»). Es justo lo que este hito entrega.
- Corrección:
  1. Añadir `setparams` a `ALLOW_FILTERS` en `common.sh:30`. Es LGPL (`vf_setparams.c`). Ya que hay que recompilar, valorar también `prores` (decodificador LGPL) para los MOV de vídeo propios, ver M7.
  2. Publicar un release nuevo del sidecar.
  3. Actualizar `app/scripts/sidecar-tag.txt`, `app/scripts/sidecar-sha256.txt`, `SIDECAR_RELEASE` (`mastercheck-bridge/src/app_support/mod.rs:33`), `docs/legal/terceros.md` §8 y el checklist (prueba 1).
  4. Corregir el comentario obsoleto de `video.rs:6-11`: dice que no hay `split`/`setpts` y da la lista de filtros sin `setparams`.
  5. Añadir la prueba de I1 para que no vuelva a pasar.
  6. Confirmar con el humo de `app` en macOS y Windows.
  - Alternativa sin recompilar (peor): quitar `setparams` y aceptar el etiquetado parcial en `h264_mf`. Esa es la regresión que arregló 6a6848a.

### Important

**I1. Nada ata los grafos de filtros del motor al sidecar que se distribuye (es la causa de C1).**
- `engine-tests.yml:38-92` instala ffmpeg GPL del sistema. `tests/media_video.rs:431` (`sidecar()`) lo usa, y los goldens solo comparan contra literales.
- Corrección mínima: un test puro en `masterout-core` que lea `tools/ffmpeg/common.sh` (`ALLOW_FILTERS`, `ALLOW_ENCODERS_BASE`, `ALLOW_MUXERS`…) más los implícitos de `accept.sh`. Para cada `build_*_args` (Canvas desde portada y desde vídeo, YouTube con y sin fondo, fondo, portada animada en los dos tamaños, MP3, sonda, miniatura) extrae los nombres de filtro de `-vf`/`-filter_complex`: partir por `;`/`,`, quitar las etiquetas `[x]`, nombre antes de `=`. También `-c:v`/`-c:a` y `-f`. Exige que todos estén en la lista blanca.
- Corrección estructural (hito 4): en los jobs de macOS y Windows de `engine-tests.yml`, ejecutar los tests de vídeo con el sidecar del release (`fetch-sidecar.sh` + `MASTERCHECK_FFMPEG`) en lugar de brew/choco.

**I2. En producción, `discover` cae a `MASTERCHECK_FFMPEG`/`MASTERCHECK_FFPROBE` y a `PATH` si falta el sidecar empaquetado.**
- Dónde: `media/mod.rs:184-200, 223-225` y `export.rs:163-169`.
- Por qué importa:
  - El binario distribuido ejecutaría cualquier `ffmpeg` del PATH o de una variable de entorno: un build GPL de choco/scoop/brew, un shim (el que ya causó los 1479 s de cancelación, task-win-report) o uno manipulado.
  - Un antivirus que pone en cuarentena el `ffmpeg.exe` estático, algo habitual, pasa a usar otro binario sin aviso.
  - La revisión del plan (Review Focus 3) y la prueba 11 del checklist esperan «sin sidecar → fichas deshabilitadas».
  - M5 de T6 ya evita que «Acerca de» anuncie el release en ese caso, pero el motor lo sigue usando.
- Corrección: en la app (bridge `default_sidecar`/`app_support`), descubrir solo junto al ejecutable. Dejar env y PATH tras `cfg(debug_assertions)` o una feature `dev-sidecar` que activen tests y CI. Test: `discover` de producción con PATH y env poblados devuelve `None` si no hay binarios junto al exe.

**I3. Tres acciones siguen por etiqueta en los workflows que construyen los artefactos distribuibles.**
- `dtolnay/rust-toolchain@stable` (rama móvil) en `app-build.yml:30,78,180` y `engine-tests.yml:31,61,82`.
- `Swatinem/rust-cache@v2` en `app-build.yml:33,81,181` y `engine-tests.yml:35,64,85`.
- `actions/setup-node@v4` en `app-build.yml:60`.
- La lista de excepciones está en `check-wiring.py:97` (`UNRESOLVED`). Aquí tampoco se pudieron resolver: `gh api` devuelve «acceso no habilitado».
- No bloquea la fusión, porque los artefactos no se distribuyen aún y el token es `contents: read`. Sí bloquea el primer release firmado (hito 4): una acción comprometida puede alterar el `.dmg` y el `.exe`.
- Corrección: resolver los SHA desde un entorno con acceso (`gh api repos/<o>/<r>/git/ref/tags/<t>`; para rust-toolchain, `…/git/ref/heads/stable`) y vaciar `UNRESOLVED`.

### Minor

- **M1 · Checklist.** `docs/pruebas/hito3-checklist.md:35-37` (prueba 5) espera un Canvas «de unos 8 s», pero el fabricado desde la portada dura 6 s (`CanvasSpec::default`, `video.rs:71`; spec adenda 15:49). Fer marcaría un fallo que no lo es. Cambiar a «6 s» (8 s como máximo con vídeo propio). Tras C1, actualizar también el tag de la prueba 1.
- **M2 · Fichas con material propio en V1.** El punto de estado se sustituye por la miniatura (`app.js:231`), así que en V1 no se ve si Portada/Canvas están encendidas o apagadas (capturas `v1-*-con-portada`: mismo aspecto que una ficha encendida). Mantener el punto (p. ej. superpuesto en una esquina de la miniatura) o atenuar la miniatura cuando la ficha está apagada.
- **M3 · Texto de tipos admitidos.** `media.bad_type` (ui i18n:101) dice «JPG, PNG… MP4, MOV», pero se aceptan además webp, m4v, mkv y webm (`app.js:527`, `media/mod.rs:57-58`). Un webp soltado como **portada** se acepta por extensión y luego da `err.cover_unreadable` en el plan, porque el crate `image` se compila sin webp (`Cargo.toml:16`). Dos arreglos posibles:
  - quitar webp de la ficha de portada (`pick_file("image")` ya lo excluye, `commands.rs:165`);
  - activar la feature `webp` de `image`.
  - En los dos casos, alinear el texto.
- **M4 · Webp/vídeo propio en exportación sin tope de píxeles.** El tope previo con ffprobe solo protege la miniatura (`thumbnail.rs:104-116`). En la exportación, Canvas, YouTube y la portada animada decodifican el webp o el vídeo de usuario sin límite de tamaño (WebP admite hasta 16383², ≈1 GB en RGBA). Aplicar `MAX_SIDE`/`MAX_PIXELS` con la sonda en `resolve_media` o en `render_media`.
- **M5 · `probe()` mata solo al hijo directo.** `probe.rs:205-213` usa `child.kill()`, sin `taskkill /T` como `video::kill`. Con un shim de Windows puede dejar huérfanos al vencer el plazo de 15 s. Ya está anotado como abierto (progress.md:85). Reutilizar `video::kill`.
- **M6 · Cancelación durante la sonda.** `export.rs:915-924` emite `Planned` con las fichas de vídeo deshabilitadas antes de `Cancelled` (anotado desde T4). Devolver `Cancelled` antes de planificar.
- **M7 · Decodificadores del sidecar.** No están ProRes, DNxHD ni Opus/AV1. Un `.mov` ProRes, el formato habitual de motion design, pasa la validación por extensión y falla al exportar (la miniatura sí falla antes y da aviso). Añadir `prores` en la misma recompilación de C1 o avisar en el texto de la ficha.
- **M8 · MP3 promo.** Le falta `TCON` (spec §5) y lleva un APIC de la portada de 3000² (≈2 MB por MP3; el humo midió 861 kbps de bit_rate de formato). Valorar una APIC de 1000-1400 px.
- **M9 · `terceros.md`.**
  - §9 (:33) nombra seis marcas, sin TuneCore, aunque el texto de «Acerca de» (:40) ya tiene siete.
  - A la tabla de componentes le faltan moxcms (BSD-3/Apache), id3 (MIT) y zune-jpeg/pxfm.
  - El documento se empaqueta tal cual como `terceros.txt` para el usuario, con notas internas de CI y «pendiente de consulta legal». En hito 4, separar un aviso para el usuario del documento interno.
- **M10 · Comentario obsoleto en `video.rs:6-11`.** Se arregla con C1 (ver arriba).
- **M11 · Capturas obsoletas.** `t7-captures/listo-adm-*` son anteriores a da25062: muestran el halo del símbolo y la ranura «Listo» antigua. Regenerarlas o borrarlas para que no sirvan de referencia (la re-revisión de T7 ya lo pedía).
- **M12 · `export::analyze` (`export.rs:191-198`) rechaza > 96 kHz después de cargar el WAV entero.** Un máster de 30 min a 192 kHz son ~2,7 GB en f32. Hoy no ocurre porque el puente comprueba la cabecera antes (`analysis.rs:66-69`), pero `export::run` es API pública. Mover la comprobación de tasa junto a `check_length`.
- **M13 · Ranura de portada vacía en claro.** Se ve como caja negra (`v1-claro-es-sin-portada`). Está aceptado por ruling de Fer; lo dejo anotado para la revisión de diseño del hito 4.

## 5. Sin confirmar por CI

La CI está parada desde el push 9e5860f (facturación). Último run real de `app` en 9c214b9. Queda sin confirmar:

1. **Humo de `app` en macOS** con la corrección I1 de T8 (cfeaa0a) y `ADM=required`. En 9c214b9 fue rojo solo por el texto del informe.
2. **Humo de `app` en Windows y macOS con `setparams` (6a6848a).** Fallará por C1 hasta que haya sidecar nuevo. Nunca se ha ejecutado una cadena de vídeo con `setparams` contra el sidecar.
3. **`engine` en las tres plataformas sobre la rama rebasada** (94184e6..9e5860f encima de 9c214b9). El run verde 42140121081 fue antes del rebase, con ffmpeg choco/brew, no con el sidecar.
4. **Los cambios de workflow de d7c77a0.** Las condiciones `if:` con `[mac]` nunca se han evaluado. Con ellas, en ramas `hito*` el humo de macOS deja de correr salvo que se pida, y una regresión como C1 llegaría a `main` sin verse.
5. **`hevc_mf` (portada animada 3840² en Windows)**: nunca codificó de verdad (los runners no tienen la extensión HEVC).
6. **Texto crudo y código de salida de afclip** en el caso +1 dBTP: el artefacto `afclip-raw` nunca se leyó. El parser pasó con los dos casos reales en 74d507f.
7. **Comandos de Tauri en tiempo de ejecución** (`open_legal`, `open_path` con `plain_path`, `allow_asset`, `media_thumbnail`): solo compilados en Linux con la feature `app`. Los cubre el checklist de Fer en Mac; en Windows, nadie (huecos.md).
8. Slice x86_64 de macOS: solo la aceptación bajo Rosetta, nunca un Mac Intel nativo.

## 6. Deuda propuesta para el hito 4

- **Bloqueantes antes de vender o distribuir:**
  - repositorio público con fuentes, `configure_line` y licencias del sidecar, y URL real en «Acerca de» y en `terceros.md` (hoy es el marcador `{URL del release…}`, y `about.licenses` afirma que el fuente «está en el documento de terceros»);
  - consulta legal sobre las patentes AAC/H.264 y sobre LAME estático dentro del ejecutable LGPL;
  - texto de la licencia SIL OFL junto a `app/ui/fonts/HankenGrotesk.ttf` (no está en el repositorio);
  - avisos MIT/Apache de los crates (`cargo about`);
  - firma y notarización, con cada binario de `externalBin` firmado;
  - I3 (SHA de las acciones).
- I1 estructural: sidecar real en los tests de ejecución del motor de macOS y Windows.
- Humo a través de Tauri (tauri-driver) y tests `.cmd` de plazo/kill en Windows (huecos.md).
- `MASTERCHECK_SMOKE` presente en el binario de producción: compilarlo solo con una feature de CI.
- Probe y capacidades con el mismo `kill` de árbol que `run_ffmpeg` (M5); validación por sonda (no solo por extensión) del material propio (M4, M7).
- Portada animada: «bucle suave» y duración 8–35 s con vídeo propio (hoy siempre `-t 10`).
- Node 20 → 24 en Actions (anotado solo en `.superpowers/`).

## 7. Proceso y documentación (G)

- **G1.** `docs/decisiones.md` termina en D-30. No hay entrada para D-31 (opción B de marcas, «pendiente de ratificar con Fer»), ni para el ruling de CI (macOS solo en `main`/manual/`[mac]`, d7c77a0), ni para el cierre del hito 3. Añadir D-31 y una D-32, o la numeración que se elija, con el estado de ratificación.
- **G2.** La historia se lee bien y los mensajes están en español y por tarea. Incidencias ya registradas en el ledger y aceptadas sin reescribir: T2 entró en commits de T1 (8ac8972/43316da), y el `git rm` de T6 quedó dentro de 4c42343 (T7). No encontré archivos ajenos más allá de esos. 63 de 67 commits tienen como autor «Claude <noreply@anthropic.com>» y no el autor del repositorio que pedían los briefs: confirmarlo con Fer si importa.
- **G3.** Los diferimientos de Node 20 y de las tres acciones sin SHA solo constan en `.superpowers/` y en `check-wiring.py`. Llevarlos a `docs/pruebas/huecos.md` o a la deuda del hito 4.
- Spec y checklist al día con D-31 (adenda 06/11, §4 con `[Apple Music]`, `[TIDAL]`, `[Portada animada]`), salvo M1.

## 8. Veredicto

**Fusionar tras corregir: C1** (sidecar con `setparams`: release nuevo, tag, hashes, `SIDECAR_RELEASE` y docs) **+ la prueba pura de I1** (filtros, códecs y muxers del motor ⊆ lista blanca del sidecar), **y solo con la CI en verde en las tres plataformas en el HEAD final**:
- `app`: Linux; macOS con el humo pasando con `ADM=required`, lanzado a mano o con `[mac]`; Windows con su humo;
- `engine`: ubuntu, macOS y Windows.

I2 es recomendable en la misma ronda (cambio pequeño). I3 y lo demás pueden pasar al hito 4 con las anotaciones de §6.
