# Informe tareas 1-3 (masterout-core)

STATUS: DONE_WITH_CONCERNS
Commits: 58324fb (T1), b8b090d (T2), 4b01c1c (T3). Sin push.

## Construido
- engine/ workspace + crate masterout-core (edition 2021; deps thiserror 2, rand 0.8; dev tempfile 3).
- audio.rs: AudioBuffer, AudioError (variantes exactas del brief).
- wav.rs lector: recorre chunks (padding a par), JUNK/otros antes o después de fmt, fmt 16/18/40, EXTENSIBLE (subformato GUID), data 0/0xFFFFFFFF/truncado => hasta EOF, RF64/BW64 => Rf64Unsupported, >2 canales => TooManyChannels (también en probe), lectura de datos por bloques de 1 MiB (nunca el archivo entero en bytes).
- wav.rs escritor: fmt 16 bytes tag 0x0001, orden fmt, bext?, LIST?, data; BufWriter y bloques de 64k frames; clamp defensivo al rango; byte de padding tras data solo si el tamaño es impar (requisito RIFF; RIFF size lo incluye). bext 602 bytes; LIST/INFO: INAM, IART, IPRD (si album), ICRD (si year), ISFT="Master Out"; nunca ISRC.
- dither.rs: Dither{None,Tpdf{seed}}, quantize, quantize_with. Seed por canal = seed+canal (StdRng::seed_from_u64). quantize usa semilla fija constante (determinista).

## Tests
- sin fixtures: `cargo test` => 22 passed (10 wav_read, 6 wav_write, 6 dither).
- con `MASTEROUT_FIXTURES=/home/claude/testdata/masters`: 22 passed (reads_real_masters ejecuta de verdad).
- cargo fmt + `cargo clippy --all-targets -- -D warnings` limpios.
- Extra: lectura m96_24 -> quantize 16 -> write con metadata -> ffprobe: pcm_s16le, 96000, 2ch, 142.406 s, tags correctos (ejemplo temporal, eliminado).
- RMS TPDF: teórico 0,50 LSB (var TPDF 1/6 + redondeo 1/12 = 1/4), ventana [0,45, 0,55], comentado en el test; se prueba con 0,25 y 0,2501.

## Concerns / desviaciones
1. El brief mapea mal duraciones de fixtures: según ffprobe m44_24=129,33 s, m96_24=142,41 s (m48=185,41, adm=187,20 correctos). El test usa los valores reales de ffprobe.
2. Entorno cargo: ~/.cargo/config.toml tenía el puerto de proxy obsoleto (38509; real 38629, lo corregí) y además hace falta `no_proxy= NO_PROXY=` al descargar crates (index.crates.io está en no_proxy y no resuelve directo). Ya hay Cargo.lock y deps cacheadas.
3. En T2 escribí implementación y tests en el mismo paso, sin ejecutar antes el rojo (T1 y T3 sí lo tuvieron). Los tests pasan y cubren lo pedido.
4. ICRD solo se escribe si year es Some; si el controlador quiere ICRD siempre, usar origination como respaldo.
5. Tests añadidos fuera del brief: rejects_non_wav, rejects_unsupported_bits_and_channel_mismatch.
6. quantize_with hace assert! si target_bits fuera de 2..=32 (no es entrada de usuario).
