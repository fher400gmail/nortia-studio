# Task 11 report: CI del motor

Commit: 21a436b (ci: tests del motor en Linux, macOS y Windows), pushed to hito1-motor.
Workflow: .github/workflows/engine-tests.yml, name "engine". Triggers push + pull_request with paths engine/** and the workflow file. Concurrency per ref with cancel-in-progress. Jobs: linux (every push), macos and windows (only PR, main, hito* branches). Steps: fmt --check, clippy --all-targets -D warnings, test --all-features, all with --manifest-path engine/Cargo.toml. Windows lame/ffmpeg install is continue-on-error.

Run: https://github.com/fher400gmail/nortia-studio/actions/runs/42056634123
- ubuntu-latest: success, 3m13s
- windows-latest: success, 3m57s (choco install succeeded; fmt, clippy, tests passed)
- macos-latest: NOT RUN. "The job was not acquired by Runner of type hosted even after multiple attempts" (macOS arm64 capacity constraints). Cancelled after 15 min, twice (original + `gh run rerun --failed`). No steps executed, so no code signal.

Code fixes: none needed.

## Fix round 1
- 09eacad fix(ci): timeout por job y runner macos-15 (timeout-minutes: 30 per job; macos-latest replaced by macos-15 with dated comment).
- 16cde07 fix(ci): timeout de 8 min en la instalacion de lame y ffmpeg (apt-get hung >20 min on ubuntu in run 42061507022; install steps now timeout 8 min + continue-on-error, apt retries).
- Run 42061507022 (09eacad): macos-15 success (54 s, all steps executed incl. fmt/clippy/test), windows success; ubuntu hung in apt install and was cancelled by the 30 min timeout.
- Final run https://github.com/fher400gmail/nortia-studio/actions/runs/42069181963 (head 77ac9cf, a code commit by another session on top of 16cde07): all green. macos-15 40 s, windows 1m54s, ubuntu 1m07s. macos-14 fallback not needed.
