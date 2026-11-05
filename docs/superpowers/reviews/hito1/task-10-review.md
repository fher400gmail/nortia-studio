# Task 10 review (5ab1718..e4edd44)

Ran `cargo test --test export`: 7/7 pass (lame absent here, so MP3 test skipped; real fixtures not set).

## SPEC COMPLIANCE: ❌ (one confirmed contract violation)
Missing / wrong:
- Apple file tag: presets.json:23 has `"tag": "Apple Music"`; spec §4 (master-out-design.md:46) requires `[Apple Digital Masters]`. Confirmed. tests/export.rs (the `exports_full_tree…` and `written_wavs_have_expected_headers` cases) hard-code "Apple Music", so they pass against the wrong value and would not catch it.
- Brief test "no empty subfolders" and exact names: present (walk + is_file asserts), OK. The Hi-Res NOT-present and TuneCore (no bext/LIST) checks exist, OK.
- Brief says dither "según job": export.rs `samples()` never reads `job.needs_dither` (plan.rs:42); it re-derives via `dither::quantize` (dither.rs:29). Equivalent for PCM, but for float masters `quantize` adds TPDF even at 24->24 (`|| source_float`), contradicting plan's needs_dither=false. Not covered by a test.
Extra: `failed_job_leaves_no_partial_files` (fine).
Verified OK: no gain anywhere; PCM 0x0001; TuneCore without metadata (embed_metadata); ISRC not in WAV INFO (wav_metadata has none); subfolders created lazily (`create_dir_all` just before write, only after samples ok) and empty ones pruned; folder ` (n)` suffix via atomic `create_dir`; 4 docs at root; JobStarted/JobDone|JobFailed once per job, Finished last; rayon pool honors `req.threads` (default physical cores); 16/44.1 shared via OnceLock only by bits==16&&sr==44100 jobs and MP3; master buffer borrowed, not copied per job; `lame` search (exe dir then PATH), args `-b 320 --cbr -m j -q 0` + id3v2-only, stderr captured into the error -> JobFailed; id3 rewritten as v2.3 with TIT2/TPE1/TALB/TYER/TSRC/TCOM/TPUB/TCOP/APIC; no ffmpeg in production; deps MIT/Apache.

## Findings
Critical
- C1 Apple tag (above). Fix: presets.json:23 -> `"Apple Digital Masters"`; update both test tuples/names to it; re-check any reports/i18n fixtures that print the tag.

Important
- I1 Document write failure aborts `run`: export.rs the `write_text(...)?` loop returns Err with no `Finished`, leaving a half-built delivery folder, although audio jobs succeeded. Fix: collect doc errors into `failed` (e.g. id "documents") and still emit Finished; or at least emit Finished before returning.
- I2 User-visible strings bypass i18n: `"lame terminó con …"` (mp3.rs:152), `"contenedor …"` (export.rs:257), `"rayon: …"`, `"id3: …"`, plus `"lame not found"` (required literal by brief, acceptable) and fixed MP3 comment "nortia. studio - master check" (export.rs id3_tags). JobFailed.error reaches the UI. Fix: give errors i18n keys (or return structured codes mapped at the UI layer) and take the comment from i18n or meta.
- I3 Float-master dither: see spec note; use `job.needs_dither` (call `quantize_with(.., if job.needs_dither {Tpdf} else {None})`) so the plan is the single source of truth, and add a test with a 32f master -> 24 bit asserting no dither.

Minor
- M1 `resample::resample` returns `buf.clone()` when rate equal (resample.rs:20), so for a 44.1 kHz master `cd_samples` copies the whole master once. Accept, or short-circuit using `&buf.channels`.
- M2 `lame.as_deref().expect("lame comprobado")` (export.rs, MP3 arm): safe by construction but is an expect on the data path; restructure so the lame path is passed into the closure by value (match on `Option` in the `Mp3` arm).
- M3 MP3 job writes a temp `x.mp3.part.wav` in the delivery subfolder; removed on both paths, but a panic/kill leaves it. Prefer `std::env::temp_dir()`/tempfile dir (also keeps the delivery folder clean while running).
- M4 Windows: `fs::rename` replaces an existing file via MoveFileEx, and targets are always fresh anyway; no issue. The `.part` suffix keeps same-volume rename. Fine.
- M5 `JobProgress` is coarse (0, 0.6, 0.8, 1.0); `fraction` for wav/flac not tied to real work. Acceptable for now.
- M6 `created_at`/bext origination in UTC (report concern 2): acceptable, document it.
- M7 `Progress` callback is invoked from rayon threads; fine given `Sync`, but a panicking callback would abort the pool; no catch_unwind. Note only.

## ⚠️ Cannot verify from diff / here
- Real `lame` CLI never run: the MP3 test skipped in my run; flags and `--add-id3v2 --id3v2-only` + `--tc` behavior with real LAME unverified (implementer used an ffmpeg wrapper). Also whether `id3` `write_to_path` replaces LAME's tag cleanly (no duplicate ID3v2) with real output, and APIC with real cover.
- `real_master_end_to_end_under_time_budget` and the <0.2 LU check not run by me (no MASTEROUT_FIXTURES set); implementer reports 26-35 s.
- Windows behavior of `lame.exe` lookup next to binary.

## Task quality: NOT APPROVED
Blocked on C1 (confirmed spec violation hidden by tests asserting the wrong tag). Fix C1, I1, I3 (and ideally I2) then re-review; the rest is solid.
