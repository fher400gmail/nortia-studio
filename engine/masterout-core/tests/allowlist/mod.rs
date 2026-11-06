//! Lista blanca del sidecar ffmpeg (`tools/ffmpeg/common.sh` + implícitos de `accept.sh` +
//! codificadores por plataforma de `build-*.sh`) y comprobación de argumentos contra ella.
//! La comparten los tests del núcleo y del puente (`#[path]`).
#![allow(dead_code)]

use std::collections::BTreeSet;
use std::path::PathBuf;

pub type Set = BTreeSet<String>;

pub struct Allow {
    pub filters: Set,
    pub decoders: Set,
    pub muxers: Set,
    pub bsfs: Set,
    pub encoders_macos: Set,
    pub encoders_windows: Set,
}

fn tools_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tools/ffmpeg")
}

fn read(name: &str) -> String {
    let p = tools_dir().join(name);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("no se puede leer {}: {e}", p.display()))
}

fn csv(v: &str) -> Set {
    v.split(',')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(String::from)
        .collect()
}

/// `NOMBRE="a,b,c"` de `common.sh`.
fn shell_var(text: &str, name: &str) -> Set {
    let key = format!("{name}=\"");
    let line = text
        .lines()
        .find(|l| l.starts_with(&key))
        .unwrap_or_else(|| panic!("falta {name} en common.sh"));
    csv(&line[key.len()..line.rfind('"').unwrap()])
}

/// Cadenas entre comillas de `NOMBRE = {"a", "b"}` en el Python de `accept.sh`.
fn py_set(text: &str, name: &str) -> Set {
    let key = format!("{name} = {{");
    let start = text
        .find(&key)
        .unwrap_or_else(|| panic!("falta {name} en accept.sh"))
        + key.len();
    let body = &text[start..start + text[start..].find('}').unwrap()];
    body.split('"')
        .skip(1)
        .step_by(2)
        .map(String::from)
        .collect()
}

/// Codificadores extra de `--enable-encoder="$FFMPEG_ENCODERS_BASE,x,y"` de un `build-*.sh`.
fn build_extra_encoders(script: &str) -> Set {
    let key = "--enable-encoder=\"$FFMPEG_ENCODERS_BASE,";
    let start = script
        .find(key)
        .unwrap_or_else(|| panic!("falta --enable-encoder en el build"))
        + key.len();
    csv(&script[start..start + script[start..].find('"').unwrap()])
}

pub fn load() -> Allow {
    let common = read("common.sh");
    let accept = read("accept.sh");
    let base_enc = shell_var(&common, "ALLOW_ENCODERS_BASE");
    let with = |script: &str| -> Set {
        base_enc
            .iter()
            .cloned()
            .chain(build_extra_encoders(&read(script)))
            .collect()
    };
    Allow {
        filters: shell_var(&common, "ALLOW_FILTERS")
            .union(&py_set(&accept, "IMPLICIT_FILTERS"))
            .cloned()
            .collect(),
        decoders: shell_var(&common, "ALLOW_DECODERS")
            .union(&py_set(&accept, "IMPLICIT_DECODERS"))
            .cloned()
            .collect(),
        muxers: shell_var(&common, "ALLOW_MUXERS"),
        bsfs: shell_var(&common, "ALLOW_BSFS"),
        encoders_macos: with("build-macos.sh"),
        encoders_windows: with("build-windows.sh"),
    }
}

/// Divide por `sep1`/`sep2` fuera de comillas simples y paréntesis (como hace ffmpeg).
fn split_top(s: &str, seps: &[char]) -> Vec<String> {
    let (mut out, mut cur) = (Vec::new(), String::new());
    let (mut quoted, mut esc) = (false, false);
    for c in s.chars() {
        if esc {
            esc = false;
            cur.push(c);
        } else if c == '\\' {
            esc = true;
            cur.push(c);
        } else if c == '\'' {
            quoted = !quoted;
            cur.push(c);
        } else if !quoted && seps.contains(&c) {
            out.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    out.push(cur);
    out
}

/// Nombres de filtro de una cadena `-vf`/`-af`/`-filter_complex`: tramos separados por `,`
/// y `;` fuera de comillas, sin etiquetas `[x]` delante ni detrás, nombre antes de `=`
/// (y sin `@instancia`).
pub fn filter_names(graph: &str) -> Vec<String> {
    split_top(graph, &[',', ';'])
        .into_iter()
        .filter_map(|seg| {
            let mut t = seg.trim();
            while t.starts_with('[') {
                t = &t[t.find(']')? + 1..];
            }
            while t.ends_with(']') {
                t = &t[..t.rfind('[')?];
            }
            let name = t.split('=').next().unwrap_or("").split('@').next()?;
            (!name.trim().is_empty()).then(|| name.trim().to_string())
        })
        .collect()
}

fn muxer_of_extension(out: &str) -> Option<&'static str> {
    let ext = out.rsplit('.').next()?.to_ascii_lowercase();
    match ext.as_str() {
        "mp4" => Some("mp4"),
        "mov" => Some("mov"),
        "mp3" => Some("mp3"),
        "png" | "jpg" | "jpeg" => Some("image2"),
        _ => None,
    }
}

/// Incumplimientos de la lista blanca en una línea de ffmpeg (vacío si todo está permitido).
/// `encoders`: los del sidecar de la plataforma. La salida de vídeo se escribe con `-f mp4`
/// (lo añade `run_ffmpeg`), por eso también se comprueba el muxer implícito de la extensión.
pub fn violations(what: &str, args: &[String], allow: &Allow, encoders: &Set) -> Vec<String> {
    let mut bad = Vec::new();
    let mut explicit_f = false;
    let mut i = 0;
    while i < args.len() {
        let (flag, val) = (args[i].as_str(), args.get(i + 1).map(String::as_str));
        match (flag, val) {
            ("-vf" | "-af" | "-filter_complex" | "-lavfi" | "-filter:v" | "-filter:a", Some(g)) => {
                for n in filter_names(g) {
                    if !allow.filters.contains(&n) {
                        bad.push(format!(
                            "{what}: filtro «{n}» fuera de la lista blanca (en `{g}`)"
                        ));
                    }
                }
            }
            (
                "-c:v" | "-c:a" | "-c" | "-codec" | "-codec:v" | "-codec:a" | "-vcodec" | "-acodec",
                Some(c),
            ) if c != "copy" && !encoders.contains(c) => {
                bad.push(format!(
                    "{what}: codificador «{c}» fuera de la lista blanca"
                ));
            }
            ("-f", Some(m)) => {
                explicit_f = true;
                if !allow.muxers.contains(m) {
                    bad.push(format!("{what}: muxer «{m}» fuera de la lista blanca"));
                }
            }
            ("-bsf" | "-bsf:v" | "-bsf:a", Some(b)) => {
                for n in b.split(',').map(|x| x.split('=').next().unwrap_or("")) {
                    if !allow.bsfs.contains(n) {
                        bad.push(format!(
                            "{what}: bitstream filter «{n}» fuera de la lista blanca"
                        ));
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    if !explicit_f {
        if let Some(m) = args.last().and_then(|o| muxer_of_extension(o)) {
            if !allow.muxers.contains(m) {
                bad.push(format!(
                    "{what}: muxer «{m}» (por la extensión de salida) fuera de la lista blanca"
                ));
            }
        }
    }
    bad
}
