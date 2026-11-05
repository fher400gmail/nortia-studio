//! Lógica de la app de escritorio que no depende de Tauri, para probarla en cualquier
//! sitio (el contenedor de desarrollo no puede compilar Tauri). `app/src-tauri` la usa tal cual.

pub mod lame;
pub mod smoke;

use crate::errors::BridgeError;
use masterout_core::i18n::Lang as EngineLang;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

/// Respuesta del comando `app_info`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AppInfo {
    pub version: String,
    pub os: String,
    pub lame_available: bool,
    /// Carpeta de salida por defecto (lo decide Tauri; el puente no la conoce).
    pub default_out_root: Option<String>,
}

pub fn app_info(version: &str, exe_dir: &Path, default_out_root: Option<String>) -> AppInfo {
    AppInfo {
        version: version.to_string(),
        os: os_name().to_string(),
        lame_available: lame::resolve_lame_path(exe_dir).is_some(),
        default_out_root,
    }
}

/// Segunda exportación mientras hay una en curso. Clave estable `err.busy`; el texto vive aquí
/// porque es un error de la app, no del motor.
pub fn busy_error(lang: EngineLang) -> BridgeError {
    let message = match lang {
        EngineLang::Es => "Ya hay una exportación en curso.",
        EngineLang::En => "An export is already running.",
    };
    BridgeError {
        key: "err.busy".into(),
        message: message.into(),
        detail: None,
    }
}

/// Solo imágenes (`.png`, `.jpg`, `.jpeg`) pueden autorizarse por `allow_asset`.
pub fn is_image_path(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .is_some_and(|e| matches!(e.as_str(), "png" | "jpg" | "jpeg"))
}
