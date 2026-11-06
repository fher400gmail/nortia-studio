//! Entrega de punta a punta con portada y material propio: portada JPG+PNG, Canvas, vídeo de
//! YouTube, portada animada y MP3 con APIC. Se ejecutan con el ffmpeg del entorno (PATH o
//! `MASTERCHECK_FFMPEG`); sin él se saltan. La portada animada (HEVC/H.264 de 3840²) tarda
//! minutos con un ffmpeg de software: solo con `MASTERCHECK_SLOW=1`.

mod common;

use masterout_core::export::{self, ExportRequest, Progress};
use masterout_core::i18n::Lang;
use masterout_core::media::probe::probe;
use masterout_core::media::{self, SidecarPaths};
use masterout_core::naming;
use masterout_core::plan::{MediaInputs, ReleaseMeta};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

const ARTIST: &str = "Artista Demo";
const TITLE: &str = "Canción de prueba";

fn sidecar() -> Option<SidecarPaths> {
    let sc = media::discover(Path::new("/nonexistent-exe-dir"));
    if sc.is_none() {
        eprintln!("skip: no hay ffmpeg/ffprobe");
    }
    sc
}

fn meta() -> ReleaseMeta {
    ReleaseMeta {
        artist: ARTIST.into(),
        title: TITLE.into(),
        year: Some(2026),
        ..Default::default()
    }
}

fn master(dir: &Path, secs: usize) -> PathBuf {
    let sr = 44_100usize;
    let ch: Vec<i32> = (0..sr * secs)
        .map(|i| {
            let t = i as f64 / sr as f64;
            (0.3 * (2.0 * std::f64::consts::PI * 330.0 * t).sin() * 8_388_607.0) as i32
        })
        .collect();
    let path = dir.join("m.wav");
    common::write_wav(&path, 44_100, 24, &[ch.clone(), ch], Default::default());
    path
}

/// Portada sintética 2400×1800 (no cuadrada: se recorta y se avisa).
fn cover(dir: &Path) -> PathBuf {
    let path = dir.join("portada.png");
    image::RgbImage::from_fn(2400, 1800, |x, y| {
        image::Rgb([(x / 10) as u8, (y / 8) as u8, 120])
    })
    .save(&path)
    .unwrap();
    path
}

fn file_in(folder: &Path, sub: &str, name: &str) -> PathBuf {
    let p = folder.join(sub).join(name);
    assert!(p.is_file(), "falta {}", p.display());
    p
}

#[test]
fn delivery_with_cover_produces_cover_canvas_youtube_and_mp3() {
    let Some(sc) = sidecar() else { return };
    let tmp = tempfile::tempdir().unwrap();
    let m = master(tmp.path(), 4);
    let req = ExportRequest::new(
        m,
        meta(),
        ["spotify", "cover", "canvas", "youtube_video", "mp3_promo"]
            .map(String::from)
            .to_vec(),
        tmp.path().join("out"),
        Lang::Es,
    )
    .with_threads(2)
    .with_sidecar(Some(sc.clone()))
    .with_media(MediaInputs {
        cover: Some(cover(tmp.path())),
        ..Default::default()
    });
    let events = Mutex::new(Vec::new());
    let d = export::run(&req, &|p| events.lock().unwrap().push(p)).unwrap();
    assert!(d.failed.is_empty(), "{:?} {:?}", d.failed, d.failed_details);
    let f = &d.folder;
    let n = |tag: &str, suffix: &str, ext: &str| {
        naming::media_file_name(ARTIST, TITLE, tag, suffix, ext)
    };

    // portada: JPG y PNG 3000×3000
    for ext in ["jpg", "png"] {
        let p = file_in(f, "09 Imagen", &n("Portada", "3000x3000", ext));
        let img = image::open(&p).unwrap();
        assert_eq!((img.width(), img.height()), (3000, 3000), "{ext}");
    }
    // Canvas 1080×1920 sin audio, 6 s
    let c = probe(
        &sc,
        &file_in(f, "09 Imagen", &n("Canvas", "1080x1920", "mp4")),
    )
    .unwrap();
    assert_eq!((c.width, c.height), (1080, 1920));
    assert!(!c.has_audio && (c.duration - 6.0).abs() < 0.3, "{c:?}");
    // YouTube 1080p, AAC a 48 kHz, la duración del máster, faststart
    let y = probe(
        &sc,
        &file_in(f, "06 YouTube", &n("YouTube", "1080p", "mp4")),
    )
    .unwrap();
    assert_eq!((y.width, y.height), (1920, 1080));
    assert_eq!(y.audio_codec.as_deref(), Some("aac"));
    assert_eq!(y.audio_sample_rate, Some(48_000));
    assert!((y.duration - 4.0).abs() < 0.3, "{y:?}");
    assert_eq!(y.moov_first, Some(true));
    // MP3 con la portada conformada incrustada
    let mp3 = file_in(f, "08 Promo", &n("MP3 320", "", "mp3"));
    let tag = id3::Tag::read_from_path(&mp3).unwrap();
    let pic = tag.pictures().next().expect("APIC");
    let img = image::load_from_memory(&pic.data).unwrap();
    assert_eq!((img.width(), img.height()), (3000, 3000));
    // el README lista los archivos extra y los avisos de la portada
    let readme = std::fs::read_to_string(f.join("README.txt")).unwrap();
    assert!(
        readme.contains(&n("Portada", "3000x3000", "png")),
        "{readme}"
    );
    assert!(readme.contains("recortado desde el centro"), "{readme}");
    assert!(readme.contains("se ha ampliado"), "{readme}");
    // un solo JobDone por destino, sin temporales
    let done = events
        .into_inner()
        .unwrap()
        .iter()
        .filter(|e| matches!(e, Progress::JobDone { .. }))
        .count();
    assert_eq!(done, 5);
    let mut leftovers = Vec::new();
    for e in walkdir(f) {
        if e.to_string_lossy().ends_with(".part") {
            leftovers.push(e);
        }
    }
    assert!(leftovers.is_empty(), "{leftovers:?}");
}

