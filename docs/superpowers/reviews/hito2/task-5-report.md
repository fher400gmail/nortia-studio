# Informe tarea 5 · build en CI

STATUS: DONE_WITH_CONCERNS · commits dfca07b (workflow) y 111850d (verificación del .app en CI).

## Workflow `.github/workflows/app-build.yml`
- Disparo: push a `main` y `hito*` (paths app/**, engine/**, el propio workflow) y `workflow_dispatch`; `concurrency` por ref con cancel-in-progress; `timeout-minutes: 45` en los builds.
- `check` (ubuntu): apt libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf; `cargo check` y `clippy -D warnings` con `--features app`; `cargo test --no-default-features`.
- `macos` (macos-15, needs check): targets aarch64+x86_64; `npx --yes @tauri-apps/cli@^2 build --target universal-apple-darwin` en `app/` (app/package.json no trae el CLI; sin build de frontend). Sin firma ni notarización. Sube `master-check-macos` (dmg + zip del .app hecho con `ditto`).
- `windows` (windows-latest, needs check): `tauri build` (NSIS). Sube `master-check-windows`.
- engine-tests.yml no necesita cambios: `engine/Cargo.toml` ya tiene `mastercheck-bridge` como miembro, así que `cargo test` lo cubre.

## Resultados
- Run 42085232876 (dfca07b): check OK (5m37), windows OK (~12 min), macos OK (~6 min, tras 7 min en cola). Compiló a la primera: no hizo falta ningún fix de código en src-tauri.
- Run 42088328399 (111850d): check OK (1m15), windows OK (6m), macos: primer intento cancelado por GitHub ("job not acquired by Runner ... capacity constraints macOS arm64", 15 min en cola); `gh run rerun --failed` OK, build 4m08.
- URLs: https://github.com/fher400gmail/nortia-studio/actions/runs/42085232876 y .../42088328399

## Fixes de código
Ninguno.

## Artefactos
- master-check-macos: 11,8 MB (zip de GitHub) con `master check_0.1.0_universal.dmg` (5,7 MB) y `master-check-macos-app.zip` (5,6 MB); .app 13 MB, universal (x86_64 + arm64).
- master-check-windows: 2,4 MB (instalador NSIS `-setup.exe`; no hay MSI porque el target es solo nsis).

## Verificación del .app
`gh run download` falló: el proxy de este contenedor bloquea `productionresultssa10.blob.core.windows.net` (403/connect_rejected). Por eso la verificación se hizo DENTRO del job de macOS (PlistBuddy, falla el job si no coincide): `LSMinimumSystemVersion=14.0`, `CFBundleIdentifier=studio.nortia.mastercheck`, `CFBundleExecutable=mastercheck-app` (ejecutable en `Contents/MacOS/mastercheck-app`, `lipo -archs` = x86_64 arm64). El binario se llama `mastercheck-app` (nombre del crate), no `master check` como decía la brief; el bundle sí es `master check.app`. Si se quiere otro nombre: `mainBinaryName` en tauri.conf.json.

## Abrir el .app sin firmar (Mac)
Descargar el zip/dmg del artefacto, arrastrar a Aplicaciones; clic derecho en la app, Abrir, Abrir de nuevo en el aviso. Alternativa: `xattr -dr com.apple.quarantine "/Applications/master check.app"`. En Windows: SmartScreen, Más información, Ejecutar de todos modos.

## Preocupaciones / sin verificar
- Compilar y empaquetar no prueba el comportamiento en ejecución: ventana overlay, transparencia en Windows, drag&drop, asset protocol, CSP y humo (`MASTERCHECK_SMOKE`) siguen sin probar en máquina real o en un job de humo.
- `lame` no se empaqueta todavía (sin `bundle.resources`); el MP3 promo quedará deshabilitado en la app salvo que haya lame en PATH.
- macos-15 arm64 tiene capacidad intermitente (colas de 7 a 15 min, un cancelado); relanzar con `gh run rerun --failed`.
- El artefacto no se puede bajar desde este contenedor (host de blobs bloqueado); descargar desde la web de Actions.
