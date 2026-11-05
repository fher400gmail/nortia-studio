use mastercheck_bridge::app_support::smoke::{parse_spec, run_and_report, run_smoke};
use mastercheck_bridge::app_support::{app_info, busy_error, is_image_path, map_locale, os_name};
use mastercheck_bridge::dto::{DeliveryDto, MasterDto, PlanDto};
use masterout_core::i18n::Lang;
use std::path::Path;

#[test]
fn system_lang_maps_locale() {
    assert_eq!(map_locale("es-ES"), "es");
    assert_eq!(map_locale("es_MX.UTF-8"), "es");
    assert_eq!(map_locale("en-US"), "en");
    assert_eq!(map_locale("fr-FR"), "en");
    assert_eq!(map_locale(""), "en");
}

#[test]
fn app_info_reports_os_and_sidecar() {
    let tmp = tempfile::tempdir().unwrap();
    let info = app_info("0.1.0", tmp.path(), Some("/x".into()));
    assert_eq!(info.os, os_name());
    assert_eq!(info.version, "0.1.0");
    let json = serde_json::to_value(&info).unwrap();
    assert!(json.get("sidecar_available").is_some());
    let on: Vec<&str> = json["default_on"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(on.len(), 13);
    assert!(on.contains(&"cover") && !on.contains(&"apple_motion"));
    assert!(json.get("lame_available").is_none());
}

#[test]
fn plan_command_roundtrips_dtos() {
    let master = MasterDto {
        path: "/tmp/Vera Alcón - Luz.wav".into(),
        file_name: "Vera Alcón - Luz.wav".into(),
        sample_rate: 96_000,
        bits: 24,
        is_float: false,
        channels: 2,
        frames: 96_000 * 232,
        duration_secs: 232.0,
        integrated_lufs: -9.1,
        true_peak_dbtp: 1.2,
        lra_lu: 5.8,
        overs_count: 3,
        suggested_artist: Some("Vera Alcón".into()),
        suggested_title: None,
    };
    let json = serde_json::to_string(&master).unwrap();
    assert!(json.contains("\"integrated_lufs\":-9.1"));
    assert!(json.contains("\"suggested_title\":null"));
    assert_eq!(serde_json::from_str::<MasterDto>(&json).unwrap(), master);

    let plan = PlanDto {
        delivery_folder: "Vera Alcón - Luz".into(),
        jobs: vec![],
        total_files: 4,
        total_bytes: 0,
    };
    let json = serde_json::to_string(&plan).unwrap();
    assert_eq!(serde_json::from_str::<PlanDto>(&json).unwrap(), plan);
}

#[test]
fn parse_spec_splits_wav_and_out_dir() {
    let (w, o) = parse_spec("/a b/máster.wav;/out dir").unwrap();
    assert_eq!(w, Path::new("/a b/máster.wav"));
    assert_eq!(o, Path::new("/out dir"));
    assert!(parse_spec("sin-separador").is_err());
    assert!(parse_spec(";/x").is_err());
}

fn fixture() -> Option<std::path::PathBuf> {
    let dir = std::env::var_os("MASTEROUT_FIXTURES")?;
    let p = Path::new(&dir).join("m44_24.wav");
    p.is_file().then_some(p)
}

#[test]
fn smoke_runs_analyze_plan_export_and_reports() {
    let Some(wav) = fixture() else {
        eprintln!("MASTEROUT_FIXTURES no definido: se omite");
        return;
    };
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("salida ñ");
    let spec = format!("{};{}", wav.display(), out.display());
    let d = run_smoke(&spec, Lang::Es, tmp.path()).unwrap();
    assert!(d.failed.is_empty());
    assert!(d.files.len() >= 5, "audio + 4 documentos: {:?}", d.files);
    assert_eq!(run_and_report(&spec, Lang::En, tmp.path()), 0);
    let written = std::fs::read_to_string(out.join("smoke-result.json")).unwrap();
    let parsed: DeliveryDto =
        serde_json::from_str(&written).expect("smoke-result.json es un DeliveryDto");
    assert!(!parsed.files.is_empty() && parsed.failed.is_empty());
}

#[test]
fn smoke_fails_with_exit_code_1_on_missing_file() {
    let tmp = tempfile::tempdir().unwrap();
    let spec = format!(
        "{};{}",
        tmp.path().join("no.wav").display(),
        tmp.path().display()
    );
    assert!(run_smoke(&spec, Lang::En, tmp.path()).is_err());
    assert_eq!(run_and_report(&spec, Lang::En, tmp.path()), 1);
}

#[test]
fn busy_error_is_localized_with_stable_key() {
    let es = busy_error(Lang::Es);
    let en = busy_error(Lang::En);
    assert_eq!((es.key.as_str(), en.key.as_str()), ("err.busy", "err.busy"));
    assert!(es.message.contains("exportación"));
    assert!(en.message.contains("export"));
}

#[test]
fn allow_asset_accepts_only_images() {
    for ok in ["/a/Portada ñ.PNG", "C:\\x\\c.jpg", "c.jpeg"] {
        assert!(is_image_path(ok), "{ok}");
    }
    for bad in [
        "/a/b.wav",
        "/etc/passwd",
        "/a/b.png.exe",
        "noext",
        "/a/.png",
    ] {
        assert!(!is_image_path(bad), "{bad}");
    }
}

// ───────────── humo ampliado (hito 3): portada, vídeo, MP3 y sondeo de capacidades ─────────────

use mastercheck_bridge::app_support::smoke::{
    run_smoke_with, smoke_ids, SmokeOptions, SmokeReport,
};

#[test]
fn smoke_ids_depend_on_sidecar_and_cover() {
    let base = ["spotify", "apple_adm", "deezer", "distributor_std"];
    assert_eq!(smoke_ids(false, false), base);
    // sin sidecar la portada se conforma igualmente (Rust puro); nada de vídeo ni MP3
    let mut with_cover = base.to_vec();
    with_cover.push("cover");
    assert_eq!(smoke_ids(false, true), with_cover);
    // con sidecar sin portada: solo MP3 (el vídeo parte de la portada)
    let mut mp3 = base.to_vec();
    mp3.push("mp3_promo");
    assert_eq!(smoke_ids(true, false), mp3);
    // con ambos: todo lo del hito 3, incluido Apple Motion
    let all = smoke_ids(true, true);
    for id in [
        "mp3_promo",
        "cover",
        "canvas",
        "youtube_video",
        "apple_motion",
    ] {
        assert!(all.iter().any(|x| x == id), "falta {id}: {all:?}");
    }
}

#[test]
fn smoke_options_read_the_cover_from_the_environment() {
    let env = |k: &str| match k {
        "MASTERCHECK_SMOKE_COVER" => Some("/tmp/portada ñ.png".into()),
        _ => None,
    };
    let o = SmokeOptions::from_env_with(&env);
    assert_eq!(o.cover.as_deref(), Some(Path::new("/tmp/portada ñ.png")));
    assert!(o.only.is_none());
    // vacío o ausente: sin portada
    assert!(SmokeOptions::from_env_with(&|_| Some("".into()))
        .cover
        .is_none());
    assert!(SmokeOptions::from_env_with(&|_| None).cover.is_none());
}

fn has_ffmpeg() -> bool {
    masterout_core::media::discover(Path::new("/nonexistent-dir")).is_some()
}

fn write_cover(path: &Path) {
    ::image::RgbImage::from_fn(1600, 1600, |x, y| {
        ::image::Rgb([(x / 7) as u8, (y / 7) as u8, ((x + y) / 13) as u8])
    })
    .save(path)
    .unwrap();
}

#[test]
fn smoke_with_cover_produces_media_and_reports_capabilities_timing() {
    let (Some(wav), true) = (fixture(), has_ffmpeg()) else {
        eprintln!("MASTEROUT_FIXTURES o ffmpeg no disponibles: se omite");
        return;
    };
    let tmp = tempfile::tempdir().unwrap();
    let cover = tmp.path().join("portada.png");
    write_cover(&cover);
    let out = tmp.path().join("salida");
    let spec = format!("{};{}", wav.display(), out.display());
    let slow = std::env::var_os("MASTERCHECK_SLOW").is_some();
    let opts = SmokeOptions {
        cover: Some(cover),
        // El vídeo de YouTube (canción entera) y el cuadrado de Apple Motion tardan minutos con
        // x264 por software: los cubre el humo de CI; aquí solo con MASTERCHECK_SLOW.
        only: Some(
            ["spotify", "mp3_promo", "cover", "canvas"]
                .into_iter()
                .chain(slow.then_some("youtube_video"))
                .map(String::from)
                .collect(),
        ),
    };
    let report: SmokeReport = run_smoke_with(&spec, Lang::Es, tmp.path(), &opts).unwrap();
    let names: Vec<String> = report
        .delivery
        .files
        .iter()
        .map(|(p, _)| p.clone())
        .collect();
    let tags: &[&str] = if slow {
        &["[Portada]", "[Canvas]", "[YouTube]", "[MP3 320]"]
    } else {
        &["[Portada]", "[Canvas]", "[MP3 320]"]
    };
    for tag in tags {
        assert!(
            names.iter().any(|n| n.contains(tag)),
            "falta {tag}: {names:?}"
        );
    }
    let caps = report.capabilities.expect("hay sidecar: hay sondeo");
    assert!(caps.h264 && caps.millis < 60_000, "{caps:?}");
    assert!(report.disabled.is_empty(), "{:?}", report.disabled);

    // smoke-result.json: la entrega más el bloque `smoke` con los tiempos
    let json = report.to_json();
    assert_eq!(
        json["smoke"]["capabilities"]["millis"].as_u64(),
        Some(caps.millis)
    );
    assert_eq!(json["smoke"]["capabilities"]["h264"], true);
    assert!(json["smoke"]["sidecar"]["ffmpeg"].is_string());
    // sigue siendo un DeliveryDto válido para quien lo lea como tal
    let d: DeliveryDto = serde_json::from_value(json).unwrap();
    assert!(d.failed.is_empty());
}

#[test]
fn smoke_without_sidecar_reports_no_capabilities() {
    let Some(wav) = fixture() else { return };
    // sin ffmpeg ni cover el humo clásico sigue valiendo; el bloque dice que no hubo sondeo
    let tmp = tempfile::tempdir().unwrap();
    let spec = format!("{};{}", wav.display(), tmp.path().join("o").display());
    let r = run_smoke_with(
        &spec,
        Lang::En,
        tmp.path(),
        &SmokeOptions {
            cover: None,
            only: Some(vec!["spotify".into()]),
        },
    );
    let r = r.unwrap();
    let j = r.to_json();
    if has_ffmpeg() {
        assert!(j["smoke"]["capabilities"]["millis"].is_u64());
    } else {
        assert!(j["smoke"]["capabilities"].is_null());
    }
}

#[test]
fn app_info_announces_the_sidecar_release_only_when_it_is_there() {
    use mastercheck_bridge::app_support::SIDECAR_RELEASE;
    // misma fuente que usa el CI para descargarlo (app/scripts/sidecar-tag.txt)
    let tag = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../app/scripts/sidecar-tag.txt"),
    )
    .unwrap();
    assert_eq!(SIDECAR_RELEASE, tag.trim());
    let tmp = tempfile::tempdir().unwrap();
    let info = app_info("0.1.0", tmp.path(), None);
    // ni junto al ejecutable ni en ninguna parte: sin release (con PATH real puede haber ffmpeg
    // y entonces `sidecar_available` es true, pero la release sigue siendo None)
    assert_eq!(info.sidecar_release, None);
    // junto al ejecutable (lo que hace externalBin): se anuncia con seguridad
    let name = if cfg!(windows) {
        ["ffmpeg.exe", "ffprobe.exe"]
    } else {
        ["ffmpeg", "ffprobe"]
    };
    for n in name {
        std::fs::write(tmp.path().join(n), b"x").unwrap();
    }
    assert_eq!(
        app_info("0.1.0", tmp.path(), None)
            .sidecar_release
            .as_deref(),
        Some(SIDECAR_RELEASE)
    );
}

