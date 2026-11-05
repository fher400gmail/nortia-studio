# Re-review Tasks 4-5 (ba019d4..2945c91) - resample.rs, tests/resample.rs, tests/loudness.rs

1. Important (Err when frames % 1024 == 0 / empty): ADDRESSED.
   - src/resample.rs:56-57: tail = (pos < frames).then(..) -> None when nothing is left over; line 59 passes tail.as_deref() = None to process_partial(None, ..) (flush), line 60 resets to None.
   - tests/resample.rs:82-88 (51200 @ 96k->44.1k, asserts 23520 +-2) and :90-102 (empty buffer: Ok, 0 frames, 44100 Hz, 2 channels). Both assert what they claim; both pass.
2. Minor (extra .to_vec()): ADDRESSED. src/resample.rs:66-73 drain(..delay) + truncate(expected). Loop at :58 guarantees len >= delay+expected, so drain cannot panic; result is identical to c[delay..delay+expected] (same length and delay compensation).
3. Minor (TP assertion): ADDRESSED. tests/loudness.rs:86-90 asserts |TP - 3.0| <= 0.5 (passes).

Test run: cargo test --test resample --test loudness -> loudness 11/11 ok, resample 7/7 ok.

New breakage (Critical/Important): none. No new unwrap() in src/resample.rs data paths (unwraps only in tests).
Deferred: the while loop at :58 has no iteration cap if process_partial ever returned zero frames (pre-existing, not reachable in tests).

Final verdict: all findings addressed - yes.
