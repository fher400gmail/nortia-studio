#!/usr/bin/env bash
# Compila ffmpeg/ffprobe LGPL universales (arm64 + x86_64) con LAME estático.
# uso: build-macos.sh <work> <out>
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
source "$HERE/common.sh"
WORK=${1:?}; OUT=${2:?}
mkdir -p "$WORK" "$OUT"; WORK=$(cd "$WORK" && pwd); OUT=$(cd "$OUT" && pwd)
"$HERE/fetch-sources.sh" "$OUT"
VER=$(cat "$OUT/FFMPEG_VERSION")
export MACOSX_DEPLOYMENT_TARGET=14.0
HOST_ARCH=$(uname -m)
: > "$OUT/configure_line.txt"

build_arch() {
  local arch=$1 cc="clang -arch $1"
  local prefix="$WORK/prefix-$arch"; mkdir -p "$prefix" "$WORK/bin-$arch"
  local hostargs=() cross=()
  if [ "$arch" != "$HOST_ARCH" ]; then
    hostargs=(--host="${arch}-apple-darwin"); cross=(--enable-cross-compile --target-os=darwin)
  fi
  echo "::group::LAME $arch"
  rm -rf "$WORK/lame-$arch"; mkdir -p "$WORK/lame-$arch"
  tar xzf "$OUT/lame-$LAME_VERSION.tar.gz" -C "$WORK/lame-$arch" --strip-components=1
  ( cd "$WORK/lame-$arch"
    sed -i '' '/lame_init_old/d' include/libmp3lame.sym
    CC="$cc" CFLAGS="-O2 -mmacosx-version-min=14.0" LDFLAGS="-mmacosx-version-min=14.0" \
      ./configure --prefix="$prefix" --disable-shared --enable-static --disable-frontend --disable-gtktest \
      ${hostargs[@]+"${hostargs[@]}"}
    make -j"$(ncpu)"; make install )
  echo "::endgroup::"
  echo "::group::ffmpeg $arch"
  rm -rf "$WORK/ff-$arch"; mkdir -p "$WORK/ff-$arch"
  tar xJf "$OUT/ffmpeg-$VER.tar.xz" -C "$WORK/ff-$arch" --strip-components=1
  local args=(--prefix="$WORK/ff-$arch/install" --arch="$arch" --cc="$cc"
    ${cross[@]+"${cross[@]}"}
    "${FFMPEG_BASE_FLAGS[@]}"
    --enable-encoder="$FFMPEG_ENCODERS_BASE,h264_videotoolbox" --enable-videotoolbox
    --extra-cflags="-I$prefix/include -mmacosx-version-min=14.0"
    --extra-ldflags="-L$prefix/lib -mmacosx-version-min=14.0")
  { echo "# macOS $arch"; printf './configure'; printf ' %q' "${args[@]}"; echo; echo; } >> "$OUT/configure_line.txt"
  ( cd "$WORK/ff-$arch"
    ./configure "${args[@]}" || { tail -n 60 ffbuild/config.log; exit 1; }
    make -j"$(ncpu)" ffmpeg ffprobe )
  cp "$WORK/ff-$arch/ffmpeg" "$WORK/ff-$arch/ffprobe" "$WORK/bin-$arch/"
  strip -x "$WORK/bin-$arch/ffmpeg" "$WORK/bin-$arch/ffprobe"
  echo "::endgroup::"
}

build_arch arm64
build_arch x86_64

for t in ffmpeg ffprobe; do
  lipo -create "$WORK/bin-arm64/$t" "$WORK/bin-x86_64/$t" -output "$OUT/$t-universal-apple-darwin"
  codesign -s - -f "$OUT/$t-universal-apple-darwin"
  chmod +x "$OUT/$t-universal-apple-darwin"
  lipo -info "$OUT/$t-universal-apple-darwin"
done

# Licencias
tar xJf "$OUT/ffmpeg-$VER.tar.xz" -C "$WORK" "ffmpeg-$VER/COPYING.LGPLv2.1"
cp "$WORK/ffmpeg-$VER/COPYING.LGPLv2.1" "$OUT/LICENSE.LGPLv2.1"
cp "$WORK/lame-arm64/COPYING" "$OUT/COPYING.LGPL"
cp "$WORK/lame-arm64/LICENSE" "$OUT/LICENSE.lame" 2>/dev/null || true
echo "ffmpeg $VER · LAME $LAME_VERSION" > "$OUT/VERSIONS.txt"
