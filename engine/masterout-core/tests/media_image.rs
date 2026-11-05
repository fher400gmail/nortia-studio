use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::{
    DynamicImage, ImageDecoder, ImageEncoder, ImageReader, Rgb, RgbImage, Rgba, RgbaImage,
};
use img_parts::jpeg::Jpeg;
use img_parts::{Bytes, ImageEXIF, ImageICC};
use masterout_core::audio::AudioError;
use masterout_core::i18n::Lang;
use masterout_core::media::image::{conform_cover, CoverOptions, CoverOutput};
use moxcms::ColorProfile;
use std::path::Path;

fn jpeg_bytes(img: &RgbImage) -> Vec<u8> {
    let mut v = Vec::new();
    JpegEncoder::new_with_quality(&mut v, 95)
        .write_image(
            img.as_raw(),
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgb8,
        )
        .unwrap();
    v
}

/// Mitad izquierda roja, mitad derecha azul.
fn two_tone(w: u32, h: u32) -> RgbImage {
    RgbImage::from_fn(w, h, |x, _| {
        if x < w / 2 {
            Rgb([255, 0, 0])
        } else {
            Rgb([0, 0, 255])
        }
    })
}

fn run(input: &Path, dir: &Path) -> Result<CoverOutput, AudioError> {
    conform_cover(
        input,
        &dir.join("c.jpg"),
        &dir.join("c.png"),
        &CoverOptions {
            size: 3000,
            jpeg_quality: 92,
        },
        Lang::Es,
    )
}

fn near(p: &Rgb<u8>, want: [u8; 3], tol: i32) -> bool {
    (0..3).all(|c| (i32::from(p[c]) - i32::from(want[c])).abs() <= tol)
}

#[test]
fn conform_image_crops_center_and_warns_upscale() {
    let d = tempfile::tempdir().unwrap();
    // 1500x1000: centro verde (500x1000 central), laterales rojos.
    let img = RgbImage::from_fn(1500, 1000, |x, _| {
        if (500..1000).contains(&x) {
            Rgb([0, 255, 0])
        } else {
            Rgb([255, 0, 0])
        }
    });
    let src = d.path().join("in.jpg");
    std::fs::write(&src, jpeg_bytes(&img)).unwrap();
    let out = run(&src, d.path()).unwrap();
    assert_eq!(out.source_px, (1500, 1000));
    assert!(out.warnings.contains(&"warn.cover_upscaled".to_string()));
    assert!(out.warnings.contains(&"warn.cover_cropped".to_string()));
    let got = image::open(&out.png).unwrap().to_rgb8();
    assert_eq!(got.dimensions(), (3000, 3000));
    // Recorte centrado 1000x1000: x en [250,1250) de la fuente; verde = [500,1000) -> 25%..75%.
    assert!(near(got.get_pixel(1500, 1500), [0, 255, 0], 6));
    assert!(near(got.get_pixel(100, 1500), [255, 0, 0], 6));
    assert!(near(got.get_pixel(2900, 1500), [255, 0, 0], 6));
    assert!(near(got.get_pixel(900, 1500), [255, 0, 0], 6));
    assert!(near(got.get_pixel(2100, 1500), [255, 0, 0], 6));
}

#[test]
fn respects_exif_orientation() {
    let d = tempfile::tempdir().unwrap();
    let mut jpeg = Jpeg::from_bytes(Bytes::from(jpeg_bytes(&two_tone(2000, 1500)))).unwrap();
    // TIFF big-endian, un IFD con Orientation(0x0112)=6.
    let tiff: Vec<u8> = vec![
        b'M', b'M', 0, 42, 0, 0, 0, 8, 0, 1, 0x01, 0x12, 0, 3, 0, 0, 0, 1, 0, 6, 0, 0, 0, 0, 0, 0,
    ];
    jpeg.set_exif(Some(Bytes::from(tiff)));
    let src = d.path().join("rot.jpg");
    std::fs::write(&src, jpeg.encoder().bytes()).unwrap();
    let out = run(&src, d.path()).unwrap();
    // Orientation 6 = girar 90 horario: la izquierda (roja) pasa arriba.
    assert_eq!(out.source_px, (1500, 2000));
    let got = image::open(&out.jpg).unwrap().to_rgb8();
    assert!(near(got.get_pixel(1500, 100), [255, 0, 0], 12));
    assert!(near(got.get_pixel(1500, 2900), [0, 0, 255], 12));
}

#[test]
fn rejects_too_small() {
    let d = tempfile::tempdir().unwrap();
    let src = d.path().join("s.jpg");
    std::fs::write(&src, jpeg_bytes(&RgbImage::new(800, 800))).unwrap();
    let e = run(&src, d.path()).unwrap_err();
    assert_eq!(e.key_and_detail().0, "err.cover_too_small");
    assert!(!d.path().join("c.jpg").exists());
}

