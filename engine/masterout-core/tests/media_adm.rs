use masterout_core::i18n::{t, Lang};
use masterout_core::loudness::{LoudnessReport, Verdict};
use masterout_core::media::adm::*;
use masterout_core::plan::{self, MasterInfo, ReleaseMeta};
use masterout_core::reports::*;
use std::path::Path;

fn strs(v: &[&str]) -> Vec<std::ffi::OsString> {
    v.iter().map(std::ffi::OsString::from).collect()
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
                "LEF32",
                "-f",
                "WAVE"
            ])
        );
        // afclip mide el WAV decodificado (float: el exceso entre muestras no se recorta)
        assert_eq!(c[3], strs(&["afclip", "/w/decode.wav"]));
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

const HEAD: &str = "afclip : \"03_Tema.wav\"    2 ch,  48000 Hz, 'lpcm' (0x0000000C) 24-bit little-endian signed integer\n";
const TABLE: &str = "CLIPPED SAMPLES:\n    SECONDS          SAMPLE    CHAN    VALUE        DECIBELS\n   12.500000       600000.50    1    -1.004000     0.034700\n   20.250000       972000.50    2     1.010000     0.086000\n\n";

fn total(ch: &str, on: u32, inter: u32) -> String {
    format!("   total clipped samples    {ch:<6} on-sample:        {on}    inter-sample:        {inter}\n")
}

#[test]
fn parser_no_clips() {
    let out = format!("{HEAD} -- no samples clipped --\n");
    assert_eq!(parse_afclip(&out), Some(0));
}

#[test]
fn parser_sums_all_totals_on_and_inter_sample() {
    let h = format!("{HEAD}{TABLE}");
    let two = format!("{h}{}{}", total("Left", 0, 6), total("Right", 0, 1));
    assert_eq!(parse_afclip(&two), Some(7));
    // el último canal limpio no puede ocultar los clips del primero
    let left_only = format!("{h}{}{}", total("Left", 0, 6), total("Right", 0, 0));
    assert_eq!(parse_afclip(&left_only), Some(6));
    let on = format!("{h}{}{}", total("Left", 4, 1), total("Right", 0, 0));
    assert_eq!(parse_afclip(&on), Some(5));
}

#[test]
fn parser_handles_three_channels_and_mono() {
    let h = format!("{HEAD}{TABLE}");
    let three = format!(
        "{h}{}{}{}",
        total("Left", 1, 2),
        total("Right", 0, 0),
        total("Center", 0, 4)
    );
    assert_eq!(parse_afclip(&three), Some(7));
    assert_eq!(
        parse_afclip(&format!("{h}{}", total("Mono", 0, 3))),
        Some(3)
    );
}

#[test]
fn parser_counts_table_rows_without_totals() {
    assert_eq!(parse_afclip(&format!("{HEAD}{TABLE}")), Some(2));
}

#[test]
fn parser_never_guesses() {
    assert_eq!(parse_afclip(""), None);
    assert_eq!(parse_afclip("Segmentation fault"), None);
    assert_eq!(parse_afclip("0 clipped samples"), None);
    assert_eq!(parse_afclip("No clipping detected."), None);
    assert_eq!(parse_afclip("usage: afclip file 12 34"), None);
    // cabecera sin filas ni totales
    assert_eq!(parse_afclip(&format!("{HEAD}CLIPPED SAMPLES:\n")), None);
    // totales sin cifras
    assert_eq!(
        parse_afclip(&format!("{HEAD}{TABLE}   total clipped samples    Left\n")),
        None
    );
    // "sin clips" contradictorio con una cabecera de tabla: no es un cero
    assert_eq!(
        parse_afclip(&format!(
            "{HEAD}-- no samples clipped --\nCLIPPED SAMPLES:\n"
        )),
        None
    );
}

#[cfg(unix)]
#[test]
fn commands_keep_non_utf8_paths_byte_for_byte() {
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    let raw = std::ffi::OsString::from_vec(b"/m/ma\xffster.wav".to_vec());
    let c = build_adm_commands(Path::new(&raw), Path::new("/w"), 48_000);
    assert_eq!(c[0][1].as_bytes(), b"/m/ma\xffster.wav");
}

