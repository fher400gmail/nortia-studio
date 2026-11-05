//! Portada: orienta por EXIF, recorta al cuadrado centrado, escala a `size`
//! (Lanczos3), convierte a sRGB y escribe JPG 4:4:4 + PNG 8 bits sin alfa,
//! sin metadatos. La imagen nunca se deforma.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::imageops::{self, FilterType};
use image::{DynamicImage, ImageDecoder, ImageEncoder, ImageReader, RgbImage};
use moxcms::{ColorProfile, Layout, TransformOptions};

use crate::audio::AudioError;
use crate::i18n::Lang;

/// Lado mínimo (tras orientar) por debajo del cual se rechaza la portada.
pub const MIN_SIDE: u32 = 1400;

#[derive(Debug, Clone)]
pub struct CoverOptions {
    pub size: u32,
    pub jpeg_quality: u8,
}

impl Default for CoverOptions {
    fn default() -> Self {
        CoverOptions {
            size: 3000,
            jpeg_quality: 92,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoverOutput {
    pub jpg: PathBuf,
    pub png: PathBuf,
    /// Tamaño de la fuente ya orientada.
    pub source_px: (u32, u32),
    /// Claves i18n: `warn.cover_upscaled`, `warn.cover_cropped`,
    /// `warn.cover_profile_assumed_srgb`.
    pub warnings: Vec<String>,
}

fn err(key: &str, detail: impl std::fmt::Display) -> AudioError {
    AudioError::UnsupportedFormat(format!("{key}: {detail}"))
}

/// ¿Es (casi con seguridad) sRGB? Las descripciones de perfil sRGB lo nombran.
fn looks_srgb(icc: &[u8]) -> bool {
    icc.windows(4).take(4096).any(|w| w == b"sRGB")
}

/// Convierte `img` de `icc` a sRGB. `Ok(false)` si el perfil no se pudo usar.
fn to_srgb(img: &mut RgbImage, icc: &[u8]) -> bool {
    let Ok(src) = ColorProfile::new_from_slice(icc) else {
        return false;
    };
    let dst = ColorProfile::new_srgb();
    let Ok(tr) =
        src.create_transform_8bit(Layout::Rgb, &dst, Layout::Rgb, TransformOptions::default())
    else {
        return false;
    };
    let src_px = img.as_raw().clone();
    tr.transform(&src_px, img.as_mut()).is_ok()
}

pub fn conform_cover(
    input: &Path,
    out_jpg: &Path,
    out_png: &Path,
    opts: &CoverOptions,
    _lang: Lang,
) -> Result<CoverOutput, AudioError> {
    let reader = ImageReader::open(input)?
        .with_guessed_format()
        .map_err(AudioError::Io)?;
    let mut decoder = reader
        .into_decoder()
        .map_err(|e| err("err.cover_unreadable", e))?;
    let icc = decoder.icc_profile().ok().flatten();
    let orientation = decoder
        .orientation()
        .unwrap_or(image::metadata::Orientation::NoTransforms);
    let mut dynimg =
        DynamicImage::from_decoder(decoder).map_err(|e| err("err.cover_unreadable", e))?;
    dynimg.apply_orientation(orientation);

    let (w, h) = (dynimg.width(), dynimg.height());
    if w.min(h) < MIN_SIDE {
        return Err(err("err.cover_too_small", format!("{w}x{h}")));
    }
    let mut warnings = Vec::new();

    // Alfa compuesto sobre blanco.
    let rgba = dynimg.to_rgba8();
    let mut rgb = RgbImage::new(w, h);
    for (dst, src) in rgb.pixels_mut().zip(rgba.pixels()) {
        let a = u32::from(src[3]);
        for c in 0..3 {
            dst[c] = ((u32::from(src[c]) * a + 255 * (255 - a) + 127) / 255) as u8;
        }
    }

    // Recorte centrado al cuadrado.
    let side = w.min(h);
    if w != h {
        warnings.push("warn.cover_cropped".to_string());
        rgb = imageops::crop_imm(&rgb, (w - side) / 2, (h - side) / 2, side, side).to_image();
    }
    if side < opts.size {
        warnings.push("warn.cover_upscaled".to_string());
    }
    if side != opts.size {
        rgb = imageops::resize(&rgb, opts.size, opts.size, FilterType::Lanczos3);
    }

    // Color: a sRGB si hay ICC distinto; si no se puede, se asume sRGB y se avisa.
    if let Some(icc) = icc.filter(|p| !looks_srgb(p)) {
        if !to_srgb(&mut rgb, &icc) {
            warnings.push("warn.cover_profile_assumed_srgb".to_string());
        }
    }

    // JPG 4:4:4 sin EXIF ni ICC.
    let mut f = BufWriter::new(File::create(out_jpg)?);
    JpegEncoder::new_with_quality(&mut f, opts.jpeg_quality.clamp(1, 100))
        .write_image(
            rgb.as_raw(),
            opts.size,
            opts.size,
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| err("err.cover_write", e))?;
    f.flush()?;

    // PNG 8 bits RGB sin alfa.
    let mut f = BufWriter::new(File::create(out_png)?);
    PngEncoder::new(&mut f)
        .write_image(
            rgb.as_raw(),
            opts.size,
            opts.size,
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| err("err.cover_write", e))?;
    f.flush()?;

    Ok(CoverOutput {
        jpg: out_jpg.to_path_buf(),
        png: out_png.to_path_buf(),
        source_px: (w, h),
        warnings,
    })
}
