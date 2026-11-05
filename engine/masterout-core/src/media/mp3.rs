//! Codificación MP3 320 kbps CBR con `libmp3lame` dentro del sidecar ffmpeg (proceso aparte,
//! nunca enlazado) y etiquetas ID3v2.3 completas con el crate `id3`.

use super::video::run_ffmpeg;
use super::SidecarPaths;
use crate::audio::AudioError;
use crate::i18n::{tf, Lang};
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

fn id3_err(lang: Lang, e: id3::Error) -> AudioError {
    unsupported(tf(lang, "export.err.id3", &[("detail", &e.to_string())]))
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

fn text_frame(id: &str, text: &str) -> Frame {
    Frame::with_content(id, Content::Text(text.to_string()))
}

/// Reescribe la etiqueta del MP3 como ID3v2.3 con todos los campos.
fn write_id3(
    lang: Lang,
    out: &Path,
    tags: &Id3Tags,
    cover: Option<&Path>,
) -> Result<(), AudioError> {
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
        tag.add_frame(Picture {
            mime_type: mime_for(path).into(),
            picture_type: PictureType::CoverFront,
            description: String::new(),
            data: std::fs::read(path)?,
        });
    }
    tag.write_to_path(out, Version::Id3v23)
        .map_err(|e| id3_err(lang, e))
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
    lang: Lang,
    wav_16_44_path: &Path,
    out: &Path,
    tags: &Id3Tags,
    cover: Option<&Path>,
    cancel: &dyn Fn() -> bool,
) -> Result<(), AudioError> {
    run_ffmpeg(sidecar, &build_mp3_args(wav_16_44_path, out), cancel)?;
    write_id3(lang, out, tags, cover)
}
