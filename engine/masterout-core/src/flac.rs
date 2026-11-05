//! Escritor FLAC de 16/24 bit con Vorbis comments (codificador `flacenc`, Rust puro).

use crate::audio::AudioError;
use flacenc::bitsink::ByteSink;
use flacenc::component::BitRepr;
use flacenc::error::Verify;
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

fn unsupported(msg: impl Into<String>) -> AudioError {
    AudioError::UnsupportedFormat(msg.into())
}

/// Bloque VORBIS_COMMENT (tipo 4) completo, con su cabecera de 4 bytes.
fn vorbis_comment_block(tags: &FlacTags, last: bool) -> Vec<u8> {
    let vendor = concat!("masterout-core ", env!("CARGO_PKG_VERSION"));
    let mut comments: Vec<String> = vec![
        format!("TITLE={}", tags.title),
        format!("ARTIST={}", tags.artist),
    ];
    for (k, v) in [
        ("ALBUM", &tags.album),
        ("DATE", &tags.date),
        ("ISRC", &tags.isrc),
        ("COPYRIGHT", &tags.copyright),
    ] {
        if let Some(v) = v.as_ref().filter(|v| !v.is_empty()) {
            comments.push(format!("{k}={v}"));
        }
    }
    let mut body = Vec::new();
    body.extend_from_slice(&(vendor.len() as u32).to_le_bytes());
    body.extend_from_slice(vendor.as_bytes());
    body.extend_from_slice(&(comments.len() as u32).to_le_bytes());
    for c in &comments {
        body.extend_from_slice(&(c.len() as u32).to_le_bytes());
        body.extend_from_slice(c.as_bytes());
    }
    let len = body.len() as u32;
    let mut block = Vec::with_capacity(body.len() + 4);
    block.push(4 | if last { 0x80 } else { 0 });
    block.extend_from_slice(&len.to_be_bytes()[1..]);
    block.extend_from_slice(&body);
    block
}

/// Inserta el bloque de comentarios justo tras STREAMINFO.
fn insert_tags(bytes: Vec<u8>, tags: &FlacTags) -> Result<Vec<u8>, AudioError> {
    const STREAMINFO_END: usize = 4 + 4 + 34;
    if bytes.len() < STREAMINFO_END || &bytes[..4] != b"fLaC" || bytes[4] & 0x7f != 0 {
        return Err(unsupported("salida FLAC inesperada"));
    }
    let streaminfo_was_last = bytes[4] & 0x80 != 0;
    let mut out = Vec::with_capacity(bytes.len() + 256);
    out.extend_from_slice(&bytes[..STREAMINFO_END]);
    out[4] &= 0x7f;
    // Si STREAMINFO era el último bloque, el nuestro pasa a serlo.
    out.extend_from_slice(&vorbis_comment_block(tags, streaminfo_was_last));
    out.extend_from_slice(&bytes[STREAMINFO_END..]);
    Ok(out)
}

/// Escribe un FLAC de 16 o 24 bit. `samples_i32[canal][frame]` en el rango de `bits`.
pub fn write(
    path: &Path,
    samples_i32: &[Vec<i32>],
    sample_rate: u32,
    bits: u16,
    tags: Option<&FlacTags>,
) -> Result<(), AudioError> {
    if bits != 16 && bits != 24 {
        return Err(unsupported(format!("FLAC de {bits} bit")));
    }
    if sample_rate > 96_000 {
        return Err(unsupported("FLAC > 96 kHz"));
    }
    let channels = samples_i32.len();
    if channels == 0 || channels > 2 {
        return Err(AudioError::TooManyChannels(channels as u16));
    }
    let frames = samples_i32[0].len();
    if samples_i32.iter().any(|c| c.len() != frames) {
        return Err(unsupported("canales de distinta longitud"));
    }
    let limit = 1i32 << (bits - 1);
    if samples_i32
        .iter()
        .flatten()
        .any(|&s| s < -limit || s >= limit)
    {
        return Err(unsupported("muestra fuera del rango de bits"));
    }
    let mut interleaved = Vec::with_capacity(frames * channels);
    for i in 0..frames {
        for c in samples_i32 {
            interleaved.push(c[i]);
        }
    }
    let config = flacenc::config::Encoder::default()
        .into_verified()
        .map_err(|e| unsupported(format!("configuración FLAC: {e:?}")))?;
    let source = flacenc::source::MemSource::from_samples(
        &interleaved,
        channels,
        bits as usize,
        sample_rate as usize,
    );
    let stream = flacenc::encode_with_fixed_block_size(&config, source, config.block_size)
        .map_err(|e| unsupported(format!("codificación FLAC: {e:?}")))?;
    let mut sink = ByteSink::new();
    stream
        .write(&mut sink)
        .map_err(|e| unsupported(format!("escritura FLAC: {e:?}")))?;
    let mut bytes = sink.as_slice().to_vec();
    if let Some(t) = tags {
        bytes = insert_tags(bytes, t)?;
    }
    std::fs::write(path, bytes)?;
    Ok(())
}