#[test]
fn sidecar_release_is_none_when_ffmpeg_comes_from_env_or_path() {
    use mastercheck_bridge::app_support::{app_info_with, SIDECAR_RELEASE};
    let exe_dir = tempfile::tempdir().unwrap();
    let other = tempfile::tempdir().unwrap();
    let (ff, fp) = if cfg!(windows) {
        ("ffmpeg.exe", "ffprobe.exe")
    } else {
        ("ffmpeg", "ffprobe")
    };
    for n in [ff, fp] {
        std::fs::write(other.path().join(n), b"x").unwrap();
    }
    let (eff, efp) = (other.path().join(ff), other.path().join(fp));
    let env = |k: &str| match k {
        "MASTERCHECK_FFMPEG" => Some(eff.clone().into_os_string()),
        "MASTERCHECK_FFPROBE" => Some(efp.clone().into_os_string()),
        _ => None,
    };
    // variable de entorno: hay componente, pero no es el empaquetado -> no se anuncia la release
    let info = app_info_with("0.1.0", exe_dir.path(), None, &env, None);
    assert!(info.sidecar_available);
    assert_eq!(info.sidecar_release, None);
    // PATH: igual
    let path = std::env::join_paths([other.path()]).ok();
    let info = app_info_with("0.1.0", exe_dir.path(), None, &|_| None, path.clone());
    assert!(info.sidecar_available);
    assert_eq!(info.sidecar_release, None);
    // junto al ejecutable gana sobre entorno y PATH: se anuncia
    for n in [ff, fp] {
        std::fs::write(exe_dir.path().join(n), b"x").unwrap();
    }
    let info = app_info_with("0.1.0", exe_dir.path(), None, &env, path);
    assert_eq!(info.sidecar_release.as_deref(), Some(SIDECAR_RELEASE));
}

