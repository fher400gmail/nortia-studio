# Review Tasks 6-9 (masterout-core), range ba019d4..8524582 (excl. 2945c91)

`cargo test`: all green (flac 5, naming 10, plan 11, reports 8). Probed `naming` from an external scratch crate (nothing modified in the worktree).

## SPEC COMPLIANCE: ❌ (one binding-contract violation, plus one panic)

Missing / diverging
- **Subfolder contract broken** (`presets.json`: ids spotify..youtube_video). Contract: `01 Spotify 02 Apple Music 03 Amazon Music 04 Tidal 05 Deezer 06 YouTube 07 Distribuidora 08 Promo 09 Imagen`, with `[Distribuidora]` AND `[Distribuidora Hi-Res]` both inside `07 Distribuidora/`. Delivered: `06 YouTube Music` (not `06 YouTube`), `09 Distribuidora Hi-Res`, `10 TuneCore`, `11 Portada`, `12 Canvas`, `13 YouTube`, so there is no `09 Imagen`. The Hi-Res tag is `Hi-Res`, not `Distribuidora Hi-Res` (presets.json `distributor_hires`). Brief T8 only fixes `"01 Spotify"` in its test, so the tests pass, but the binding constraint does not.
- Everything else verified:
  - Folder and file formats and kHz labels (44/48/96/192, plus 88/176 per brief): `naming.rs:50-60,88-100`.
  - Forbidden chars to `-` without collapsing; accents and non-Latin kept.
  - FLAC: ffprobe `s32` / `bits_per_raw_sample=24` and ffmpeg decode with empty stderr are asserted in `tests/flac.rs`; claxon round-trip is lossless at 16 and 24 bit; metaflac reads TITLE/ARTIST/DATE/ISRC/ALBUM/COPYRIGHT.
  - Presets: 13 entries, all with non-empty `source_url`. Values match the brief and `docs/investigacion/specs-plataformas-2026-11-05.md` (Spotify -14/-1, CD Baby, DistroKid, TuneCore 24/192 no-embed, Canvas, YouTube, Apple ADM).
  - `cover`, `canvas` and `youtube_video` return `Disabled{"not_in_this_build"}` (`plan.rs:66`).
  - Apple 16 bit returns `apple_needs_24bit`; native never upsamples; TuneCore sets `is_upsampling`; the verdict uses preset targets and `tp_over_zero` appears with target None.
  - i18n: es/en key sets identical (the test passes and I diffed them); missing key returns `⟦k⟧`; Spanish uses comma and U+2212 (`reports.rs:19-33`), English uses dot; CSV header exact (`reports.rs:~185`); README lists each job once in order and contains `readme.master_untouched`; there is no GPL dependency (flacenc Apache-2.0, claxon/metaflac dev only).
- Extra (acceptable): `Preset.min_bits` / `min_bits_reason_key` (plan.rs:75-85) and `youtube_music` with min 16, which the brief did not ask for.

## Findings

**Critical**
1. Subfolder numbering/tags diverge from the contract (see above). Fix: restore the contract.
   - Subfolders: `06 YouTube`, `07 Distribuidora` for `distributor_std`, `distributor_hires` AND `tunecore`, `08 Promo`, `09 Imagen` for cover/canvas/youtube_video.
   - Tags: `Distribuidora` (std), `Distribuidora Hi-Res` (hires).
   - Update `tests/plan.rs` and the README test (the "05 Deezer/", "08 Promo/" order still holds). `presets::all()` sorts by `subfolder`; with shared subfolders keep a stable order, or sort by JSON order.
   - If the renumbering was intended, the spec owner must amend the contract first; it can't be changed unilaterally.

**Important**
2. `delivery_folder` panics with a version >= 198 bytes (`naming.rs:75`, `MAX_BYTES - suffix.len()` underflows; I reproduced `attempt to subtract with overflow` with 199 chars). In release it wraps and the name exceeds 200 bytes. Fix: `MAX_BYTES.saturating_sub(suffix.len())` and truncate the version first (e.g. `sanitize_max(v, 100)`), then add a test with a 300-char version.
3. `audio_file_name`/`media_file_name` can exceed 200 bytes when `tag`/`suffix`/`ext` are long (probe: 300-char tag gives 220 bytes). Internal inputs only, but the contract says <= 200. Fix: truncate the tag/suffix too (`compose` only trims artist/title).

**Minor**
4. Path safety: `..`, `.`, empty and `\0` can't escape. `sanitize_component("..")` gives `""` and `delivery_folder("","..")` gives `-`; there are no separators in any component. Still, nothing rejects a leading dot (".hidden" is allowed), Windows reserved names (CON, NUL) or bidi controls (U+202E is kept). Consider a `-` prefix for leading `.` and reserved names.
5. README shows `{bits} bit` for MP3 (`reports.rs` readme) using the master's bits (`plan.rs` Job.bits for mp3_promo, bits null). Print "MP3 320" without bits, or set `Job.bits=0`/Option.
6. `source_url` for tidal/deezer/youtube_music is a doc path plus text, not a URL (honest, matches the research: "POR VERIFICAR"). The UI must not render it as a link. Better: add `source_kind` or make `source_url` nullable once the spec allows it. `distributor_hires` cites a DistroKid page that mentions no hi-res requirement; mark its status honestly.
7. `distributor_std.embed_metadata=true`, while the v2 research table says "sin embeber" for the standard distributor. The brief is silent; confirm with the owner.
8. `loudness_report` / `readme` print `preset.name` in Spanish only (`reports.rs preset_name`), and `presets::all()` re-parses JSON on every call. Also the i18n strings and `wav.rs:340` still say "Master Out" (the app is now "master check"). One string change (es/en `readme.master_untouched`, `readme.generated`) is needed, but note the brief fixes `readme.master_untouched` verbatim as "Master Out mide y avisa", so ask before changing it.
9. `presets.rs` uses `panic!`/`unwrap()` on the embedded JSON. It is a compile-time constant covered by the 13-entry test, so acceptable. No `unwrap()` on data paths elsewhere.
10. CSV: fields starting with `=+-@` are not neutralised (spreadsheet formula injection from user metadata). Optional.
11. Process: tests were written alongside the implementation, not red-first (the implementer disclosed it).

## FLAC block layout (verified)
Vendor LE u32 + string, comment count LE u32, each comment LE-length-prefixed. Block header is `0x04 | last`, followed by a 24-bit BE length. The insertion point after STREAMINFO is correct (42 bytes). The last flag is cleared on STREAMINFO and moved to the Vorbis block only if STREAMINFO was last. metaflac and ffmpeg read it with no errors.

## ⚠️ Cannot verify from diff
- Whether `ffprobe`/`ffmpeg` ran in CI. The test skips silently when absent; it ran here.
- Seek table: flacenc writes none, which is fine for delivery but slows seeking.
- Official sources behind -15 (Deezer), -2 TP (Amazon), Tidal and YouTube Music loudness: none exist in docs (already flagged as recommendation).

## Task quality: NOT APPROVED
Blocked by #1 (contract) and #2 (panic on user metadata). Everything else is solid: the FLAC writer, the number formatting, the i18n parity and the verdict logic are good. After #1-#3 are fixed with tests, I would approve.
