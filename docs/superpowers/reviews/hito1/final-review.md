# Final review: hito 1 motor (c17c0e8..09eacad, 16 commits)

Checked locally: `cargo test` with fixtures passes 87/87; `cargo clippy --all-targets -D warnings` is clean; `cargo fmt --check` is clean. CI run 42061507022: windows-latest and macos-15 passed (lame installed); ubuntu was still running when this was written. Licences in Cargo.lock: nothing GPL (flacenc and claxon are Apache-2.0, the rest MIT/Apache/BSD/Zlib; r-efi is MIT OR Apache OR LGPL, so a permissive option exists).

I also ran a scratch probe crate outside the repo (nothing in the repo was changed). It exercised: a mono master with empty artist and title; a silent master; a 0-frame master; masters at 88.2, 176.4, 192 and 384 kHz; absurd sample rates; and a real m96_24 master to 9 destinations.

## 1. Verdict

**APPROVE WITH FIXES.** The crate holds together and the plan's five focus points work. But FLAC export fails for every master above 96 kHz, the Amazon tag breaks contract §4 (the same bug class as the earlier Apple tag), and the English documents contain Spanish destination names. All three are small, contained fixes.

## 2. Findings (most severe first)

### Before merge

**C1 (Critical). FLAC fails for masters above 96 kHz.**
- Location: `src/flac.rs:105-115`.
- Cause: flacenc 0.5.1 rejects any rate above 96 kHz (`component/verify.rs:144`: `verify_range!("sample_rate", …, ..=96_000)`).
- Reproduced: with 176.4, 192 and 384 kHz masters, Spotify, Amazon and Tidal each give `JobFailed`, with the raw Debug text "must be less than or equal to 96000". 24/192 is a normal hi-res master format.
- Why tests missed it: no test or fixture goes above 96 kHz.
- Minimum fix: in `plan::resolve`, return `Disabled { reason_key: "flac_rate_over_96k" }` (with es/en keys) for FLAC jobs when the rate is above 96000, plus a 192 kHz synthetic test in `tests/plan.rs` and `tests/export.rs`.
- Proper fix (hito 2 at the latest): the plan's own plan B, `libflac-sys` (BSD-3), built on all three CI runners.

**I1. Amazon tag is outside contract §4.**
- Location: `presets.json:42`.
- Problem: the tag is `"Amazon"`, but §4 requires `[Amazon Music]`. The file is currently `… [Amazon] 24bit-96kHz.flac` (seen in the probe).
- Why tests missed it: `tests/plan.rs::subfolder_contract` pins subfolders only, never tags.
- Fix: set the tag to `"Amazon Music"`, and add one table test that asserts the `(subfolder, tag)` pair of all 13 presets against literal strings copied from §4 and the rulings. That test also covers the deferred "ordered subfolder set" item.

**I2. User-visible strings outside i18n in the documents.**
- `src/reports.rs:117-122`: `preset_name()` uses `Preset.name`, which exists only in Spanish. The English informe and README print "Distribuidora estándar (16 bit / 44,1 kHz)" and "TuneCore (24 bit / 192 kHz)".
- `src/reports.rs:135-141`: the prefixes `OK/WARN/FAIL` are hard-coded. In English this prints "[OK] OK"; in Spanish, untranslated English.
- Fix: add keys `preset.<id>` and `verdict.tag.{ok,warn,fail}` to es.json and en.json. The existing parity test then covers them.

### Hito 2

**I3. Memory: peak RSS was 1.1 GB on the real m96_24 master (2:22, 9 destinations, 2 threads, 19 s).**
- That is about 10× the 109 MB f32 master.
- `export.rs:212-228`: every native job (Spotify, Apple, Amazon, Tidal, Hi-Res) quantizes its own identical full `Vec<Vec<i32>>`.
- `flac.rs:99-104` makes another interleaved copy. `flac.rs:120-122` encodes to memory, then copies again with `to_vec()` and `insert_tags`.
- This grows with thread count and duration (an 8-core Mac with a 6-minute master would reach several GB).
- Fix: cache quantized samples by `(bits, rate, dither)` the way `cd` is cached, write the FLAC to the file as it is encoded, and drop the `to_vec`.

