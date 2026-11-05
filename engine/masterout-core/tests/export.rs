mod common;

use masterout_core::export::{self, ExportRequest, Progress};
use masterout_core::i18n::Lang;
use masterout_core::loudness;
use masterout_core::media::{self, SidecarPaths};
use masterout_core::naming;
use masterout_core::plan::MediaInputs;
use masterout_core::plan::ReleaseMeta;
use masterout_core::wav;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// ffmpeg del entorno (PATH o `MASTERCHECK_FFMPEG`) con `libmp3lame`; si no, el MP3 se salta.
fn mp3_sidecar() -> Option<SidecarPaths> {
    let sc = media::discover(Path::new("/nonexistent-exe-dir"))?;
    let out = std::process::Command::new(&sc.ffmpeg)
        .args(["-hide_banner", "-encoders"])
        .output()
        .ok()?;
    String::from_utf8_lossy(&out.stdout)
        .contains("libmp3lame")
        .then_some(sc)
}

fn has_mp3() -> bool {
    mp3_sidecar().is_some()
}

/// Un "ffmpeg" que siempre falla: el destino de MP3/vídeo da `err.ffmpeg_failed`.
#[cfg(unix)]
fn failing_sidecar(dir: &Path) -> SidecarPaths {
    use std::os::unix::fs::PermissionsExt;
    let p = dir.join("ffmpeg");
    std::fs::write(&p, "#!/bin/sh\necho 'boom' >&2\nexit 1\n").unwrap();
    std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
    SidecarPaths {
        ffmpeg: p.clone(),
        ffprobe: p,
    }
}

const ARTIST: &str = "Artista Demo";
const TITLE: &str = "Canción de prueba";

fn meta() -> ReleaseMeta {
    ReleaseMeta {
        artist: ARTIST.into(),
        title: TITLE.into(),
        year: Some(2026),
        isrc: Some("ESABC2600001".into()),
        composer: Some("Compositor".into()),
        label: Some("Sello".into()),
        copyright: Some("(c) 2026 Sello".into()),
        ..Default::default()
    }
}

/// Máster sintético estéreo 96 kHz / 24 bit de `secs` segundos.
fn synthetic_master(dir: &Path, secs: usize) -> PathBuf {
    let sr = 96_000usize;
    let n = sr * secs;
    let mk = |f: f64, phase: f64| -> Vec<i32> {
        (0..n)
            .map(|i| {
                let t = i as f64 / sr as f64;
                let v = 0.25 * (2.0 * std::f64::consts::PI * f * t + phase).sin()
                    + 0.1 * (2.0 * std::f64::consts::PI * f * 3.1 * t).sin();
                (v * 8_388_607.0) as i32
            })
            .collect()
    };
    let path = dir.join("synthetic (master v1).wav");
    common::write_wav(
        &path,
        96_000,
        24,
        &[mk(220.0, 0.0), mk(330.0, 0.5)],
        Default::default(),
    );
    path
}

fn request(master: &Path, out_root: &Path, ids: &[&str]) -> ExportRequest {
    ExportRequest::new(
        master.to_path_buf(),
        meta(),
        ids.iter().map(|s| s.to_string()).collect(),
        out_root.to_path_buf(),
        Lang::Es,
    )
    .with_threads(2)
}

fn six_ids() -> Vec<&'static str> {
    let mut ids = vec![
        "spotify",
        "apple_adm",
        "deezer",
        "distributor_std",
        "tunecore",
    ];
    if has_mp3() {
        ids.push("mp3_promo");
    } else {
        eprintln!("skip parcial: ffmpeg con libmp3lame no encontrado, se omite mp3_promo");
        ids.push("youtube_music");
    }
    ids
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, out);
        }
        out.push(p);
    }
}

fn name(tag: &str, bits: u16, sr: u32, ext: &str) -> String {
    naming::audio_file_name(ARTIST, TITLE, tag, bits, sr, ext)
}

