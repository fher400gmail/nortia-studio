//! Escritor FLAC de 16/24 bit con Vorbis comments (codificador `flac-codec`, Rust puro).
//!
//! No se usa `flacenc`: solo implementa Rice de 4 bit (parámetro <= 14) y, con audio de 24 bit
//! fuerte, sus predictores producen subtramas mayores que el PCM (el FLAC salía 2,6x el WAV).
//! `flac-codec` usa Rice de 5 bit y comprime como `flac -5`.

use crate::audio::AudioError;
use flac_codec::encode::{FlacChannelWriter, Options};
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct FlacTags {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub date: Option<String>,
    pub isrc: Option<String>,
    pub copyright: Option<String>,
}

/// Muestras por bloque FLAC (4096 = valor por defecto de `flac`, válido en perfil subset).
const BLOCK_SIZE: usize = 4096;

fn unsupported(msg: impl Into<String>) -> AudioError {
    AudioError::UnsupportedFormat(msg.into())
}

/// Escribe un FLAC de 16 o 24 bit. `samples_i32[canal][frame]` en el rango de `bits`.
pub fn write(
    path: &Path,
    samples_i32: &[Vec<i32>],
    sample_rate: u32,
    bits: u16,
    tags: Option<&FlacTags>,
) -> Result<(), AudioError> {
    write_cancellable(path, samples_i32, sample_rate, bits, tags, &|| false)
}

/// Como `write`, comprobando `should_cancel` cada 64 bloques; si devuelve `true` termina con
/// `AudioError::Cancelled` (el archivo parcial lo borra quien lo llama, p. ej. `atomic`).
pub fn write_cancellable(
    path: &Path,
    samples_i32: &[Vec<i32>],
    sample_rate: u32,
    bits: u16,
    tags: Option<&FlacTags>,
    should_cancel: &dyn Fn() -> bool,
) -> Result<(), AudioError> {
    if bits != 16 && bits != 24 {
        return Err(unsupported(format!("err.flac: {bits} bit")));
    }
    if sample_rate > 96_000 {
        return Err(unsupported("err.rate_over_96k"));
    }
    let channels = samples_i32.len();
    if channels == 0 || channels > 2 {
        return Err(AudioError::TooManyChannels(channels as u16));
    }
    let frames = samples_i32[0].len();
    if samples_i32.iter().any(|c| c.len() != frames) {
        return Err(unsupported("err.flac: channel length mismatch"));
    }
    let limit = 1i32 << (bits - 1);
    if samples_i32
        .iter()
        .flatten()
        .any(|&s| s < -limit || s >= limit)
    {
        return Err(unsupported("err.flac: sample out of range"));
    }
    let enc = |e: flac_codec::Error| unsupported(format!("err.flac: encode {e:?}"));
    let mut options = Options::default().no_padding().no_seektable().overwrite();
    if let Some(t) = tags {
        options = options.tag("TITLE", &t.title).tag("ARTIST", &t.artist);
        for (k, v) in [
            ("ALBUM", &t.album),
            ("DATE", &t.date),
            ("ISRC", &t.isrc),
            ("COPYRIGHT", &t.copyright),
        ] {
            if let Some(v) = v.as_ref().filter(|v| !v.is_empty()) {
                options = options.tag(k, v);
            }
        }
    }
    let block_size = BLOCK_SIZE;
    // Se codifica y escribe bloque a bloque, directamente desde los canales (sin copia
    // entrelazada del audio completo ni flujo FLAC completo en memoria).
    let mut writer = FlacChannelWriter::create(
        path,
        options,
        sample_rate,
        u32::from(bits),
        channels as u8,
        Some(frames as u64),
    )
    .map_err(enc)?;
    let mut head = 0;
    let mut blocks = 0usize;
    while head < frames {
        if blocks.is_multiple_of(64) && should_cancel() {
            return Err(AudioError::Cancelled);
        }
        blocks += 1;
        let end = (head + block_size).min(frames);
        let block: Vec<&[i32]> = samples_i32.iter().map(|c| &c[head..end]).collect();
        writer.write(&block).map_err(enc)?;
        head = end;
    }
    writer.finalize().map_err(enc)?;
    Ok(())
}