#[test]
fn singular_plural_and_lowercase_reason_in_the_report_line() {
    let one = rep(AdmStatus::Clipping, 1);
    assert!(report(Some(&one), Lang::Es)
        .contains("Apple Digital Masters: 1 muestra con clipping en el AAC"));
    assert!(
        report(Some(&one), Lang::En).contains("Apple Digital Masters: 1 clipped sample in the AAC")
    );
    let na = rep(
        AdmStatus::Unavailable {
            reason_key: "reason.adm_mac_only".into(),
        },
        0,
    );
    assert!(report(Some(&na), Lang::Es).contains("no disponible: la validación"));
    assert!(report(Some(&na), Lang::En).contains("not available: validation"));
}

#[test]
fn missing_tools_reason_links_to_apple() {
    for lang in [Lang::Es, Lang::En] {
        assert!(t(lang, "reason.adm_tools_missing")
            .contains("https://www.apple.com/apple-music/apple-digital-masters/"));
        assert!(!t(lang, "err.adm_log").starts_with('⟦'));
    }
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
        let r = run(
            "printf 'CLIPPED SAMPLES:\\n   total clipped samples    Left   on-sample:   4    inter-sample:   8\\n'",
            "exit 0",
        );
        assert_eq!(r.status, AdmStatus::Clipping);
        assert_eq!(r.clipped_samples, 12);
        assert!(r.log.contains("--soundcheck-generate"));
        let r = run("echo ' -- no samples clipped --'", "exit 0");
        assert_eq!(r.status, AdmStatus::Ok);
        assert!(r.aac_path.is_some());
    }

    #[test]
    fn afclip_runs_on_the_decoded_wav_and_decode_failure_is_fatal() {
        let ok = run(
            "case \"$1\" in *decode.wav) [ -f \"$1\" ] && echo ' -- no samples clipped --';; esac",
            "touch \"$2\"",
        );
        assert_eq!(ok.status, AdmStatus::Ok, "{}", ok.log);
        let r = run(
            "echo ' -- no samples clipped --'",
            "case \"$*\" in *\"-d LEF32 \"*) exit 1;; esac; touch \"$2\"",
        );
        assert_eq!(
            r.status,
            AdmStatus::Unavailable {
                reason_key: "reason.adm_failed".into()
            }
        );
    }

    #[test]
    fn rate_comes_from_the_header_and_unreadable_wav_is_a_failure() {
        let d = tempfile::tempdir().unwrap();
        let conv = script(d.path(), "afconvert", "touch \"$2\"");
        let clip = script(d.path(), "afclip", "echo ' -- no samples clipped --'");
        let wav96 = d.path().join("m96.wav");
        let ch = vec![0i32; 9600];
        masterout_core::wav::write(
            &wav96,
            &[ch.clone(), ch],
            96_000,
            &masterout_core::wav::WavWriteOptions {
                bits: 24,
                metadata: None,
            },
        )
        .unwrap();
        let r = validate_with_tools(&conv, &clip, &wav96, &d.path().join("w"), &|| false);
        assert_eq!(r.status, AdmStatus::Ok, "{}", r.log);
        assert!(
            r.log.contains("LEF32@48000") && r.log.contains("bats"),
            "{}",
            r.log
        );
        // cabecera ilegible: no se adivina 48 kHz
        let bad = d.path().join("bad.wav");
        std::fs::write(&bad, b"no es un wav").unwrap();
        let r = validate_with_tools(&conv, &clip, &bad, &d.path().join("w2"), &|| false);
        assert_eq!(
            r.status,
            AdmStatus::Unavailable {
                reason_key: "reason.adm_failed".into()
            }
        );
        assert!(!r.log.is_empty());
    }

    #[test]
    fn tool_arguments_are_absolute_paths() {
        let d = tempfile::tempdir().unwrap();
        let wav = d.path().join("m.wav");
        adm_smoke_case(-1.0, &wav).unwrap();
        let conv = script(d.path(), "afconvert", "touch \"$2\"");
        let clip = script(d.path(), "afclip", "echo ' -- no samples clipped --'");
        let odd = d.path().join(".").join("m.wav");
        let r = validate_with_tools(&conv, &clip, &odd, &d.path().join("w"), &|| false);
        assert_eq!(r.status, AdmStatus::Ok, "{}", r.log);
        assert!(!r.log.contains("/./"), "{}", r.log);
        for line in r.log.lines().filter(|l| l.starts_with("$ ")) {
            assert!(line.split(' ').nth(2).unwrap().starts_with('/'), "{line}");
        }
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
    fn afclip_exit_code_nonzero_with_clips_is_still_parsed() {
        // afclip puede salir con código distinto de 0 justo cuando encuentra clips
        let r = run(
            "printf 'CLIPPED SAMPLES:\\n   total clipped samples    Left   on-sample:   4    inter-sample:   8\\n'; exit 1",
            "exit 0",
        );
        assert_eq!(r.status, AdmStatus::Clipping);
        assert_eq!(r.clipped_samples, 12);
        // sin salida reconocible y con código 1: fallo del paso
        let key = |r: AdmReport| match r.status {
            AdmStatus::Unavailable { reason_key } => reason_key,
            s => panic!("{s:?}"),
        };
        assert_eq!(key(run("echo roto; exit 1", "exit 0")), "reason.adm_failed");
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
        t(Lang::Es, "reason.adm_mac_only").replacen('L', "l", 1)
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
            "report.adm_clipping",
            "report.adm_clipping_one",
            "report.adm_unavailable",
            "err.adm_log",
        ] {
            assert!(!t(lang, k).starts_with('⟦'), "{k}");
        }
    }
}

