# Huecos de prueba conocidos

Lo que hoy ningún test automático cubre. Cada hueco dice por qué y cuándo se cierra.

## Humo a través de Tauri

El humo del CI entra por `smoke_hook` (`app/src-tauri/src/lib.rs`) antes del `Builder`: ejecuta el motor y el sidecar, pero no pasa por la ventana ni por los comandos de Tauri (`commands.rs`). Ese tramo se lee a mano y lo recorre la persona en la lista de pruebas del hito. Se cierra en el hito 4 con `tauri-driver` (WebDriver).

## Plazo y kill de ffmpeg con `.cmd` en Windows

Los tests de plazo y de kill (`run_ffmpeg_with_deadline` y similares) son `cfg(unix)`: usan un script de shell como ffmpeg falso. En Windows un `.cmd` se lanza a través de `cmd.exe`, y matar el proceso hijo no mata al nieto. No hay test de eso hasta tener un PC con Windows. Lo cubre el humo de Windows en el CI, solo para el camino sin plazo vencido.
