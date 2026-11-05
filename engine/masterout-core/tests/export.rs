mod common;

use masterout_core::export::{self, ExportRequest, Progress};
use masterout_core::i18n::Lang;
use masterout_core::loudness;
use masterout_core::mp3;
use masterout_core::naming;
use masterout_core::plan::ReleaseMeta;
use masterout_core::wav;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

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
    ExportRequest {
        master_path: master.to_path_buf(),
        meta: meta(),
        active_ids: ids.iter().map(|s| s.to_string()).collect(),
        out_root: out_root.to_path_buf(),
        lang: Lang::Es,
        threads: Some(2),
    }
}

fn six_ids() -> Vec<&'static str> {
    let mut ids = vec![
        "spotify",
        "apple_adm",
        "deezer",
        "distributor_std",
        "tunecore",
    ];
    if mp3::find_lame().is_some() {
        ids.push("mp3_promo");
    } else {
        eprintln!("skip parcial: lame no encontrado, se omite mp3_promo");
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
            .join(name("TuneCore", 24, 192_000, "wav")),
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
    if mp3::find_lame().is_some() {
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
        ("07 Distribuidora", "TuneCore", 24, 192_000, false),
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

#[test]
fn failed_job_leaves_no_partial_files() {
    // Sin `lame`, el MP3 falla: debe quedar en `failed`, sin archivos ni carpetas residuales.
    if mp3::find_lame().is_some() {
        eprintln!("skip: lame presente");
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 2);
    let out = tmp.path().join("out");
    let d = export::run(&request(&master, &out, &["deezer", "mp3_promo"]), &|_| {}).unwrap();
    assert_eq!(d.failed.len(), 1);
    assert_eq!(d.failed[0].0, "mp3_promo");
    assert!(!d.folder.join("08 Promo").exists());
    let mut all = Vec::new();
    walk(&d.folder, &mut all);
    assert!(all.iter().all(|p| !p.to_string_lossy().ends_with(".part")));
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
    if mp3::find_lame().is_some() {
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
    let Some(lame) = mp3::find_lame() else {
        eprintln!("skip: lame no encontrado");
        return;
    };
    eprintln!("lame: {}", lame.display());
    let tmp = tempfile::tempdir().unwrap();
    let master = synthetic_master(tmp.path(), 3);
    let out = tmp.path().join("out");
    let d = export::run(&request(&master, &out, &["mp3_promo"]), &|_| {}).unwrap();
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