/// Primera ejecución real de todo el flujo (afconvert -> CAF -> AAC -> decodificación ->
/// afclip) con las herramientas de Apple. Solo en macOS (el job `engine-tests` de macos-15);
/// si `afconvert` o `afclip` no están, se salta con un mensaje. Si el formato de `afclip`
/// difiere del esperado, el fallo trae el log crudo de cada paso.
#[cfg(target_os = "macos")]
mod real_tools {
    use super::*;

    fn tools_present() -> bool {
        let found = |n: &str| {
            ["/usr/bin", "/usr/local/bin"]
                .iter()
                .any(|d| Path::new(d).join(n).is_file())
                || std::env::var_os("PATH")
                    .is_some_and(|p| std::env::split_paths(&p).any(|d| d.join(n).is_file()))
        };
        let ok = found("afconvert") && found("afclip");
        if !ok {
            // CI define MASTEROUT_REQUIRE_ADM solo si instaló las herramientas: entonces su
            // ausencia es un fallo, no un SKIP silencioso.
            assert!(
                std::env::var_os("MASTEROUT_REQUIRE_ADM").is_none(),
                "MASTEROUT_REQUIRE_ADM definida pero faltan afconvert/afclip"
            );
            eprintln!("SKIP: afconvert/afclip no están instalados en este macOS");
        }
        ok
    }

    fn run(dbtp: f64) -> AdmReport {
        let d = tempfile::tempdir().unwrap();
        let wav = d.path().join("synthetic.wav");
        adm_smoke_case(dbtp, &wav).unwrap();
        let r = validate_adm(&wav, &d.path().join("adm"), &|| false);
        // CI recoge el log crudo (órdenes y salida de afclip) como artefacto
        if let Some(dir) = std::env::var_os("MASTEROUT_ADM_LOG_DIR") {
            let dir = Path::new(&dir);
            let _ = std::fs::create_dir_all(dir);
            let _ = std::fs::write(dir.join(format!("afclip-raw-{dbtp:+.0}dbtp.txt")), &r.log);
        }
        r
    }

    #[test]
    fn minus_one_dbtp_master_has_no_clipping() {
        if !tools_present() {
            return;
        }
        let r = run(-1.0);
        assert_eq!(r.status, AdmStatus::Ok, "log crudo:\n{}", r.log);
        assert_eq!(r.clipped_samples, 0, "log crudo:\n{}", r.log);
    }

    #[test]
    fn plus_one_dbtp_master_clips() {
        if !tools_present() {
            return;
        }
        let r = run(1.0);
        assert_eq!(r.status, AdmStatus::Clipping, "log crudo:\n{}", r.log);
        assert!(r.clipped_samples > 0, "log crudo:\n{}", r.log);
    }
}