// ---------------------------------------------------------------- tarea 7 (hito 3)

mod reveal {
    use mastercheck_bridge::app_support::{check_reveal, last_delivery};
    use mastercheck_bridge::dto::DeliveryDto;
    use masterout_core::i18n::Lang;
    use std::path::{Path, PathBuf};

    /// `entrega/{a.wav, sub/b.wav}` y `entrega-2/x.wav` y `otra/y.wav`.
    fn tree() -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let d = tmp.path().join("entrega");
        for f in [
            "entrega/a.wav",
            "entrega/sub/b.wav",
            "entrega-2/x.wav",
            "otra/y.wav",
        ] {
            let p = tmp.path().join(f);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, b"x").unwrap();
        }
        (tmp, d)
    }

    fn allowed(p: &Path, last: Option<&Path>) -> bool {
        check_reveal(&p.to_string_lossy(), last, Lang::Es).is_ok()
    }

    #[test]
    fn only_the_last_delivery_and_what_is_inside_it_can_be_revealed() {
        let (tmp, d) = tree();
        assert!(allowed(&d, Some(&d)));
        assert!(allowed(&d.join("a.wav"), Some(&d)));
        assert!(allowed(&d.join("sub").join("b.wav"), Some(&d)));
        assert!(!allowed(&tmp.path().join("otra"), Some(&d)));
        assert!(!allowed(&tmp.path().join("otra/y.wav"), Some(&d)));
        // mismo prefijo de texto, otra carpeta
        assert!(!allowed(&tmp.path().join("entrega-2"), Some(&d)));
        // el padre y la raíz tampoco
        assert!(!allowed(tmp.path(), Some(&d)));
        assert!(!allowed(Path::new("/"), Some(&d)));
        // salir con `..`
        assert!(!allowed(&d.join("..").join("otra"), Some(&d)));
        // sin entrega conocida, nada; ruta vacía o inexistente, nada
        assert!(!allowed(&d, None));
        assert!(!allowed(Path::new(""), Some(&d)));
        assert!(!allowed(&d.join("no-existe.wav"), Some(&d)));
        assert!(!allowed(&d, Some(Path::new(""))));
    }

    #[cfg(unix)]
    #[test]
    fn a_symlink_inside_the_delivery_cannot_point_outside() {
        let (tmp, d) = tree();
        std::os::unix::fs::symlink(tmp.path().join("otra"), d.join("enlace")).unwrap();
        assert!(!allowed(&d.join("enlace"), Some(&d)));
        assert!(!allowed(&d.join("enlace").join("y.wav"), Some(&d)));
    }

    #[test]
    fn a_refusal_is_err_internal_in_the_requested_language() {
        let (tmp, d) = tree();
        let e = check_reveal(
            &tmp.path().join("otra").to_string_lossy(),
            Some(&d),
            Lang::En,
        )
        .unwrap_err();
        assert_eq!(e.key, "err.internal");
        assert!(e.message.contains("nexpected"), "{}", e.message);
        assert!(
            e.detail
                .is_some_and(|x| !x.contains(&*tmp.path().to_string_lossy())),
            "el detalle no repite la ruta pedida"
        );
    }

    #[test]
    fn the_last_delivery_is_remembered_from_the_dto_unless_it_was_cancelled_empty() {
        let mk = |folder: &str| DeliveryDto {
            folder: folder.into(),
            cancelled: folder.is_empty(),
            files: vec![],
            failed: vec![],
            failed_keys: vec![],
            failed_details: vec![],
            verdicts: vec![],
            elapsed_ms: 0,
        };
        assert_eq!(
            last_delivery(&mk("/x/entrega")),
            Some(PathBuf::from("/x/entrega"))
        );
        assert_eq!(last_delivery(&mk("")), None);
    }
}

