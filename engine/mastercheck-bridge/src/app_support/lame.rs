//! Localización del ejecutable `lame` empaquetado con la app.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

pub fn exe_name() -> &'static str {
    if cfg!(windows) {
        "lame.exe"
    } else {
        "lame"
    }
}

fn is_executable(p: &Path) -> bool {
    if !p.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(p).is_ok_and(|m| m.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        true
    }
}

/// Directorios donde puede estar `lame` empaquetado: junto al ejecutable (Windows y
/// macOS `Contents/MacOS`) y, en macOS, `Contents/Resources` (y su subcarpeta `lame`).
pub fn bundle_dirs(exe_dir: &Path) -> Vec<PathBuf> {
    let mut dirs = vec![exe_dir.to_path_buf(), exe_dir.join("lame")];
    if let Some(contents) = exe_dir.parent() {
        dirs.push(contents.join("Resources"));
        dirs.push(contents.join("Resources").join("lame"));
    }
    dirs
}

/// Como [`resolve_lame_path`], con el valor de `PATH` explícito (para probar sin tocar el entorno).
pub fn resolve_lame_in(exe_dir: &Path, path_var: Option<&OsStr>) -> Option<PathBuf> {
    let mut dirs = bundle_dirs(exe_dir);
    if let Some(p) = path_var {
        dirs.extend(std::env::split_paths(p));
    }
    dirs.into_iter()
        .map(|d| d.join(exe_name()))
        .find(|p| is_executable(p))
}

/// Primero junto al ejecutable (paquete), después `PATH`.
pub fn resolve_lame_path(exe_dir: &Path) -> Option<PathBuf> {
    resolve_lame_in(exe_dir, std::env::var_os("PATH").as_deref())
}

/// `PATH` con `dir` delante. El motor busca `lame` junto al ejecutable y en `PATH`; la app
/// añade así `Contents/Resources` (macOS) sin tocar el motor.
pub fn path_with_dir_first(dir: &Path, existing: Option<OsString>) -> OsString {
    let mut parts = vec![dir.to_path_buf()];
    if let Some(e) = existing {
        parts.extend(std::env::split_paths(&e));
    }
    std::env::join_paths(parts).unwrap_or_else(|_| dir.as_os_str().to_os_string())
}
