#!/usr/bin/env bash
# Compila ffmpeg/ffprobe LGPL estáticos para Windows x64 (MSYS2 MINGW64) con LAME estático.
# uso: build-windows.sh <work> <out>
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
source "$HERE/common.sh"
WORK=${1:?}; OUT=${2:?}
mkdir -p "$WORK" "$OUT"; WORK=$(cd "$WORK" && pwd); OUT=$(cd "$OUT" && pwd)
"$HERE/fetch-sources.sh" "$OUT"
VER=$(cat "$OUT/FFMPEG_VERSION")
PREFIX="$WORK/prefix"; mkdir -p "$PREFIX"

echo "::group::LAME"
mkdir -p "$WORK/lame"
tar xzf "$OUT/lame-$LAME_VERSION.tar.gz" -C "$WORK/lame" --strip-components=1
( cd "$WORK/lame"
  sed -i '/lame_init_old/d' include/libmp3lame.sym
  CFLAGS="-O2" ./configure --prefix="$PREFIX" --disable-shared --enable-static --disable-frontend --disable-gtktest
  make -j"$(ncpu)"; make install )
echo "::endgroup::"

echo "::group::ffmpeg"
mkdir -p "$WORK/ff"
tar xJf "$OUT/ffmpeg-$VER.tar.xz" -C "$WORK/ff" --strip-components=1
args=(--prefix="$WORK/ff/install" --arch=x86_64 --target-os=mingw32
  "${FFMPEG_BASE_FLAGS[@]}"
  --enable-encoder="$FFMPEG_ENCODERS_BASE,h264_mf,hevc_mf" --enable-mediafoundation
  --pkg-config-flags=--static
  --extra-cflags="-I$PREFIX/include"
  --extra-ldflags="-L$PREFIX/lib -static")
{ echo "# Windows x86_64 (MSYS2 MINGW64)"; printf './configure'; printf ' %q' "${args[@]}"; echo; } > "$OUT/configure_line.txt"
( cd "$WORK/ff"
  ./configure "${args[@]}" || { tail -n 60 ffbuild/config.log; exit 1; }
  make -j"$(ncpu)" ffmpeg.exe ffprobe.exe )
strip "$WORK/ff/ffmpeg.exe" "$WORK/ff/ffprobe.exe"
cp "$WORK/ff/ffmpeg.exe" "$OUT/ffmpeg-x86_64-pc-windows-msvc.exe"
cp "$WORK/ff/ffprobe.exe" "$OUT/ffprobe-x86_64-pc-windows-msvc.exe"
echo "::endgroup::"

cp "$WORK/ff/COPYING.LGPLv2.1" "$OUT/LICENSE.LGPLv2.1"
cp "$WORK/lame/COPYING" "$OUT/COPYING.LGPL"
cp "$WORK/lame/LICENSE" "$OUT/LICENSE.lame"
echo "ffmpeg $VER · LAME $LAME_VERSION" > "$OUT/VERSIONS.txt"
