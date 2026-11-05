//! Validación Apple Digital Masters (solo macOS): máster -> CAF con Sound Check -> AAC 256 kbps
//! -> `afclip` cuenta las muestras con clipping del AAC decodificado.
//!
//! Flujo verificado (docs/skills-baseline/verificacion-2026-11-05.md, bloque C, pto 7):
//!
//! ```text
//! afconvert master.wav intermediate.caf -d 0 -f caff --soundcheck-generate
//!   (al bajar de tasa: -d LEF32@<tasa> ... --src-complexity bats -r 127)
//! afconvert intermediate.caf final.m4a -d aac -f m4af -u pgcm 2 --soundcheck-read -b 256000 -q 127 -s 2
//! afconvert final.m4a decode.wav -d LEI24 -f WAVE
//! afclip final.m4a
//! ```
//!
//! El AAC de ADM se codifica como mucho a 48 kHz: familia 44,1 kHz -> 44100; el resto -> 48000.
//! `afclip` lo distribuye Apple con las herramientas de Apple Digital Masters y no siempre está.
//!
//! Prueba real (solo en el humo de CI de macOS): `adm_smoke_case(-1.0, ..)` debe dar 0 muestras
//! con clipping y `adm_smoke_case(1.0, ..)` más de 0.

use crate::audio::AudioError;
use crate::wav::{self, WavWriteOptions};
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

fn s(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn p(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

/// Las cuatro órdenes (programa en la posición 0) de la validación, en orden. Pura.
pub fn build_adm_commands(master_wav: &Path, workdir: &Path, master_rate: u32) -> Vec<Vec<String>> {
    let caf = p(&workdir.join("intermediate.caf"));
    let m4a = p(&workdir.join("final.m4a"));
    let dec = p(&workdir.join("decode.wav"));
    let mut step1 = vec!["afconvert".to_string(), p(master_wav), caf.clone()];
    if master_rate > 48_000 {
        let fmt = format!("LEF32@{}", aac_rate(master_rate));
        step1.extend(s(&["-d", &fmt, "-f", "caff", "--soundcheck-generate"]));
        step1.extend(s(&["--src-complexity", "bats", "-r", "127"]));
    } else {
        step1.extend(s(&["-d", "0", "-f", "caff", "--soundcheck-generate"]));
    }
    let mut step2 = vec!["afconvert".to_string(), caf, m4a.clone()];
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
    let mut step3 = vec!["afconvert".to_string(), m4a.clone(), dec];
    step3.extend(s(&["-d", "LEI24", "-f", "WAVE"]));
    vec![step1, step2, step3, vec!["afclip".to_string(), m4a]]
}

/// Número de muestras con clipping según la salida de `afclip`; `None` si el formato no se
/// reconoce. Mira solo las líneas que mencionan "clip" (sin el nombre del programa ni rutas):
/// una línea "total" manda; si no, se suman los enteros de cada línea (p. ej. por canal).
/// "no clipping", "none" o ausencia de número en una línea de clip cuentan como cero.
pub fn parse_afclip(output: &str) -> Option<u64> {
    let mut total: Option<u64> = None;
    let mut sum: Option<u64> = None;
    for line in output.lines() {
        let tokens: Vec<&str> = line
            .split_whitespace()
            .filter(|t| !t.contains('/') && !t.contains('\\') && !t.ends_with(".m4a"))
            .collect();
        let text = tokens.join(" ").to_lowercase().replace("afclip", "");
        if !text.contains("clip") {
            continue;
        }
        let words: Vec<&str> = text.split_whitespace().collect();
        let num = |w: &str| -> Option<u64> {
            let d: String = w.chars().filter(|c| c.is_ascii_digit()).collect();
            let only_num = w.chars().all(|c| c.is_ascii_digit() || c == ',');
            if only_num {
                d.parse().ok()
            } else {
                None
            }
        };
        // el número pegado a la palabra "clip": antes ("12 clipped") o después ("clipped: 12");
        // si no, el último de la línea (así "channel 0: 3 clipped" no toma el canal).
        let at = words.iter().position(|w| w.contains("clip")).unwrap_or(0);
        let near = [
            at.checked_sub(1),
            at.checked_sub(2),
            Some(at + 1),
            Some(at + 2),
        ]
        .into_iter()
        .flatten()
        .filter_map(|i| words.get(i))
        .find_map(|w| num(w.trim_end_matches(':')));
        let nums: Vec<u64> = near
            .or_else(|| {
                words
                    .iter()
                    .rev()
                    .find_map(|w| num(w.trim_end_matches(':')))
            })
            .into_iter()
            .collect();
        let n = match nums.first() {
            Some(n) => *n,
            None => {
                // sin número: solo vale si dice claramente que no hay
                if ["no clip", "not clip", "without clip", "none", "no samples"]
                    .iter()
                    .any(|k| text.contains(k))
                {
                    0
                } else {
                    continue;
                }
            }
        };
        if text.contains("total") {
            total = Some(n);
        }
        sum = Some(sum.unwrap_or(0).saturating_add(n));
    }
    total.or(sum)
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

fn run_one(program: &Path, args: &[String], log_file: &Path, cancel: &dyn Fn() -> bool) -> Run {
    let Ok(out) = std::fs::File::create(log_file) else {
        return Run::Done(false, "cannot create log".into());
    };
    let Ok(err) = out.try_clone() else {
        return Run::Done(false, "cannot create log".into());
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
    let rate = wav::read(master_wav)
        .map(|b| b.sample_rate)
        .unwrap_or(48_000);
    if std::fs::create_dir_all(workdir).is_err() {
        return AdmReport::unavailable("reason.adm_failed", String::new());
    }
    let cmds = build_adm_commands(master_wav, workdir, rate);
    let mut log = String::new();
    let mut clip_out = String::new();
    for (i, cmd) in cmds.iter().enumerate() {
        let program = if cmd[0] == "afclip" {
            afclip
        } else {
            afconvert
        };
        log.push_str(&format!("$ {}\n", cmd.join(" ")));
        match run_one(
            program,
            &cmd[1..],
            &workdir.join(format!("step{i}.log")),
            cancel,
        ) {
            Run::Cancelled => return AdmReport::unavailable("reason.adm_cancelled", log),
            Run::Done(ok, text) => {
                log.push_str(&text);
                // la descodificación a WAV es informativa: su fallo no impide contar clips
                if !ok && i != 2 {
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
