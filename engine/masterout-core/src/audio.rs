//! Tipos de audio compartidos por el motor.

use thiserror::Error;

/// Audio en memoria, en coma flotante `f32` y de 1 o 2 canales.
///
/// Los enteros de 16/24/32 bit se escalan dividiendo por 2^(bits-1), de modo que
/// el rango es -1.0 <= x < 1.0. Los WAV float32 se copian tal cual (pueden superar +-1.0).
#[derive(Debug, Clone, PartialEq)]
pub struct AudioBuffer {
    pub sample_rate: u32,
    /// Muestras por canal; todos los canales tienen la misma longitud (el motor lo da por supuesto).
    pub channels: Vec<Vec<f32>>,
    pub source_bits: u16,
    pub source_float: bool,
}

impl AudioBuffer {
    /// Número de frames (muestras por canal).
    pub fn frames(&self) -> usize {
        self.channels.first().map_or(0, Vec::len)
    }

    /// Duración en segundos.
    pub fn duration_secs(&self) -> f64 {
        if self.sample_rate == 0 {
            return 0.0;
        }
        self.frames() as f64 / f64::from(self.sample_rate)
    }

    pub fn num_channels(&self) -> usize {
        self.channels.len()
    }
}

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("error de E/S: {0}")]
    Io(#[from] std::io::Error),
    #[error("el archivo no es un RIFF/WAVE válido")]
    NotRiffWave,
    #[error("falta el chunk fmt")]
    MissingFmt,
    #[error("falta el chunk data")]
    MissingData,
    #[error("formato no soportado: {0}")]
    UnsupportedFormat(String),
    #[error("demasiados canales: {0} (solo se admiten mono y estéreo)")]
    TooManyChannels(u16),
    #[error("los archivos RF64 no están soportados")]
    Rf64Unsupported,
    /// La operación se detuvo porque se pidió cancelar; no es un fallo.
    #[error("cancelado")]
    Cancelled,
}

impl AudioError {
    /// Clave i18n estable y detalle técnico (no localizado). `UnsupportedFormat("err.x: detalle")`
    /// aporta su propia clave; E/S es `err.io`; el resto, `err.generic` con el texto como detalle.
    pub fn key_and_detail(&self) -> (String, Option<String>) {
        match self {
            AudioError::UnsupportedFormat(s) if s.starts_with("err.") => match s.split_once(':') {
                Some((k, d)) => (k.trim().to_string(), Some(d.trim().to_string())),
                None => (s.clone(), None),
            },
            AudioError::Io(e) => ("err.io".into(), Some(e.to_string())),
            AudioError::Cancelled => ("err.cancelled".into(), None),
            AudioError::NotRiffWave => ("err.not_wav".into(), None),
            AudioError::MissingFmt | AudioError::MissingData => {
                ("err.not_wav".into(), Some(self.to_string()))
            }
            AudioError::TooManyChannels(_) => {
                ("err.too_many_channels".into(), Some(self.to_string()))
            }
            AudioError::Rf64Unsupported => ("err.rf64_unsupported".into(), None),
            AudioError::UnsupportedFormat(s) => ("err.generic".into(), Some(s.clone())),
        }
    }
}
