# Review Tasks 4-5 (4b01c1c..ba019d4)

## Spec compliance: ✅ (with one functional defect, see Important 1)
Every brief test exists with the brief's values. Missing: none. Extra, all justified:
- `verdict_louder_than_target_and_no_target` (tests/loudness.rs), `TARGET_TOLERANCE_LU = 1.0` (loudness.rs:8, implementer's choice), `serde_json` dev-dep, `[profile.dev] opt-level = 2` (accepted).
- Deviations accepted: 100 ms feed instead of 4096 (needed for `prev_true_peak`; I/LRA/TP are unaffected); LRA ±0.5 vs ffmpeg follows the brief (tighter than the plan's ±1).

Suite run with MASTEROUT_FIXTURES: loudness 11/11, resample 5/5, everything else green.

Verified:
- Generators: peak = 10^(dBFS/20); L and R identical and in phase (right for Tech 3341); 20 s (40 s for LRA) is more than enough for gating and the 3 s windows.
- `measure()`: planar f32 slices are passed straight through, nothing is scaled. `prev_true_peak` is per call, so the per-100 ms `overs_count` matches the doc at loudness.rs:19/45. Mono: ebur128 0.1.11 `default_channel_map` gives [Left], weight 1.0, so −26.0 as the brief expects. TP oversampling is 4x below 96 kHz and 2x from 96 to 192 kHz (crate docs, ebur128.rs:964). The silence case and the empty buffer give -inf with no error (I ran both).
- Inter-sample test: it does prove detection. Samples land at ±1.0 (sample peak ≈ 0 dBFS, asserted) while TP comes out ≈ +3 dBTP, and `overs_count > 0`.
- Resampler parameters match the brief exactly. `output_delay` is discarded, the tail is flushed with `None`, length is trimmed to `expected`, no gain is applied, and `unwrap()` appears only in tests.

## Findings

**Important 1: `resample()` returns Err when `frames % 1024 == 0`, including an empty buffer.**
- `resample.rs:53-57`. When no remainder is left, `tail` is `Some` of empty slices. rubato's `process_partial_into_buffer` then clears the padded vector and fails with "Insufficient buffer size 0 ... expected 1024".
- I reproduced it with a scratch crate: 51200 frames gives ERR, 0 frames gives ERR, 51201 and 100 are fine.
- Any real file with a multiple of 1024 frames would fail, roughly 1 in 1024. No test covers it.
- Fix: `let mut tail = (pos < frames).then(|| buf.channels.iter().map(|c| &c[pos..]).collect::<Vec<_>>());`. If the remainder is empty, go straight to `process_partial(None, ..)`. Add a test with 1024*k frames (for example 96000→44100 at exactly 1024*94) and one with an empty buffer.

**Minor 1: `resample.rs:62-65`.** `.to_vec()` copies every channel again, so peak memory is 2x the output. Use `c.drain(..delay); c.truncate(expected);`. Also, `process()` allocates a new output Vec on every chunk. `process_into_buffer` with a reusable `output_buffer_allocate` would avoid that. Neither is a correctness issue.

**Minor 2: `loudness.rs:93-112` and `verdict`.** `to_db(tp) > 0.0` is strict with no epsilon. A master that peaks exactly at 0 dBFS gets TP of +1e-7 and a spurious `Warn("tp_over_zero")`. Likewise `chunk_tp > 1.0` (loudness.rs:84) and the `tp > max_tp` comparison in `verdict` are strict. Fix: a tolerance such as `> 0.005` dB, or document that the comparison is strict.

**Minor 3: tests/loudness.rs `tech3341_true_peak_minus6`.** The comment says the peak falls between samples, but 997 Hz at 48 kHz sweeps the phase over 5 s, so some peak lands almost on a sample. The test is still valid against the tolerances but proves nothing about inter-sample detection. The test above it covers that. Soften the comment, or assert TP ≈ +3.0 ±0.5 in the inter-sample test, which is currently only `> 0.5`.

**Minor 4: tests/resample.rs `upsample_44_to_192_is_allowed`.** The second assertion (`frames/44100 ≈ 4.353`) is redundant with the first. The brief's "×4.353" would be clearer as `out/in`.

**Minor 5: error types.** ebur128 and rubato errors are mapped to `AudioError::UnsupportedFormat` (loudness.rs:35, resample.rs:11). The meaning is imprecise. Consider an `AudioError::Dsp(String)` variant (touches audio.rs, so it is a controller decision).

**Minor 6: `measure()` and `resample()` assume equal-length channels.** `&c[pos..end]` would panic on a ragged `AudioBuffer`. Probably guaranteed by wav.rs, but not enforced. Check `frames()` consistency, or note the invariant in `AudioBuffer`.

## ⚠️ Cannot verify from diff
- The accuracy of the 4x true-peak interpolator against ffmpeg beyond the four fixtures. Those pass here with ±0.3 dB, but the fixtures are not in the diff.
- At sample rates ≥ 192 kHz ebur128 does no oversampling, so TP equals sample peak. This is acceptable, but the 192 kHz upsample output has no inter-sample check.
- Sample peak vs TP definitions for the ADM fixture. It passes.

## Task quality: **Not approved** until Important 1 is fixed and a regression test is added. Everything else is Minor. I did not modify any project files; the scratch reproduction lives in the scratchpad (`chk/`).
