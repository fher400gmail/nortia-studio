#!/usr/bin/env bash
# Descarga el sidecar ffmpeg/ffprobe del release fijado en sidecar-tag.txt, VERIFICA su SHA256
# contra el SHA256SUMS del propio release y solo entonces lo copia a <destino>/binaries con el
# nombre que espera Tauri (`externalBin`): ffmpeg-<triple>[.exe]. Las licencias LGPL van a
# <destino>/licenses (se empaquetan como recursos).
#
# Uso:  fetch-sidecar.sh macos|windows <destino>        (destino = app/src-tauri)
#       fetch-sidecar.sh --placeholder <triple> <destino>  (ficheros vacíos: solo para que
#           `cargo check` con la feature `app` encuentre lo que declara externalBin)
# Requiere `gh` autenticado (GH_TOKEN). GH_BIN permite sustituirlo en las pruebas.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
REPO="fher400gmail/nortia-studio"
TAG="$(tr -d '[:space:]' < "$HERE/sidecar-tag.txt")"
GH="${GH_BIN:-gh}"
# Hashes fijados en el repositorio: un release manipulado (binario y SHA256SUMS a la vez) no pasa.
SHA_FILE="${SIDECAR_SHA_FILE:-$HERE/sidecar-sha256.txt}"
LICENSES=(LICENSE.LGPLv2.1 COPYING.LGPL LICENSE.lame)

die() { echo "::error::fetch-sidecar: $*" >&2; exit 1; }
sha256() { if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1; else shasum -a 256 "$1" | cut -d' ' -f1; fi; }

if [ "${1:-}" = "--placeholder" ]; then
  triple="${2:?triple}"; dest="${3:?destino}"
  mkdir -p "$dest/binaries" "$dest/licenses"
  for b in ffmpeg ffprobe; do : > "$dest/binaries/$b-$triple"; done
  for l in "${LICENSES[@]}"; do : > "$dest/licenses/$l"; done
  echo "marcadores de posición para $triple en $dest"
  exit 0
fi

platform="${1:?macos|windows}"; dest="${2:?destino}"
case "$platform" in
  macos)   sources=(ffmpeg-universal-apple-darwin ffprobe-universal-apple-darwin) ;;
  windows) sources=(ffmpeg-x86_64-pc-windows-msvc.exe ffprobe-x86_64-pc-windows-msvc.exe) ;;
  *) die "plataforma desconocida: $platform" ;;
esac

work="$(mktemp -d)"; trap 'rm -rf "$work"' EXIT
pats=(-p SHA256SUMS)
for f in "${sources[@]}" "${LICENSES[@]}"; do pats+=(-p "$f"); done
echo "descargando $TAG ($REPO)"
"$GH" release download "$TAG" --repo "$REPO" "${pats[@]}" -D "$work" --clobber \
  || die "no se pudo descargar el release $TAG"

[ -s "$SHA_FILE" ] || die "falta $SHA_FILE"
[ -s "$work/SHA256SUMS" ] || die "el release $TAG no trae SHA256SUMS"
for f in "${sources[@]}" "${LICENSES[@]}"; do
  [ -f "$work/$f" ] || die "falta $f en el release $TAG"
  want="$(awk -v n="$f" '$2 == n { print $1 }' "$work/SHA256SUMS")"
  [ -n "$want" ] || die "$f no figura en SHA256SUMS"
  pinned="$(awk -v n="$f" '$1 !~ /^#/ && $2 == n { print $1 }' "$SHA_FILE")"
  [ -n "$pinned" ] || die "$f no figura en el SHA256 del repositorio ($SHA_FILE)"
  [ "$pinned" = "$want" ] || die "SHA256SUMS del release difiere del hash del repositorio para $f (repositorio $pinned, release $want)"
  got="$(sha256 "$work/$f")"
  [ "$got" = "$want" ] || die "SHA256 de $f no coincide (esperado $want, obtenido $got)"
  echo "sha256 ok  $f (release y repositorio)"
done

mkdir -p "$dest/binaries" "$dest/licenses"
if [ "$platform" = macos ]; then
  # build universal: Tauri compila cada arquitectura y luego une; cada paso busca su triple,
  # así que el mismo binario universal va con los tres nombres.
  for tr in aarch64-apple-darwin x86_64-apple-darwin universal-apple-darwin; do
    cp "$work/ffmpeg-universal-apple-darwin"  "$dest/binaries/ffmpeg-$tr"
    cp "$work/ffprobe-universal-apple-darwin" "$dest/binaries/ffprobe-$tr"
    chmod +x "$dest/binaries/ffmpeg-$tr" "$dest/binaries/ffprobe-$tr"
  done
else
  cp "$work/ffmpeg-x86_64-pc-windows-msvc.exe"  "$dest/binaries/"
  cp "$work/ffprobe-x86_64-pc-windows-msvc.exe" "$dest/binaries/"
fi
for l in "${LICENSES[@]}"; do cp "$work/$l" "$dest/licenses/$l"; done
echo "sidecar $TAG listo en $dest/binaries"
