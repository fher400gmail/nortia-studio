# El Mac de Fer como runner de compilación

Guía corta para Fer. Decisión: D-35 (CI sin pagar). Los workflows están en `.github/workflows/`.

## Qué hace

Es un programa de GitHub (el «runner») instalado en tu Mac. Cuando se sube código al repositorio privado `fher400gmail/nortia-studio`, GitHub le manda trabajos y tu Mac los ejecuta:

- **engine** (`engine-tests.yml`): formato, clippy (también cruzado a Windows), todos los tests del motor y la prueba real de Apple Music (`afclip`).
- **app** (`app-build.yml`): comprobaciones de la app, build universal (`.dmg`) y humo con el sidecar de ffmpeg junto al ejecutable.
- **ffmpeg-sidecar** (`ffmpeg-sidecar.yml`): compila ffmpeg/ffprobe y publica el release. Sin Windows: el release lleva solo los binarios de macOS.

Los trabajos de Windows y los de los runners de GitHub quedan apagados. Solo se encienden si creas la variable de repositorio `HOSTED_RUNNERS` con valor `true` (Settings → Secrets and variables → Actions → Variables). No lo hagas mientras GitHub tenga la facturación bloqueada: saldrían en rojo.

## Seguridad

- Ejecuta **solo lo que hay en este repositorio privado**. No aceptes ni actives forks ni colaboradores que no conozcas: quien pueda lanzar un workflow ejecuta código en tu Mac.
- No usa tokens personales: solo el `github.token` temporal de cada ejecución. El workflow que publica el release es el único con permiso de escritura.
- Los workflows no usan `sudo` ni instalan nada: todo lo necesario se instala una vez, a mano (abajo).
- Trabaja **solo mientras el Mac está despierto, con tu sesión iniciada** (el servicio es un agente de usuario). Si está dormido o apagado, los trabajos esperan en cola y se ejecutan al volver.
- Va de uno en uno y con un máximo de 4 hilos de compilación, para que el Mac siga usable. Un build universal o el sidecar pueden tardar bastante.

## Pararlo y arrancarlo

En la carpeta donde instalaste el runner (por ejemplo `~/actions-runner`):

```
./svc.sh status
./svc.sh stop     # lo para (los trabajos quedan en cola)
./svc.sh start    # lo arranca
```

Estado también en GitHub: Settings → Actions → Runners (debe verse «Idle» o «Active»).

## Dónde queda cada cosa

- **El `.dmg`**: en GitHub, Actions → el run de «app» → abajo, **Artifacts** → `master-check-macos` (contiene el `.dmg` y el `.app` en zip). Se conserva unos 90 días. Otros artefactos: `master-check-humo` (informe del humo) y `afclip-raw` (registro de Apple Music).
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
