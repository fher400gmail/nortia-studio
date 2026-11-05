use masterout_core::i18n::{self, t, Lang};
use masterout_core::loudness::LoudnessReport;
use masterout_core::plan::{self, MasterInfo, ReleaseMeta, Resolved};
use masterout_core::reports::*;

fn master(rate: u32, bits: u16) -> MasterInfo {
    MasterInfo {
        sample_rate: rate,
        bits,
        is_float: false,
        channels: 2,
        frames: u64::from(rate) * 200,
        loudness: LoudnessReport {
            integrated_lufs: -9.1,
            lra_lu: 4.2,
            true_peak_dbtp: 0.4,
            sample_peak_dbfs: -0.3,
            momentary_max_lufs: -6.0,
            short_term_max_lufs: -7.0,
            overs_count: 3,
        },
    }
}

fn meta() -> ReleaseMeta {
    ReleaseMeta {
        artist: "Vera Alcón".into(),
        title: "Luz de Noviembre".into(),
        ..Default::default()
    }
}

fn summary<'a>(
    m: &'a ReleaseMeta,
    ms: &'a MasterInfo,
    jobs: &'a [Resolved],
) -> DeliverySummary<'a> {
    DeliverySummary {
        meta: m,
        master: ms,
        jobs,
        created_at: "2026-11-05T12:00:00".into(),
        app_version: "0.1.0",
    }
}

#[test]
fn es_and_en_have_identical_key_sets() {
    assert_eq!(i18n::keys(Lang::Es), i18n::keys(Lang::En));
}

#[test]
fn missing_key_is_visible_not_panic() {
    assert_eq!(t(Lang::Es, "nope"), "⟦nope⟧");
    assert_eq!(t(Lang::En, "nope"), "⟦nope⟧");
}

#[test]
fn lang_from_tag() {
    assert_eq!(Lang::from_tag("es-MX"), Lang::Es);
    assert_eq!(Lang::from_tag("es-ES"), Lang::Es);
    assert_eq!(Lang::from_tag("EN_us"), Lang::En);
    assert_eq!(Lang::from_tag("fr"), Lang::En);
}

#[test]
fn readme_lists_each_active_job_once_in_order() {
    let (m, ms) = (meta(), master(96000, 24));
    let jobs = plan::resolve(&ms, &m, &["mp3_promo", "deezer", "spotify"]);
    for lang in [Lang::Es, Lang::En] {
        let r = readme(lang, &summary(&m, &ms, &jobs));
        assert!(r.starts_with("Vera Alcón - Luz de Noviembre\n"));
        let pos: Vec<usize> = ["01 Spotify/", "05 Deezer/", "08 Promo/"]
            .iter()
            .map(|k| {
                assert_eq!(r.matches(k).count(), 1, "{k}");
                r.find(k).unwrap()
            })
            .collect();
        assert!(pos[0] < pos[1] && pos[1] < pos[2]);
        assert!(r.contains("01 Spotify/  →  Vera Alcón - Luz de Noviembre [Spotify] 24bit-96kHz.flac  (24 bit · 96 kHz · FLAC)"));
        assert!(r.contains(&t(lang, "readme.master_untouched")));
        assert!(t(lang, "readme.master_untouched").contains("master check"));
        assert!(
            r.contains("08 Promo/  →  Vera Alcón - Luz de Noviembre [MP3 320].mp3  (MP3 320 kbps)")
        );
    }
}

#[test]
fn readme_marks_upsampling_and_disabled() {
    let m = meta();
    let ms = master(44100, 24);
    let jobs = plan::resolve(&ms, &m, &["youtube_music"]);
    let r = readme(Lang::Es, &summary(&m, &ms, &jobs));
    assert!(r.contains(&t(Lang::Es, "readme.upsampling")));

    let ms16 = master(44100, 16);
    let jobs = plan::resolve(&ms16, &m, &["apple_adm"]);
    let r = readme(Lang::En, &summary(&m, &ms16, &jobs));
    assert!(r.contains(&t(Lang::En, "readme.disabled")));
    assert!(r.contains(&t(Lang::En, "apple_needs_24bit")));
}

#[test]
fn loudness_report_has_numbers_with_comma_in_es_and_dot_in_en() {
    let (m, ms) = (meta(), master(96000, 24));
    let jobs = plan::resolve(&ms, &m, &["spotify"]);
    let es = loudness_report(Lang::Es, &summary(&m, &ms, &jobs));
    let en = loudness_report(Lang::En, &summary(&m, &ms, &jobs));
    assert!(es.contains("−9,1 LUFS"), "{es}");
    assert!(en.contains("-9.1 LUFS"), "{en}");
    assert!(es.contains("4,2 LU") && en.contains("4.2 LU"));
    assert!(!es.contains('⟦') && !en.contains('⟦'));
}

#[test]
fn csv_has_fixed_header_and_one_row() {
    let mut m = meta();
    m.title = "Luz, de \"Noviembre\"".into();
    m.isrc = Some("ESABC2600001".into());
    let ms = master(96000, 24);
    let jobs = plan::resolve(&ms, &m, &["spotify"]);
    let csv = release_sheet_csv(&summary(&m, &ms, &jobs));
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "artist,title,version,featuring,album,year,isrc,composer,label,copyright,duration_sec,sample_rate,bits,integrated_lufs,true_peak_dbtp,lra_lu");
    assert_eq!(
        lines[1],
        "Vera Alcón,\"Luz, de \"\"Noviembre\"\"\",,,,,ESABC2600001,,,,200.0,96000,24,-9.1,0.4,4.2"
    );
}

#[test]
fn release_sheet_txt_in_both_languages_no_missing_keys() {
    let (m, ms) = (meta(), master(44100, 24));
    let jobs = plan::resolve(&ms, &m, &["spotify", "cover"]);
    for lang in [Lang::Es, Lang::En] {
        let s = summary(&m, &ms, &jobs);
        assert!(!release_sheet_txt(lang, &s).contains('⟦'));
        assert!(!readme(lang, &s).contains('⟦'));
        assert!(!loudness_report(lang, &s).contains('⟦'));
    }
}

#[test]
fn english_documents_are_fully_localized() {
    let (m, ms) = (meta(), master(96000, 24));
    let jobs = plan::resolve(&ms, &m, &["distributor_std", "spotify", "apple_adm"]);
    let sm = summary(&m, &ms, &jobs);
    let en = loudness_report(Lang::En, &sm);
    assert!(en.contains("Standard distributor"), "{en}");
    assert!(!en.contains("Distribuidora"), "{en}");
    let tag = format!("[{}] ", t(Lang::En, "verdict.tag.fail"));
    let tag_w = format!("[{}] ", t(Lang::En, "verdict.tag.warn"));
    assert!(en.contains(&tag) || en.contains(&tag_w), "{en}");
    assert!(!en.contains("[OK] OK"));
    let rd = readme(Lang::En, &sm);
    assert!(!rd.contains("estándar"), "{rd}");
    let es = loudness_report(Lang::Es, &sm);
    assert!(es.contains("Distribuidora estándar"), "{es}");
}
