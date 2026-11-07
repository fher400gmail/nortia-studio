//! Lógica de la app de escritorio que no depende de Tauri, para probarla en cualquier
//! sitio (el contenedor de desarrollo no puede compilar Tauri). `app/src-tauri` la usa tal cual.

pub mod panic_log;
pub mod smoke;

use crate::errors::BridgeError;
use masterout_core::i18n::Lang as EngineLang;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// `"es-ES"`, `"es_MX.UTF-8"` -> `"es"`; cualquier otro (incluido vacío) -> `"en"`.
pub fn map_locale(tag: &str) -> &'static str {
    match EngineLang::from_tag(tag) {
        EngineLang::Es => "es",
        EngineLang::En => "en",
    }
}

/// `"mac"`, `"win"` o `"linux"`.
pub fn os_name() -> &'static str {
    if cfg!(target_os = "macos") {
        "mac"
    } else if cfg!(target_os = "windows") {
        "win"
    } else {
        "linux"
    }
}

/// Release del sidecar ffmpeg que empaqueta esta versión (el mismo que descarga el CI;
/// `app/scripts/sidecar-tag.txt`). Lo muestra «Acerca de».
pub const SIDECAR_RELEASE: &str = "ffmpeg-sidecar-v7.1.5-c74d220";

/// Respuesta del comando `app_info`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AppInfo {
    pub version: String,
    pub os: String,
    /// ffmpeg/ffprobe (vídeo y MP3) disponibles: junto al ejecutable (en producción solo ahí; en
    /// depuración también `MASTERCHECK_FFMPEG`/`MASTERCHECK_FFPROBE` y PATH).
    pub sidecar_available: bool,
    /// Release del componente de vídeo y MP3 empaquetado; `None` si no está junto al ejecutable.
    #[serde(default)]
    pub sidecar_release: Option<String>,
    /// Destinos que arrancan encendidos (`default_on` de presets.json): la interfaz no duplica la lista.
    #[serde(default)]
    pub default_on: Vec<String>,
    /// Carpeta de salida por defecto (lo decide Tauri; el puente no la conoce).
    pub default_out_root: Option<String>,
}

pub fn app_info(version: &str, exe_dir: &Path, default_out_root: Option<String>) -> AppInfo {
    app_info_with(
        version,
        exe_dir,
        default_out_root,
        &|k| std::env::var_os(k),
        std::env::var_os("PATH"),
    )
}

/// Como [`app_info`], con el entorno y el PATH inyectados (para probarlo). La release del
/// sidecar solo se anuncia si ffmpeg y ffprobe están junto al ejecutable (lo empaquetado por
/// `externalBin`): en depuración, uno de `MASTERCHECK_FFMPEG` o del PATH puede ser cualquier
/// otra compilación (p. ej. con GPL) y «Acerca de» no debe atribuirle la release. En
/// producción el motor ni siquiera los busca (`media::EXTERNAL_SIDECAR`).
pub fn app_info_with(
    version: &str,
    exe_dir: &Path,
    default_out_root: Option<String>,
    env: &dyn Fn(&str) -> Option<std::ffi::OsString>,
    path_var: Option<std::ffi::OsString>,
) -> AppInfo {
    let found = masterout_core::media::discover_with(exe_dir, env, path_var);
    let bundled = found
        .as_ref()
        .is_some_and(|s| s.ffmpeg.parent() == Some(exe_dir) && s.ffprobe.parent() == Some(exe_dir));
    AppInfo {
        version: version.to_string(),
        os: os_name().to_string(),
        sidecar_available: found.is_some(),
        sidecar_release: bundled.then(|| SIDECAR_RELEASE.to_string()),
        default_on: masterout_core::presets::all()
            .into_iter()
            .filter(|p| p.default_on)
            .map(|p| p.id)
            .collect(),
        default_out_root,
    }
}

/// Segunda exportación mientras hay una en curso. Clave estable `err.busy`, texto por i18n.
pub fn busy_error(lang: EngineLang) -> BridgeError {
    BridgeError::from_key("err.busy", lang, None)
}

/// Imágenes y vídeos admitidos en las fichas (`allow_asset` y las miniaturas).
pub use crate::thumbnail::is_media_path;

/// Solo imágenes (`.png`, `.jpg`, `.jpeg`).
pub fn is_image_path(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .is_some_and(|e| matches!(e.as_str(), "png" | "jpg" | "jpeg"))
}

/// Carpeta de la última exportación completada o conservada (`None` si se canceló sin dejar
/// archivos: la carpeta llega vacía).
pub fn last_delivery(d: &crate::dto::DeliveryDto) -> Option<PathBuf> {
    (!d.folder.is_empty()).then(|| PathBuf::from(&d.folder))
}

/// Quita el prefijo `\\?\` que `canonicalize` pone en Windows (el explorador no lo siempre
/// entiende); las rutas UNC verbatim se dejan como están.
pub fn plain_path(p: &Path) -> PathBuf {
    let s = p.to_string_lossy();
    match s.strip_prefix(r"\\?\") {
        Some(rest) if !rest.starts_with("UNC\\") => PathBuf::from(rest),
        _ => p.to_path_buf(),
    }
}

/// ¿Puede la interfaz pedir que se revele `path`? Solo la última entrega que conoce el backend y
/// lo que hay dentro de ella. Se compara en forma canónica (sin `..` ni enlaces simbólicos que
/// salgan) y la ruta tiene que existir. Cualquier otra cosa es `err.internal`, sin repetir en el
/// detalle la ruta pedida.
pub fn check_reveal(
    path: &str,
    last: Option<&Path>,
    lang: EngineLang,
) -> Result<PathBuf, BridgeError> {
    let refused = || {
        BridgeError::from_key(
            "err.internal",
            lang,
            Some("ruta fuera de la última entrega".to_string()),
        )
    };
    let Some(last) = last.filter(|l| !l.as_os_str().is_empty()) else {
        return Err(refused());
    };
    if path.trim().is_empty() {
        return Err(refused());
    }
    let (Ok(root), Ok(target)) = (std::fs::canonicalize(last), std::fs::canonicalize(path)) else {
        return Err(refused());
    };
    if target.starts_with(&root) {
        Ok(target)
    } else {
        Err(refused())
    }
}
