# El Mac de Fer como runner de compilación (OPCIONAL)

Guía corta para Fer. Desde D-38 el repositorio es público y la CI corre por defecto en los runners de GitHub (minutos ilimitados), así que **este runner ya no hace falta**. Se conserva como opción (D-35), por ejemplo si GitHub vuelve a bloquear los runners alojados o quieres compilar en tu Mac. Los workflows están en `.github/workflows/`.

## Cómo se activa o desactiva

- Mac propio: crea la variable de repositorio `SELF_HOSTED` con valor `true` (Settings → Secrets and variables → Actions → Variables). Entonces corren además `mac-propio` (engine) y `check-propio` / `macos-propio` (app). Sin la variable, esos trabajos no corren aunque el runner esté instalado.
- Runners de GitHub: corren siempre; se apagan creando la variable `HOSTED_RUNNERS` con valor `false`.
- Solo el Mac alojado de GitHub se apaga con la variable `HOSTED_MAC` = `false` (Linux y Windows siguen corriendo).
- El release del sidecar (`ffmpeg-sidecar.yml`) corre en ubuntu de GitHub y exige Windows y **uno** de los dos macOS: el alojado (`macos`) o el de tu Mac (`macos-propio`).

## Cuando GitHub no da runners de Mac

Si los trabajos `macos-15` se cancelan sin runner (06/11/2026, D-38), Linux y Windows siguen en GitHub y lo de macOS se hace en tu Mac. En GitHub: Settings → Secrets and variables → Actions → **Variables** → New repository variable:

| Variable | Valor | Qué hace |
|---|---|---|
| `SELF_HOSTED` | `true` | Activan los trabajos del Mac propio: `macos-propio` (sidecar), `mac-propio` (engine), `check-propio` y `macos-propio` (app). |
| `HOSTED_MAC` | `false` | Apaga solo el macOS alojado (sidecar `macos`, app `macos`, engine `macos`). Linux y Windows no se tocan. |

- No pongas `HOSTED_RUNNERS=false`: apagaría también Linux, Windows y el trabajo `prepare` del sidecar, del que dependen los demás.
- Sidecar: `macos-propio` sube el artefacto `ffmpeg-sidecar-macos` (el mismo nombre que el alojado), así que `release` lo recoge sin cambios. Con `SELF_HOSTED=true` y `HOSTED_MAC` sin definir correrían los dos y el segundo en subir fallaría por nombre repetido: pon siempre las dos variables a la vez.
- App: el Mac propio sube `master-check-macos-propio` (y `master-check-humo-propio`, `afclip-raw-propio`); el `.dmg` está ahí, no en `master-check-macos`.
- En Apple Silicon el sidecar necesita Rosetta para probar el slice x86_64; sin ella, el release falla pronto con un mensaje claro (el release exige las pruebas de los dos slices).
- Para volver a lo normal: borra las dos variables (o pon `SELF_HOSTED=false` y `HOSTED_MAC=true`).

**Seguridad (repo público + runner propio):** Settings → Actions → General → «Fork pull request workflows from outside collaborators» → **«Require approval for all outside collaborators»**. Los pull requests de otras personas no deben correr en tu Mac: no apruebes ninguno que no hayas leído entero. Los trabajos del Mac propio solo corren en `push` a ramas del repo y `workflow_dispatch`; el motor (`engine-tests.yml`) también corre en `pull_request`, así que esa aprobación es imprescindible.

## Qué hace

Un programa de GitHub (el «runner») instalado en tu Mac recibe trabajos del repositorio y los ejecuta:

- **engine** (`engine-tests.yml`): formato, clippy (también cruzado a Windows), todos los tests del motor y la prueba real de Apple Music (`afclip`).
- **app** (`app-build.yml`): comprobaciones de la app, build universal (`.dmg`) y humo con el sidecar de ffmpeg junto al ejecutable.

## Seguridad