#[test]
fn exports_full_tree_for_synthetic_master() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 5);
    let out = tmp.path().join("out");
    let ids = six_ids();
    let d = export::run(&request(&master, &out, &ids), &|_| {}).unwrap();
    assert!(d.failed.is_empty(), "{:?}", d.failed);
    assert_eq!(
        d.folder.file_name().unwrap().to_string_lossy(),
        naming::delivery_folder(ARTIST, TITLE, None)
    );
    let f = &d.folder;
    for p in [
        f.join("01 Spotify")
            .join(name("Spotify", 24, 96_000, "flac")),
        f.join("02 Apple Music")
            .join(name("Apple Digital Masters", 24, 96_000, "wav")),
        f.join("05 Deezer").join(name("Deezer", 16, 44_100, "wav")),
        f.join("07 Distribuidora")
            .join(name("Distribuidora", 16, 44_100, "wav")),
        f.join("07 Distribuidora")
            .join(name("TuneCore", 24, 96_000, "wav")),
        f.join("README.txt"),
        f.join("informe-loudness.txt"),
        f.join("ficha-lanzamiento.csv"),
        f.join("ficha-lanzamiento.txt"),
    ] {
        assert!(p.is_file(), "falta {}", p.display());
    }
    assert!(!f
        .join("07 Distribuidora")
        .join(name("Distribuidora Hi-Res", 24, 96_000, "wav"))
        .exists());
    if has_mp3() {
        assert!(f.join("08 Promo").is_dir());
    } else {
        assert!(!f.join("08 Promo").exists());
    }
    let mut all = Vec::new();
    walk(f, &mut all);
    for p in &all {
        if p.is_dir() {
            assert!(
                std::fs::read_dir(p).unwrap().next().is_some(),
                "subcarpeta vacía {}",
                p.display()
            );
        } else {
            assert!(
                !p.to_string_lossy().ends_with(".part"),
                "archivo temporal {}",
                p.display()
            );
        }
    }
    // Delivery.files cubre los archivos reales.
    assert_eq!(d.files.len(), all.iter().filter(|p| p.is_file()).count());
    for (p, b) in &d.files {
        assert_eq!(std::fs::metadata(p).unwrap().len(), *b);
    }
}

#[test]
fn written_wavs_have_expected_headers() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 5);
    let out = tmp.path().join("out");
    let ids = ["apple_adm", "deezer", "distributor_std", "tunecore"];
    let d = export::run(&request(&master, &out, &ids), &|_| {}).unwrap();
    assert!(d.failed.is_empty(), "{:?}", d.failed);
    let cases = [
        ("02 Apple Music", "Apple Digital Masters", 24, 96_000, true),
        ("05 Deezer", "Deezer", 16, 44_100, true),
        ("07 Distribuidora", "Distribuidora", 16, 44_100, true),
        ("07 Distribuidora", "TuneCore", 24, 96_000, false),
    ];
    for (sub, tag, bits, sr, meta) in cases {
        let p = d.folder.join(sub).join(name(tag, bits, sr, "wav"));
        let info = wav::probe(&p).unwrap();
        assert_eq!(info.format_tag, 0x0001, "{tag}");
        assert_eq!((info.bits, info.sample_rate), (bits, sr), "{tag}");
        assert_eq!(info.channels, 2);
        let has = |c: &str| info.extra_chunks.iter().any(|x| x == c);
        assert_eq!(has("bext"), meta, "{tag} bext");
        assert_eq!(has("LIST"), meta, "{tag} LIST");
    }
}

#[test]
fn existing_delivery_folder_gets_numbered_suffix() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let out = tmp.path().join("out");
    let req = request(&master, &out, &["deezer"]);
    let d1 = export::run(&req, &|_| {}).unwrap();
    let d2 = export::run(&req, &|_| {}).unwrap();
    let d3 = export::run(&req, &|_| {}).unwrap();
    let base = naming::delivery_folder(ARTIST, TITLE, None);
    let n = |d: &export::Delivery| d.folder.file_name().unwrap().to_string_lossy().to_string();
    assert_eq!(n(&d1), base);
    assert_eq!(n(&d2), format!("{base} (2)"));
    assert_eq!(n(&d3), format!("{base} (3)"));
}

#[test]
fn progress_events_cover_every_job_once() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 3);
    let out = tmp.path().join("out");
    let ids = six_ids();
    let events = Mutex::new(Vec::new());
    let d = export::run(&request(&master, &out, &ids), &|p| {
        events.lock().unwrap().push(p)
    })
    .unwrap();
    let events = events.into_inner().unwrap();
    assert!(matches!(events.first(), Some(Progress::Analyzed(_))));
    assert!(matches!(events.get(1), Some(Progress::Planned(_))));
    match events.last() {
        Some(Progress::Finished(f)) => assert_eq!(f.folder, d.folder),
        other => panic!("el último evento debe ser Finished: {other:?}"),
    }
    assert_eq!(
        events
            .iter()
            .filter(|e| matches!(e, Progress::Finished(_)))
            .count(),
        1
    );
    for id in &ids {
        let started = events
            .iter()
            .filter(|e| matches!(e, Progress::JobStarted { preset_id } if preset_id == id))
            .count();
        let done = events
            .iter()
            .filter(|e| matches!(e, Progress::JobDone { preset_id, .. } if preset_id == id))
            .count();
        assert_eq!((started, done), (1, 1), "{id}");
        let progress: Vec<f32> = events
            .iter()
            .filter_map(|e| match e {
                Progress::JobProgress {
                    preset_id,
                    fraction,
                } if preset_id == id => Some(*fraction),
                _ => None,
            })
            .collect();
        assert!(progress.iter().all(|f| (0.0..=1.0).contains(f)));
    }
}

