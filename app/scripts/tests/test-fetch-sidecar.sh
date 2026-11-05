#!/usr/bin/env bash
# Prueba de fetch-sidecar.sh con un `gh` falso (sin red). Uso: bash tests/test-fetch-sidecar.sh
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
FETCH="$HERE/../fetch-sidecar.sh"
T="$(mktemp -d)"; trap 'rm -rf "$T"' EXIT
fail=0
ok()   { echo "ok   $1"; }
bad()  { echo "FAIL $1"; fail=1; }
sum()  { if command -v sha256sum >/dev/null; then sha256sum "$1" | cut -d' ' -f1; else shasum -a 256 "$1" | cut -d' ' -f1; fi; }

# gh falso: `gh release download TAG --repo R -p PAT... -D DIR [--clobber]` copia de $FAKE_RELEASE
cat > "$T/gh" <<'GH'
#!/usr/bin/env bash
[ "$1" = release ] && [ "$2" = download ] || { echo "gh falso: $*" >&2; exit 9; }
shift 2; tag="$1"; shift
echo "$tag" >> "$FAKE_LOG"
pats=(); dir=.
while [ $# -gt 0 ]; do case "$1" in -p) pats+=("$2"); shift 2;; -D) dir="$2"; shift 2;; --repo) echo "repo=$2" >> "$FAKE_LOG"; shift 2;; *) shift;; esac; done
for p in "${pats[@]}"; do for f in "$FAKE_RELEASE"/$p; do [ -f "$f" ] && cp "$f" "$dir/"; done; done
GH
chmod +x "$T/gh"

mkrelease() { # $1 dir
  mkdir -p "$1"
  for n in ffmpeg-universal-apple-darwin ffprobe-universal-apple-darwin ffmpeg-x86_64-pc-windows-msvc.exe ffprobe-x86_64-pc-windows-msvc.exe LICENSE.LGPLv2.1 COPYING.LGPL LICENSE.lame; do echo "contenido de $n" > "$1/$n"; done
  ( cd "$1" && for f in *; do echo "$(sum "$f")  $f"; done > SHA256SUMS )
}
export GH_BIN="$T/gh" FAKE_LOG="$T/log"

# 1) macOS: tres nombres por herramienta (aarch64, x86_64, universal), licencias, tag y repo correctos
mkrelease "$T/rel1"; export FAKE_RELEASE="$T/rel1"; : > "$T/log"
"$FETCH" macos "$T/d1" >"$T/out1" 2>&1 && ok "macos descarga" || { bad "macos descarga"; cat "$T/out1"; }
for tr in aarch64-apple-darwin x86_64-apple-darwin universal-apple-darwin; do
  for b in ffmpeg ffprobe; do [ -x "$T/d1/binaries/$b-$tr" ] && ok "$b-$tr" || bad "$b-$tr"; done
done
[ -f "$T/d1/licenses/LICENSE.LGPLv2.1" ] && [ -f "$T/d1/licenses/COPYING.LGPL" ] && [ -f "$T/d1/licenses/LICENSE.lame" ] && ok licencias || bad licencias
grep -q "^$(cat "$HERE/../sidecar-tag.txt")$" "$T/log" && grep -q '^repo=fher400gmail/nortia-studio$' "$T/log" && ok "tag y repo" || bad "tag y repo"

# 2) Windows: nombre con triple msvc y .exe
rm -rf "$T/d2"; "$FETCH" windows "$T/d2" >"$T/out2" 2>&1 && ok "windows descarga" || { bad "windows descarga"; cat "$T/out2"; }
[ -f "$T/d2/binaries/ffmpeg-x86_64-pc-windows-msvc.exe" ] && [ -f "$T/d2/binaries/ffprobe-x86_64-pc-windows-msvc.exe" ] && ok "windows nombres" || bad "windows nombres"

# 3) un binario alterado: falla y no deja nada en binaries/
mkrelease "$T/rel3"; echo manipulado > "$T/rel3/ffmpeg-universal-apple-darwin"; export FAKE_RELEASE="$T/rel3"
"$FETCH" macos "$T/d3" >"$T/out3" 2>&1 && bad "hash alterado debe fallar" || ok "hash alterado falla"
grep -qi 'sha256' "$T/out3" && ok "mensaje menciona SHA256" || bad "mensaje SHA256"
[ -z "$(ls "$T/d3/binaries" 2>/dev/null)" ] && ok "nada copiado" || bad "nada copiado"

# 4) fichero sin línea en SHA256SUMS: falla
mkrelease "$T/rel4"; grep -v ffprobe-universal "$T/rel4/SHA256SUMS" > "$T/s" && mv "$T/s" "$T/rel4/SHA256SUMS"; export FAKE_RELEASE="$T/rel4"
"$FETCH" macos "$T/d4" >/dev/null 2>&1 && bad "sin línea de hash debe fallar" || ok "sin línea de hash falla"

# 5) sin SHA256SUMS: falla
mkrelease "$T/rel5"; rm "$T/rel5/SHA256SUMS"; export FAKE_RELEASE="$T/rel5"
"$FETCH" macos "$T/d5" >/dev/null 2>&1 && bad "sin SHA256SUMS debe fallar" || ok "sin SHA256SUMS falla"

# 6) marcador de posición (job de Linux): ficheros vacíos con el triple del host
"$FETCH" --placeholder x86_64-unknown-linux-gnu "$T/d6" >/dev/null 2>&1 && ok placeholder || bad placeholder
[ -f "$T/d6/binaries/ffmpeg-x86_64-unknown-linux-gnu" ] && [ -f "$T/d6/binaries/ffprobe-x86_64-unknown-linux-gnu" ] && [ -f "$T/d6/licenses/COPYING.LGPL" ] && ok "placeholder ficheros" || bad "placeholder ficheros"

# 7) plataforma desconocida
"$FETCH" solaris "$T/d7" >/dev/null 2>&1 && bad "plataforma desconocida" || ok "plataforma desconocida falla"
exit $fail
