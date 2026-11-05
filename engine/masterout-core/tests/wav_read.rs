mod common;

use common::{fixtures_dir, write_wav, write_wav_f32, Opts};
use masterout_core::audio::AudioError;
use masterout_core::wav;

fn sine(frames: usize, sr: f64, amp: f64) -> Vec<i32> {
    (0..frames)
        .map(|i| (amp * (2.0 * std::f64::consts::PI * 1000.0 * i as f64 / sr).sin()) as i32)
        .collect()
}

#[test]
fn reads_pcm16_stereo_sine() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.wav");
    let ch = sine(4800, 48000.0, 16384.0);
    write_wav(&p, 48000, 16, &[ch.clone(), ch], Opts::default());
    let a = wav::read(&p).unwrap();
    assert_eq!(a.sample_rate, 48000);
    assert_eq!(a.num_channels(), 2);
    assert_eq!(a.frames(), 4800);
    assert_eq!(a.source_bits, 16);
    assert!(!a.source_float);
    let peak = a.channels[0].iter().fold(0f32, |m, v| m.max(v.abs()));
    assert!((peak - 0.5).abs() < 1e-3, "pico {peak}");
}

#[test]
fn reads_pcm24_scaled_to_unit() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.wav");
    write_wav(
        &p,
        44100,
        24,
        &[vec![0x7FFFFF, -0x800000, 0], vec![0, 0x7FFFFF, -0x800000]],
        Opts::default(),
    );
    let a = wav::read(&p).unwrap();
    assert!((a.channels[0][0] - (1.0 - 2f32.powi(-23))).abs() < 1e-6);
    assert_eq!(a.channels[0][1], -1.0);
    assert_eq!(a.channels[1][2], -1.0);
    assert_eq!(a.source_bits, 24);
}

#[test]
fn reads_wav_with_junk_chunk_before_fmt() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.wav");
    let ch = sine(1000, 44100.0, 1000.0);
    let opts = Opts {
        junk_before_fmt: true,
        ..Opts::default()
    };
    write_wav(&p, 44100, 16, &[ch.clone(), ch], opts);
    let info = wav::probe(&p).unwrap();
    assert!(info.extra_chunks.iter().any(|c| c == "JUNK"));
    assert_eq!(wav::read(&p).unwrap().frames(), 1000);
}

#[test]
fn reads_extensible_24bit() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.wav");
    let opts = Opts {
        extensible: true,
        ..Opts::default()
    };
    write_wav(
        &p,
        48000,
        24,
        &[vec![0x400000, -0x400000], vec![1, -1]],
        opts,
    );
    let info = wav::probe(&p).unwrap();
    assert_eq!(info.format_tag, 0xFFFE);
    assert_eq!(info.bits, 24);
    assert!(!info.is_float);
    let a = wav::read(&p).unwrap();
    assert_eq!(a.channels[0], vec![0.5, -0.5]);
    assert_eq!(a.channels[1][0], 2f32.powi(-23));
}

#[test]
fn reads_float32() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.wav");
    write_wav_f32(
        &p,
        48000,
        &[vec![1.5, -2.0], vec![0.25, 0.0]],
        Opts::default(),
    );
    let a = wav::read(&p).unwrap();
    assert!(a.source_float);
    assert_eq!(a.source_bits, 32);
    assert_eq!(a.channels[0], vec![1.5, -2.0]);
    assert_eq!(a.channels[1], vec![0.25, 0.0]);
}

#[test]
fn rejects_more_than_two_channels() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.wav");
    let ch: Vec<Vec<i32>> = (0..6).map(|_| vec![0; 10]).collect();
    write_wav(&p, 48000, 16, &ch, Opts::default());
    assert!(matches!(wav::read(&p), Err(AudioError::TooManyChannels(6))));
}

#[test]
fn rejects_rf64() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.wav");
    let mut b = b"RF64".to_vec();
    b.extend_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
    b.extend_from_slice(b"WAVE");
    b.extend_from_slice(&[0u8; 64]);
    std::fs::write(&p, b).unwrap();
    assert!(matches!(wav::read(&p), Err(AudioError::Rf64Unsupported)));
}

#[test]
fn rejects_non_wav() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.wav");
    std::fs::write(&p, b"hello world, not a wav file").unwrap();
    assert!(matches!(wav::probe(&p), Err(AudioError::NotRiffWave)));
}

#[test]
fn probe_matches_read() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.wav");
    let ch = sine(3333, 44100.0, 8000.0);
    write_wav(&p, 44100, 24, &[ch.clone(), ch], Opts::default());
    let info = wav::probe(&p).unwrap();
    assert_eq!(info.frames, wav::read(&p).unwrap().frames() as u64);
    assert_eq!(info.sample_rate, 44100);
    assert_eq!(info.channels, 2);
}

#[test]
fn reads_real_masters() {
    let Some(dir) = fixtures_dir() else { return };
    let cases = [
        ("m44_24.wav", 44100, 129.33),
        ("m48_24.wav", 48000, 185.41),
        ("m96_24.wav", 96000, 142.41),
        ("m44_24_adm.wav", 44100, 187.20),
    ];
    for (name, sr, dur) in cases {
        let a = wav::read(&dir.join(name)).unwrap();
        assert_eq!(a.sample_rate, sr, "{name}");
        assert_eq!(a.source_bits, 24, "{name}");
        assert_eq!(a.num_channels(), 2, "{name}");
        assert!(
            (a.duration_secs() - dur).abs() < 0.01,
            "{name}: {}",
            a.duration_secs()
        );
    }
}