#[cfg(unix)]
#[test]
fn failed_job_leaves_no_partial_files() {
    // Con un ffmpeg que falla, el MP3 queda en `failed`, sin archivos ni carpetas residuales.
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let out = tmp.path().join("out");
    let req = request(&master, &out, &["deezer", "mp3_promo"])
        .with_sidecar(Some(failing_sidecar(tmp.path())));
    let d = export::run(&req, &|_| {}).unwrap();
    assert_eq!(d.failed.len(), 1);
    assert_eq!(d.failed[0].0, "mp3_promo");
    assert!(!d.folder.join("08 Promo").exists());
    let mut all = Vec::new();
    walk(&d.folder, &mut all);
    assert!(all.iter().all(|p| !p.to_string_lossy().ends_with(".part")));
}

#[test]
fn without_sidecar_the_mp3_is_disabled_and_the_rest_exports() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let out = tmp.path().join("out");
    let req = request(
        &master,
        &out,
        &["deezer", "mp3_promo", "youtube_video", "canvas"],
    )
    .with_sidecar(None);
    let d = export::run(&req, &|_| {}).unwrap();
    assert!(d.failed.is_empty(), "{:?}", d.failed);
    assert!(!d.folder.join("08 Promo").exists());
    assert!(d.folder.join("05 Deezer").is_dir());
    let readme = std::fs::read_to_string(d.folder.join("README.txt")).unwrap();
    assert!(readme.contains("ffmpeg"), "{readme}");
}

#[test]
fn real_master_end_to_end_under_time_budget() {
    let Some(dir) = common::fixtures_dir() else {
        return;
    };
    let master = dir.join("m96_24.wav");
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("out");
    let mut ids = vec![
        "spotify",
        "apple_adm",
        "deezer",
        "distributor_std",
        "tunecore",
        "youtube_music",
    ];
    if has_mp3() {
        ids.push("mp3_promo");
    }
    let t0 = std::time::Instant::now();
    let mut req = request(&master, &out, &ids);
    req.threads = None;
    let d = export::run(&req, &|_| {}).unwrap();
    let secs = t0.elapsed().as_secs_f64();
    eprintln!("exportación completa en {secs:.1} s");
    assert!(d.failed.is_empty(), "{:?}", d.failed);
    assert!(secs < 60.0, "{secs} s");

    let (_, info) = export::analyze(&master).unwrap();
    let p = d
        .folder
        .join("05 Deezer")
        .join(name("Deezer", 16, 44_100, "wav"));
    let rep = loudness::measure(&wav::read(&p).unwrap()).unwrap();
    let diff = (rep.integrated_lufs - info.loudness.integrated_lufs).abs();
    assert!(diff < 0.2, "diferencia de {diff} LU");
}

