//! Errores del motor traducidos a clave estable + mensaje en el idioma pedido.

use masterout_core::audio::AudioError;
use masterout_core::i18n::{t, Lang};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, thiserror::Error)]
#[error("{key}: {message}")]
pub struct BridgeError {
    /// Clave estable, p. ej. `"err.rate_over_96k"`, `"err.not_wav"`, `"err.io"`.
    pub key: String,
    /// Texto en el idioma pedido.
    pub message: String,
    /// Detalle técnico (no localizado), si lo hay.
    pub detail: Option<String>,
}

impl BridgeError {
    /// Error con el texto de `key` en `lang`.
    pub fn from_key(key: &str, lang: Lang, detail: Option<String>) -> BridgeError {
        BridgeError {
            key: key.to_string(),
            message: t(lang, key),
            detail,
        }
    }
}

pub fn localize(err: &AudioError, lang: Lang) -> BridgeError {
    let (key, detail) = err.key_and_detail();
    BridgeError::from_key(&key, lang, detail)
}
