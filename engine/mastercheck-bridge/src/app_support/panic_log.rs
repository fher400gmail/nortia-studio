//! Registro de panics de la app: un `std::panic::set_hook` que apunta el panic en un archivo de
//! registro del usuario y no escribe nada en stderr (en una app de escritorio no hay terminal, y
//! el mensaje por defecto solo ensucia la salida de quien la lance a mano). Los destinos de una
//! exportación ya contienen sus panics (`catch_unwind`); esto deja constancia para soporte.

use std::ffi::OsString;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Nombre del archivo de registro.
pub const LOG_FILE: &str = "panics.log";
/// Tope del archivo: pasado este tamaño se reinicia (no crece sin límite).
pub const MAX_LOG_BYTES: u64 = 256 * 1024;

/// Carpeta de registros del usuario según el sistema (`"mac"`, `"win"`, otro = Linux/XDG).
/// `env` da las variables de entorno (inyectable en pruebas). `None` si no hay dónde.
pub fn log_dir_for(os: &str, env: &dyn Fn(&str) -> Option<String>) -> Option<PathBuf> {
    let non_empty = |k: &str| env(k).filter(|v| !v.is_empty());
    match os {
        "mac" => non_empty("HOME").map(|h| {
            PathBuf::from(h)
                .join("Library")
                .join("Logs")
                .join("master check")
        }),
        "win" => {
            non_empty("LOCALAPPDATA").map(|d| PathBuf::from(d).join("master check").join("logs"))
        }
        _ => non_empty("XDG_STATE_HOME")
            .map(|d| PathBuf::from(d).join("master check"))
            .or_else(|| {
                non_empty("HOME").map(|h| {
                    PathBuf::from(h)
                        .join(".local")
                        .join("state")
                        .join("master check")
                })
            }),
    }
}

/// Carpeta de registros de este equipo.
pub fn log_dir() -> Option<PathBuf> {
    log_dir_for(super::os_name(), &|k| {
        std::env::var_os(k).and_then(|v: OsString| v.into_string().ok())
    })
}

/// Una línea del registro: `<segundos unix> <archivo:línea> <mensaje>`; el mensaje va en una sola
/// línea.
pub fn format_entry(unix_secs: u64, location: Option<(&str, u32)>, message: &str) -> String {
    let place = match location {
        Some((file, line)) => format!("{file}:{line}"),
        None => "?".to_string(),
    };
    let message = message.replace(['\r', '\n'], " ");
    format!("{unix_secs} {place} {message}\n")
}

fn payload_text(info: &std::panic::PanicHookInfo<'_>) -> String {
    let p = info.payload();
    if let Some(s) = p.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = p.downcast_ref::<String>() {
        s.clone()
    } else {
        "(carga del panic sin texto)".to_string()
    }
}

fn append(dir: &Path, entry: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let file = dir.join(LOG_FILE);
    let too_big = std::fs::metadata(&file).is_ok_and(|m| m.len() > MAX_LOG_BYTES);
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(!too_big)
        .truncate(too_big)
        .open(file)?;
    f.write_all(entry.as_bytes())
}

/// Instala el gancho: cada panic se apunta en `dir/panics.log` y no sale nada por stderr. Si el
/// registro no se puede escribir, el panic se descarta en silencio (nunca otro panic dentro del
/// gancho).
pub fn install(dir: PathBuf) {
    std::panic::set_hook(Box::new(move |info| {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let entry = format_entry(
            secs,
            info.location().map(|l| (l.file(), l.line())),
            &payload_text(info),
        );
        let _ = append(&dir, &entry);
    }));
}

/// Anota en el mismo registro un temporal que no se pudo borrar tras los reintentos (ruta, error).
pub fn install_leak_log(dir: PathBuf) {
    masterout_core::fsutil::set_failure_hook(move |path, e| {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let _ = append(&dir, &leak_entry(secs, path, &e.to_string()));
    });
}

/// Línea del registro para un temporal que quedó sin borrar.
pub fn leak_entry(unix_secs: u64, path: &Path, error: &str) -> String {
    format_entry(
        unix_secs,
        None,
        &format!("temporal sin borrar {}: {error}", path.display()),
    )
}