#[test]
fn mp3_has_id3v23_frames() {
    let Some(sc) = mp3_sidecar() else {
        eprintln!("skip: ffmpeg con libmp3lame no encontrado");
        return;
    };
    eprintln!("ffmpeg: {}", sc.ffmpeg.display());
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 3);
    let out = tmp.path().join("out");
    let cover = tmp.path().join("cover.png");
    image::RgbImage::from_pixel(2000, 2000, image::Rgb([200, 60, 30]))
        .save(&cover)
        .unwrap();
    let req = request(&master, &out, &["mp3_promo"])
        .with_sidecar(Some(sc.clone()))
        .with_media(MediaInputs {
            cover: Some(cover),
            ..Default::default()
        });
    let d = export::run(&req, &|_| {}).unwrap();
    assert!(d.failed.is_empty(), "{:?}", d.failed);
    let file = std::fs::read_dir(d.folder.join("08 Promo"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    use id3::TagLike;
    let tag = id3::Tag::read_from_path(&file).unwrap();
    assert_eq!(tag.version(), id3::Version::Id3v23);
    assert_eq!(tag.title(), Some(TITLE));
    assert_eq!(tag.artist(), Some(ARTIST));
    assert_eq!(tag.year(), Some(2026));
    assert_eq!(
        tag.get("TSRC").and_then(|f| f.content().text()),
        Some("ESABC2600001")
    );
    assert!(tag.get("TCOM").is_some() && tag.get("TPUB").is_some() && tag.get("TCOP").is_some());
    // portada conformada (JPG 3000×3000) incrustada como APIC
    let pic = tag.pictures().next().expect("sin APIC");
    assert_eq!(pic.mime_type, "image/jpeg");
    assert_eq!(pic.picture_type, id3::frame::PictureType::CoverFront);
    let img = image::load_from_memory(&pic.data).unwrap();
    assert_eq!((img.width(), img.height()), (3000, 3000));
    // 320 kbps CBR con cabecera Xing, medido por ffprobe
    let info = media::probe::probe(&sc, &file).unwrap();
    assert_eq!(info.audio_codec.as_deref(), Some("mp3"));
    assert_eq!(info.audio_bitrate, Some(320_000), "{info:?}");
}

#[test]
fn document_failure_is_reported_and_finished_still_fires() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let out = tmp.path().join("out");
    let events = Mutex::new(Vec::new());
    let d = export::run(&request(&master, &out, &["deezer"]), &|p| {
        // Tras el primer destino, un directorio "README.txt" impide escribir el documento.
        if let Progress::JobDone { path, .. } = &p {
            let folder = path.parent().unwrap().parent().unwrap();
            std::fs::create_dir(folder.join("README.txt")).unwrap();
        }
        events.lock().unwrap().push(p);
    })
    .unwrap();
    assert!(d.failed.iter().any(|(id, _)| id == "documents"));
    assert_eq!(d.failed_details.len(), d.failed.len());
    assert!(d
        .failed_details
        .iter()
        .all(|(id, det)| id == "documents" && det.is_some()));
    assert!(matches!(
        events.into_inner().unwrap().last(),
        Some(Progress::Finished(_))
    ));
    assert!(d.folder.join("informe-loudness.txt").is_file());
    assert!(d
        .files
        .iter()
        .any(|(p, _)| p.to_string_lossy().contains("[Deezer]")));
}

/// Máster float32 96 kHz de 2 s con valores que no caen en enteros de 24 bit.
fn float_master(dir: &Path) -> PathBuf {
    let n = 96_000 * 2;
    let ch: Vec<f32> = (0..n)
        .map(|i| 0.2 * (i as f32 * 0.0123).sin() + 0.0000001234 * (i % 7) as f32)
        .collect();
    let path = dir.join("float.wav");
    common::write_wav_f32(&path, 96_000, &[ch.clone(), ch], Default::default());
    path
}

#[test]
fn dither_follows_plan_for_float_master() {
    let tmp = tempfile::tempdir().unwrap();
    let master = float_master(tmp.path());
    let out = tmp.path().join("out");
    let events = Mutex::new(Vec::new());
    let d = export::run(&request(&master, &out, &["apple_adm"]), &|p| {
        events.lock().unwrap().push(p)
    })
    .unwrap();
    assert!(d.failed.is_empty(), "{:?}", d.failed);
    let needs = events
        .into_inner()
        .unwrap()
        .into_iter()
        .find_map(|e| match e {
            Progress::Planned(v) => v.into_iter().find_map(|r| match r {
                masterout_core::plan::Resolved::Job(j) => Some(j.needs_dither),
                _ => None,
            }),
            _ => None,
        })
        .unwrap();
    let src = wav::read(&master).unwrap();
    let out_buf = wav::read(&d.folder.join("02 Apple Music").join(name(
        "Apple Digital Masters",
        24,
        96_000,
        "wav",
    )))
    .unwrap();
    let direct: Vec<Vec<f32>> = src
        .channels
        .iter()
        .map(|c| {
            c.iter()
                .map(|&x| {
                    ((f64::from(x) * 8_388_608.0)
                        .round()
                        .clamp(-8_388_608.0, 8_388_607.0)
                        / 8_388_608.0) as f32
                })
                .collect()
        })
        .collect();
    // Con dither el resultado debe diferir de la conversión directa; sin él, ser idéntico.
    assert_eq!(out_buf.channels == direct, !needs);
}