#[test]
fn strips_alpha_onto_white() {
    let d = tempfile::tempdir().unwrap();
    // Cuadrada 3000: mitad izquierda transparente, derecha opaca negra.
    let img = RgbaImage::from_fn(3000, 3000, |x, _| {
        if x < 1500 {
            Rgba([255, 0, 0, 0])
        } else {
            Rgba([0, 0, 0, 255])
        }
    });
    let src = d.path().join("a.png");
    let mut v = Vec::new();
    PngEncoder::new(&mut v)
        .write_image(img.as_raw(), 3000, 3000, image::ExtendedColorType::Rgba8)
        .unwrap();
    std::fs::write(&src, v).unwrap();
    let out = run(&src, d.path()).unwrap();
    assert!(out.warnings.is_empty(), "{:?}", out.warnings);
    let png = image::open(&out.png).unwrap();
    assert!(
        matches!(png, DynamicImage::ImageRgb8(_)),
        "PNG sin alfa, 8 bits"
    );
    let got = png.to_rgb8();
    assert_eq!(got.get_pixel(200, 1500).0, [255, 255, 255]);
    assert_eq!(got.get_pixel(2800, 1500).0, [0, 0, 0]);
}

fn with_icc(img: &RgbImage, icc: Vec<u8>) -> Vec<u8> {
    let mut jpeg = Jpeg::from_bytes(Bytes::from(jpeg_bytes(img))).unwrap();
    jpeg.set_icc_profile(Some(Bytes::from(icc)));
    jpeg.encoder().bytes().to_vec()
}

#[test]
fn non_srgb_profile_is_converted_or_flagged() {
    let d = tempfile::tempdir().unwrap();
    let flat = RgbImage::from_pixel(1500, 1500, Rgb([40, 200, 60]));
    let adobe = ColorProfile::new_adobe_rgb().encode().unwrap();
    let src = d.path().join("adobe.jpg");
    std::fs::write(&src, with_icc(&flat, adobe)).unwrap();
    let out = run(&src, d.path()).unwrap();
    let got = image::open(&out.jpg).unwrap().to_rgb8();
    let p = got.get_pixel(1500, 1500);
    if out
        .warnings
        .contains(&"warn.cover_profile_assumed_srgb".to_string())
    {
        assert!(near(p, [40, 200, 60], 6));
    } else {
        // Verde Adobe RGB -> sRGB: más saturado (R baja, G sube).
        assert!(p[0] < 30 && p[1] > 205, "no convertido: {p:?}");
    }
    // Nunca queda un perfil incrustado distinto de sRGB.
    for f in [&out.jpg, &out.png] {
        let mut dec = ImageReader::open(f)
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .into_decoder()
            .unwrap();
        assert!(dec.icc_profile().unwrap().is_none());
    }
    // Un sRGB declarado no se toca.
    let srgb = ColorProfile::new_srgb().encode().unwrap();
    let src2 = d.path().join("srgb.jpg");
    std::fs::write(&src2, with_icc(&flat, srgb)).unwrap();
    let out2 = conform_cover(
        &src2,
        &d.path().join("s.jpg"),
        &d.path().join("s.png"),
        &CoverOptions::default(),
        Lang::En,
    )
    .unwrap();
    assert!(!out2
        .warnings
        .contains(&"warn.cover_profile_assumed_srgb".to_string()));
    assert!(near(
        image::open(&out2.jpg).unwrap().to_rgb8().get_pixel(10, 10),
        [40, 200, 60],
        4
    ));
    // Perfil ilegible: se avisa.
    let src3 = d.path().join("bad.jpg");
    std::fs::write(&src3, with_icc(&flat, vec![1, 2, 3, 4, 5, 6, 7, 8])).unwrap();
    let out3 = conform_cover(
        &src3,
        &d.path().join("b.jpg"),
        &d.path().join("b.png"),
        &CoverOptions::default(),
        Lang::En,
    )
    .unwrap();
    assert!(out3
        .warnings
        .contains(&"warn.cover_profile_assumed_srgb".to_string()));
}

#[test]
fn output_dimensions_and_formats_via_image_crate() {
    let d = tempfile::tempdir().unwrap();
    let src = d.path().join("sq.jpg");
    std::fs::write(
        &src,
        jpeg_bytes(&RgbImage::from_pixel(3000, 3000, Rgb([10, 20, 30]))),
    )
    .unwrap();
    let out = run(&src, d.path()).unwrap();
    assert!(out.warnings.is_empty());
    let jb = std::fs::read(&out.jpg).unwrap();
    let pb = std::fs::read(&out.png).unwrap();
    assert_eq!(image::guess_format(&jb).unwrap(), image::ImageFormat::Jpeg);
    assert_eq!(image::guess_format(&pb).unwrap(), image::ImageFormat::Png);
    assert_eq!(
        image::load_from_memory(&jb).unwrap().to_rgb8().dimensions(),
        (3000, 3000)
    );
    assert!(matches!(
        image::load_from_memory(&pb).unwrap(),
        DynamicImage::ImageRgb8(_)
    ));
    // Sin EXIF/ICC y 4:4:4 (3 componentes con muestreo 1x1 en SOF0).
    let j = Jpeg::from_bytes(Bytes::from(jb.clone())).unwrap();
    assert!(j.exif().is_none() && j.icc_profile().is_none());
    let sof = jb.windows(2).position(|w| w == [0xFF, 0xC0]).unwrap();
    assert_eq!(&jb[sof + 9..sof + 9 + 1], &[3]);
    for i in 0..3 {
        assert_eq!(jb[sof + 11 + i * 3 + 1], 0x11);
    }
}
