# Report tasks 6-9 (masterout-core)
STATUS: DONE_WITH_CONCERNS
Commits: T7 47970d9, T8 d438891, T9 7cba0c1, T6 8524582 (all on hito1-motor, not pushed).
Tests: full suite green (naming 10, plan 11, reports 8, flac 5 + earlier tasks); cargo fmt and clippy --all-targets -D warnings clean.

## Decisions
- T6: flacenc 0.5.1 (Apache-2.0) does 24 bit (ffprobe: s32, bits_per_raw_sample=24, ffmpeg decodes with no errors); libflac-sys not needed. flacenc has no Vorbis comment API, so a VORBIS_COMMENT block is inserted after STREAMINFO (clears/moves the last-block flag). Dev-deps: claxon, metaflac.
- T8: serde_json moved to a normal dependency (presets parsed via Value). Preset has two extra fields: min_bits, min_bits_reason_key (apple 24 -> "apple_needs_24bit"; youtube_music 16). Subfolders: 01 Spotify, 02 Apple Music, 03 Amazon Music, 04 Tidal, 05 Deezer, 06 YouTube Music, 07 Distribuidora, 08 Promo (mp3), 09 Distribuidora Hi-Res, 10 TuneCore, 11 Portada, 12 Canvas, 13 YouTube. Float masters count as 32 bit: native delivers 24 with dither. estimate_bytes WAV has no header (matches 34,560,000).
- T9: keys for verdicts/reasons are plain (tp_over_platform, apple_needs_24bit, ...). Spanish numbers use "," and U+2212; CSV uses dots and ASCII minus.

## Concerns
- Tidal, Deezer, YouTube Music have no official source in docs/investigacion (marked POR VERIFICAR). Their source_url points to "docs/investigacion/specs-plataformas-ronda2.md (sin fuente oficial: por verificar)", status recommendation; not a real URL.
- Task order: tests were written alongside implementation, not strictly red-first (naming, plan, reports); flac's first run compiled with one test-API fix.
- Cargo.lock updated in the flac commit.

## Fix round 1 (5ab1718)
- Subfolders per spec: 06 YouTube (music+video), 07 Distribuidora (std, hires, tunecore), 08 Promo, 09 Imagen; tags Distribuidora / Distribuidora Hi-Res / TuneCore; presets::all() keeps JSON order (no sort). Tests: subfolder_contract, distributor_tags_and_note_sources.
- naming: delivery_folder uses saturating_sub and a 40-byte cap on version; tag, suffix and ext capped too; test with 300-char values.
- README prints "MP3 320 kbps" for MP3.
- "Master Out" -> "master check" in i18n, wav.rs ISFT (and its assertion in tests/wav_write.rs). src/lib.rs doc comment still says Master Out (outside allowed files).
- Preset.source_kind ("url" default, "note" for tidal/deezer/youtube_music).
- fmt + clippy clean; full suite green.
