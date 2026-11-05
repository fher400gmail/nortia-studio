# Task 10 report

STATUS: DONE_WITH_CONCERNS
Commits: ff325fd (mp3 + deps + doc de audio.rs), e4edd44 (export + lib.rs + tests)

Tests: 7/7 in tests/export.rs and the whole suite pass, with fixtures (MASTEROUT_FIXTURES set), both without lame
(mp3 test skips with message) and with a local ffmpeg-based `lame` wrapper on PATH (outside the repo, verification only).
Real master 2:22 with 6 destinations: 26-35 s on this 2-core container. fmt and clippy -D warnings clean.

Implemented: mp3::{Id3Tags, find_lame, encode, encode_with}; export::{ExportRequest, Progress, Delivery, analyze, run}.
- Atomic writes (`<name>.part` in the same folder, rename on success, removed on error), also for the 4 documents.
- Subfolders created lazily by the job that writes; empty ones removed after failures.
- 16/44.1 quantized samples computed once (OnceLock) and shared by Deezer, distributor_std and MP3.
- Extra test failed_job_leaves_no_partial_files (without lame).
- audio.rs: doc comment on `channels` about equal length.
- Deps added: rayon, id3, num_cpus (all MIT/Apache).

Concerns:
1. Brief names the Apple file tag "[Apple Digital Masters]" but presets.json tag is "Apple Music"; tests use the real preset tag.
2. created_at / bext origination use UTC (no tz database without extra deps).
3. The real lame CLI was never run here (not installed); the lame path was verified only through an ffmpeg wrapper. Option set follows the brief; ID3 is rewritten by the id3 crate afterwards.
4. Cargo in this container needed CARGO_HTTP_PROXY=http://127.0.0.1:46509 (stale proxy in ~/.cargo/config.toml).
5. MP3 comment text is fixed "nortia. studio - master check"; ReleaseMeta.featuring/version not written to tags.

## Fix round 1
- C1: presets.json Apple tag is now "Apple Digital Masters"; export tests updated.
- I1: document write errors go to `failed` ("documents"); Finished always fires; test document_failure_is_reported_and_finished_still_fires (a directory named README.txt forces the failure; chmod is useless as root).
- I3: dither comes from `job.needs_dither` via `quantize_with` (shared 16/44.1 cache uses the plan's rule for 16 bit). Note: plan.resolve marks needs_dither=true for float32 -> 24 bit (compares against 32), so dither IS applied there; tests assert the output follows the plan flag. Integer 24->24 master: bit-exact (test).
- I2: new i18n keys export.err.{lame_missing,lame_failed,container,threadpool,id3,documents} and mp3.comment (es+en); mp3::encode_with now takes a Lang first (it is my own function; `encode` signature unchanged, uses Lang::from_system).
- M1: no clone when master is already 44.1 kHz; M2: match instead of expect; M3: MP3 intermediate WAV in std::env::temp_dir(). UTC documented.
- Tests: export 10/10, plan 13, reports 8, full suite green, with and without fixtures (and with the ffmpeg lame wrapper). fmt and clippy clean.
