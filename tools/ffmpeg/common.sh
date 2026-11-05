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

ncpu() { nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 2; }

# Última versión 7.x publicada en ffmpeg.org/releases (o $FFMPEG_VERSION si está fijada).
resolve_ffmpeg_version() {
  if [ -n "${FFMPEG_VERSION:-}" ]; then echo "$FFMPEG_VERSION"; return; fi
  curl_r https://ffmpeg.org/releases/ \
    | grep -oE 'ffmpeg-7\.[0-9]+(\.[0-9]+)?\.tar\.xz' | sed -E 's/^ffmpeg-//; s/\.tar\.xz$//' \
    | sort -uV | tail -n1
}

# Flags comunes (LGPL, mínimo imprescindible). Los específicos de cada SO se añaden en su script.
FFMPEG_BASE_FLAGS=(
  --disable-gpl --disable-nonfree --disable-autodetect --disable-everything
  --disable-doc --disable-network --disable-debug
  --enable-zlib
  --enable-protocol=file,pipe
  --enable-demuxer=wav,aiff,flac,mp3,mov,matroska,image2,png_pipe,mjpeg,webp_pipe
  --enable-decoder=pcm_s16le,pcm_s24le,pcm_s32le,pcm_f32le,flac,mp3,aac,h264,hevc,vp9,png,mjpeg,webp
  --enable-parser=aac,h264,hevc,vp9,mjpeg,png,webp,flac,mpegaudio
  --enable-bsf=aac_adtstoasc,h264_mp4toannexb,hevc_mp4toannexb,extract_extradata,vp9_superframe,vp9_superframe_split
  --enable-muxer=mp4,mov,ipod,mp3,image2
  --enable-filter=scale,crop,zoompan,gblur,boxblur,overlay,format,fps,setsar,trim,atrim,aresample,aformat,anull,null,color,loop,pad
  --enable-indev=lavfi
  --enable-libmp3lame
)
# Codificadores comunes; el H.264 del sistema se suma después.
FFMPEG_ENCODERS_BASE="aac,libmp3lame,png,mjpeg"
