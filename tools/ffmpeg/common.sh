# Funciones y flags compartidos por build-macos.sh / build-windows.sh (se usa con `source`).
LAME_VERSION=3.100
LAME_SHA256=ddfe36cab873794038ae2c1210557ad34857a4b6bdc515785d1da9e175b1da1e
LAME_URL="https://downloads.sourceforge.net/project/lame/lame/${LAME_VERSION}/lame-${LAME_VERSION}.tar.gz"
# Clave de firma de releases de FFmpeg (huella publicada en ffmpeg.org/download.html), solo para el plan B.
FFMPEG_GPG_FPR=FCF986EA15E6E293A5644F10B4322F04D67658D8

curl_r() { curl -fsSL --retry 6 --retry-all-errors --retry-delay 5 -A 'Mozilla/5.0 (nortia-ci)' "$@"; }

sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1
  else shasum -a 256 "$1" | cut -d' ' -f1; fi
}

# NCPU limita los hilos de compilación (runner en un Mac personal).
ncpu() { echo "${NCPU:-$(ncpu_all)}"; }
ncpu_all() { nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 2; }

# Versión de ffmpeg fijada (no flota a la última). Se puede sobrescribir con $FFMPEG_VERSION.
FFMPEG_VERSION_PINNED=7.1.5
resolve_ffmpeg_version() { echo "${FFMPEG_VERSION:-$FFMPEG_VERSION_PINNED}"; }

# Lista blanca de componentes: fuente única para los flags de configure Y para el inventario de accept.sh.
ALLOW_PROTOCOLS="file,pipe"
# Nombres de configure; en tiempo de ejecución image_png_pipe aparece como png_pipe (accept.sh quita "image_").
ALLOW_DEMUXERS="wav,aiff,flac,mp3,mov,matroska,image2,image_png_pipe,mjpeg,image_webp_pipe"
# prores: decodificador LGPL (libavcodec/proresdec2.c); los vídeos de motion design suelen venir en ProRes (.mov).
ALLOW_DECODERS="pcm_s16le,pcm_s24le,pcm_s32le,pcm_f32le,pcm_s16be,pcm_s24be,pcm_s32be,pcm_f32be,alac,flac,mp3,aac,h264,hevc,vp9,prores,png,mjpeg,webp"
ALLOW_PARSERS="aac,h264,hevc,vp9,mjpeg,png,webp,flac,mpegaudio"
ALLOW_BSFS="aac_adtstoasc,h264_mp4toannexb,hevc_mp4toannexb,extract_extradata,vp9_superframe,vp9_superframe_split"
ALLOW_MUXERS="mp4,mov,ipod,mp3,image2"
# boxblur es GPL-only en ffmpeg 7.1 (se desactivaría en silencio): no se incluye.
# setparams (LGPL, libavfilter/vf_setparams.c): etiqueta color/rango en los fotogramas; lo usa BT709 de video.rs.
# Fuente única de lo que el motor puede pedir: engine/masterout-core/tests/sidecar_allowlist.rs lo ata a esta lista.
ALLOW_FILTERS="scale,crop,zoompan,gblur,overlay,format,fps,setsar,trim,atrim,aresample,aformat,anull,null,color,loop,pad,setpts,split,fade,setparams"
ALLOW_ENCODERS_BASE="aac,libmp3lame,png,mjpeg"

FFMPEG_BASE_FLAGS=(
  --disable-gpl --disable-nonfree --disable-autodetect --disable-everything
  --disable-doc --disable-network --disable-debug
  --enable-zlib
  --enable-protocol="$ALLOW_PROTOCOLS"
  --enable-demuxer="$ALLOW_DEMUXERS"
  --enable-decoder="$ALLOW_DECODERS"
  --enable-parser="$ALLOW_PARSERS"
  --enable-bsf="$ALLOW_BSFS"
  --enable-muxer="$ALLOW_MUXERS"
  --enable-filter="$ALLOW_FILTERS"
  --enable-indev=lavfi
  --enable-libmp3lame
)
FFMPEG_ENCODERS_BASE="$ALLOW_ENCODERS_BASE"
