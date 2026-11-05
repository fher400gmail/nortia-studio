//! Miniaturas de 160 px para las fichas con material propio: imágenes con el crate `image`;
//! vídeo (y las imágenes que `image` no abre) con el primer fotograma vía ffmpeg.

use crate::errors::{localize, BridgeError};
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, ImageDecoder, ImageEncoder, ImageReader};
use masterout_core::audio::AudioError;
use masterout_core::export::default_sidecar;
use masterout_core::i18n::Lang;
use masterout_core::media::image::{MAX_PIXELS, MAX_SIDE};
use masterout_core::media::probe::probe;
use masterout_core::media::video::run_ffmpeg_with_deadline;
use masterout_core::media::SidecarPaths;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

/// Lado mayor de la miniatura.
pub const THUMB_PX: u32 = 160;

/// Extensiones de imagen y vídeo que acepta una ficha (el sidecar sabe decodificarlas).
pub use masterout_core::media::{IMAGE_EXTS, VIDEO_EXTS};

fn ext(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
}

/// ¿Imagen o vídeo de los admitidos?
pub fn is_media_path(path: &str) -> bool {
    ext(Path::new(path))
        .is_some_and(|e| IMAGE_EXTS.contains(&e.as_str()) || VIDEO_EXTS.contains(&e.as_str()))
}

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Base64 estándar con relleno.
pub fn base64(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let n = (u32::from(chunk[0]) << 16)
            | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8)
            | u32::from(*chunk.get(2).unwrap_or(&0));
        out.push(B64[(n >> 18) as usize & 63] as char);
        out.push(B64[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 {
            B64[(n >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            B64[n as usize & 63] as char
        } else {
            '='
        });
    }
    out
}

fn data_url(mime: &str, bytes: &[u8]) -> String {
    format!("data:{mime};base64,{}", base64(bytes))
}

fn unreadable(detail: impl std::fmt::Display) -> AudioError {
    AudioError::UnsupportedFormat(format!("err.media_unreadable: {detail}"))
}

/// Miniatura JPEG de una imagen JPG/PNG, orientada según su EXIF.
fn image_thumbnail(path: &Path) -> Result<String, AudioError> {
    let reader = ImageReader::open(path)?
        .with_guessed_format()
        .map_err(AudioError::Io)?;
    let mut decoder = reader.into_decoder().map_err(unreadable)?;
    let (w, h) = decoder.dimensions();
    if w.max(h) > MAX_SIDE || u64::from(w) * u64::from(h) > MAX_PIXELS {
        return Err(AudioError::UnsupportedFormat(format!(
            "err.cover_too_large: {w}x{h}"
        )));
    }
    let orientation = decoder
        .orientation()
        .unwrap_or(image::metadata::Orientation::NoTransforms);
    let mut img = DynamicImage::from_decoder(decoder).map_err(unreadable)?;
    img.apply_orientation(orientation);
    let small = img.thumbnail(THUMB_PX, THUMB_PX).to_rgb8();
    let mut buf = Vec::new();
    JpegEncoder::new_with_quality(&mut buf, 82)
        .write_image(
            small.as_raw(),
            small.width(),
            small.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(unreadable)?;
    Ok(data_url("image/jpeg", &buf))
}

/// Primer fotograma (o la imagen) con ffmpeg, a un PNG de 160 px como máximo.
fn ffmpeg_thumbnail(path: &Path, sidecar: Option<SidecarPaths>) -> Result<String, AudioError> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let sidecar =
        sidecar.ok_or_else(|| AudioError::UnsupportedFormat("err.sidecar_missing".into()))?;
    // Una imagen que no abre el crate `image` (webp) la decodifica ffmpeg sin más tope que el
    // plazo: se leen sus dimensiones con ffprobe antes y se aplica el tope de la portada.
    if ext(path).is_some_and(|e| IMAGE_EXTS.contains(&e.as_str())) {
        let info = probe(&sidecar, path)?;
        if info.width.max(info.height) > MAX_SIDE
            || u64::from(info.width) * u64::from(info.height) > MAX_PIXELS
        {
            return Err(AudioError::UnsupportedFormat(format!(
                "err.cover_too_large: {}x{}",
                info.width, info.height
            )));
        }
    }
    let out: PathBuf = std::env::temp_dir().join(format!(
        "masterout-thumb-{}-{}.png",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    let args: Vec<String> = ["-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i"]
        .map(String::from)
        .into_iter()
        .chain([
            path.to_string_lossy().into_owned(),
            "-frames:v".into(),
            "1".into(),
            "-vf".into(),
            format!(
                "scale={THUMB_PX}:{THUMB_PX}:force_original_aspect_ratio=decrease:flags=bicubic"
            ),
            "-update".into(),
            "1".into(),
            out.to_string_lossy().into_owned(),
        ])
        .collect();
    let r = run_ffmpeg_with_deadline(&sidecar, &args, &|| false, Some(Duration::from_secs(10)));
    let bytes = r.and_then(|()| std::fs::read(&out).map_err(AudioError::Io));
    let _ = std::fs::remove_file(&out);
    let bytes = bytes.map_err(|e| match e.key_and_detail().0.as_str() {
        "err.sidecar_missing" | "err.encoder_unavailable" => e,
        _ => unreadable(e),
    })?;
    Ok(data_url("image/png", &bytes))
}

/// Miniatura (data URL de 160 px como máximo) de una imagen o un vídeo.
pub fn media_thumbnail(path: &str, lang: Lang) -> Result<String, BridgeError> {
    media_thumbnail_with(path, lang, default_sidecar())
}

/// Como [`media_thumbnail`], con el sidecar de ffmpeg indicado (`None`: sin vídeo ni webp).
pub fn media_thumbnail_with(
    path: &str,
    lang: Lang,
    sidecar: Option<SidecarPaths>,
) -> Result<String, BridgeError> {
    if !is_media_path(path) {
        return Err(BridgeError::from_key(
            "err.media_unreadable",
            lang,
            Some(format!("tipo de archivo no permitido: {path}")),
        ));
    }
    let p = Path::new(path);
    let direct = ext(p).is_some_and(|e| matches!(e.as_str(), "jpg" | "jpeg" | "png"));
    let r = if direct {
        image_thumbnail(p)
    } else {
        ffmpeg_thumbnail(p, sidecar)
    };
    r.map_err(|e| localize(&e, lang))
}