#[test]
fn integer_master_native_destination_has_no_dither() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let out = tmp.path().join("out");
    let d = export::run(&request(&master, &out, &["apple_adm"]), &|_| {}).unwrap();
    let o = wav::read(&d.folder.join("02 Apple Music").join(name(
        "Apple Digital Masters",
        24,
        96_000,
        "wav",
    )))
    .unwrap();
    assert_eq!(o.channels, wav::read(&master).unwrap().channels);
}

#[test]
fn master_above_96k_is_rejected_with_i18n_key() {
    let tmp = tempfile::tempdir().unwrap();
    let ch: Vec<i32> = (0..192_000).map(|i| (i % 100) * 1000).collect();
    let master = tmp.path().join("hires.wav");
    common::write_wav(&master, 192_000, 24, &[ch.clone(), ch], Default::default());
    match export::analyze(&master) {
        Err(masterout_core::audio::AudioError::UnsupportedFormat(k)) => {
            assert_eq!(k, "err.rate_over_96k")
        }
        other => panic!("{:?}", other.map(|_| ())),
    }
    let d = export::run(
        &request(&master, &tmp.path().join("out"), &["spotify"]),
        &|_| {},
    );
    assert!(d.is_err());
}

#[test]
fn cancel_mid_export_leaves_no_partials() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 4);
    let out = tmp.path().join("out");
    let cancel = Arc::new(AtomicBool::new(false));
    let req = request(
        &master,
        &out,
        &[
            "spotify",
            "apple_adm",
            "deezer",
            "distributor_std",
            "tunecore",
        ],
    )
    .with_threads(1)
    .with_cancel(cancel.clone());
    let events = Mutex::new(Vec::new());
    let d = export::run(&req, &|p| {
        if matches!(p, Progress::JobDone { .. }) {
            cancel.store(true, Ordering::Relaxed);
        }
        events.lock().unwrap().push(p);
    })
    .unwrap();
    let events = events.into_inner().unwrap();
    let kept = match events.last() {
        Some(Progress::Cancelled { files_kept }) => *files_kept,
        other => panic!("se esperaba Cancelled, llegó {other:?}"),
    };
    assert!(!events.iter().any(|e| matches!(e, Progress::Finished(_))));
    assert!(kept >= 1 && (kept as usize) < 5, "files_kept = {kept}");
    assert_eq!(d.files.len(), kept as usize);
    assert!(d.cancelled);
    let mut all = Vec::new();
    walk(&d.folder, &mut all);
    assert!(all.iter().all(|p| !p.to_string_lossy().ends_with(".part")));
    for p in &all {
        if p.is_dir() {
            assert!(
                std::fs::read_dir(p).unwrap().next().is_some(),
                "carpeta vacía {}",
                p.display()
            );
        }
    }
    assert!(!d.folder.join("README.txt").exists());
}

#[test]
fn cancel_before_any_file_removes_delivery_folder() {
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let out = tmp.path().join("out");
    let req = request(&master, &out, &["deezer"]).with_cancel(Arc::new(AtomicBool::new(true)));
    let d = export::run(&req, &|_| {}).unwrap();
    assert!(d.files.is_empty());
    assert!(!d.folder.exists());
}

#[test]
fn unwritable_out_root_reports_failure() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let out = tmp.path().join("not-a-dir");
    std::fs::write(&out, b"x").unwrap();
    let r = export::run(&request(&master, &out, &["deezer"]), &|_| {});
    assert!(matches!(r, Err(masterout_core::audio::AudioError::Io(_))));
}

#[test]
fn panicking_progress_callback_does_not_abort_run() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let out = tmp.path().join("out");
    let boom = AtomicBool::new(true);
    let finished = AtomicBool::new(false);
    let d = export::run(&request(&master, &out, &["deezer", "spotify"]), &|p| {
        if matches!(p, Progress::JobStarted { .. }) && boom.swap(false, Ordering::SeqCst) {
            panic!("callback roto");
        }
        if matches!(p, Progress::Finished(_)) {
            finished.store(true, Ordering::SeqCst);
        }
    })
    .unwrap();
    assert!(finished.load(Ordering::SeqCst));
    assert!(d.failed.is_empty(), "{:?}", d.failed);
    assert!(d.folder.join("README.txt").is_file());
}

