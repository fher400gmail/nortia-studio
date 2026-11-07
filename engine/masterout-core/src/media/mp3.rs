//! Codificación MP3 320 kbps CBR con `libmp3lame` dentro del sidecar ffmpeg (proceso aparte,
//! nunca enlazado) y etiquetas ID3v2.3 completas con el crate `id3`.

use super::video::run_ffmpeg;
use super::SidecarPaths;
use crate::audio::AudioError;
use id3::frame::{Comment, Content, Picture, PictureType};
use id3::{Frame, Tag, TagLike, Version};
use std::path::Path;

/// Etiquetas ID3v2.3 de un MP3.
#[derive(Debug, Clone, Default)]
pub struct Id3Tags {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub year: Option<u16>,
    pub comment: Option<String>,
    pub isrc: Option<String>,
    pub composer: Option<String>,
    pub label: Option<String>,
    pub copyright: Option<String>,
}

fn unsupported(msg: impl Into<String>) -> AudioError {
    AudioError::UnsupportedFormat(msg.into())
}

/// `err.id3` con el detalle técnico aparte (la interfaz traduce la clave).
fn id3_err(e: id3::Error) -> AudioError {
    unsupported(format!("err.id3: {e}"))
}

fn mime_for(path: &Path) -> &'static str {
    match path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .as_deref()
    {
        Some("png") => "image/png",
        _ => "image/jpeg",
    }
}

/// Lado largo máximo de la carátula incrustada (APIC). La portada conformada es de 3000²
/// (≈2 MB por MP3); para una carátula de reproductor sobran 1400 px.
pub const APIC_MAX_SIDE: u32 = 1400;

/// Tipo MIME y bytes de la carátula a incrustar: la imagen tal cual si cabe en
/// [`APIC_MAX_SIDE`]; si no, reducida a ese lado largo y recodificada como JPEG (calidad 90).
pub fn embedded_cover(path: &Path) -> Result<(String, Vec<u8>), AudioError> {
    let bytes = std::fs::read(path)?;
    let img = ::image::load_from_memory(&bytes)
        .map_err(|e| unsupported(format!("err.cover_unreadable: {e}")))?;
    if img.width().max(img.height()) <= APIC_MAX_SIDE {
        return Ok((mime_for(path).into(), bytes));
    }
    let small = img
        .resize(
            APIC_MAX_SIDE,
            APIC_MAX_SIDE,
            ::image::imageops::FilterType::Lanczos3,
        )
        .to_rgb8();
    let mut out = Vec::new();
    ::image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 90)
        .encode_image(&small)
        .map_err(|e| unsupported(format!("err.cover_unreadable: {e}")))?;
    Ok(("image/jpeg".into(), out))
}

fn text_frame(id: &str, text: &str) -> Frame {
    Frame::with_content(id, Content::Text(text.to_string()))
}

/// Reescribe la etiqueta del MP3 como ID3v2.3 con todos los campos.
fn write_id3(out: &Path, tags: &Id3Tags, cover: Option<&Path>) -> Result<(), AudioError> {
    let mut tag = Tag::new();
    tag.set_title(&tags.title);
    tag.set_artist(&tags.artist);
    if let Some(a) = tags.album.as_deref().filter(|s| !s.is_empty()) {
        tag.set_album(a);
    }
    if let Some(y) = tags.year {
        tag.set_year(i32::from(y));
    }
    if let Some(c) = tags.comment.as_deref().filter(|s| !s.is_empty()) {
        tag.add_frame(Comment {
            lang: "eng".into(),
            description: String::new(),
            text: c.to_string(),
        });
    }
    for (id, v) in [
        ("TSRC", &tags.isrc),
        ("TCOM", &tags.composer),
        ("TPUB", &tags.label),
        ("TCOP", &tags.copyright),
    ] {
        if let Some(v) = v.as_deref().filter(|s| !s.is_empty()) {
            tag.add_frame(text_frame(id, v));
        }
    }
    if let Some(path) = cover {
        let (mime_type, data) = embedded_cover(path)?;
        tag.add_frame(Picture {
            mime_type,
            picture_type: PictureType::CoverFront,
            description: String::new(),
            data,
        });
    }
    tag.write_to_path(out, Version::Id3v23).map_err(id3_err)
}

/// Argumentos de ffmpeg para el MP3 320 kbps CBR (libmp3lame, máxima calidad de codificación,
/// ID3v2.3 con cabecera Xing). `-f mp3` porque la salida puede llevar la extensión `.part`.
pub fn build_mp3_args(wav_16_44: &Path, out: &Path) -> Vec<String> {
    let mut a: Vec<String> = ["-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i"]
        .map(String::from)
        .to_vec();
    a.push(wav_16_44.to_string_lossy().into_owned());
    a.extend(
        [
            "-vn",
            "-c:a",
            "libmp3lame",
            "-b:a",
            "320k",
            "-compression_level",
            "0",
            "-id3v2_version",
            "3",
            "-write_xing",
            "1",
            "-f",
            "mp3",
        ]
        .map(String::from),
    );
    a.push(out.to_string_lossy().into_owned());
    a
}

/// Codifica un WAV 16 bit / 44,1 kHz a MP3 320 kbps CBR con el sidecar ffmpeg y reescribe
/// la etiqueta como ID3v2.3 completa (con la portada conformada como APIC, si la hay).
/// `cancel` se consulta mientras ffmpeg trabaja.
pub fn encode(
    sidecar: &SidecarPaths,
    wav_16_44_path: &Path,
    out: &Path,
    tags: &Id3Tags,
    cover: Option<&Path>,
    cancel: &dyn Fn() -> bool,
) -> Result<(), AudioError> {
    run_ffmpeg(sidecar, &build_mp3_args(wav_16_44_path, out), cancel)?;
    write_id3(out, tags, cover)
}
