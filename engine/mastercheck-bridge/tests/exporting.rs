mod common;

use mastercheck_bridge::dto::{ExportRequestDto, ReleaseMetaDto};
use mastercheck_bridge::exporting::export;
use masterout_core::i18n::Lang;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

fn request(master: &Path, out: &Path, ids: &[&str]) -> ExportRequestDto {
    ExportRequestDto {
        master_path: master.to_string_lossy().into_owned(),
        out_root: out.to_string_lossy().into_owned(),
        meta: ReleaseMetaDto {
            artist: "Vera Alcón".into(),
            title: "Luz de Noviembre".into(),
            ..Default::default()
        },
        active_ids: ids.iter().map(|s| s.to_string()).collect(),
    }
}

fn master(dir: &Path) -> std::path::PathBuf {
    let path = dir.join("máster con espacios (master v1).wav");
    common::write_wav(&path, 48_000, 24, &common::sine_stereo(48_000, 24, 3, 0.4));
    path
}

#[test]
fn export_emits_progress_dtos_in_order_and_returns_delivery() {
    let tmp = tempfile::tempdir().unwrap();
    let m = master(tmp.path());
    let events = Mutex::new(Vec::new());
    let d = export(
        &request(&m, &tmp.path().join("salida"), &["spotify", "deezer"]),
        Lang::Es,
        &|p| events.lock().unwrap().push(p),
        Arc::new(AtomicBool::new(false)),
    )
    .unwrap();
    let ev = events.into_inner().unwrap();
    let kinds: Vec<&str> = ev.iter().map(|e| e.kind.as_str()).collect();
    assert_eq!(&kinds[..2], ["analyzed", "planned"]);
    assert_eq!(*kinds.last().unwrap(), "finished");
    for id in ["spotify", "deezer"] {
        let pos = |k: &str| {
            ev.iter()
                .position(|e| e.kind == k && e.preset_id.as_deref() == Some(id))
                .unwrap_or_else(|| panic!("falta {k} de {id}"))
        };
        assert!(pos("job_started") < pos("job_progress"));
        assert!(pos("job_progress") < pos("job_done"));
    }
    let done = ev.iter().find(|e| e.kind == "job_done").unwrap();
    assert!(done.path.is_some() && done.bytes.unwrap() > 0);
    assert!(ev
        .iter()
        .filter(|e| e.kind == "job_progress")
        .all(|e| (0.0..=1.0).contains(&e.fraction.unwrap())));
    // 2 destinos + 4 documentos.
    assert_eq!(d.files.len(), 6);
    assert!(d.failed.is_empty());
    assert!(d.folder.ends_with("Vera Alcón - Luz de Noviembre"));
    assert!(d
        .files
        .iter()
        .all(|(p, b)| Path::new(p).is_file() && *b > 0));
    assert_eq!(ev.last().unwrap().delivery.as_ref().unwrap(), &d);
    // Todo evento viaja como JSON.
    for e in &ev {
        let json = serde_json::to_string(e).unwrap();
        assert_eq!(
            serde_json::from_str::<mastercheck_bridge::dto::ProgressDto>(&json).unwrap(),
            *e
        );
    }
}

#[test]
fn export_in_english_uses_english_names() {
    let tmp = tempfile::tempdir().unwrap();
    let m = master(tmp.path());
    let d = export(
        &request(&m, &tmp.path().join("out"), &["distributor_std"]),
        Lang::En,
        &|_| {},
        Arc::new(AtomicBool::new(false)),
    )
    .unwrap();
    assert!(d
        .files
        .iter()
        .any(|(p, _)| p.contains("07 Distributor") && p.contains("[Distributor]")));
    assert!(d
        .files
        .iter()
        .any(|(p, _)| p.ends_with("loudness-report.txt")));
}

#[test]
fn cancel_emits_cancelled_and_keeps_only_complete_files() {
    let tmp = tempfile::tempdir().unwrap();
    let m = master(tmp.path());
    let cancel = Arc::new(AtomicBool::new(false));
    let events = Mutex::new(Vec::new());
    let d = export(
        &request(
            &m,
            &tmp.path().join("out"),
            &["spotify", "apple_adm", "deezer", "tunecore"],
        ),
        Lang::Es,
        &|p| {
            if p.kind == "job_done" {
                cancel.store(true, Ordering::Relaxed);
            }
            events.lock().unwrap().push(p);
        },
        cancel.clone(),
    )
    .unwrap();
    let ev = events.into_inner().unwrap();
    assert_eq!(ev.last().unwrap().kind, "cancelled");
    assert!(!ev.iter().any(|e| e.kind == "finished"));
    assert!(!d.files.is_empty());
    assert!(d
        .files
        .iter()
        .all(|(p, _)| !p.ends_with(".part") && Path::new(p).is_file()));
}

#[test]
fn unwritable_out_root_reports_failure() {
    let tmp = tempfile::tempdir().unwrap();
    let m = master(tmp.path());
    let out = tmp.path().join("archivo");
    std::fs::write(&out, b"no soy una carpeta").unwrap();
    let e = export(
        &request(&m, &out, &["deezer"]),
        Lang::Es,
        &|_| {},
        Arc::new(AtomicBool::new(false)),
    )
    .unwrap_err();
    assert_eq!(e.key, "err.io");
    assert!(e.detail.is_some());
}

#[test]
fn non_wav_master_is_reported_with_key() {
    let tmp = tempfile::tempdir().unwrap();
    let txt = tmp.path().join("nota.txt");
    std::fs::write(&txt, "hola, esto no es un wav de verdad, ni de lejos").unwrap();
    let e = export(
        &request(&txt, &tmp.path().join("out"), &["deezer"]),
        Lang::En,
        &|_| {},
        Arc::new(AtomicBool::new(false)),
    )
    .unwrap_err();
    assert_eq!(e.key, "err.not_wav");
}