mod panic_log {
    use mastercheck_bridge::app_support::panic_log::{format_entry, log_dir_for};

    #[test]
    fn log_dir_per_platform_uses_the_user_log_locations() {
        let env = |k: &str| match k {
            "HOME" => Some("/Users/ana".to_string()),
            "LOCALAPPDATA" => Some("C:\\Users\\ana\\AppData\\Local".to_string()),
            "XDG_STATE_HOME" => Some("/home/ana/.state".to_string()),
            _ => None,
        };
        assert_eq!(
            log_dir_for("mac", &env).unwrap(),
            std::path::PathBuf::from("/Users/ana/Library/Logs/master check")
        );
        let win = log_dir_for("win", &env).unwrap();
        assert!(
            win.starts_with("C:\\Users\\ana\\AppData\\Local") && win.ends_with("logs"),
            "{win:?}"
        );
        assert_eq!(
            log_dir_for("linux", &env).unwrap(),
            std::path::PathBuf::from("/home/ana/.state/master check")
        );
        let no_xdg = |k: &str| (k == "HOME").then(|| "/home/ana".to_string());
        assert_eq!(
            log_dir_for("linux", &no_xdg).unwrap(),
            std::path::PathBuf::from("/home/ana/.local/state/master check")
        );
        assert!(log_dir_for("mac", &|_| None).is_none());
    }

    #[test]
    fn an_entry_has_time_place_and_message_on_one_line() {
        let e = format_entry(1_762_300_800, Some(("src/export.rs", 42)), "algo\nfalló");
        assert_eq!(e, "1762300800 src/export.rs:42 algo falló\n");
        assert_eq!(format_entry(1, None, "x"), "1 ? x\n");
    }

    #[test]
    fn a_leaked_temp_is_logged_with_its_path_and_error() {
        let e = mastercheck_bridge::app_support::panic_log::leak_entry(
            5,
            std::path::Path::new("/x/a.part"),
            "acceso denegado",
        );
        assert_eq!(e, "5 ? temporal sin borrar /x/a.part: acceso denegado\n");
    }
}
