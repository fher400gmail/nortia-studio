//! Validación Apple Digital Masters (solo macOS): máster -> CAF con Sound Check -> AAC 256 kbps
//! -> se decodifica a WAV float -> `afclip` cuenta las muestras con clipping del AAC decodificado.
//!
//! Flujo verificado (docs/skills-baseline/verificacion-2026-11-05.md, bloque C, pto 7):
//!
//! ```text
//! afconvert master.wav intermediate.caf -d 0 -f caff --soundcheck-generate
//!   (al bajar de tasa: -d LEF32@<tasa> ... --src-complexity bats -r 127)
//! afconvert intermediate.caf final.m4a -d aac -f m4af -u pgcm 2 --soundcheck-read -b 256000 -q 127 -s 2
//! afconvert final.m4a decode.wav -d LEF32 -f WAVE
//! afclip decode.wav
//! ```
//!
//! El AAC de ADM se codifica como mucho a 48 kHz: familia 44,1 kHz -> 44100; el resto -> 48000.
//! `afclip` lo distribuye Apple con las herramientas de Apple Digital Masters y no siempre está.
//!
//! Prueba real (solo en el humo de CI de macOS): `adm_smoke_case(-1.0, ..)` debe dar 0 muestras
//! con clipping y `adm_smoke_case(1.0, ..)` más de 0.

