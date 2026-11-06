#!/usr/bin/env python3
"""Comprueba el cableado del sidecar entre tauri.conf.json, los scripts y app-build.yml
(lo que `cargo test` no ve). Uso, desde la raíz del repositorio: python3 -I app/scripts/check-wiring.py"""
import json
import os
import re
import sys

import yaml

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
errors = []


def check(cond, msg):
    if not cond:
        errors.append(msg)
        print("FAIL", msg)
    else:
        print("ok  ", msg)


def read(*p):
    with open(os.path.join(ROOT, *p), encoding="utf-8") as f:
        return f.read()


# --- tauri.conf.json
conf = json.loads(read("app", "src-tauri", "tauri.conf.json"))
check(conf["bundle"].get("externalBin") == ["binaries/ffmpeg", "binaries/ffprobe"], "externalBin = ffmpeg y ffprobe")
res = conf["bundle"].get("resources", {})
check(res.get("../../docs/legal/terceros.md") == "legal/terceros.txt", "terceros.md empaquetado como legal/terceros.txt")
for lic in ("LICENSE.LGPLv2.1", "COPYING.LGPL", "LICENSE.lame"):
    check(res.get(f"licenses/{lic}") == f"legal/{lic}", f"licencia {lic} empaquetada")
caps = read("app", "src-tauri", "capabilities", "default.json")
check("shell" not in caps, "sin capacidad shell (el motor lanza el proceso desde Rust)")
cmds = read("app", "src-tauri", "src", "commands.rs")
check(len(re.findall(r"\bopen_legal\b", cmds)) >= 2, "open_legal definido y registrado")
check("/binaries/" in read("app", "src-tauri", ".gitignore"), "binaries/ fuera de git")

# --- etiqueta del sidecar: una sola fuente
tag = read("app", "scripts", "sidecar-tag.txt").strip()
check(re.fullmatch(r"ffmpeg-sidecar-v\d+\.\d+\.\d+-[0-9a-f]+", tag) is not None, f"etiqueta del sidecar válida ({tag})")
check(f'"{tag}"' in read("engine", "mastercheck-bridge", "src", "app_support", "mod.rs"), "SIDECAR_RELEASE de Rust = sidecar-tag.txt")
wf_text = read(".github", "workflows", "app-build.yml")
check(not re.search(r"ffmpeg-sidecar-v\d", wf_text), "app-build.yml no repite la etiqueta (la lee fetch-sidecar.sh)")
fetch = read("app", "scripts", "fetch-sidecar.sh")
check("gh" in fetch and "release download" in fetch and "--repo" in fetch and "fher400gmail/nortia-studio" in fetch, "fetch-sidecar.sh usa gh release download --repo")
check("SHA256SUMS" in fetch and "no coincide" in fetch, "fetch-sidecar.sh verifica SHA256 contra SHA256SUMS")

# --- workflow
wf = yaml.safe_load(wf_text)
jobs = wf["jobs"]


def steps(job):
    return jobs[job]["steps"]


def find(job, needle):
    return [s for s in steps(job) if needle in json.dumps(s, ensure_ascii=False)]


check(bool(find("check", "--placeholder")), "job linux crea marcadores del sidecar antes de cargo check")
idx = {s.get("name", s.get("uses", "")): i for i, s in enumerate(steps("check"))}
ph = next(i for i, s in enumerate(steps("check")) if "--placeholder" in json.dumps(s))
cc = next(i for i, s in enumerate(steps("check")) if "cargo check" in json.dumps(s))
check(ph < cc, "los marcadores van antes de cargo check")
for job, plat in (("macos", "macos"), ("windows", "windows")):
    fs = find(job, f"fetch-sidecar.sh {plat}")
    check(len(fs) == 1 and fs[0].get("env", {}).get("GH_TOKEN") == "${{ github.token }}", f"{job}: descarga el sidecar con GH_TOKEN de github.token")
    b = next(i for i, s in enumerate(steps(job)) if "tauri-apps/cli" in json.dumps(s))
    f = next(i for i, s in enumerate(steps(job)) if f"fetch-sidecar.sh {plat}" in json.dumps(s))
    check(f < b, f"{job}: el sidecar se descarga antes de compilar")
check(bool(find("macos", "smoke-mac.sh")), "macos ejecuta smoke-mac.sh")
apple = find("macos", "apple-digital-masters/")
check(len(apple) == 1 and "-w '%{http_code}'" in apple[0]["run"] and "::warning::" in apple[0]["run"] and "exit 1" not in apple[0]["run"],
      "macos comprueba la página de Apple con aviso (::warning::) y sin romper el build")
