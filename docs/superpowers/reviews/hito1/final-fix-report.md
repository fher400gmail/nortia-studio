
# Final fix report (C1, I1, I2, dither)

- C1 (ruling revised by CEO): masters above 96 kHz are rejected. `export::analyze` returns `AudioError::UnsupportedFormat("err.rate_over_96k")` (es/en keys added). `flac::write` returns `UnsupportedFormat("FLAC > 96 kHz")` above 96 kHz. The WAV fallback and note_key were not kept.
- tunecore preset now 24 bit, native rate, no metadata (name "TuneCore (24 bit)"). `is_upsampling` kept: youtube_music (fixed 48 kHz) still produces it.
- I1: Amazon tag "Amazon Music"; 13-row (id, subfolder, tag) table test in tests/plan.rs.
- I2: `preset.<id>` and `verdict.tag.*` keys; reports.rs uses them; English report test added.
- Dither: bits lower, or float source, or resampled with target <= 24 bit; Deezer 16/48 -> 16/44.1 now true.
- Tests: 92 pass with and without MASTEROUT_FIXTURES; fmt and clippy clean.
