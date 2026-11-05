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
