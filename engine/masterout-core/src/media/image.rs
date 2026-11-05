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

/// Lado mayor mínimo (tras orientar) por debajo del cual se rechaza la portada.
pub const MIN_SIDE: u32 = 1400;
/// Límites de decodificación (lado mayor y área) para no agotar memoria.
pub const MAX_SIDE: u32 = 16384;
pub const MAX_PIXELS: u64 = 120_000_000;

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

/// ¿Se puede convertir este perfil ICC a sRGB?
fn profile_usable(icc: &[u8]) -> bool {
    let Ok(src) = ColorProfile::new_from_slice(icc) else {
        return false;
    };
    src.create_transform_8bit(
        Layout::Rgb,
        &ColorProfile::new_srgb(),
        Layout::Rgb,
        TransformOptions::default(),
    )
    .is_ok()
}

fn check_dims(hw: u32, hh: u32) -> Result<(), AudioError> {
    if hw.max(hh) > MAX_SIDE || u64::from(hw) * u64::from(hh) > MAX_PIXELS {
        return Err(err("err.cover_too_large", format!("{hw}x{hh}")));
    }
    if hw.max(hh) < MIN_SIDE {
        return Err(err("err.cover_too_small", format!("{hw}x{hh}")));
    }
    Ok(())
}

/// Mira solo la cabecera de la portada (sin decodificarla) y devuelve los avisos que
/// `conform_cover` dará (`warn.cover_*`), o el mismo error que daría (`err.cover_too_small`,
/// `err.cover_too_large`, `err.cover_unreadable`). Sirve para el plan, antes de exportar.
pub fn inspect_cover(input: &Path, opts: &CoverOptions) -> Result<Vec<String>, AudioError> {
    let reader = ImageReader::open(input)?
        .with_guessed_format()
        .map_err(AudioError::Io)?;
    let mut decoder = reader
        .into_decoder()
        .map_err(|e| err("err.cover_unreadable", e))?;
    let (hw, hh) = decoder.dimensions();
    check_dims(hw, hh)?;
    let mut warnings = Vec::new();
    if hw != hh {
        warnings.push("warn.cover_cropped".to_string());
    }
    // El recorte es al lado menor, con o sin giro EXIF.
    if hw.min(hh) < opts.size {
        warnings.push("warn.cover_upscaled".to_string());
    }
    if let Ok(Some(icc)) = decoder.icc_profile() {
        if !profile_usable(&icc) {
            warnings.push("warn.cover_profile_assumed_srgb".to_string());
        }
    }
    Ok(warnings)
}

pub fn conform_cover(
    input: &Path,
    out_jpg: &Path,
    out_png: &Path,
    opts: &CoverOptions,
    _lang: Lang, // los avisos son claves; el texto lo resuelve quien llama
) -> Result<CoverOutput, AudioError> {
    let reader = ImageReader::open(input)?
        .with_guessed_format()
        .map_err(AudioError::Io)?;
    let mut decoder = reader
        .into_decoder()
        .map_err(|e| err("err.cover_unreadable", e))?;

    // Guardas antes de decodificar (la cabecera basta).
    let (hw, hh) = decoder.dimensions();
    check_dims(hw, hh)?;

    let icc = decoder.icc_profile().ok().flatten();
    let orientation = decoder
        .orientation()
        .unwrap_or(image::metadata::Orientation::NoTransforms);
    let mut dynimg =
        DynamicImage::from_decoder(decoder).map_err(|e| err("err.cover_unreadable", e))?;
    dynimg.apply_orientation(orientation);

    let (w, h) = (dynimg.width(), dynimg.height());
    let mut warnings = Vec::new();

    // Recorte centrado al cuadrado (antes de componer/convertir, para copiar menos).
    let side = w.min(h);
    if w != h {
        warnings.push("warn.cover_cropped".to_string());
        dynimg = dynimg.crop_imm((w - side) / 2, (h - side) / 2, side, side);
    }

    // Alfa compuesto sobre blanco; sin alfa, conversión directa a RGB8.
    let mut rgb = if dynimg.color().has_alpha() {
        let rgba = dynimg.to_rgba8();
        drop(dynimg);
        let mut out = RgbImage::new(side, side);
        for (dst, src) in out.pixels_mut().zip(rgba.pixels()) {
            let a = u32::from(src[3]);
            for c in 0..3 {
                dst[c] = ((u32::from(src[c]) * a + 255 * (255 - a) + 127) / 255) as u8;
            }
        }
        out
    } else {
        dynimg.into_rgb8()
    };

    if side < opts.size {
        warnings.push("warn.cover_upscaled".to_string());
    }
    if side != opts.size {
        rgb = imageops::resize(&rgb, opts.size, opts.size, FilterType::Lanczos3);
    }

    // Color: con ICC legible se convierte siempre a sRGB (sRGB→sRGB es ~identidad);
    // si no se puede usar, se asume sRGB y se avisa.
    if let Some(icc) = icc {
        if !to_srgb(&mut rgb, &icc) {
            warnings.push("warn.cover_profile_assumed_srgb".to_string());
        }
    }

    // JPG 4:4:4 sin EXIF ni ICC; PNG 8 bits RGB sin alfa. Si falla algo no queda nada a medias.
    let r = write_both(&rgb, out_jpg, out_png, opts);
    if r.is_err() {
        let _ = crate::fsutil::remove_temp(out_jpg);
        let _ = crate::fsutil::remove_temp(out_png);
    }
    r?;

    Ok(CoverOutput {
        jpg: out_jpg.to_path_buf(),
        png: out_png.to_path_buf(),
        source_px: (w, h),
        warnings,
    })
}

fn write_both(
    rgb: &RgbImage,
    out_jpg: &Path,
    out_png: &Path,
    opts: &CoverOptions,
) -> Result<(), AudioError> {
    let tmp = |p: &Path| {
        let mut n = p.as_os_str().to_owned();
        n.push(".part");
        PathBuf::from(n)
    };
    let (tj, tp) = (tmp(out_jpg), tmp(out_png));
    let res = (|| -> Result<(), AudioError> {
        let mut f = BufWriter::new(File::create(&tj)?);
        JpegEncoder::new_with_quality(&mut f, opts.jpeg_quality.clamp(1, 100))
            .write_image(
                rgb.as_raw(),
                opts.size,
                opts.size,
                image::ExtendedColorType::Rgb8,
            )
            .map_err(|e| err("err.cover_write", e))?;
        f.flush()?;
        let mut f = BufWriter::new(File::create(&tp)?);
        PngEncoder::new(&mut f)
            .write_image(
                rgb.as_raw(),
                opts.size,
                opts.size,
                image::ExtendedColorType::Rgb8,
            )
            .map_err(|e| err("err.cover_write", e))?;
        f.flush()?;
        std::fs::rename(&tj, out_jpg)?;
        std::fs::rename(&tp, out_png)?;
        Ok(())
    })();
    if res.is_err() {
        let _ = crate::fsutil::remove_temp(&tj);
        let _ = crate::fsutil::remove_temp(&tp);
    }
    res
}
