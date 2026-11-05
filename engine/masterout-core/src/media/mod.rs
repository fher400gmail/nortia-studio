//! Medios (imagen, vídeo, sonda) y localización del sidecar ffmpeg/ffprobe.

pub mod image;
pub mod probe;
pub mod video;

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// Plataforma de destino de los codificadores de vídeo. `Linux` existe SOLO para
/// pruebas locales y de CI con un ffmpeg del sistema (`libx264`): nunca se distribuye.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    MacOS,
    Windows,
    Linux,
}

impl Platform {
    pub fn current() -> Platform {
        if cfg!(target_os = "macos") {
            Platform::MacOS
        } else if cfg!(target_os = "windows") {
            Platform::Windows
        } else {
            Platform::Linux
        }
    }
}

/// Rutas de los ejecutables del sidecar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidecarPaths {
    pub ffmpeg: PathBuf,
    pub ffprobe: PathBuf,
}

const EXE_SUFFIX: &str = if cfg!(windows) { ".exe" } else { "" };

fn find_one(
    name: &str,
    env_var: &str,
    exe_dir: &Path,
    env: &dyn Fn(&str) -> Option<OsString>,
    path_var: Option<OsString>,
) -> Option<PathBuf> {
    let file = format!("{name}{EXE_SUFFIX}");
    // 1) junto al ejecutable (Tauri externalBin, sin sufijo de target en el bundle)
    let next_to_exe = exe_dir.join(&file);
    if next_to_exe.is_file() {
        return Some(next_to_exe);
    }
    // 2) variable de entorno
    if let Some(v) = env(env_var).filter(|v| !v.is_empty()) {
        let p = PathBuf::from(v);
        if p.is_file() {
            return Some(p);
        }
    }
    // 3) PATH
    let path_var = path_var?;
    std::env::split_paths(&path_var)
        .map(|d| d.join(&file))
        .find(|p| p.is_file())
}

/// Versión pura de [`discover`] (entorno y PATH inyectados) para poder probarla.
pub fn discover_with(
    exe_dir: &Path,
    env: &dyn Fn(&str) -> Option<OsString>,
    path_var: Option<OsString>,
) -> Option<SidecarPaths> {
    Some(SidecarPaths {
        ffmpeg: find_one(
            "ffmpeg",
            "MASTERCHECK_FFMPEG",
            exe_dir,
            env,
            path_var.clone(),
        )?,
        ffprobe: find_one("ffprobe", "MASTERCHECK_FFPROBE", exe_dir, env, path_var)?,
    })
}

/// Busca `ffmpeg` y `ffprobe`: junto al ejecutable, luego `MASTERCHECK_FFMPEG` /
/// `MASTERCHECK_FFPROBE`, luego PATH. `None` si falta alguno de los dos.
pub fn discover(exe_dir: &Path) -> Option<SidecarPaths> {
    discover_with(exe_dir, &|k| std::env::var_os(k), std::env::var_os("PATH"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn touch(p: &Path) {
        std::fs::write(p, b"x").unwrap();
    }
    fn exe(n: &str) -> String {
        format!("{n}{EXE_SUFFIX}")
    }

    #[test]
    fn discover_prefers_exe_dir_then_env_then_path() {
        let exe_dir = tempfile::tempdir().unwrap();
        let env_dir = tempfile::tempdir().unwrap();
        let path_dir = tempfile::tempdir().unwrap();
        for d in [&env_dir, &path_dir] {
            touch(&d.path().join(exe("ffmpeg")));
            touch(&d.path().join(exe("ffprobe")));
        }
        let path_var = std::env::join_paths([path_dir.path()]).ok();
        let env_ff = env_dir.path().join(exe("ffmpeg"));
        let env_fp = env_dir.path().join(exe("ffprobe"));
        let env = |k: &str| match k {
            "MASTERCHECK_FFMPEG" => Some(env_ff.clone().into_os_string()),
            "MASTERCHECK_FFPROBE" => Some(env_fp.clone().into_os_string()),
            _ => None,
        };
        // PATH y entorno: gana el entorno
        let got = discover_with(exe_dir.path(), &env, path_var.clone()).unwrap();
        assert_eq!(got.ffmpeg, env_ff);
        // sin entorno: gana PATH
        let got = discover_with(exe_dir.path(), &|_| None, path_var.clone()).unwrap();
        assert_eq!(got.ffprobe, path_dir.path().join(exe("ffprobe")));
        // junto al ejecutable: gana sobre todo
        touch(&exe_dir.path().join(exe("ffmpeg")));
        touch(&exe_dir.path().join(exe("ffprobe")));
        let got = discover_with(exe_dir.path(), &env, path_var).unwrap();
        assert_eq!(got.ffmpeg, exe_dir.path().join(exe("ffmpeg")));
    }

    #[test]
    fn discover_none_when_either_binary_is_missing() {
        let d = tempfile::tempdir().unwrap();
        touch(&d.path().join(exe("ffmpeg")));
        assert!(discover_with(d.path(), &|_| None, None).is_none());
        assert!(discover_with(Path::new("/nonexistent-dir"), &|_| None, None).is_none());
    }
}