#[test]
fn english_export_uses_english_contract() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let out = tmp.path().join("out");
    let mut req = request(&master, &out, &["distributor_std", "deezer"]);
    req.lang = Lang::En;
    let d = export::run(&req, &|_| {}).unwrap();
    assert!(d.failed.is_empty(), "{:?}", d.failed);
    let f = &d.folder;
    assert!(f
        .join("07 Distributor")
        .join(name("Distributor", 16, 44_100, "wav"))
        .is_file());
    assert!(f.join("loudness-report.txt").is_file());
    assert!(f.join("release-sheet.csv").is_file());
    assert!(!f.join("informe-loudness.txt").exists());
}

#[cfg(unix)]
#[test]
fn job_errors_are_keys_never_spanish_text() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let events = Mutex::new(Vec::new());
    let mut req = request(&master, &tmp.path().join("out"), &["deezer", "mp3_promo"])
        .with_sidecar(Some(failing_sidecar(tmp.path())));
    req.lang = Lang::En;
    let d = export::run(&req, &|p| events.lock().unwrap().push(p)).unwrap();
    assert_eq!(
        d.failed,
        vec![("mp3_promo".to_string(), "err.ffmpeg_failed".to_string())]
    );
    let failed = events
        .into_inner()
        .unwrap()
        .into_iter()
        .find_map(|e| match e {
            Progress::JobFailed { error, .. } => Some(error),
            _ => None,
        })
        .unwrap();
    assert_eq!(failed, "err.ffmpeg_failed");
    assert!(!failed.contains("formato no soportado"));
    assert!(masterout_core::i18n::t(Lang::En, &failed).contains("could not be created"));
}

#[test]
fn mp3_shares_the_dithered_cd_version_with_deezer() {
    use masterout_core::plan::{self, Resolved};
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let (_, info) = export::analyze(&master).unwrap();
    let plan = plan::resolve(
        &info,
        &meta(),
        &["deezer", "mp3_promo", "distributor_std"],
        &MediaInputs::default(),
    );
    let jobs: Vec<&plan::Job> = plan
        .iter()
        .filter_map(|r| match r {
            Resolved::Job(j) => Some(j),
            Resolved::Disabled { .. } => None,
        })
        .collect();
    assert_eq!(jobs.len(), 3);
    assert!(jobs.iter().all(|j| j.needs_dither));
    assert_eq!(export::plan_sample_keys(&jobs), vec![(16, 44_100, true)]);
}

#[test]
fn panic_inside_a_job_is_contained_and_leaves_no_partials() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    *export::TEST_PANIC_PRESET.lock().unwrap() = Some("tidal".into());
    let events = Mutex::new(Vec::new());
    let d = export::run(
        &request(&master, &tmp.path().join("out"), &["tidal", "deezer"]),
        &|p| events.lock().unwrap().push(p),
    )
    .unwrap();
    let ev = events.into_inner().unwrap();
    assert!(ev.iter().any(|e| matches!(
        e,
        Progress::JobFailed { preset_id, error, .. } if preset_id == "tidal" && error == "err.internal"
    )));
    assert!(matches!(ev.last(), Some(Progress::Finished(_))));
    assert_eq!(
        d.failed,
        vec![("tidal".to_string(), "err.internal".to_string())]
    );
    // sin detalle técnico (un panic no lo tiene): sigue habiendo una entrada por posición
    assert_eq!(d.failed_details, vec![("tidal".to_string(), None)]);
    assert!(d
        .files
        .iter()
        .any(|(p, _)| p.to_string_lossy().contains("[Deezer]")));
    let mut all = Vec::new();
    walk(&d.folder, &mut all);
    assert!(all.iter().all(|p| !p.to_string_lossy().ends_with(".part")));
    assert!(!d.folder.join("04 Tidal").exists());
}

#[test]
fn cancel_at_job_start_is_much_faster_than_a_full_flac_run() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Instant;
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 60);
    let t = Instant::now();
    let full = export::run(
        &request(&master, &tmp.path().join("full"), &["spotify"]),
        &|_| {},
    )
    .unwrap();
    let full_time = t.elapsed();
    assert!(!full.cancelled && full.failed.is_empty());

    let cancel = Arc::new(AtomicBool::new(false));
    let req =
        request(&master, &tmp.path().join("cancelled"), &["spotify"]).with_cancel(cancel.clone());
    let events = Mutex::new(Vec::new());
    let t = Instant::now();
    let d = export::run(&req, &|p| {
        if matches!(p, Progress::JobStarted { .. }) {
            cancel.store(true, Ordering::Relaxed);
        }
        events.lock().unwrap().push(p);
    })
    .unwrap();
    let cancelled_time = t.elapsed();
    assert!(d.cancelled && d.files.is_empty());
    assert!(!d.folder.exists());
    assert!(matches!(
        events.into_inner().unwrap().last(),
        Some(Progress::Cancelled { files_kept: 0 })
    ));
    eprintln!("completo {full_time:?}, cancelado {cancelled_time:?}");
    assert!(
        cancelled_time < full_time / 2,
        "cancelar tardó {cancelled_time:?} de {full_time:?}"
    );
}

