mod common;

use masterout_core::flac::{self, FlacTags};
use std::f64::consts::PI;
use std::process::Command;

fn sine(bits: u16, rate: u32, frames: usize, channels: usize) -> Vec<Vec<i32>> {
    let amp = f64::from((1i32 << (bits - 1)) - 1) * 0.5;
    (0..channels)
        .map(|c| {
            (0..frames)
                .map(|i| {
                    let f = 440.0 * (c as f64 + 1.0);
                    (amp * (2.0 * PI * f * i as f64 / f64::from(rate)).sin()).round() as i32
                })
                .collect()
        })
        .collect()
}

fn decode(path: &std::path::Path) -> (claxon::metadata::StreamInfo, Vec<i32>) {
    let mut r = claxon::FlacReader::open(path).unwrap();
    let info = r.streaminfo();
    let v: Vec<i32> = r.samples().map(|s| s.unwrap()).collect();
    (info, v)
}

fn have(cmd: &str) -> bool {
    Command::new(cmd).arg("-version").output().is_ok()
}

#[test]
fn writes_24bit_flac_decodable_by_ffmpeg() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.flac");
    let s = sine(24, 96000, 96000, 2);
    flac::write(&p, &s, 96000, 24, None).unwrap();
    let (info, _) = decode(&p);
    assert_eq!((info.bits_per_sample, info.sample_rate), (24, 96000));
    if !have("ffprobe") || !have("ffmpeg") {
        eprintln!("ffprobe/ffmpeg no disponibles: se omite la comprobación externa");
        return;
    }
    let out = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "stream=sample_fmt,bits_per_raw_sample,sample_rate",
            "-of",
            "default=nw=1",
        ])
        .arg(&p)
        .output()
        .unwrap();
    let txt = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(txt.contains("sample_fmt=s32"), "{txt}");
    assert!(txt.contains("bits_per_raw_sample=24"), "{txt}");
    assert!(txt.contains("sample_rate=96000"), "{txt}");
    let dec = Command::new("ffmpeg")
        .args(["-v", "error", "-i"])
        .arg(&p)
        .args(["-f", "null", "-"])
        .output()
        .unwrap();
    assert!(dec.status.success());
    assert!(
        dec.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&dec.stderr)
    );
}

#[test]
fn roundtrip_is_lossless() {
    let dir = tempfile::tempdir().unwrap();
    for (bits, rate) in [(24u16, 96000u32), (16, 44100)] {
        let p = dir.path().join(format!("r{bits}.flac"));
        let s = sine(bits, rate, 20000, 2);
        flac::write(&p, &s, rate, bits, None).unwrap();
        let (_, dec) = decode(&p);
        let expect: Vec<i32> = s[0].iter().zip(&s[1]).flat_map(|(l, r)| [*l, *r]).collect();
        assert_eq!(dec, expect);
    }
}

#[test]
fn writes_vorbis_comments() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("t.flac");
    let s = sine(24, 48000, 10000, 2);
    let tags = FlacTags {
        title: "Luz de Noviembre".into(),
        artist: "Vera Alcón".into(),
        album: Some("Álbum".into()),
        date: Some("2026".into()),
        isrc: Some("ESABC2600001".into()),
        copyright: Some("© 2026 Vera Alcón".into()),
    };
    flac::write(&p, &s, 48000, 24, Some(&tags)).unwrap();
    let tag = metaflac::Tag::read_from_path(&p).unwrap();
    let vc = tag.vorbis_comments().unwrap();
    assert_eq!(vc.title().unwrap()[0], "Luz de Noviembre");
    assert_eq!(vc.artist().unwrap()[0], "Vera Alcón");
    assert_eq!(vc.get("DATE").unwrap()[0], "2026");
    assert_eq!(vc.get("ISRC").unwrap()[0], "ESABC2600001");
    assert_eq!(vc.get("ALBUM").unwrap()[0], "Álbum");
    assert_eq!(vc.get("COPYRIGHT").unwrap()[0], "© 2026 Vera Alcón");
    // El audio sigue intacto con etiquetas.
    let (info, dec) = decode(&p);
    assert_eq!(info.bits_per_sample, 24);
    assert_eq!(dec.len(), 20000);
    if have("ffmpeg") {
        let o = Command::new("ffmpeg")
            .args(["-v", "error", "-i"])
            .arg(&p)
            .args(["-f", "null", "-"])
            .output()
            .unwrap();
        assert!(o.status.success() && o.stderr.is_empty());
    }
}

#[test]
fn writes_16bit_flac_too() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("s.flac");
    flac::write(&p, &sine(16, 44100, 5000, 1), 44100, 16, None).unwrap();
    let (info, dec) = decode(&p);
    assert_eq!((info.bits_per_sample, info.channels), (16, 1));
    assert_eq!(dec.len(), 5000);
}