**I4. Errors are not localized.**
- `AudioError`'s Display text is Spanish (`audio.rs:39-53`). The inner messages in wav, flac, resample and loudness are Spanish or `{:?}` Debug dumps.
- `export.rs:419` forwards them verbatim into `JobFailed.error` and `Delivery.failed`, so the English UI would show Spanish or Debug text. The T10 fix only localized the export-level messages.
- Fix: `AudioError` carries an i18n key plus arguments (this absorbs the deferred `AudioError::Dsp` item), and Debug text is never shown to users.

**I5. A panic in a job or in the progress callback propagates out of `run`.**
- Location: `export.rs:394-429`.
- Effect: `.part` files are left behind and `Finished` is never sent.
- Fix: wrap each job in `catch_unwind` and turn a panic into `JobFailed`. Add a cancel token at the same time; the UI will need one.

**M1. Empty artist gives "- Title" names.**
- Seen in the probe: folder `- mono` and file `- mono [Spotify] 24bit-48kHz.flac` (`naming.rs:60-66`).
- `fallback_title` is applied only inside `run` (`export.rs:357`), so the UI preview built from `plan::resolve` would show different names than the export.
- Fix: normalise artist and title (fallback or validation) inside `resolve`.

**M2. A silent master prints "−inf".**
- Seen in the probe: "−inf LUFS/dBTP" in the informe and `-inf` in the CSV (`reports.rs:105-111, 257-259`).
- Fix: print "—" in the text documents and an empty field in the CSV.

**M3. The dither rule for the 16/44.1 path is written twice.**
- `export.rs:198-204` re-implements the plan's rule instead of using `job.needs_dither`, while the comment at `export.rs:171` says "only the plan decides".
- The result is the same today. Fix: use a single `plan::needs_dither(master, bits)` helper in both places.

**M4. `verdict` ignores `max_tp_dbtp` when the target is `None`.**
- Location: `loudness.rs:179-186`.
- Effect: Apple ADM and the distributor presets declare −1 dBTP but only warn above 0 dBTP, so that field is dead data.
- Fix: either use it as the warning threshold or remove it from presets.json.

**M5. Integer overflow in `wav::write`.**
- `wav.rs:413`: `sample_rate * block_align as u32` overflows and panics, reproduced via the public `wav::write`.
- From the export path it is not reachable, because ebur128 already fails with NoMem on absurd rates.
- Fix: `checked_mul`, and have the reader reject rates outside 8 kHz–768 kHz.

**M6. Windows long paths.**
- A folder of up to 200 bytes, plus the subfolder, plus a file of up to 200 bytes, plus `.part`, can exceed MAX_PATH.
- Rust's std copes with long paths, but `lame.exe` receives the long output path on its command line (`mp3.rs:283`).
- Fix: have lame write to a short temporary name in the same folder, or lower the name budget on Windows. Add a long-name test on the Windows runner.

**M7. Mono MP3 is untested.**
- Location: `mp3.rs:269` (`-m j` with mono input).
- Mono WAV and FLAC export work (7 destinations checked in the probe), but no test covers mono export.
- Fix: use `-m m` when the master has one channel, and add a mono export test.

**M8. Spec question: no dither when a 16-bit master is resampled.**
- A 16/48 master going to 16/44.1 (Deezer, distributor) is requantized after resampling without dither (`plan.rs:247`).
- This matches spec §5 ("only when lowering bits"), but common practice is to dither after a rate change. The CEOs should decide.

**M9. Positive true peak has no "+" sign.** The informe shows "1.2 dBTP" instead of "+1.2 dBTP" (`reports.rs:211-215`).

**M10. Test hygiene.**
- `tests/export.rs:422` only checks that the output is consistent with `needs`. It should also assert `needs == true`, which is the ruling for float masters.
- `failed_job_leaves_no_partial_files` is skipped whenever lame is present, which means on every CI runner. Trigger the failure another way.
- `tests/wav_write.rs:13,116` and `lib.rs:1` still say "Master Out".

**M11. Bidi characters are not stripped from names.**
- Location: `naming.rs:9-11`. `is_control()` does not catch format characters such as U+202E, which can make an extension look different from what it is.
- Fix: replace Unicode Cf characters as well.

