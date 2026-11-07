//! MP3 por ffmpeg: argumentos golden (puros) y ejecución contra el ffmpeg del entorno.

#[cfg(unix)]
use masterout_core::i18n::Lang;
use masterout_core::media::mp3::{build_mp3_args, encode, Id3Tags};
use masterout_core::media::probe::probe;
use masterout_core::media::{discover, SidecarPaths};
use std::path::Path;

#[test]
fn mp3_args_golden() {
    let got = build_mp3_args(Path::new("in16.wav"), Path::new("o.mp3.part"));
    let want: Vec<&str> = vec![
        "-hide_banner",
        "-loglevel",
        "error",
        "-nostdin",
        "-y",
        "-i",
        "in16.wav",
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
        "o.mp3.part",
    ];
    assert_eq!(got, want);
}

fn sidecar() -> Option<SidecarPaths> {
    let sc = discover(Path::new("/nonexistent-exe-dir"))?;
    let out = std::process::Command::new(&sc.ffmpeg)
        .args(["-hide_banner", "-encoders"])
        .output()
        .ok()?;
    String::from_utf8_lossy(&out.stdout)
        .contains("libmp3lame")
        .then_some(sc)
}

fn write_wav(path: &Path, secs: usize) {
    let sr = 44_100u32;
    let n = sr as usize * secs;
    let mut data = Vec::with_capacity(n * 4);
    for i in 0..n {
        let v = ((i as f64 * 0.05).sin() * 12_000.0) as i16;
        data.extend_from_slice(&v.to_le_bytes());
        data.extend_from_slice(&v.to_le_bytes());
    }
    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&sr.to_le_bytes());
    out.extend_from_slice(&(sr * 4).to_le_bytes());
    out.extend_from_slice(&4u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(&data);
    std::fs::write(path, out).unwrap();
}

#[test]
fn encode_writes_320k_cbr_with_id3v23_and_apic() {
    let Some(sc) = sidecar() else {
        eprintln!("skip: ffmpeg con libmp3lame no encontrado");
        return;
    };
    let d = tempfile::tempdir().unwrap();
    let (wav, out, jpg) = (
        d.path().join("a.wav"),
        d.path().join("a.mp3.part"),
        d.path().join("c.jpg"),
    );
    write_wav(&wav, 2);
    image::RgbImage::from_pixel(600, 600, image::Rgb([10, 200, 90]))
        .save(&jpg)
        .unwrap();
    let tags = Id3Tags {
        title: "Luz".into(),
        artist: "Vera".into(),
        year: Some(2026),
        ..Default::default()
    };
    encode(&sc, &wav, &out, &tags, Some(&jpg), &|| false).unwrap();
    let info = probe(&sc, &out).unwrap();
    assert_eq!(info.audio_codec.as_deref(), Some("mp3"));
    assert_eq!(info.audio_bitrate, Some(320_000));
    assert_eq!(info.audio_sample_rate, Some(44_100));
    use id3::TagLike;
    let tag = id3::Tag::read_from_path(&out).unwrap();
    assert_eq!(tag.version(), id3::Version::Id3v23);
    assert_eq!(tag.title(), Some("Luz"));
    assert_eq!(tag.pictures().count(), 1);
    // la duración sigue siendo la del WAV (la cabecera Xing y el ID3 no alteran el audio)
    assert!((info.duration - 2.0).abs() < 0.15, "{info:?}");
}

#[test]
fn encode_without_cover_has_no_picture() {
    let Some(sc) = sidecar() else { return };
    let d = tempfile::tempdir().unwrap();
    let (wav, out) = (d.path().join("a.wav"), d.path().join("a.mp3"));
    write_wav(&wav, 1);
    encode(&sc, &wav, &out, &Id3Tags::default(), None, &|| false).unwrap();
    let tag = id3::Tag::read_from_path(&out).unwrap();
    assert_eq!(tag.pictures().count(), 0);
}

/// Un fallo al escribir las etiquetas ID3 es una clave estable (`err.id3`) con el detalle
/// aparte, no `err.generic` con un texto ya traducido dentro.
#[cfg(unix)]
#[test]
fn id3_write_failure_is_a_stable_key_with_the_detail_apart() {
    use std::os::unix::fs::PermissionsExt;
    let d = tempfile::tempdir().unwrap();
    let ffmpeg = d.path().join("ffmpeg");
    std::fs::write(&ffmpeg, "#!/bin/sh\nexit 0\n").unwrap();
    std::fs::set_permissions(&ffmpeg, std::fs::Permissions::from_mode(0o755)).unwrap();
    let sc = SidecarPaths {
        ffmpeg: ffmpeg.clone(),
        ffprobe: ffmpeg,
    };
    let wav = d.path().join("in.wav");
    write_wav(&wav, 1);
    // la salida es una carpeta: ffmpeg «termina bien» y la etiqueta no se puede escribir
    let out = d.path().join("salida");
    std::fs::create_dir(&out).unwrap();
    let e = encode(&sc, &wav, &out, &Id3Tags::default(), None, &|| false).unwrap_err();
    let (key, detail) = e.key_and_detail();
    assert_eq!(key, "err.id3");
    assert!(detail.is_some_and(|x| !x.is_empty()));
    for l in [Lang::Es, Lang::En] {
        assert!(!masterout_core::i18n::t(l, "err.id3").starts_with('⟦'));
    }
}

/// La carátula embebida no pasa de 1400 px de lado largo (la de 3000² pesa ≈2 MB por MP3);
/// las pequeñas se incrustan tal cual.
#[test]
fn embedded_cover_is_capped_at_1400_px() {
    use masterout_core::media::mp3::{embedded_cover, APIC_MAX_SIDE};
    assert_eq!(APIC_MAX_SIDE, 1400);
    let d = tempfile::tempdir().unwrap();
    let big = d.path().join("big.png");
    image::RgbImage::from_fn(3000, 3000, |x, y| {
        image::Rgb([(x / 12) as u8, (y / 12) as u8, 90])
    })
    .save(&big)
    .unwrap();
    let (mime, bytes) = embedded_cover(&big).unwrap();
    assert_eq!(mime, "image/jpeg");
    let img = image::load_from_memory(&bytes).unwrap();
    assert_eq!((img.width(), img.height()), (1400, 1400));
    assert!(bytes.len() < 600_000, "{} bytes", bytes.len());

    let small = d.path().join("small.jpg");
    image::RgbImage::from_pixel(600, 600, image::Rgb([10, 200, 90]))
        .save(&small)
        .unwrap();
    let (mime, bytes) = embedded_cover(&small).unwrap();
    assert_eq!(mime, "image/jpeg");
    assert_eq!(bytes, std::fs::read(&small).unwrap());

    let png = d.path().join("small.png");
    image::RgbImage::from_pixel(1400, 900, image::Rgb([1, 2, 3]))
        .save(&png)
        .unwrap();
    let (mime, bytes) = embedded_cover(&png).unwrap();
    assert_eq!(mime, "image/png");
    assert_eq!(bytes, std::fs::read(&png).unwrap());
}
