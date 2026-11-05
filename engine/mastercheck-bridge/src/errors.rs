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
    match err {
        AudioError::UnsupportedFormat(s) if s.starts_with("err.") => {
            BridgeError::from_key(s, lang, None)
        }
        AudioError::UnsupportedFormat(s) => {
            BridgeError::from_key("err.generic", lang, Some(s.clone()))
        }
        AudioError::Io(e) => BridgeError::from_key("err.io", lang, Some(e.to_string())),
        AudioError::NotRiffWave => BridgeError::from_key("err.not_wav", lang, None),
        AudioError::MissingFmt | AudioError::MissingData => {
            BridgeError::from_key("err.not_wav", lang, Some(err.to_string()))
        }
        AudioError::TooManyChannels(_) => {
            BridgeError::from_key("err.too_many_channels", lang, Some(err.to_string()))
        }
        AudioError::Rf64Unsupported => BridgeError::from_key("err.rf64_unsupported", lang, None),
    }
}
