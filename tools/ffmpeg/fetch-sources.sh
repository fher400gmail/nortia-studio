#!/usr/bin/env bash
# Descarga y verifica las fuentes exactas (ffmpeg 7.x + LAME) en <destino>.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
source "$HERE/common.sh"
DEST=${1:?uso: fetch-sources.sh <destino>}
mkdir -p "$DEST"
VER=$(resolve_ffmpeg_version)
[ -n "$VER" ] || { echo "no se pudo resolver la versión de ffmpeg" >&2; exit 1; }
echo "ffmpeg $VER"
TAR="ffmpeg-$VER.tar.xz"
curl -fsSL -o "$DEST/$TAR" "https://ffmpeg.org/releases/$TAR"
GOT=$(sha256_of "$DEST/$TAR")
if PUB=$(curl -fsSL "https://ffmpeg.org/releases/$TAR.sha256" 2>/dev/null | awk '{print $1}' | head -n1) && [ -n "$PUB" ]; then
  [ "$GOT" = "$PUB" ] || { echo "SHA256 de $TAR no coincide: $GOT != $PUB" >&2; exit 1; }
  echo "SHA256 de $TAR verificado contra ffmpeg.org: $GOT"
else
  echo "ffmpeg.org no publica .sha256; verificando firma GPG" >&2
  command -v gpg >/dev/null || { echo "sin .sha256 ni gpg" >&2; exit 1; }
  curl -fsSL -o "$DEST/$TAR.asc" "https://ffmpeg.org/releases/$TAR.asc"
  curl -fsSL -o "$DEST/ffmpeg-devel.asc" "https://ffmpeg.org/ffmpeg-devel.asc"
  GNUPGHOME=$(mktemp -d); export GNUPGHOME
  gpg --batch --import "$DEST/ffmpeg-devel.asc"
  gpg --batch --status-fd 1 --verify "$DEST/$TAR.asc" "$DEST/$TAR" | grep -q "VALIDSIG $FFMPEG_GPG_FPR" \
    || { echo "firma GPG no válida" >&2; exit 1; }
  rm -f "$DEST/ffmpeg-devel.asc"
  echo "firma GPG verificada; SHA256 calculado: $GOT"
fi
curl -fsSL -o "$DEST/lame-$LAME_VERSION.tar.gz" "$LAME_URL"
[ "$(sha256_of "$DEST/lame-$LAME_VERSION.tar.gz")" = "$LAME_SHA256" ] \
  || { echo "SHA256 de LAME no coincide" >&2; exit 1; }
echo "SHA256 de LAME verificado"
echo "$VER" > "$DEST/FFMPEG_VERSION"
