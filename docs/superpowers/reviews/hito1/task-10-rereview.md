# Task 10 re-review (e4edd44..872fccd)
Tests: export 10/10, reports 8/8 pass.
- C1 ADDRESSED: presets.json:23 tag "Apple Digital Masters"; tests/export.rs asserts the file name in both full-tree and header tests.
- I1 ADDRESSED: export.rs:466-480 no `?`, pushes ("documents", i18n msg) to failed; Finished at :488 always reached. Test tests/export.rs:339 (README.txt dir) asserts failed + last event Finished.
- I3 ADDRESSED: export.rs:175 dither_for, :226 job.needs_dither via quantize_with; cd path (:204) reproduces plan rule (float counts as 32). Tests :377 (float follows plan flag) and :426 (24-bit integer native bit-exact).
- I2 ADDRESSED: export.rs:259,273,289,310,369,475; mp3.rs:30,129,163 use t/tf. es/en both have 42 keys, symmetric set (also enforced by reports test). export calls encode_with(self.lang, ...) at export.rs:326.
- M1 ADDRESSED (export.rs:187, no clone at 44.1k). M2 ADDRESSED (let-else at :309, no expect). M3 ADDRESSED (temp_dir at :314; removed unconditionally at :335 after the and_then, so on success and on error; unique name via pid+counter+preset). UTC doc at :89 (cosmetic: stray blank `///` line).
New Critical/Important breakage: none.
Deferred: (1) no test asserts the temp WAV is gone or that error text is localized; (2) mp3::encode (no-lang) uses Lang::from_system(), fine for public API; (3) crash/kill leaks temp WAV in temp_dir; (4) float test would pass trivially if needs==false, but plan marks true for float->24 per controller ruling; (5) tidy the now_utc doc comment.
Verdict: yes
