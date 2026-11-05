//! Idioma a partir del locale del sistema.

use masterout_core::i18n::Lang as EngineLang;

/// Espacio de nombres de la conversión de locale a idioma del motor.
pub struct Lang;

impl Lang {
    /// `"es-ES"`, `"es_MX.UTF-8"` -> `Es`; cualquier otro (incluido vacío) -> `En`.
    pub fn from_locale(tag: &str) -> EngineLang {
        EngineLang::from_tag(tag)
    }
}
