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
curl_r -o "$DEST/$TAR" "https://ffmpeg.org/releases/$TAR"
GOT=$(sha256_of "$DEST/$TAR")
# 1) Firma GPG: obligatoria (GOODSIG + VALIDSIG con la huella fijada). Nunca se omite.
command -v gpg >/dev/null || { echo "falta gpg: la verificación de firma es obligatoria" >&2; exit 1; }
curl_r -o "$DEST/$TAR.asc" "https://ffmpeg.org/releases/$TAR.asc"
curl_r -o "$DEST/ffmpeg-devel.asc" "https://ffmpeg.org/ffmpeg-devel.asc"
GNUPGHOME=$(mktemp -d); export GNUPGHOME
gpg --batch --import "$DEST/ffmpeg-devel.asc"
GPGSTATUS=$(gpg --batch --status-fd 1 --verify "$DEST/$TAR.asc" "$DEST/$TAR" 2>/dev/null || true)
grep -q "^\[GNUPG:\] GOODSIG " <<<"$GPGSTATUS" || { echo "firma GPG no es GOODSIG" >&2; exit 1; }
grep -q "^\[GNUPG:\] VALIDSIG $FFMPEG_GPG_FPR " <<<"$GPGSTATUS" || { echo "firma GPG no válida para la huella fijada" >&2; exit 1; }
rm -f "$DEST/ffmpeg-devel.asc"
echo "firma GPG verificada (huella $FFMPEG_GPG_FPR); SHA256 calculado: $GOT"
# 2) .sha256 publicado: comprobación adicional si existe (nunca sustituye a la firma).
if PUB=$(curl -fsSL --retry 1 "https://ffmpeg.org/releases/$TAR.sha256" 2>/dev/null | awk '{print $1}' | head -n1) && [ -n "$PUB" ]; then
  [ "$GOT" = "$PUB" ] || { echo "SHA256 de $TAR no coincide con el publicado: $GOT != $PUB" >&2; exit 1; }
  echo "SHA256 coincide con el publicado en ffmpeg.org"
else
  echo "ffmpeg.org no publica .sha256 (solo se aplica la firma GPG)"
fi
LAME_TAR="$DEST/lame-$LAME_VERSION.tar.gz"
ok=0
for url in "$LAME_URL" \
  "https://sourceforge.net/projects/lame/files/lame/$LAME_VERSION/lame-$LAME_VERSION.tar.gz/download" \
  "https://ftp.osuosl.org/pub/blfs/conglomeration/lame/lame-$LAME_VERSION.tar.gz" \
  "https://fossies.org/linux/misc/lame-$LAME_VERSION.tar.gz"; do
  echo "LAME: $url"
  if curl -fsSL --retry 2 --retry-delay 3 -o "$LAME_TAR" "$url" && [ "$(sha256_of "$LAME_TAR")" = "$LAME_SHA256" ]; then ok=1; break; fi
  echo "  descarte (error o SHA256 distinto)" >&2
done
[ "$ok" = 1 ] || { echo "no se pudo obtener LAME con SHA256 correcto" >&2; exit 1; }
echo "SHA256 de LAME verificado"
echo "$VER" > "$DEST/FFMPEG_VERSION"