use crate::audio::AudioError;
use crate::wav::{self, WavWriteOptions};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmStatus {
    /// Cero muestras con clipping en el AAC.
    Ok,
    /// Una o más muestras con clipping.
    Clipping,
    /// No se pudo validar; `reason_key` es una clave i18n `reason.adm_*`.
    Unavailable { reason_key: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmReport {
    pub status: AdmStatus,
    pub clipped_samples: u64,
    pub aac_path: Option<PathBuf>,
    pub log: String,
}

impl AdmReport {
    fn unavailable(key: &str, log: String) -> AdmReport {
        AdmReport {
            status: AdmStatus::Unavailable {
                reason_key: key.to_string(),
            },
            clipped_samples: 0,
            aac_path: None,
            log,
        }
    }
}

/// Tasa del AAC: 44100 para la familia 44,1 kHz, 48000 para el resto.
pub fn aac_rate(master_rate: u32) -> u32 {
    if master_rate.is_multiple_of(44_100) {
        44_100
    } else {
        48_000
    }
}

fn s(v: &[&str]) -> Vec<OsString> {
    v.iter().map(OsString::from).collect()
}

fn p(path: &Path) -> OsString {
    path.as_os_str().to_owned()
}

/// Las cuatro órdenes (programa en la posición 0) de la validación, en orden. Pura.
pub fn build_adm_commands(
    master_wav: &Path,
    workdir: &Path,
    master_rate: u32,
) -> Vec<Vec<OsString>> {
    let caf = p(&workdir.join("intermediate.caf"));
    let m4a = p(&workdir.join("final.m4a"));
    let dec = p(&workdir.join("decode.wav"));
    let mut step1 = vec![OsString::from("afconvert"), p(master_wav), caf.clone()];
    if master_rate > 48_000 {
        let fmt = format!("LEF32@{}", aac_rate(master_rate));
        step1.extend(s(&["-d", &fmt, "-f", "caff", "--soundcheck-generate"]));
        step1.extend(s(&["--src-complexity", "bats", "-r", "127"]));
    } else {
        step1.extend(s(&["-d", "0", "-f", "caff", "--soundcheck-generate"]));
    }
    let mut step2 = vec![OsString::from("afconvert"), caf, m4a.clone()];
    step2.extend(s(&[
        "-d",
        "aac",
        "-f",
        "m4af",
        "-u",
        "pgcm",
        "2",
        "--soundcheck-read",
        "-b",
        "256000",
        "-q",
        "127",
        "-s",
        "2",
    ]));
    let mut step3 = vec![OsString::from("afconvert"), m4a, dec];
    step3.extend(s(&["-d", "LEF32", "-f", "WAVE"]));
    vec![
        step1,
        step2,
        step3,
        vec![OsString::from("afclip"), p(&workdir.join("decode.wav"))],
    ]
}

/// Número de muestras con clipping según la salida real de `afclip` (macOS); `None` si no se
/// reconoce (el flujo lo degrada a `reason.adm_parse` y conserva el log). Formatos:
///
/// ```text
///  -- no samples clipped --
/// ```
/// ```text
/// CLIPPED SAMPLES:
///     SECONDS          SAMPLE    CHAN    VALUE        DECIBELS
///    57.385431       2530697.50    1    -1.006987     0.060476
///
///    total clipped samples    Left   on-sample:        0    inter-sample:        6
///    total clipped samples    Right  on-sample:        0    inter-sample:        1
/// ```
///
/// Prioridad: suma de `on-sample` + `inter-sample` de TODAS las líneas de totales (hay una por
/// canal); si no hay totales, filas numéricas bajo la cabecera; "no samples clipped" solo vale
/// sin cabecera de tabla. Nada de adivinar con otras frases.
pub fn parse_afclip(output: &str) -> Option<u64> {
    let mut no_clip = false;
    let mut totals: Option<u64> = None;
    let mut header = false;
    let mut rows = 0u64;
    for raw in output.lines() {
        let line = raw.trim().to_lowercase();
        if line.contains("no samples clipped") {
            no_clip = true;
        } else if line.starts_with("total clipped samples") {
            let mut found = 0;
            let mut it = line.split_whitespace().peekable();
            while let Some(w) = it.next() {
                if w == "on-sample:" || w == "inter-sample:" {
                    if let Some(n) = it.peek().and_then(|v| v.parse::<u64>().ok()) {
                        totals = Some(totals.unwrap_or(0).saturating_add(n));
                        found += 1;
                    }
                }
            }
            if found == 0 {
                return None; // totales en otro formato: no adivinar
            }
        } else if line.starts_with("clipped samples:") {
            header = true;
        } else if header {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() == 5 && cols.iter().all(|c| c.parse::<f64>().is_ok()) {
                rows += 1;
            }
        }
    }
    if let Some(n) = totals {
        return Some(n);
    }
    if header && rows > 0 {
        return Some(rows);
    }
    if no_clip && !header {
        return Some(0);
    }
    None
}

fn find_tool(name: &str, path_var: Option<std::ffi::OsString>) -> Option<PathBuf> {
    let mut dirs: Vec<PathBuf> = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];
    if let Some(pv) = path_var {
        dirs.extend(std::env::split_paths(&pv));
    }
    dirs.into_iter().map(|d| d.join(name)).find(|f| f.is_file())
}

/// Valida el máster con las herramientas de Apple. Sin macOS o sin herramientas devuelve
/// `Unavailable` (`reason.adm_mac_only` / `reason.adm_tools_missing`).
pub fn validate_adm(master_wav: &Path, workdir: &Path, cancel: &dyn Fn() -> bool) -> AdmReport {
    if !cfg!(target_os = "macos") {
        return AdmReport::unavailable("reason.adm_mac_only", String::new());
    }
    let tools = (
        find_tool("afconvert", std::env::var_os("PATH")),
        find_tool("afclip", std::env::var_os("PATH")),
    );
    match tools {
        (Some(c), Some(f)) => validate_with_tools(&c, &f, master_wav, workdir, cancel),
        _ => AdmReport::unavailable("reason.adm_tools_missing", String::new()),
    }
}

enum Run {
    Done(bool, String),
    Cancelled,
}

fn run_one(program: &Path, args: &[OsString], log_file: &Path, cancel: &dyn Fn() -> bool) -> Run {
    let Ok(out) = std::fs::File::create(log_file) else {
        return Run::Done(false, "err.adm_log".into());
    };
    let Ok(err) = out.try_clone() else {
        return Run::Done(false, "err.adm_log".into());
    };
    let Ok(mut child) = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err)
        .spawn()
    else {
        return Run::Done(false, format!("cannot start {}", program.display()));
    };
    let status = loop {
        if cancel() {
            let _ = child.kill();
            let _ = child.wait();
            return Run::Cancelled;
        }
        match child.try_wait() {
            Ok(Some(st)) => break st,
            Ok(None) => std::thread::sleep(Duration::from_millis(50)),
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return Run::Done(false, String::new());
            }
        }
    };
    let text = std::fs::read_to_string(log_file).unwrap_or_default();
    Run::Done(status.success(), text)
}

