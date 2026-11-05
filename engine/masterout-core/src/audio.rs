//! Tipos de audio compartidos por el motor.

use thiserror::Error;

/// Audio en memoria, en coma flotante `f32` y de 1 o 2 canales.
///
/// Los enteros de 16/24/32 bit se escalan dividiendo por 2^(bits-1), de modo que
/// el rango es -1.0 <= x < 1.0. Los WAV float32 se copian tal cual (pueden superar +-1.0).
#[derive(Debug, Clone, PartialEq)]
pub struct AudioBuffer {
    pub sample_rate: u32,
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
}
