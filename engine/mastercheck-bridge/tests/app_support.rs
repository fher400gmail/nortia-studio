use mastercheck_bridge::app_support::lame::{path_with_dir_first, resolve_lame_in};
use mastercheck_bridge::app_support::smoke::{parse_spec, run_and_report, run_smoke};
use mastercheck_bridge::app_support::{app_info, map_locale, os_name};
use mastercheck_bridge::dto::{MasterDto, PlanDto};
use masterout_core::i18n::Lang;
use std::ffi::OsString;
use std::path::Path;

fn fake_lame(dir: &Path) -> std::path::PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let p = dir.join(if cfg!(windows) { "lame.exe" } else { "lame" });
    std::fs::write(&p, b"#!/bin/sh\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    p
}

#[test]
fn resolve_lame_path_prefers_bundle_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let exe_dir = tmp.path().join("Mi App ñ");
    let bundled = fake_lame(&exe_dir);
    let other = fake_lame(&tmp.path().join("otra"));
    let path_var: OsString = std::env::join_paths([tmp.path().join("otra")]).unwrap();
    assert_eq!(resolve_lame_in(&exe_dir, Some(&path_var)), Some(bundled));
    assert!(other.is_file());
}

#[test]
fn resolve_lame_path_finds_macos_resources_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let exe_dir = tmp.path().join("master check.app/Contents/MacOS");
    std::fs::create_dir_all(&exe_dir).unwrap();
    let res = fake_lame(&tmp.path().join("master check.app/Contents/Resources"));
    assert_eq!(resolve_lame_in(&exe_dir, None), Some(res));
}

#[test]
fn resolve_lame_path_falls_back_to_path_env() {
    let tmp = tempfile::tempdir().unwrap();
    let exe_dir = tmp.path().join("vacio");
    std::fs::create_dir_all(&exe_dir).unwrap();
    let in_path = fake_lame(&tmp.path().join("bin"));
    let path_var = std::env::join_paths([tmp.path().join("nada"), tmp.path().join("bin")]).unwrap();
    assert_eq!(resolve_lame_in(&exe_dir, Some(&path_var)), Some(in_path));
    assert_eq!(resolve_lame_in(&exe_dir, None), None);
}

#[cfg(unix)]
#[test]
fn resolve_lame_path_ignores_non_executable() {
    let tmp = tempfile::tempdir().unwrap();
    let p = fake_lame(tmp.path());
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(resolve_lame_in(tmp.path(), None), None);
}

#[test]
fn path_with_dir_first_puts_dir_first() {
    let joined = path_with_dir_first(
        Path::new("/a/b"),
        Some(std::env::join_paths(["/x", "/y"]).unwrap()),
    );
    let parts: Vec<_> = std::env::split_paths(&joined).collect();
    assert_eq!(parts, [Path::new("/a/b"), Path::new("/x"), Path::new("/y")]);
}

#[test]
fn system_lang_maps_locale() {
    assert_eq!(map_locale("es-ES"), "es");
    assert_eq!(map_locale("es_MX.UTF-8"), "es");
    assert_eq!(map_locale("en-US"), "en");
    assert_eq!(map_locale("fr-FR"), "en");
    assert_eq!(map_locale(""), "en");
}

#[test]
fn app_info_reports_os_and_lame() {
    let tmp = tempfile::tempdir().unwrap();
    let info = app_info("0.1.0", tmp.path(), Some("/x".into()));
    assert_eq!(info.os, os_name());
    assert_eq!(info.version, "0.1.0");
    let json = serde_json::to_value(&info).unwrap();
    assert!(json.get("lame_available").is_some());
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
    assert!(out.join("smoke-result.json").is_file());
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
