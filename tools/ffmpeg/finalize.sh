#!/usr/bin/env bash
# Genera SHA256SUMS de todo lo que hay en <out>.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
source "$HERE/common.sh"
OUT=${1:?}; cd "$OUT"; rm -f SHA256SUMS
for f in *; do [ "$f" = SHA256SUMS ] || [ -d "$f" ] || echo "$(sha256_of "$f")  $f"; done > SHA256SUMS
cat SHA256SUMS