- **El repositorio es público: no instales este runner salvo que sea imprescindible.** En un repo público, cualquiera puede abrir un pull request y, si el runner propio lo acepta, ejecutaría su código en tu Mac. Si lo activas: Settings → Actions → General → «Fork pull request workflows» → «Require approval for all outside collaborators», y no apruebes PRs que no entiendas.
- No usa tokens personales: solo el `github.token` temporal de cada ejecución.
- Los workflows del Mac propio no usan `sudo` ni instalan nada: todo se instala una vez, a mano (abajo).
- Trabaja **solo mientras el Mac está despierto, con tu sesión iniciada** (el servicio es un agente de usuario). Si está dormido o apagado, los trabajos esperan en cola.
- Va de uno en uno y con un máximo de 4 hilos de compilación, para que el Mac siga usable.

## Pararlo y arrancarlo

En la carpeta donde instalaste el runner (por ejemplo `~/actions-runner`):

```
./svc.sh status
./svc.sh stop     # lo para (los trabajos quedan en cola)
./svc.sh start    # lo arranca
```

Estado también en GitHub: Settings → Actions → Runners (debe verse «Idle» o «Active»).

## Dónde queda cada cosa

- **El `.dmg`**: en GitHub, Actions → el run de «app» → abajo, **Artifacts** → `master-check-macos` (en el Mac propio, `master-check-macos-propio`; contiene el `.dmg` y el `.app` en zip). Se conserva unos 90 días. Otros: `master-check-humo` (informe del humo) y `afclip-raw` (registro de Apple Music), con sufijo `-propio` en el Mac propio.
- **Copia local**: el runner trabaja en `~/actions-runner/_work/nortia-studio/nortia-studio/`; el `.dmg` queda en `app/src-tauri/target/universal-apple-darwin/release/bundle/dmg/`. La carpeta `target/` se conserva entre ejecuciones para compilar más deprisa; si ocupa demasiado, bórrala (la siguiente ejecución tarda más una vez).
- **Release del sidecar**: Releases del repositorio, etiqueta `ffmpeg-sidecar-v…`.
- **Fixtures de másters reales (opcional)**: `~/mastercheck-fixtures`. Si la carpeta existe, los tests del motor la usan; si no, esos tests se saltan solos.

## Instalación (una vez)

1. Herramientas de línea de comandos de Apple: `xcode-select --install`
2. Homebrew (instalador en brew.sh) y las herramientas:
   `brew install gh gnupg nasm pkg-config node ffmpeg`
3. Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`, y después
   `rustup target add aarch64-apple-darwin x86_64-apple-darwin x86_64-pc-windows-gnu`
   (con rustfmt y clippy, que vienen en el perfil por defecto).
4. Solo si tu Mac es Apple Silicon: Rosetta, para probar el slice x86_64 del sidecar:
   `softwareupdate --install-rosetta --agree-to-license`
5. El runner como servicio: en GitHub, Settings → Actions → Runners → New self-hosted runner → macOS; sigue los comandos que muestra (`./config.sh --url … --token …`, etiquetas por defecto) y termina con
   `./svc.sh install` y `./svc.sh start`
6. Que el Mac no se duerma mientras haya trabajo: Ajustes del Sistema → Batería/Energía → evitar reposo automático con el adaptador conectado (o `sudo pmset -c sleep 0`). Conviene activar el inicio de sesión automático para que el servicio arranque tras reiniciar.
7. `gh auth login` no hace falta para los workflows (usan el token de la ejecución); sí te sirve para lanzarlos desde la terminal: `gh workflow run app-build.yml --ref hito3-media --repo fher400gmail/nortia-studio`.

## Si algo falla

El primer paso de cada trabajo comprueba las herramientas y dice cuál falta (por ejemplo «falta 'nasm' en el runner»). Instálala con Homebrew o rustup y vuelve a lanzar el run (Actions → Re-run).
