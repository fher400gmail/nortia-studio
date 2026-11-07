#!/usr/bin/env bash
# Comprobación previa en el runner propio (Mac de Fer): falla pronto y con mensaje claro si falta
# una herramienta, en vez de un «command not found» a mitad del build.
# Uso: runner-preflight.sh [--cmd nombre]... [--target triple]...
# El PATH del servicio del runner no incluye Homebrew ni rustup: los jobs lo amplían antes
# (echo ... >> $GITHUB_PATH); aquí solo se verifica.
set -uo pipefail
miss=0
while [ $# -gt 0 ]; do
  case "$1" in
    --cmd)
      if p=$(command -v "$2"); then echo "ok   $2 -> $p"; else echo "::error::falta '$2' en el runner (PATH=$PATH). Consulta docs/runner-mac.md"; miss=1; fi
      shift 2 ;;
    --target)
      if rustup target list --installed 2>/dev/null | grep -qx "$2"; then echo "ok   rustup target $2"; else echo "::error::falta el target de Rust '$2': rustup target add $2"; miss=1; fi
      shift 2 ;;
    *) echo "argumento desconocido: $1" >&2; exit 2 ;;
  esac
done
echo "arquitectura del runner: $(uname -m) · macOS $(sw_vers -productVersion 2>/dev/null || echo '?')"
exit "$miss"