**Spec question (not a bug).** For English users the subfolders ("07 Distribuidora", "09 Imagen") and the document file names ("informe-loudness.txt", "ficha-lanzamiento.csv") stay in Spanish. Contract §4 makes them binding as written; the CEOs should confirm before hito 2.

## 3. Triage of the ledger

| Deferred / parked item | Decision | Reason |
|---|---|---|
| checked_mul for byte_rate | hito 2 | This is M5: not reachable from export, but the public API can panic. |
| Tests: odd-size chunk, data not last, fmt 18, EXTENSIBLE float, odd mono writer | hito 2 | The real fixtures already cover JUNK + bext + fmt 40 + minf/elm1; the rest are edge cases. |
| data size 0 → read to EOF even when more chunks follow | hito 2 | Rare (streaming writers); at worst a few bytes of a trailing chunk are read as audio. |
| Dither seed per channel with mixing | drop | `seed_from_u64` already mixes the seed; consecutive seeds are independent (comment at dither.rs:53). |
| ChaCha8Rng for stable output | hito 2 | Cargo.lock pins rand 0.8; only matters when rand is upgraded. |
| Temp file + rename in the export layer | drop (done) | `atomic()` with `.part`, export.rs:144. |
| Epsilon on TP > 0 | drop | The ruling stands; a TP just above 0 is valid information. |
| Comment on the 997 Hz test; redundant assertion in the upsample test | drop | Cosmetic. |
| `AudioError::Dsp` variant | hito 2 | Fold into I4 (errors with i18n keys). |
| Invariant that all channels have equal length | drop (done) | Documented at audio.rs:13; the writers check it. |
| ebur128 does not oversample at ≥192 kHz | drop | Accepted; the error at those rates is negligible. |
| `while` loop without a cap in resample.rs:257 | hito 2 | Terminates today (`process_partial` always produces output); add a cap as a safeguard. |
| Windows reserved names (CON, NUL) | drop | Names always contain " - " plus a tag or extension, so they can never be exactly a reserved name. |
| Bidi control characters | hito 2 | M11. |
| distributor_hires cites DistroKid without a hi-res requirement | hito 2 | Data fix before release, not code. |
| `preset.name` only in Spanish | **before merge** | I2: already shows in the English documents. |
| `presets::all()` re-parses the JSON | hito 2 | Cache with OnceLock; cheap today, but `preset_name` calls it on every line. |
| Neutralize CSV fields starting with = + - @ | hito 2 | Low risk; the file is user-owned data. |
| No seek table in FLAC | drop | Accepted; short files decode fine. |
| Test of the ordered subfolder set | **before merge** | Extend it to tags (I1); that gap let Amazon slip through. |
| UTF-8 case in truncation | drop (done) | The test with 東 covers it. |
| lib.rs and test fixtures still say "Master Out" | hito 2 | Cosmetic (M10). |
| UI must not group destinations by folder name alone | hito 2 | Applies to the UI: 06 and 07 hold several destinations each. |
| JobProgress is coarse (0 / 0.6 / 0.8 / 1) | hito 2 | Fine without a UI; real progress needs encoding in steps. |
| A panicking callback aborts the pool | hito 2 | I5. |
| Real lame untested | drop | Tested on CI: lame is installed on Linux, Windows and macOS. |
| APIC with a real cover image untested | hito 2 | Covers arrive in hito 3; add a test with a tiny JPEG. |
| Tests for temp WAV cleanup and error localization | hito 2 | Goes with I4. |
| now_utc comment | drop | Cosmetic (a stray line at export.rs:91). |
| Rulings: UTC timestamps; float→24 bit with dither | keep | Consistent in code; M10 asks for a test that asserts the float case. |

## 4. Conventions worth keeping

1. **The plan decides, export executes.** `Job` carries bits, rate, resampling, dither, upsampling and metadata. Presets are data with their source and status. M3 is the only place that drifts from this.
2. **No silent overwrites and no half-written files.** Each file is written to `.part` and then renamed. Numbered delivery folders are created with a `create_dir` loop, so two exports cannot claim the same folder.
3. **Tests check against outside references.** Real masters are compared with ffmpeg values, untouched destinations must come out bit-exact, a converted file must stay within 0.2 LU of the master, contract strings are written out literally in the tests, and es/en must have the same keys. I1 shows why the literal-string rule has to cover every preset.