// ---------------------------------------------------------------- tarea 7 (hito 3)

#[test]
fn preparing_progress_precedes_the_first_job_and_ends_at_one() {
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let events = Mutex::new(Vec::new());
    // spotify (nativo) y deezer (16 bit / 44,1 kHz): dos combinaciones de muestras
    export::run(
        &request(&master, &tmp.path().join("out"), &["spotify", "deezer"]),
        &|p| events.lock().unwrap().push(p),
    )
    .unwrap();
    let ev = events.into_inner().unwrap();
    let fractions: Vec<f32> = ev
        .iter()
        .filter_map(|e| match e {
            Progress::Preparing { fraction } => Some(*fraction),
            _ => None,
        })
        .collect();
    assert_eq!(fractions.len(), 2, "{fractions:?}");
    assert!(fractions.windows(2).all(|w| w[0] < w[1]), "{fractions:?}");
    assert!(fractions[0] > 0.0 && fractions[1] == 1.0, "{fractions:?}");
    let first_job = ev
        .iter()
        .position(|e| matches!(e, Progress::JobStarted { .. }))
        .unwrap();
    let last_prep = ev
        .iter()
        .rposition(|e| matches!(e, Progress::Preparing { .. }))
        .unwrap();
    assert!(
        last_prep < first_job,
        "preparar va antes del primer destino"
    );
}

/// WAV mono 16 bit de `secs` segundos a 8 kHz, esparcido (solo cabecera real).
fn sparse_wav(path: &Path, secs: u64) {
    use std::io::Write;
    let data_len = secs * 8000 * 2;
    let mut f = std::fs::File::create(path).unwrap();
    let mut h = Vec::new();
    h.extend_from_slice(b"RIFF");
    h.extend_from_slice(&((36 + data_len) as u32).to_le_bytes());
    h.extend_from_slice(b"WAVEfmt ");
    h.extend_from_slice(&16u32.to_le_bytes());
    h.extend_from_slice(&1u16.to_le_bytes());
    h.extend_from_slice(&1u16.to_le_bytes());
    h.extend_from_slice(&8000u32.to_le_bytes());
    h.extend_from_slice(&16_000u32.to_le_bytes());
    h.extend_from_slice(&2u16.to_le_bytes());
    h.extend_from_slice(&16u16.to_le_bytes());
    h.extend_from_slice(b"data");
    h.extend_from_slice(&(data_len as u32).to_le_bytes());
    f.write_all(&h).unwrap();
    f.set_len(44 + data_len).unwrap();
}

#[test]
fn master_longer_than_30_minutes_is_rejected_before_reading_it() {
    let tmp = tempfile::tempdir().unwrap();
    let long = tmp.path().join("larga.wav");
    sparse_wav(&long, 30 * 60 + 1);
    match export::analyze(&long) {
        Err(masterout_core::audio::AudioError::UnsupportedFormat(k)) => {
            assert_eq!(k, "err.too_long")
        }
        other => panic!("{:?}", other.map(|_| ())),
    }
    assert!(export::run(
        &request(&long, &tmp.path().join("out"), &["spotify"]),
        &|_| {}
    )
    .is_err());
    // justo en el tope se acepta (la cabecera pasa; el contenido es silencio)
    let ok = tmp.path().join("justa.wav");
    sparse_wav(&ok, 30 * 60);
    assert!(export::check_length(&masterout_core::wav::probe(&ok).unwrap()).is_ok());
    // la clave existe en los dos idiomas
    for l in [Lang::Es, Lang::En] {
        assert!(!masterout_core::i18n::t(l, "err.too_long").starts_with('⟦'));
    }
}

#[cfg(unix)]
mod adm_in_export {
    use super::*;
    use masterout_core::loudness::Verdict;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    fn script(dir: &Path, name: &str, body: &str) -> PathBuf {
        let p = dir.join(name);
        std::fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
        p
    }

