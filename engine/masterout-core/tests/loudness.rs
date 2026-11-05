mod common;

use masterout_core::audio::AudioBuffer;
use masterout_core::loudness::{measure, verdict, LoudnessReport, Verdict};
use masterout_core::wav;
use std::f64::consts::PI;

/// Amplitud lineal a partir de dBFS (pico).
fn amp(dbfs: f64) -> f64 {
    10f64.powf(dbfs / 20.0)
}

/// Seno de `secs` segundos con amplitud pico `amplitude` (lineal).
fn sine(sr: u32, freq: f64, amplitude: f64, phase: f64, secs: f64) -> Vec<f32> {
    let n = (f64::from(sr) * secs).round() as usize;
    (0..n)
        .map(|i| (amplitude * (2.0 * PI * freq * i as f64 / f64::from(sr) + phase).sin()) as f32)
        .collect()
}

fn buf(sr: u32, chans: Vec<Vec<f32>>) -> AudioBuffer {
    AudioBuffer {
        sample_rate: sr,
        channels: chans,
        source_bits: 24,
        source_float: false,
    }
}

fn stereo(sr: u32, samples: Vec<f32>) -> AudioBuffer {
    buf(sr, vec![samples.clone(), samples])
}

fn report(i: f64, tp: f64) -> LoudnessReport {
    LoudnessReport {
        integrated_lufs: i,
        lra_lu: 5.0,
        true_peak_dbtp: tp,
        sample_peak_dbfs: tp,
        momentary_max_lufs: i,
        short_term_max_lufs: i,
        overs_count: 0,
    }
}

#[test]
fn tech3341_case1_stereo_minus23_dbfs_is_minus23_lufs() {
    let r = measure(&stereo(48000, sine(48000, 1000.0, amp(-23.0), 0.0, 20.0))).unwrap();
    assert!(
        (r.integrated_lufs + 23.0).abs() <= 0.1,
        "{}",
        r.integrated_lufs
    );
}

#[test]
fn tech3341_case2_minus33() {
    let r = measure(&stereo(48000, sine(48000, 1000.0, amp(-33.0), 0.0, 20.0))).unwrap();
    assert!(
        (r.integrated_lufs + 33.0).abs() <= 0.1,
        "{}",
        r.integrated_lufs
    );
}

#[test]
fn tech3341_true_peak_minus6() {
    // Fase elegida para que el pico real caiga entre dos muestras.
    let r = measure(&stereo(48000, sine(48000, 997.0, amp(-6.0), 0.3, 5.0))).unwrap();
    let d = r.true_peak_dbtp + 6.0;
    assert!((-0.4..=0.2).contains(&d), "TP = {}", r.true_peak_dbtp);
}

#[test]
fn true_peak_of_full_scale_sine_between_samples_is_positive() {
    // Seno de 11025 Hz a 44,1 kHz con fase pi/4: las muestras caen siempre en
    // +-0,7071 de la amplitud. Se escala para que el pico de MUESTRA sea 0 dBFS
    // (amplitud sqrt(2)); el pico real entre muestras es entonces ~ +3 dBTP.
    let a = 1.0 / (PI / 4.0).sin();
    let r = measure(&stereo(44100, sine(44100, 11025.0, a, PI / 4.0, 5.0))).unwrap();
    assert!(
        r.sample_peak_dbfs.abs() < 0.1,
        "SP = {}",
        r.sample_peak_dbfs
    );
    assert!(r.true_peak_dbtp > 0.5, "TP = {}", r.true_peak_dbtp);
    assert!(r.overs_count > 0);
}

#[test]
fn tech3342_case1_lra_is_10() {
    let mut s = sine(48000, 1000.0, amp(-20.0), 0.0, 20.0);
    s.extend(sine(48000, 1000.0, amp(-30.0), 0.0, 20.0));
    let r = measure(&stereo(48000, s)).unwrap();
    assert!((r.lra_lu - 10.0).abs() <= 1.0, "LRA = {}", r.lra_lu);
}

#[test]
fn gating_ignores_silence() {
    let mut s = sine(48000, 1000.0, amp(-23.0), 0.0, 20.0);
    s.extend(vec![0.0; 48000 * 20]);
    let r = measure(&stereo(48000, s)).unwrap();
    assert!(
        (r.integrated_lufs + 23.0).abs() <= 0.1,
        "{}",
        r.integrated_lufs
    );
}

#[test]
fn measures_mono() {
    let r = measure(&buf(
        48000,
        vec![sine(48000, 1000.0, amp(-23.0), 0.0, 20.0)],
    ))
    .unwrap();
    assert!(
        (r.integrated_lufs + 26.0).abs() <= 0.1,
        "{}",
        r.integrated_lufs
    );
}

#[test]
fn works_at_96k_and_44k() {
    let a = measure(&stereo(96000, sine(96000, 1000.0, amp(-23.0), 0.0, 20.0))).unwrap();
    let b = measure(&stereo(44100, sine(44100, 1000.0, amp(-23.0), 0.0, 20.0))).unwrap();
    assert!(
        (a.integrated_lufs - b.integrated_lufs).abs() <= 0.1,
        "{} vs {}",
        a.integrated_lufs,
        b.integrated_lufs
    );
}

#[test]
fn verdict_rules() {
    assert_eq!(
        verdict(&report(-9.1, 1.2), Some(-14.0), -1.0),
        Verdict::Fail("tp_over_platform".into())
    );
    assert_eq!(
        verdict(&report(-14.0, -1.5), Some(-14.0), -1.0),
        Verdict::Ok
    );
    assert_eq!(
        verdict(&report(-18.0, -2.0), Some(-14.0), -1.0),
        Verdict::Warn("quieter_than_target".into())
    );
}

#[test]
fn verdict_louder_than_target_and_no_target() {
    assert_eq!(
        verdict(&report(-9.0, -2.0), Some(-14.0), -1.0),
        Verdict::Warn("louder_than_target".into())
    );
    assert_eq!(
        verdict(&report(-9.0, 0.4), None, -1.0),
        Verdict::Warn("tp_over_zero".into())
    );
    assert_eq!(verdict(&report(-9.0, -0.5), None, -1.0), Verdict::Ok);
}

#[test]
fn real_masters_match_ffmpeg() {
    let Some(dir) = common::fixtures_dir() else {
        return;
    };
    let refs: serde_json::Value = serde_json::from_str(include_str!("references.json")).unwrap();
    for name in ["m44_24", "m48_24", "m96_24", "m44_24_adm"] {
        let b = wav::read(&dir.join(format!("{name}.wav"))).unwrap();
        let r = measure(&b).unwrap();
        let want = &refs[name];
        let (i, lra, tp) = (
            want["i"].as_f64().unwrap(),
            want["lra"].as_f64().unwrap(),
            want["tp"].as_f64().unwrap(),
        );
        assert!(
            (r.integrated_lufs - i).abs() <= 0.15,
            "{name} I {}",
            r.integrated_lufs
        );
        assert!((r.lra_lu - lra).abs() <= 0.5, "{name} LRA {}", r.lra_lu);
        assert!(
            (r.true_peak_dbtp - tp).abs() <= 0.3,
            "{name} TP {}",
            r.true_peak_dbtp
        );
    }
}
