# Report Tasks 4 y 5

Commits: Task 4 = 5b5e17f, Task 5 = see `git log` (second commit after it).

Tests: con fixtures: loudness 11/11, resample 5/5 (suite completa verde); sin fixtures: loudness 11/11, resample 5/5 (los de fixtures hacen skip). fmt y clippy -D warnings limpios.

Notas / decisiones:
- Test "true_peak_of_full_scale_sine...": un seno de amplitud 1.0 con fase pi/4 da muestras de +-0.707 y TP = 0 dBTP, no > +0.5. Interpretado como pico de MUESTRA = 0 dBFS (amplitud sqrt(2) en f32), TP ~ +3 dBTP; se asserta tambien sample_peak ~ 0.
- measure() alimenta en tramos de 100 ms (sr/10 frames) en vez de 4096, para poder usar prev_true_peak por tramo y contar overs_count (tramos de 100 ms con TP > 0 dBTP, documentado). Resultados I/LRA/TP identicos.
- verdict(report, Option<f64>, max_tp): con target: TP > max => Fail tp_over_platform; I > target+1 LU => Warn louder_than_target; I < target-1 => Warn quieter_than_target; sin target: TP > 0 => Warn tp_over_zero. Tolerancia de 1 LU es decision mia (el brief no la fija); test extra verdict_louder_than_target_and_no_target.
- Versiones: ebur128 0.1.11, rubato 0.16.2 (API SincFixedIn). serde_json en dev-deps.
- Cambio fuera de los ficheros pedidos: engine/Cargo.toml (workspace) con [profile.dev] opt-level = 2; en debug puro real_masters_match_ffmpeg tardaba ~196 s.
- Resample: SincFixedIn chunk 1024, max_relative_ratio 1.0, descarta output_delay, trunca a round(frames*ratio), vacia con process_partial. Sin ganancia; master m96 -> 44.1k mantiene I < 0.1 LU.
- No se toco wav.rs, audio.rs ni dither.rs. El paso rojo de resample se hizo con stub (el test no compilaba/fallaba antes de implementar).

## Fix round 1
- resample(): tail is now None when no frames are left over (frames % 1024 == 0 or empty buffer), going straight to process_partial(None); removed the extra to_vec copy (drain + truncate).
- New tests: exact_multiple_of_chunk_size_works (51200 frames, 96k->44.1k, 23520 +-2) and empty_buffer_returns_empty (Ok, 0 frames, 2 channels).
- Inter-sample test tightened to TP = +3.0 +-0.5 dBTP.
- Tests: resample 7/7 and loudness 11/11, with and without fixtures; fmt and clippy clean.