/// Ejecuta el flujo con las herramientas indicadas (separado para poder probarlo con falsos).
pub fn validate_with_tools(
    afconvert: &Path,
    afclip: &Path,
    master_wav: &Path,
    workdir: &Path,
    cancel: &dyn Fn() -> bool,
) -> AdmReport {
    // Solo la cabecera (no se decodifica el máster entero); sin tasa no se adivina.
    let rate = match wav::probe(master_wav) {
        Ok(i) => i.sample_rate,
        Err(e) => {
            return AdmReport::unavailable(
                "reason.adm_failed",
                format!("cannot read the master header: {e}"),
            )
        }
    };
    if let Err(e) = std::fs::create_dir_all(workdir) {
        return AdmReport::unavailable("reason.adm_failed", format!("{}: {e}", workdir.display()));
    }
    // Rutas absolutas y canónicas: nunca una ruta relativa que empiece por "-" como opción.
    let (master_wav, workdir) = match (master_wav.canonicalize(), workdir.canonicalize()) {
        (Ok(m), Ok(w)) => (m, w),
        (Err(e), _) | (_, Err(e)) => {
            return AdmReport::unavailable("reason.adm_failed", format!("path: {e}"))
        }
    };
    let (master_wav, workdir) = (master_wav.as_path(), workdir.as_path());
    let cmds = build_adm_commands(master_wav, workdir, rate);
    let mut log = String::new();
    let mut clip_out = String::new();
    for (i, cmd) in cmds.iter().enumerate() {
        let program = if cmd[0] == "afclip" {
            afclip
        } else {
            afconvert
        };
        let shown: Vec<String> = cmd
            .iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        log.push_str(&format!("$ {}\n", shown.join(" ")));
        match run_one(
            program,
            &cmd[1..],
            &workdir.join(format!("step{i}.log")),
            cancel,
        ) {
            Run::Cancelled => return AdmReport::unavailable("reason.adm_cancelled", log),
            Run::Done(ok, text) => {
                log.push_str(&text);
                if !ok {
                    return AdmReport::unavailable("reason.adm_failed", log);
                }
                if i == 3 {
                    clip_out = text;
                }
            }
        }
    }
    let _ = std::fs::remove_file(workdir.join("decode.wav"));
    let aac = workdir.join("final.m4a");
    match parse_afclip(&clip_out) {
        Some(0) => AdmReport {
            status: AdmStatus::Ok,
            clipped_samples: 0,
            aac_path: Some(aac),
            log,
        },
        Some(n) => AdmReport {
            status: AdmStatus::Clipping,
            clipped_samples: n,
            aac_path: Some(aac),
            log,
        },
        None => AdmReport::unavailable("reason.adm_parse", log),
    }
}

/// Máster sintético del humo de macOS: estéreo 48 kHz, 24 bit, seno a fs/4 con fase de 45°
/// cuyo pico real es `true_peak_dbtp` y cuyo pico de muestra queda 3 dB por debajo (el exceso
/// es entre muestras). −1 dBTP debe dar 0 muestras con clipping en el AAC; +1 dBTP, más de 0.
pub fn adm_smoke_case(true_peak_dbtp: f64, path: &Path) -> Result<(), AudioError> {
    let amp = 10f64.powf(true_peak_dbtp / 20.0);
    let full = 8_388_607.0;
    let n = 48_000 * 3;
    let ch: Vec<i32> = (0..n)
        .map(|i| {
            let ph = std::f64::consts::FRAC_PI_2 * i as f64 + std::f64::consts::FRAC_PI_4;
            (amp * ph.sin() * full).round().clamp(-full, full) as i32
        })
        .collect();
    wav::write(
        path,
        &[ch.clone(), ch],
        48_000,
        &WavWriteOptions {
            bits: 24,
            metadata: None,
        },
    )
}
