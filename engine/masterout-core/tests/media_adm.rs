use masterout_core::i18n::{t, Lang};
use masterout_core::loudness::{LoudnessReport, Verdict};
use masterout_core::media::adm::*;
use masterout_core::plan::{self, MasterInfo, ReleaseMeta};
use masterout_core::reports::*;
use std::path::Path;

fn strs(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

#[test]
fn golden_commands_44k_and_48k_have_no_rate_conversion() {
    for rate in [44_100, 48_000] {
        let c = build_adm_commands(Path::new("/m/master.wav"), Path::new("/w"), rate);
        assert_eq!(c.len(), 4);
        assert_eq!(
            c[0],
            strs(&[
                "afconvert",
                "/m/master.wav",
                "/w/intermediate.caf",
                "-d",
                "0",
                "-f",
                "caff",
                "--soundcheck-generate"
            ])
        );
        assert_eq!(
            c[1],
            strs(&[
                "afconvert",
                "/w/intermediate.caf",
                "/w/final.m4a",
                "-d",
                "aac",
                "-f",
                "m4af",
                "-u",
                "pgcm",
                "2",
                "--soundcheck-read",
                "-b",
                "256000",
                "-q",
                "127",
                "-s",
                "2"
            ])
        );
        assert_eq!(
            c[2],
            strs(&[
                "afconvert",
                "/w/final.m4a",
                "/w/decode.wav",
                "-d",
                "LEI24",
                "-f",
                "WAVE"
            ])
        );
        assert_eq!(c[3], strs(&["afclip", "/w/final.m4a"]));
    }
}

#[test]
fn golden_commands_high_rates_use_bats_and_target_rate_family() {
    let c96 = build_adm_commands(Path::new("/m/a.wav"), Path::new("/w"), 96_000);
    assert_eq!(
        c96[0],
        strs(&[
            "afconvert",
            "/m/a.wav",
            "/w/intermediate.caf",
            "-d",
            "LEF32@48000",
            "-f",
            "caff",
            "--soundcheck-generate",
            "--src-complexity",
            "bats",
            "-r",
            "127"
        ])
    );
    for (rate, tag) in [
        (88_200, "LEF32@44100"),
        (176_400, "LEF32@44100"),
        (192_000, "LEF32@48000"),
    ] {
        let c = build_adm_commands(Path::new("/m/a.wav"), Path::new("/w"), rate);
        assert_eq!(c[0][4], tag, "{rate}");
    }
    // el resto de pasos no cambia
    let c48 = build_adm_commands(Path::new("/m/a.wav"), Path::new("/w"), 48_000);
    assert_eq!(c96[1..], c48[1..]);
}

#[test]
fn parser_reads_zero_and_n_clips() {
    assert_eq!(parse_afclip("0 clipped samples\n"), Some(0));
    assert_eq!(parse_afclip("No clipping detected.\n"), Some(0));
    assert_eq!(parse_afclip("Clipped samples: 0"), Some(0));
    assert_eq!(parse_afclip("Found 1,234 clipped samples"), Some(1234));
    assert_eq!(
        parse_afclip("afclip /tmp/masterout-123-4/final.m4a\n17 clipped samples\n"),
        Some(17)
    );
    // por canal: se suman; con total manda el total
    assert_eq!(
        parse_afclip("channel 0: 3 clipped samples\nchannel 1: 4 clipped samples\n"),
        Some(7)
    );
    assert_eq!(
        parse_afclip("ch 0: 3 clipped samples\nch 1: 4 clipped samples\ntotal clipped: 7\n"),
        Some(7)
    );
}

#[test]
fn parser_rejects_garbage() {
    assert_eq!(parse_afclip(""), None);
    assert_eq!(parse_afclip("Segmentation fault"), None);
    assert_eq!(parse_afclip("usage: afclip file 12 34"), None);
    // la ruta con dígitos no cuenta
    assert_eq!(parse_afclip("afclip /tmp/clip-99/final.m4a"), None);
}

#[test]
fn unavailable_off_macos() {
    if cfg!(target_os = "macos") {
        return;
    }
    let r = validate_adm(Path::new("/nope.wav"), Path::new("/nope"), &|| false);
    assert_eq!(
        r.status,
        AdmStatus::Unavailable {
            reason_key: "reason.adm_mac_only".into()
        }
    );
    assert!(r.aac_path.is_none());
}

#[cfg(unix)]
mod fake_tools {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn script(dir: &Path, name: &str, body: &str) -> std::path::PathBuf {
        let p = dir.join(name);
        std::fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
        p
    }

    fn run(clip_body: &str, convert_body: &str) -> AdmReport {
        let d = tempfile::tempdir().unwrap();
        let wav = d.path().join("m.wav");
        adm_smoke_case(-1.0, &wav).unwrap();
        let conv = script(d.path(), "afconvert", convert_body);
        let clip = script(d.path(), "afclip", clip_body);
        validate_with_tools(&conv, &clip, &wav, &d.path().join("w"), &|| false)
    }

    #[test]
    fn runs_the_flow_and_reports_clipping() {
        let r = run("echo '12 clipped samples'", "exit 0");
        assert_eq!(r.status, AdmStatus::Clipping);
        assert_eq!(r.clipped_samples, 12);
        assert!(r.log.contains("--soundcheck-generate"));
        let r = run("echo '0 clipped samples'", "exit 0");
        assert_eq!(r.status, AdmStatus::Ok);
        assert!(r.aac_path.is_some());
    }

    #[test]
    fn failures_map_to_reason_keys() {
        let key = |r: AdmReport| match r.status {
            AdmStatus::Unavailable { reason_key } => reason_key,
            s => panic!("{s:?}"),
        };
        assert_eq!(key(run("echo hola", "exit 0")), "reason.adm_parse");
        assert_eq!(key(run("echo '0 clipped'", "exit 1")), "reason.adm_failed");
    }

    #[test]
    fn cancel_stops_the_flow() {
        let d = tempfile::tempdir().unwrap();
        let wav = d.path().join("m.wav");
        adm_smoke_case(-1.0, &wav).unwrap();
        let conv = script(d.path(), "afconvert", "sleep 30");
        let clip = script(d.path(), "afclip", "exit 0");
        let r = validate_with_tools(&conv, &clip, &wav, &d.path().join("w"), &|| true);
        assert_eq!(
            r.status,
            AdmStatus::Unavailable {
                reason_key: "reason.adm_cancelled".into()
            }
        );
    }
}

fn master() -> MasterInfo {
    MasterInfo {
        sample_rate: 48_000,
        bits: 24,
        is_float: false,
        channels: 2,
        frames: 48_000 * 10,
        loudness: LoudnessReport {
            integrated_lufs: -16.0,
            lra_lu: 4.0,
            true_peak_dbtp: -1.5,
            sample_peak_dbfs: -2.0,
            momentary_max_lufs: -12.0,
            short_term_max_lufs: -13.0,
            overs_count: 0,
        },
    }
}

fn report(adm: Option<&AdmReport>, lang: Lang) -> String {
    let (m, ms) = (ReleaseMeta::default(), master());
    let jobs = plan::resolve(&ms, &m, &["apple_adm"], &Default::default());
    let s = DeliverySummary {
        meta: &m,
        master: &ms,
        jobs: &jobs,
        created_at: "2026-11-05T12:00:00".into(),
        app_version: "0.1.0",
    };
    loudness_report_with_adm(lang, &s, adm)
}

fn rep(status: AdmStatus, n: u64) -> AdmReport {
    AdmReport {
        status,
        clipped_samples: n,
        aac_path: None,
        log: String::new(),
    }
}

#[test]
fn report_line_renders_in_es_and_en() {
    let clip = rep(AdmStatus::Clipping, 42);
    let es = report(Some(&clip), Lang::Es);
    assert!(es.contains("Apple Digital Masters: 42 muestras con clipping en el AAC"));
    assert!(es.contains(&t(Lang::Es, "adm_clipping")));
    let en = report(Some(&clip), Lang::En);
    assert!(en.contains("Apple Digital Masters: 42 clipped samples in the AAC"));
    let na = rep(
        AdmStatus::Unavailable {
            reason_key: "reason.adm_mac_only".into(),
        },
        0,
    );
    let es = report(Some(&na), Lang::Es);
    assert!(es.contains(&format!(
        "Apple Digital Masters: no disponible: {}",
        t(Lang::Es, "reason.adm_mac_only")
    )));
    assert!(!report(None, Lang::Es).contains("Apple Digital Masters:"));
    // sin ADM el informe es idéntico al de siempre
    let ok = report(Some(&rep(AdmStatus::Ok, 0)), Lang::Es);
    assert!(ok.contains("0 muestras con clipping"));
    assert!(!ok.contains(&t(Lang::Es, "adm_clipping")));
}

#[test]
fn verdict_gets_adm_clipping_warning_only_for_apple() {
    let clip = rep(AdmStatus::Clipping, 1);
    assert_eq!(
        verdict_with_adm("apple_adm", &Verdict::Ok, Some(&clip)),
        Verdict::Warn("adm_clipping".into())
    );
    assert_eq!(
        verdict_with_adm("spotify", &Verdict::Ok, Some(&clip)),
        Verdict::Ok
    );
    assert_eq!(
        verdict_with_adm("apple_adm", &Verdict::Fail("x".into()), Some(&clip)),
        Verdict::Fail("x".into())
    );
    assert_eq!(
        verdict_with_adm("apple_adm", &Verdict::Ok, None),
        Verdict::Ok
    );
}

#[test]
fn i18n_keys_exist_in_both_languages() {
    for lang in [Lang::Es, Lang::En] {
        for k in [
            "reason.adm_mac_only",
            "reason.adm_tools_missing",
            "reason.adm_parse",
            "reason.adm_failed",
            "reason.adm_cancelled",
            "adm_clipping",
        ] {
            assert!(!t(lang, k).starts_with('⟦'), "{k}");
        }
    }
}
