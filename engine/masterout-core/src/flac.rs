//! Escritor FLAC de 16/24 bit con Vorbis comments (codificador `flacenc`, Rust puro).

use crate::audio::AudioError;
use flacenc::bitsink::ByteSink;
use flacenc::component::BitRepr;
use flacenc::error::Verify;
use flacenc::source::Fill;
use std::io::{Seek, SeekFrom, Write};
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
    let config = flacenc::config::Encoder::default()
        .into_verified()
        .map_err(|e| unsupported(format!("configuración FLAC: {e:?}")))?;
    let block_size = config.block_size;
    let enc = |e: flacenc::error::EncodeError| unsupported(format!("codificación FLAC: {e:?}"));
    let mut stream = flacenc::component::Stream::new(sample_rate as usize, channels, bits as usize)
        .map_err(enc_verify)?;
    stream
        .stream_info_mut()
        .set_block_sizes(block_size, block_size)
        .map_err(enc_verify)?;

    // Se codifica y escribe bloque a bloque: ni el audio entrelazado completo ni el flujo
    // FLAC completo están nunca en memoria. La cabecera (STREAMINFO con tamaños, MD5 y total
    // de muestras, más los comentarios) se escribe al final, sobre el hueco reservado.
    let comment_block = tags.map(|t| vorbis_comment_block(t, true));
    let header_len = 4 + 4 + 34 + comment_block.as_ref().map_or(0, Vec::len);
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    file.write_all(&vec![0u8; header_len])?;

    let mut fb_ctx = (
        flacenc::source::FrameBuf::with_size(channels, block_size).map_err(enc_verify)?,
        flacenc::source::Context::new(bits as usize, channels),
    );
    let mut scratch: Vec<i32> = Vec::with_capacity(block_size * channels);
    let mut sink = ByteSink::new();
    let mut head = 0;
    while head < frames {
        let end = (head + block_size).min(frames);
        scratch.clear();
        for i in head..end {
            for c in samples_i32 {
                scratch.push(c[i]);
            }
        }
        fb_ctx
            .fill_interleaved(&scratch)
            .map_err(|e| unsupported(format!("fuente FLAC: {e:?}")))?;
        head = end;
        let number = fb_ctx
            .1
            .current_frame_number()
            .ok_or_else(|| unsupported("numeración de bloques FLAC"))?;
        let frame =
            flacenc::encode_fixed_size_frame(&config, &fb_ctx.0, number, stream.stream_info())
                .map_err(enc)?;
        stream.stream_info_mut().update_frame_info(&frame);
        sink.clear();
        frame
            .write(&mut sink)
            .map_err(|e| unsupported(format!("escritura FLAC: {e:?}")))?;
        file.write_all(sink.as_slice())?;
    }
    stream
        .stream_info_mut()
        .set_md5_digest(&fb_ctx.1.md5_digest());
    stream.stream_info_mut().set_total_samples(frames);

    // Cabecera: "fLaC" + STREAMINFO (sin frames) + comentarios.
    let mut head_sink = ByteSink::new();
    stream
        .write(&mut head_sink)
        .map_err(|e| unsupported(format!("escritura FLAC: {e:?}")))?;
    let mut header = head_sink.as_slice().to_vec();
    if let Some(block) = &comment_block {
        header[4] &= 0x7f;
        header.extend_from_slice(block);
    }
    if header.len() != header_len || &header[..4] != b"fLaC" {
        return Err(unsupported("salida FLAC inesperada"));
    }
    file.flush()?;
    let mut file = file.into_inner().map_err(|e| e.into_error())?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&header)?;
    file.flush()?;
    Ok(())
}

fn enc_verify(e: impl std::fmt::Debug) -> AudioError {
    unsupported(format!("configuración FLAC: {e:?}"))
}