    fn adm_request(
        master: &Path,
        out: &Path,
        tools: &Path,
        clip: &str,
        convert: &str,
    ) -> ExportRequest {
        let conv = script(tools, "afconvert", convert);
        let clip = script(tools, "afclip", clip);
        request(master, out, &["apple_adm", "deezer"]).with_adm_tools(conv, clip)
    }

    const CLIPS: &str = "printf 'CLIPPED SAMPLES:\\n   total clipped samples    Left   on-sample:   4    inter-sample:   8\\n'";
    const CLEAN: &str = "echo ' -- no samples clipped --'";

    #[test]
    fn the_delivery_carries_the_final_verdict_with_the_adm_result() {
        let tmp = tempfile::tempdir().unwrap();
        let master = synthetic_master(tmp.path(), 2);
        let d = export::run(
            &adm_request(
                &master,
                &tmp.path().join("o1"),
                tmp.path(),
                CLIPS,
                "touch \"$2\"",
            ),
            &|_| {},
        )
        .unwrap();
        let v: std::collections::HashMap<_, _> = d.verdicts.iter().cloned().collect();
        assert_eq!(
            v["apple_adm"],
            Verdict::Warn("adm_clipping".into()),
            "{:?}",
            d.verdicts
        );
        assert!(!v.contains_key("tunecore"));
        let report = std::fs::read_to_string(d.folder.join("informe-loudness.txt")).unwrap();
        assert!(
            report.contains("12"),
            "el informe cuenta las 12 muestras: {report}"
        );
        // sin clipping, el veredicto es el del plan
        let d = export::run(
            &adm_request(
                &master,
                &tmp.path().join("o2"),
                tmp.path(),
                CLEAN,
                "touch \"$2\"",
            ),
            &|_| {},
        )
        .unwrap();
        let v: std::collections::HashMap<_, _> = d.verdicts.iter().cloned().collect();
        assert_eq!(v["apple_adm"], Verdict::Ok);
        assert!(v.contains_key("deezer"));
    }

    #[test]
    fn cancelling_in_the_middle_of_adm_writes_no_report_and_no_adm_cancelled_text() {
        let tmp = tempfile::tempdir().unwrap();
        let master = synthetic_master(tmp.path(), 2);
        let cancel = Arc::new(AtomicBool::new(false));
        // afconvert tarda: la cancelación llega con la validación en marcha
        let req = adm_request(&master, &tmp.path().join("o"), tmp.path(), CLEAN, "sleep 5")
            .with_cancel(cancel.clone());
        let events = Mutex::new(Vec::new());
        let started = std::time::Instant::now();
        let d = export::run(&req, &|p| {
            if matches!(&p, Progress::JobProgress { preset_id, fraction } if preset_id == "apple_adm" && *fraction >= 0.9)
            {
                let c = cancel.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    c.store(true, Ordering::Relaxed);
                });
            }
            events.lock().unwrap().push(p);
        })
        .unwrap();
        assert!(
            started.elapsed() < std::time::Duration::from_secs(4),
            "la validación se corta al cancelar"
        );
        assert!(d.cancelled);
        assert!(
            d.verdicts.is_empty(),
            "una entrega cancelada no promete veredictos: {:?}",
            d.verdicts
        );
        assert!(matches!(
            events.into_inner().unwrap().last(),
            Some(Progress::Cancelled { .. })
        ));
        // ningún documento (el informe con `adm_cancelled` no se escribe) y sin temporales
        let mut all = Vec::new();
        walk(&d.folder, &mut all);
        assert!(
            all.iter().all(|p| {
                let n = p.file_name().unwrap().to_string_lossy().into_owned();
                n != "informe-loudness.txt" && n != "README.txt" && !n.ends_with(".part")
            }),
            "{all:?}"
        );
    }
}

#[cfg(unix)]
#[test]
fn failed_details_line_up_with_failed_by_position() {
    // un destino con detalle técnico (ffmpeg falla) y los documentos (detalle propio)
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let req = request(&master, &tmp.path().join("out"), &["deezer", "mp3_promo"])
        .with_sidecar(Some(failing_sidecar(tmp.path())));
    let d = export::run(&req, &|_| {}).unwrap();
    assert_eq!(d.failed.len(), 1);
    assert_eq!(
        d.failed_details.len(),
        d.failed.len(),
        "{:?}",
        d.failed_details
    );
    assert_eq!(d.failed_details[0].0, d.failed[0].0);
    assert!(
        d.failed_details[0]
            .1
            .as_deref()
            .is_some_and(|x| x.contains("boom")),
        "{:?}",
        d.failed_details
    );
}
