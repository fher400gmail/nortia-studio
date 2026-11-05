//! Codificación MP3 320 kbps CBR con el ejecutable externo `lame` (no se enlaza ninguna
//! biblioteca GPL) y etiquetas ID3v2.3 completas con el crate `id3`.

use crate::audio::AudioError;
use crate::i18n::{t, tf, Lang};
use id3::frame::{Comment, Content, Picture, PictureType};
use id3::{Frame, Tag, TagLike, Version};
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn exe_name() -> &'static str {
    if cfg!(windows) {
        "lame.exe"
    } else {
        "lame"
    }
}

/// Busca `lame` junto al ejecutable actual y después en `PATH`.
pub fn find_lame() -> Option<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Some(dir) = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
    {
        dirs.push(dir);
    }
    if let Some(path) = std::env::var_os("PATH") {
        dirs.extend(std::env::split_paths(&path));
    }
    dirs.into_iter()
        .map(|d| d.join(exe_name()))
        .find(|p| p.is_file())
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

/// Codifica un WAV 16 bit / 44,1 kHz a MP3 320 kbps CBR (joint stereo, `-q 0`).
///
/// Busca `lame` (PATH o junto al ejecutable); si no existe devuelve
/// `UnsupportedFormat("lame not found")`.
pub fn encode(
    wav_16_44_path: &Path,
    out: &Path,
    tags: &Id3Tags,
    cover: Option<&Path>,
) -> Result<(), AudioError> {
    let lang = Lang::from_system();
    let lame = find_lame().ok_or_else(|| unsupported(t(lang, "export.err.lame_missing")))?;
    encode_with(lang, &lame, wav_16_44_path, out, tags, cover)
}

/// Como [`encode`], con la ruta de `lame` indicada y el idioma de los mensajes de error.
pub fn encode_with(
    lang: Lang,
    lame: &Path,
    wav_16_44_path: &Path,
    out: &Path,
    tags: &Id3Tags,
    cover: Option<&Path>,
) -> Result<(), AudioError> {
    let mut cmd = Command::new(lame);
    cmd.args(["--silent", "-b", "320", "--cbr", "-m", "j", "-q", "0"]);
    cmd.args(["--add-id3v2", "--id3v2-only"]);
    let year = tags.year.map(|y| y.to_string());
    for (flag, val) in [
        ("--tt", Some(tags.title.as_str())),
        ("--ta", Some(tags.artist.as_str())),
        ("--tl", tags.album.as_deref()),
        ("--ty", year.as_deref()),
        ("--tc", tags.comment.as_deref()),
    ] {
        if let Some(v) = val.filter(|s| !s.is_empty()) {
            cmd.arg(flag).arg(v);
        }
    }
    cmd.arg(wav_16_44_path).arg(out);
    let output = cmd.output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(unsupported(tf(
            lang,
            "export.err.lame_failed",
            &[
                ("status", &output.status.to_string()),
                ("detail", err.trim()),
            ],
        )));
    }
    write_id3(lang, out, tags, cover)
}
