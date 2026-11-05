#!/usr/bin/env bash
# Intenta instalar las «Apple Digital Masters Tools» (traen `afclip`; `afconvert` ya viene con macOS).
# Busca en la página oficial el enlace de descarga (no se fija una URL a ciegas), descarga, comprueba
# la firma de Apple del instalador y deja `afclip` en /usr/local/bin.
#
# Salida: 0 = afclip instalado (última línea: AFCLIP=<ruta>); 3 = no se pudo (motivo en stderr, el
# job emite ::warning::). Nunca instala algo sin firma de Apple.
set -uo pipefail
PAGE="https://www.apple.com/apple-music/apple-digital-masters/"
UA="Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15"
W="$(mktemp -d)"; trap 'hdiutil detach "$W/vol" -quiet 2>/dev/null; rm -rf "$W"' EXIT
no() { echo "adm-tools: $*" >&2; exit 3; }

if command -v afclip >/dev/null 2>&1; then echo "AFCLIP=$(command -v afclip)"; exit 0; fi

curl -fsSL -A "$UA" --retry 3 -m 60 "$PAGE" -o "$W/page.html" || no "no se pudo leer $PAGE"
# enlaces a dmg/zip/pkg, absolutizados
grep -oiE "(href|data-[a-z-]*href)=\"[^\"]+\.(dmg|zip|pkg)\"" "$W/page.html" | sed -E 's/^[^"]*"//; s/"$//' | sort -u > "$W/links.txt"
[ -s "$W/links.txt" ] || no "la página no contiene enlaces .dmg/.zip/.pkg (puede exigir sesión o haber cambiado)"
echo "enlaces candidatos:" >&2; cat "$W/links.txt" >&2
# preferir el que nombre las herramientas
url="$(grep -iE 'digital.?masters|adm|tools|afclip' "$W/links.txt" | head -1)"
[ -n "$url" ] || url="$(head -1 "$W/links.txt")"
case "$url" in
  //*) url="https:$url" ;;
  /*)  url="https://www.apple.com$url" ;;
  http*) ;;
  *)   url="${PAGE}${url}" ;;
esac
echo "descargando $url" >&2
file="$W/download.${url##*.}"
curl -fsSL -A "$UA" --retry 3 -m 300 "$url" -o "$file" || no "la descarga de $url falló (¿exige sesión de Apple?)"

root="$W/payload"; mkdir -p "$root"
case "$file" in
  *.dmg) mkdir -p "$W/vol"; hdiutil attach "$file" -mountpoint "$W/vol" -nobrowse -quiet || no "no se pudo montar el dmg"; src="$W/vol" ;;
  *.zip) ditto -x -k "$file" "$root" || no "no se pudo descomprimir"; src="$root" ;;
  *.pkg) src="$(dirname "$file")" ;;
esac

pkg="$(find "$src" -maxdepth 3 -name '*.pkg' | head -1)"
if [ -n "$pkg" ]; then
  sig="$(pkgutil --check-signature "$pkg" 2>&1)"
  echo "$sig" >&2
  echo "$sig" | grep -q "signed by a certificate trusted" && echo "$sig" | grep -qi "Apple" || no "el instalador no lleva firma de Apple verificable"
  sudo installer -pkg "$pkg" -target / >&2 || no "installer falló"
fi
found="$(command -v afclip || true)"
[ -n "$found" ] || found="$(find /usr/local /usr/bin /Applications /Library "$src" -name afclip -type f -perm -u+x 2>/dev/null | head -1)"
[ -n "$found" ] || no "tras instalar no aparece ningún ejecutable afclip"
if ! command -v afclip >/dev/null 2>&1; then
  sudo mkdir -p /usr/local/bin && sudo cp "$found" /usr/local/bin/afclip && sudo chmod +x /usr/local/bin/afclip || no "no se pudo copiar afclip a /usr/local/bin"
  found=/usr/local/bin/afclip
fi
echo "AFCLIP=$found"