#[test]
fn rejects_unsupported_bits_and_out_of_range() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("x.flac");
    assert!(flac::write(&p, &[vec![0; 10]], 44100, 20, None).is_err());
    assert!(flac::write(&p, &[vec![40000; 10]], 44100, 16, None).is_err());
}

#[test]
fn write_at_96k_works_and_above_is_a_clean_error() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.flac");
    flac::write(&p, &sine(24, 96_000, 4800, 2), 96_000, 24, None).unwrap();
    assert!(p.exists());
    let q = dir.path().join("b.flac");
    let err = flac::write(&q, &sine(24, 192_000, 4800, 2), 192_000, 24, None).unwrap_err();
    assert_eq!(
        err.to_string(),
        masterout_core::audio::AudioError::UnsupportedFormat("err.rate_over_96k".into())
            .to_string()
    );
    assert!(!q.exists());
}

#[test]
fn cancel_is_checked_every_64_blocks() {
    use std::cell::Cell;
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("c.flac");
    // ~20 s a 48 kHz: más de 200 bloques de 4096.
    let s = sine(24, 48_000, 48_000 * 20, 2);
    let calls = Cell::new(0u32);
    let r = flac::write_cancellable(&p, &s, 48_000, 24, None, &|| {
        calls.set(calls.get() + 1);
        calls.get() >= 3
    });
    assert!(matches!(
        r,
        Err(masterout_core::audio::AudioError::Cancelled)
    ));
    // Se detuvo en la tercera comprobación (bloque 128), no al final.
    assert_eq!(calls.get(), 3);
}

/// Tono fuerte (medio fondo de escala) más ruido uniforme determinista de ~18 bit: el residuo
/// de la predicción es grande (parámetros de Rice > 14), como en un máster real a 24 bit.
fn loud_sine_plus_noise(frames: usize) -> Vec<Vec<i32>> {
    let amp = f64::from((1i32 << 23) - 1) * 0.5;
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    (0..2)
        .map(|c| {
            (0..frames)
                .map(|i| {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    let noise = ((state >> 40) as i32 - (1 << 23)) >> 6; // +-2^17
                    let t = i as f64 / 44_100.0;
                    let tone = amp * (2.0 * PI * (220.0 + 110.0 * c as f64) * t).sin();
                    (tone as i32 + noise).clamp(-(1 << 23), (1 << 23) - 1)
                })
                .collect()
        })
        .collect()
}

#[test]
fn loud_24bit_flac_compresses_below_80_percent_of_pcm() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("loud.flac");
    let s = loud_sine_plus_noise(44_100 * 20);
    flac::write(&p, &s, 44_100, 24, None).unwrap();
    let raw = (s[0].len() * 2 * 3) as f64;
    let size = std::fs::metadata(&p).unwrap().len() as f64;
    assert!(
        size < raw * 0.80,
        "FLAC {size} B = {:.1} % del PCM ({raw} B)",
        100.0 * size / raw
    );
    let (info, dec) = decode(&p);
    assert_eq!(info.bits_per_sample, 24);
    let expect: Vec<i32> = s[0].iter().zip(&s[1]).flat_map(|(l, r)| [*l, *r]).collect();
    assert_eq!(dec, expect);
}

#[test]
fn real_master_flac_compresses_below_85_percent_of_wav() {
    let Some(dir) = common::fixtures_dir() else {
        return;
    };
    let wav = dir.join("m44_24.wav");
    let audio = masterout_core::wav::read(&wav).unwrap();
    let scale = f64::from(1u32 << 23);
    let s: Vec<Vec<i32>> = audio
        .channels
        .iter()
        .map(|c| {
            c.iter()
                .map(|&x| (f64::from(x) * scale).round() as i32)
                .collect()
        })
        .collect();
    let tmp = tempfile::tempdir().unwrap();
    let p = tmp.path().join("m44.flac");
    flac::write(&p, &s, audio.sample_rate, 24, None).unwrap();
    let wav_size = std::fs::metadata(&wav).unwrap().len() as f64;
    let size = std::fs::metadata(&p).unwrap().len() as f64;
    // ffmpeg -compression_level 5 da ~82 % en este máster (muy fuerte); el listón es 85 %.
    assert!(
        size < wav_size * 0.85,
        "FLAC {size} B = {:.1} % del WAV ({wav_size} B)",
        100.0 * size / wav_size
    );
    let (info, dec) = decode(&p);
    assert_eq!(info.bits_per_sample, 24);
    let expect: Vec<i32> = s[0].iter().zip(&s[1]).flat_map(|(l, r)| [*l, *r]).collect();
    assert!(dec == expect, "audio decodificado distinto del original");
}