fn walkdir(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            out.extend(walkdir(&p));
        }
        out.push(p);
    }
    out
}

fn make_video(sc: &SidecarPaths, path: &Path, size: &str, secs: u32) {
    let st = Command::new(&sc.ffmpeg)
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-f",
            "lavfi",
            "-i",
        ])
        .arg(format!("testsrc2=s={size}:r=30:d={secs}"))
        .args(["-c:v", "libx264", "-pix_fmt", "yuv420p"])
        .arg(path)
        .status()
        .unwrap();
    assert!(st.success());
}

#[test]
fn own_media_replaces_the_cover_derived_files() {
    let Some(sc) = sidecar() else { return };
    let tmp = tempfile::tempdir().unwrap();
    let m = master(tmp.path(), 4);
    let vid = tmp.path().join("mio.mp4");
    make_video(&sc, &vid, "1280x720", 12);
    // sin portada de la entrega: el vídeo propio basta para Canvas y YouTube
    let mut media = MediaInputs::default();
    media.per_preset.insert("canvas".into(), vid.clone());
    media.per_preset.insert("youtube_video".into(), vid);
    let req = ExportRequest::new(
        m,
        meta(),
        ["canvas", "youtube_video", "cover"]
            .map(String::from)
            .to_vec(),
        tmp.path().join("out"),
        Lang::En,
    )
    .with_threads(2)
    .with_sidecar(Some(sc.clone()))
    .with_media(media);
    let d = export::run(&req, &|_| {}).unwrap();
    assert!(d.failed.is_empty(), "{:?} {:?}", d.failed, d.failed_details);
    let f = &d.folder;
    let c = probe(
        &sc,
        &file_in(
            f,
            "09 Artwork",
            &naming::media_file_name(ARTIST, TITLE, "Canvas", "1080x1920", "mp4"),
        ),
    )
    .unwrap();
    // 12 s de vídeo propio → máximo 8 s de Canvas, recorte 9:16, sin audio
    assert_eq!((c.width, c.height), (1080, 1920));
    assert!(!c.has_audio && (c.duration - 8.0).abs() < 0.4, "{c:?}");
    let y = probe(
        &sc,
        &file_in(
            f,
            "06 YouTube",
            &naming::media_file_name(ARTIST, TITLE, "YouTube", "1080p", "mp4"),
        ),
    )
    .unwrap();
    assert!((y.duration - 4.0).abs() < 0.3, "{y:?}");
    // sin portada la ficha de portada no sale (y no es un fallo)
    assert!(!f
        .join("09 Artwork")
        .join(naming::media_file_name(
            ARTIST,
            TITLE,
            "Cover",
            "3000x3000",
            "jpg"
        ))
        .exists());
}

#[test]
fn apple_motion_end_to_end_when_slow_tests_are_enabled() {
    if std::env::var_os("MASTERCHECK_SLOW").is_none() {
        eprintln!("skip: MASTERCHECK_SLOW no definido");
        return;
    }
    let Some(sc) = sidecar() else { return };
    let tmp = tempfile::tempdir().unwrap();
    let m = master(tmp.path(), 2);
    let req = ExportRequest::new(
        m,
        meta(),
        vec!["apple_motion".to_string()],
        tmp.path().join("out"),
        Lang::Es,
    )
    .with_sidecar(Some(sc.clone()))
    .with_media(MediaInputs {
        cover: Some(cover(tmp.path())),
        ..Default::default()
    });
    let d = export::run(&req, &|_| {}).unwrap();
    assert!(d.failed.is_empty(), "{:?} {:?}", d.failed, d.failed_details);
    for (suffix, w, h) in [("3840x3840", 3840, 3840), ("2048x2732", 2048, 2732)] {
        let p = file_in(
            &d.folder,
            "02 Apple Music",
            &naming::media_file_name(ARTIST, TITLE, "Portada animada", suffix, "mp4"),
        );
        let i = probe(&sc, &p).unwrap();
        assert_eq!((i.width, i.height), (w, h));
        assert!(!i.has_audio && (i.duration - 10.0).abs() < 0.4, "{i:?}");
    }
}
