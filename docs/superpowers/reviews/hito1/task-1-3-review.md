# Review Tasks 1-3 (c17c0e8..4b01c1c)

Verified: wav_read/wav_write/dither pass (22 tests), also with MASTEROUT_FIXTURES. (Full `cargo test` currently fails to compile only because the working tree holds uncommitted Task 4 files, tests/loudness.rs; not part of this diff.)

## Spec compliance: OK
- All brief test names exist with the brief's values (fixture durations per ffprobe ruling; TPDF window [0.45,0.55] with calculation documented in tests/dither.rs:1054-1060).
- Header offsets correct (wav_write.rs test checks 16..36). bext field sizes: 256+32+32+10+8+8+2+64+10+180 = 602, debug_assert + test. No ISRC. Chunk order fmt, bext, LIST, data. Odd-chunk padding in reader (wav.rs:609) and writer (put_chunk, data pad wav.rs:903). Dither per channel seed+c, StdRng; clamp [-2^(b-1), 2^(b-1)-1] correct.
- Extra (harmless, not asked): tests rejects_non_wav and rejects_unsupported_bits_and_channel_mismatch; writer clamp of samples (wav.rs:897, brief says write "only serializes"); IPRD from album (wav.rs:797, brief silent); `pub const BEXT_SIZE`.
- Missing: none.

## Findings
- Critical: none.
- Important: none.
- Minor
  1. wav.rs:876 `sample_rate * block_align as u32` overflows (panics in debug) for sample_rate > ~715M (bogus input). Fix: `sample_rate.checked_mul(block_align as u32).ok_or(UnsupportedFormat)?`.
  2. Test gaps for behavior that the code does implement: no test for odd-sized chunk (pad byte) in the reader, `data` followed by another chunk, `data` size 0xFFFFFFFF/0, fmt of 18 bytes, EXTENSIBLE float, or writer with odd data_len (mono 24-bit, odd frames, pad counted in RIFF size). tests/wav_write.rs:1350 uses stereo so data_len is always even. Fix: add a mono 24-bit 1001-frame write test asserting riff == len-8 and the pad byte, and one hand-built reader test with an odd JUNK (size 3) before fmt and a LIST after data.
  3. wav.rs:596-603: a legitimate empty `data` (size 0) followed by other chunks is read as "to EOF" (garbage frames). Brief mandates the rule; consider treating size 0 as EOF-length only if it is the last chunk by header (RIFF size) — optional.
  4. dither.rs:431 seed+channel: Tpdf{seed:s} channel 1 equals Tpdf{seed:s+1} channel 0, so sequential per-track seeds share noise streams. Fix: derive `seed ^ (c as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)`.
  5. dither.rs:382 `StdRng` algorithm is not guaranteed stable across rand major versions, which weakens "deterministic output". Brief mandates StdRng; Cargo.lock pins 0.8.8. Optional: `rand_chacha::ChaCha8Rng` if bit-exact reproducibility across upgrades matters.
  6. dither.rs:405 `|| source_float` is redundant (float sources are 32-bit, so target<source already true); keep only if source_float with bits<32 ever appears. dither.rs:416 `assert!` panics on bad target_bits; acceptable given the mandated signature, but callers must validate.
  7. wav.rs:867 writer leaves a partial file on I/O error; ICRD is omitted when `year` is None (brief test only covers Some). Document or write to temp + rename in the CLI layer.
  8. wav.rs:786/807 `body.len() as u32` truncating casts; safe in practice (LIST is tiny, bext fixed), no action.

## Cannot verify from diff
- Whether downstream tasks call `quantize` with `source_bits` for float inputs consistently (float 32 → 24 gets TPDF; matches the plan's "lowering bit depth" reading only if float 32 counts as higher than 24).
- bext Version=1 with five loudness i16 fields: wire-identical to v1 (all zero), but the layout is the v2 split; confirm the plan intends v1 labeling with zeroed loudness (matches the brief verbatim).
- ffprobe duration values per the ruling; real-fixture test ran and passed here.

## Task quality: Approved
