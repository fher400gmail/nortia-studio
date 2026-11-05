mod common;

use masterout_core::audio::AudioBuffer;
use masterout_core::loudness::measure;
use masterout_core::resample::resample;
use masterout_core::wav;
use std::f64::consts::PI;

fn sine_buf(sr: u32, freq: f64, dbfs: f64, secs: f64) -> AudioBuffer {
    let a = 10f64.powf(dbfs / 20.0);
    let n = (f64::from(sr) * secs).round() as usize;
    let ch: Vec<f32> = (0..n)
        .map(|i| (a * (2.0 * PI * freq * i as f64 / f64::from(sr)).sin()) as f32)
        .collect();
    AudioBuffer {
        sample_rate: sr,
        channels: vec![ch.clone(), ch],
        source_bits: 24,
        source_float: false,
    }
}

fn rms_db(x: &[f32]) -> f64 {
    let s: f64 = x.iter().map(|&v| f64::from(v) * f64::from(v)).sum();
    10.0 * (s / x.len() as f64).log10()
}

#[test]
fn length_matches_ratio() {
    let b = sine_buf(96000, 1000.0, -6.0, 1.0);
    assert_eq!(b.frames(), 96000);
    let o = resample(&b, 44100).unwrap();
    assert_eq!(o.sample_rate, 44100);
    assert_eq!(o.num_channels(), 2);
    assert!((o.frames() as i64 - 44100).abs() <= 2, "{}", o.frames());
}

#[test]
fn preserves_tone_level_and_frequency() {
    let b = sine_buf(96000, 1000.0, -6.0, 2.0);
    let o = resample(&b, 44100).unwrap();
    // Se excluyen los bordes para no medir el arranque del filtro.
    let x = &o.channels[0][4410..o.frames() - 4410];
    let want = rms_db(&b.channels[0][9600..b.frames() - 9600]);
    assert!((rms_db(x) - want).abs() <= 0.1, "{} vs {}", rms_db(x), want);
    let crossings = x.windows(2).filter(|w| w[0] <= 0.0 && w[1] > 0.0).count();
    let hz = crossings as f64 / (x.len() as f64 / 44100.0);
    assert!((hz - 1000.0).abs() <= 1.0, "{hz}");
}

#[test]
fn upsample_44_to_192_is_allowed() {
    let b = sine_buf(44100, 1000.0, -6.0, 1.0);
    let o = resample(&b, 192000).unwrap();
    let want = (44100.0f64 * 192000.0 / 44100.0).round() as i64;
    assert!((o.frames() as i64 - want).abs() <= 2, "{}", o.frames());
    assert!((o.frames() as f64 / 44100.0 - 4.353).abs() < 0.01);
}

#[test]
fn same_rate_is_identity() {
    let b = sine_buf(48000, 1000.0, -6.0, 0.5);
    assert_eq!(resample(&b, 48000).unwrap(), b);
}

#[test]
fn no_gain_change_on_real_master() {
    let Some(dir) = common::fixtures_dir() else {
        return;
    };
    let b = wav::read(&dir.join("m96_24.wav")).unwrap();
    let o = resample(&b, 44100).unwrap();
    let (a, c) = (measure(&b).unwrap(), measure(&o).unwrap());
    assert!(
        (a.integrated_lufs - c.integrated_lufs).abs() < 0.1,
        "{} vs {}",
        a.integrated_lufs,
        c.integrated_lufs
    );
}

#[test]
fn exact_multiple_of_chunk_size_works() {
    let b = sine_buf(96000, 1000.0, -6.0, 51200.0 / 96000.0);
    assert_eq!(b.frames(), 51200);
    let o = resample(&b, 44100).unwrap();
    assert!((o.frames() as i64 - 23520).abs() <= 2, "{}", o.frames());
}

#[test]
fn empty_buffer_returns_empty() {
    let b = AudioBuffer {
        sample_rate: 96000,
        channels: vec![vec![], vec![]],
        source_bits: 24,
        source_float: false,
    };
    let o = resample(&b, 44100).unwrap();
    assert_eq!(o.frames(), 0);
    assert_eq!(o.sample_rate, 44100);
    assert_eq!(o.num_channels(), 2);
}
