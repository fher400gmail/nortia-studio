//! Localización de `lame` (la lógica vive en el puente para poder probarla sin Tauri).

use std::path::{Path, PathBuf};

pub use mastercheck_bridge::app_support::lame::{path_with_dir_first, resolve_lame_in};

/// Junto al ejecutable (macOS: `Contents/MacOS` y `Contents/Resources`; Windows: mismo
/// directorio), después `PATH`.
pub fn resolve_lame_path(exe_dir: &Path) -> Option<PathBuf> {
    mastercheck_bridge::app_support::lame::resolve_lame_path(exe_dir)
}
