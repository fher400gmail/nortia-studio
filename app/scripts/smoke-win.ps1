# Humo headless de master check en Windows (PowerShell).
# Uso: pwsh -File smoke-win.ps1 -Exe "<ruta al .exe instalado por NSIS (lo descubre el workflow: el único que no es ffmpeg/ffprobe/uninstall)>" -Work "<carpeta>"
# Mismo guion que smoke-mac.sh, más:
#   - ffprobe debe aceptar rutas canonicalizadas con prefijo \\?\ (las que produce media::probe);
#   - si hevc_mf no existe en el equipo, portada animada solo sale en 2048x2732 y hay ::warning::
#     (lo comprueba smoke_verify.py a partir del bloque `smoke` de smoke-result.json).
param(
  [Parameter(Mandatory = $true)][string]$Exe,
  [Parameter(Mandatory = $true)][string]$Work
)
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
if (-not (Test-Path -LiteralPath $Exe)) { throw "no existe el ejecutable: $Exe" }
$exeDir = Split-Path -Parent $Exe
foreach ($b in 'ffmpeg.exe', 'ffprobe.exe') {
  if (-not (Test-Path -LiteralPath (Join-Path $exeDir $b))) {
    Write-Host "::error::falta $b junto al ejecutable (externalBin)"
    Get-ChildItem $exeDir | Format-Table Name, Length | Out-String | Write-Host
    exit 2
  }
}
New-Item -ItemType Directory -Force -Path $Work | Out-Null
$wav = Join-Path $Work 'máster de prueba.wav'
$cover = Join-Path $Work 'portada de prueba.png'
$out = Join-Path $Work 'salida humo'
if (Test-Path -LiteralPath $out) { Remove-Item -Recurse -Force -LiteralPath $out }

python -X utf8 -I (Join-Path $here 'smoke_assets.py') wav $wav
if ($LASTEXITCODE -ne 0) { throw 'no se pudo crear el WAV de prueba' }
python -X utf8 -I (Join-Path $here 'smoke_assets.py') cover $cover
if ($LASTEXITCODE -ne 0) { throw 'no se pudo crear la portada de prueba' }

# 1) ffprobe del sidecar acepta una ruta canonicalizada con prefijo \\?\ (con espacios y acentos)
$ffprobe = Join-Path $exeDir 'ffprobe.exe'
$ext = '\\?\' + (Resolve-Path -LiteralPath $wav).ProviderPath
Write-Host "ffprobe con ruta extendida: $ext"
$dur = & $ffprobe -v error -show_entries format=duration -of csv=p=0 $ext 2>&1
if ($LASTEXITCODE -ne 0) {
  Write-Host "::error::ffprobe no acepta rutas \\?\ ($dur). Usar dunce::canonicalize o quitar el prefijo en media::probe."
  exit 1
}
Write-Host "ok   ffprobe lee \\?\: duración $dur s"

# 2) la app real, con el idioma forzado a español y la portada sintética
$env:LANG = 'es_ES.UTF-8'
$env:LC_ALL = 'es_ES.UTF-8'
$env:MASTERCHECK_SMOKE_COVER = $cover
$env:MASTERCHECK_SMOKE = "$wav;$out"
$p = Start-Process -FilePath $Exe -PassThru -NoNewWindow
if (-not $p.WaitForExit(900000)) {
  $p.Kill()
  throw 'el humo superó los 15 minutos'
}
Write-Host "código de salida: $($p.ExitCode)"
if ($p.ExitCode -ne 0) { exit 1 }

$result = Join-Path $out 'smoke-result.json'
if (-not (Test-Path -LiteralPath $result)) { throw 'no hay smoke-result.json' }
$d = Get-Content -LiteralPath $result -Raw -Encoding UTF8 | ConvertFrom-Json
Write-Host "capabilities() en frío (Windows): h264=$($d.smoke.capabilities.h264) hevc=$($d.smoke.capabilities.hevc) $($d.smoke.capabilities.millis) ms"
if (-not $d.smoke.capabilities.hevc) {
  Write-Host '::warning::HEVC no disponible (hevc_mf): portada animada solo sale en 2048x2732 (3840x3840 requiere «HEVC Video Extensions»).'
}
Get-ChildItem -Recurse -File $out | ForEach-Object { $_.FullName } | Sort-Object | Tee-Object -FilePath (Join-Path $Work 'listado.txt')

# 3) comprobaciones con el ffprobe del propio sidecar
python -X utf8 -I (Join-Path $here 'smoke_verify.py') --out $out --platform win
if ($LASTEXITCODE -ne 0) { exit 1 }
Write-Host 'HUMO OK'
