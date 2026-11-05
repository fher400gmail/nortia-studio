use masterout_core::loudness::{LoudnessReport, Verdict};
use masterout_core::plan::*;
use masterout_core::presets::{self, Container};

fn loud(i: f64, tp: f64) -> LoudnessReport {
    LoudnessReport {
        integrated_lufs: i,
        lra_lu: 5.0,
        true_peak_dbtp: tp,
        sample_peak_dbfs: tp - 0.2,
        momentary_max_lufs: i + 3.0,
        short_term_max_lufs: i + 2.0,
        overs_count: 0,
    }
}

fn master(rate: u32, bits: u16) -> MasterInfo {
    MasterInfo {
        sample_rate: rate,
        bits,
        is_float: false,
        channels: 2,
        frames: u64::from(rate) * 60,
        loudness: loud(-14.0, -1.5),
    }
}

fn meta() -> ReleaseMeta {
    ReleaseMeta {
        artist: "Vera Alcón".into(),
        title: "Luz de Noviembre".into(),
        ..Default::default()
    }
}

fn job(r: &Resolved) -> &Job {
    match r {
        Resolved::Job(j) => j,
        other => panic!("se esperaba Job, hay {other:?}"),
    }
}

#[test]
fn presets_json_loads_and_has_13_entries_with_sources() {
    let all = presets::all();
    assert_eq!(all.len(), 13);
    for p in &all {
        assert!(!p.source_url.is_empty(), "{} sin source_url", p.id);
    }
}

#[test]
fn native_destinations_keep_master_resolution() {
    let r = resolve(&master(96000, 24), &meta(), &["spotify"]);
    let j = job(&r[0]);
    assert_eq!(j.bits, 24);
    assert_eq!(j.sample_rate, 96000);
    assert_eq!(j.container, Container::Flac);
    assert!(!j.needs_resample);
    assert!(!j.needs_dither);
    assert_eq!(
        j.file_name,
        "Vera Alcón - Luz de Noviembre [Spotify] 24bit-96kHz.flac"
    );
    assert_eq!(j.subfolder, "01 Spotify");
}

#[test]
fn fixed_destinations_convert_with_dither_when_lowering_bits() {
    let r = resolve(&master(96000, 24), &meta(), &["deezer"]);
    let j = job(&r[0]);
    assert_eq!((j.bits, j.sample_rate), (16, 44100));
    assert!(j.needs_resample);
    assert!(j.needs_dither);
    assert!(!j.is_upsampling);
}

#[test]
fn apple_disabled_for_16bit_master() {
    let r = resolve(&master(44100, 16), &meta(), &["apple_adm"]);
    match &r[0] {
        Resolved::Disabled {
            preset_id,
            reason_key,
        } => {
            assert_eq!(preset_id, "apple_adm");
            assert_eq!(reason_key, "apple_needs_24bit");
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn tunecore_marks_upsampling_and_no_metadata() {
    let r = resolve(&master(48000, 24), &meta(), &["tunecore"]);
    let j = job(&r[0]);
    assert_eq!((j.bits, j.sample_rate), (24, 192000));
    assert!(j.is_upsampling);
    assert!(!j.embed_metadata);
}

#[test]
fn never_upsample_native() {
    let r = resolve(&master(44100, 16), &meta(), &["spotify"]);
    let j = job(&r[0]);
    assert_eq!((j.bits, j.sample_rate), (16, 44100));
    assert!(!j.is_upsampling);
}

#[test]
fn verdict_uses_preset_targets() {
    let mut m = master(44100, 24);
    m.loudness = loud(-9.1, 1.2);
    let r = resolve(
        &m,
        &meta(),
        &["spotify", "youtube_music", "distributor_std"],
    );
    assert_eq!(r.len(), 3);
    assert!(matches!(job(&r[0]).verdict, Verdict::Fail(_)));
    assert!(matches!(
        job(&r[1]).verdict,
        Verdict::Warn(_) | Verdict::Fail(_)
    ));
    assert_eq!(job(&r[2]).preset_id, "distributor_std");
    assert_eq!(job(&r[2]).verdict, Verdict::Warn("tp_over_zero".into()));
}

#[test]
fn inactive_ids_are_not_resolved() {
    assert_eq!(resolve(&master(96000, 24), &meta(), &["spotify"]).len(), 1);
}

#[test]
fn not_in_this_build_presets_are_disabled() {
    let r = resolve(
        &master(96000, 24),
        &meta(),
        &["cover", "canvas", "youtube_video"],
    );
    assert_eq!(r.len(), 3);
    for x in &r {
        assert!(
            matches!(x, Resolved::Disabled { reason_key, .. } if reason_key == "not_in_this_build")
        );
    }
}

#[test]
fn youtube_music_fixes_48k_keeps_native_bits() {
    let r = resolve(&master(44100, 16), &meta(), &["youtube_music"]);
    let j = job(&r[0]);
    assert_eq!((j.bits, j.sample_rate), (16, 48000));
    assert!(j.is_upsampling);
}

#[test]
fn estimate_bytes_wav_exact() {
    let wav = job(&resolve(&master(96000, 24), &meta(), &["distributor_hires"])[0]).clone();
    assert_eq!(estimate_bytes(&wav, 96000 * 60, 2), 34_560_000);
    let flac = job(&resolve(&master(96000, 24), &meta(), &["spotify"])[0]).clone();
    assert_eq!(estimate_bytes(&flac, 96000 * 60, 2), 21_427_200);
}
