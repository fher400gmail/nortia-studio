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
pub const SIDECAR_RELEASE: &str = "ffmpeg-sidecar-v7.1.5-3a21763";

/// Respuesta del comando `app_info`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AppInfo {
    pub version: String,
    pub os: String,
    /// ffmpeg/ffprobe (vídeo y MP3) disponibles junto al ejecutable, en el entorno o en PATH.
    pub sidecar_available: bool,
    /// Release del componente de vídeo y MP3 (vacío si no está disponible).
    #[serde(default)]
    pub sidecar_release: String,
    /// Destinos que arrancan encendidos (`default_on` de presets.json): la interfaz no duplica la lista.
    #[serde(default)]
    pub default_on: Vec<String>,
    /// Carpeta de salida por defecto (lo decide Tauri; el puente no la conoce).
    pub default_out_root: Option<String>,
}

pub fn app_info(version: &str, exe_dir: &Path, default_out_root: Option<String>) -> AppInfo {
    let sidecar_available = masterout_core::media::discover(exe_dir).is_some();
    AppInfo {
        version: version.to_string(),
        os: os_name().to_string(),
        sidecar_available,
        sidecar_release: if sidecar_available {
            SIDECAR_RELEASE.to_string()
        } else {
            String::new()
        },
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