check(not os.path.exists(os.path.join(ROOT, "app", "scripts", "install-adm-tools-mac.sh")), "install-adm-tools-mac.sh borrado (afclip viene con macOS)")
check("install-adm-tools" not in wf_text, "el workflow no instala herramientas de Apple")
detect = [s for s in steps("macos") if s.get("id") == "adm"]
check(len(detect) == 1 and "command -v afclip" in detect[0]["run"] and "::warning::" in detect[0]["run"], "macos detecta afclip con command -v y avisa si falta")
adm = [s for s in steps("macos") if "MASTEROUT_REQUIRE_ADM" in json.dumps(s)]
check(len(adm) == 1 and adm[0].get("if") == "steps.adm.outputs.present == 'true'", "MASTEROUT_REQUIRE_ADM solo si afclip está presente")
check("steps.adm.outputs.present == 'true' && 'required'" in wf_text, "el humo exige ADM solo si afclip está presente")
for name in os.listdir(os.path.join(ROOT, "app", "scripts")):
    if name.endswith((".sh", ".ps1", ".py")) and name != "check-wiring.py":
        check("sudo" not in read("app", "scripts", name), f"{name} no usa sudo")

# --- hashes fijados en el repositorio
pinned = {l.split()[1]: l.split()[0] for l in read("app", "scripts", "sidecar-sha256.txt").splitlines() if l.strip() and not l.startswith("#")}
for n in ("ffmpeg-universal-apple-darwin", "ffprobe-universal-apple-darwin", "ffmpeg-x86_64-pc-windows-msvc.exe", "ffprobe-x86_64-pc-windows-msvc.exe"):
    check(bool(re.fullmatch(r"[0-9a-f]{64}", pinned.get(n, ""))), f"sidecar-sha256.txt fija {n}")
check("sidecar-sha256.txt" in fetch, "fetch-sidecar.sh lee sidecar-sha256.txt")

# --- acciones de terceros: por SHA, salvo las tres que no se pudieron resolver (documentadas en el informe)
UNRESOLVED = {"dtolnay/rust-toolchain@stable", "Swatinem/rust-cache@v2", "actions/setup-node@v4"}
side = yaml.safe_load(read(".github", "workflows", "ffmpeg-sidecar.yml"))
ref = {}
for j in side["jobs"].values():
    for st in j["steps"]:
        u = st.get("uses", "")
        if u.startswith(("actions/checkout@", "actions/upload-artifact@")):
            ref[u.split("@")[0]] = u.split("@")[1]
for wfname in ("app-build.yml", "engine-tests.yml"):
    w = yaml.safe_load(read(".github", "workflows", wfname))
    check(w.get("permissions") == {"contents": "read"}, f"{wfname}: permissions contents: read")
    for jn, j in w["jobs"].items():
        for st in j["steps"]:
            u = st.get("uses")
            if not u or u in UNRESOLVED:
                continue
            name, _, sha = u.partition("@")
            check(re.fullmatch(r"[0-9a-f]{40}", sha) is not None and ref.get(name) == sha, f"{wfname}/{jn}: {name} pinneada por SHA (la de ffmpeg-sidecar.yml)")
eng = yaml.safe_load(read(".github", "workflows", "engine-tests.yml"))
check(isinstance(eng["jobs"]["macos"].get("timeout-minutes"), int) and eng["jobs"]["macos"].get("continue-on-error") is False,
      "engine macos: timeout-minutes y continue-on-error: false explícitos")
check(bool(find("macos", "afclip-raw")), "el log crudo de afclip se sube como artefacto")
check(bool(find("windows", "smoke-win.ps1")), "windows ejecuta smoke-win.ps1")

# --- humo: etiquetas exigidas y comprobaciones
verify = read("app", "scripts", "smoke_verify.py")
for tag_ in ("Portada", "Canvas", "YouTube", "MP3 320", "Portada animada"):
    check(f'"{tag_}"' in verify, f"smoke_verify exige [{tag_}]")
check("capabilities" in verify and "CAPS_TARGET_MS" in verify, "smoke_verify anota el tiempo de capabilities()")
ps = read("app", "scripts", "smoke-win.ps1")
check("\\\\?\\" in ps and "ffprobe" in ps, "smoke-win.ps1 prueba rutas \\\\?\\ con ffprobe")
check("hevc" in ps.lower() and "::warning::" in ps, "smoke-win.ps1 avisa si no hay HEVC")
mac = read("app", "scripts", "smoke-mac.sh")
check("MASTERCHECK_SMOKE_COVER" in mac and "smoke_verify.py" in mac, "smoke-mac.sh usa portada y smoke_verify.py")
check("CFBundleExecutable" in mac and "mastercheck-app" not in mac, "smoke-mac.sh lee CFBundleExecutable (mainBinaryName cambia el nombre del ejecutable)")
check("mastercheck-app" not in ps, "smoke-win.ps1 no fija el nombre del ejecutable")
inst = [st for st in steps("windows") if "APP_EXE=" in json.dumps(st)]
check(len(inst) == 1 and "Count -ne 1" in inst[0]["run"], "windows descubre en runtime el único .exe de la app (y falla si no es único)")

print("RESULTADO:", "FALLA" if errors else "OK")
sys.exit(1 if errors else 0)
