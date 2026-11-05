mod common;

use mastercheck_bridge::analysis::analyze;
use mastercheck_bridge::lang::Lang as LangTag;
use masterout_core::i18n::Lang;

#[test]
fn analyze_real_or_synthetic_master_returns_waveform_with_requested_bins() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("synthetic.wav");
    common::write_wav(&path, 48_000, 24, &common::sine_stereo(48_000, 24, 4, 0.5));
    let (m, w) = analyze(&path, Lang::Es, 600).unwrap();
    assert_eq!(w.bins.len(), 600);
    assert!(w.bins.iter().all(|b| (0.0..=1.0).contains(b)));
    let max = w.bins.iter().cloned().fold(0.0f32, f32::max);
    assert!((max - 0.5).abs() < 0.01, "máximo {max}");
    assert_eq!(m.sample_rate, 48_000);
    assert_eq!(m.bits, 24);
    assert_eq!(m.channels, 2);
    assert_eq!(m.frames, 48_000 * 4);
    assert!((m.duration_secs - 4.0).abs() < 1e-9);
    assert_eq!(m.file_name, "synthetic.wav");
    assert!(m.integrated_lufs.is_finite() && m.true_peak_dbtp.is_finite());
    // El DTO sobrevive a un viaje por JSON (sin valores no finitos).
    let json = serde_json::to_string(&m).unwrap();
    assert_eq!(
        serde_json::from_str::<mastercheck_bridge::dto::MasterDto>(&json).unwrap(),
        m
    );

    // Con fixtures reales (si hay), la onda también tiene los tramos pedidos.
    if let Some(dir) = std::env::var_os("MASTEROUT_FIXTURES") {
        let real = std::path::Path::new(&dir).join("m96_24.wav");
        if real.is_file() {
            let (m, w) = analyze(&real, Lang::En, 300).unwrap();
            assert_eq!(w.bins.len(), 300);
            assert_eq!(m.sample_rate, 96_000);
        }
    }
}

#[test]
fn silent_master_serializes_to_json() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("silence.wav");
    common::write_wav(&path, 44_100, 16, &[vec![0; 44_100], vec![0; 44_100]]);
    let (m, w) = analyze(&path, Lang::En, 10).unwrap();
    assert!(w.bins.iter().all(|&b| b == 0.0));
    let json = serde_json::to_string(&m).unwrap();
    assert!(serde_json::from_str::<mastercheck_bridge::dto::MasterDto>(&json).is_ok());
}

#[test]
fn analyze_dto_rejects_non_wav_with_localized_message() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("notas.txt");
    std::fs::write(
        &path,
        "esto no es audio, pero es bastante largo para tener cabecera",
    )
    .unwrap();
    let e = analyze(&path, Lang::Es, 100).unwrap_err();
    assert_eq!(e.key, "err.not_wav");
    assert!(e.message.contains("WAV"), "{}", e.message);
    let e = analyze(&path, Lang::En, 100).unwrap_err();
    assert!(
        e.message.contains("WAV") && e.message.contains("not"),
        "{}",
        e.message
    );
    // Carpeta y archivo inexistente: sin cuelgues, mensaje claro.
    assert_eq!(
        analyze(tmp.path(), Lang::Es, 100).unwrap_err().key,
        "err.not_wav"
    );
    assert_eq!(
        analyze(&tmp.path().join("no-existe.wav"), Lang::Es, 100)
            .unwrap_err()
            .key,
        "err.io"
    );
    // Un fichero pequeño o vacío tampoco rompe.
    let tiny = tmp.path().join("vacío.mp3");
    std::fs::write(&tiny, b"ID3").unwrap();
    assert!(analyze(&tiny, Lang::Es, 100).is_err());
}

#[test]
fn analyze_rejects_192k_with_key() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("hires.wav");
    common::write_wav(
        &path,
        192_000,
        24,
        &[vec![1000; 19_200], vec![1000; 19_200]],
    );
    let e = analyze(&path, Lang::Es, 100).unwrap_err();
    assert_eq!(e.key, "err.rate_over_96k");
    assert!(e.message.contains("96"));
}

#[test]
fn suggested_artist_title_from_file_name() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp
        .path()
        .join("Vera Alcón - Luz de Noviembre (master v3).wav");
    common::write_wav(&path, 44_100, 16, &common::sine_stereo(44_100, 16, 1, 0.3));
    let (m, _) = analyze(&path, Lang::Es, 10).unwrap();
    assert_eq!(m.suggested_artist.as_deref(), Some("Vera Alcón"));
    assert_eq!(m.suggested_title.as_deref(), Some("Luz de Noviembre"));

    let plain = tmp.path().join("mezcla final.wav");
    common::write_wav(&plain, 44_100, 16, &common::sine_stereo(44_100, 16, 1, 0.3));
    let (m, _) = analyze(&plain, Lang::Es, 10).unwrap();
    assert_eq!(m.suggested_artist, None);
    assert_eq!(m.suggested_title.as_deref(), Some("mezcla final"));
}

#[test]
fn lang_from_locale() {
    assert_eq!(LangTag::from_locale("es-ES"), Lang::Es);
    assert_eq!(LangTag::from_locale("es_MX.UTF-8"), Lang::Es);
    assert_eq!(LangTag::from_locale("en-US"), Lang::En);
    assert_eq!(LangTag::from_locale(""), Lang::En);
}

#[test]
fn waveform_handles_mono_few_frames_and_zero_frames() {
    let tmp = tempfile::tempdir().unwrap();
    // Mono: un solo canal, pico 0,5.
    let mono = tmp.path().join("mono.wav");
    let ch: Vec<i32> = (0..44_100)
        .map(|i| ((i as f64 * 0.05).sin() * 0.5 * 32767.0) as i32)
        .collect();
    common::write_wav(&mono, 44_100, 16, &[ch]);
    let (m, w) = analyze(&mono, Lang::Es, 50).unwrap();
    assert_eq!((m.channels, w.bins.len()), (1, 50));
    let max = w.bins.iter().cloned().fold(0.0f32, f32::max);
    assert!((max - 0.5).abs() < 0.01, "{max}");

    // Más tramos que frames: se devuelven los tramos pedidos y nada entra en pánico.
    let tiny = tmp.path().join("tiny.wav");
    common::write_wav(&tiny, 44_100, 16, &[vec![1000; 10], vec![-2000; 10]]);
    let (_, w) = analyze(&tiny, Lang::Es, 100).unwrap();
    assert_eq!(w.bins.len(), 100);
    assert!(w.bins.iter().all(|b| (0.0..=1.0).contains(b)));
    assert!(w.bins.iter().any(|&b| b > 0.0));
    // 0 tramos: lista vacía.
    assert!(analyze(&tiny, Lang::Es, 0).unwrap().1.bins.is_empty());

    // Cero frames: sin pánico, ya sea Ok con onda a 0 o un error con clave.
    let empty = tmp.path().join("vacio.wav");
    common::write_wav(&empty, 44_100, 16, &[vec![], vec![]]);
    match analyze(&empty, Lang::Es, 20) {
        Ok((m, w)) => {
            assert_eq!(m.frames, 0);
            assert_eq!(w.bins.len(), 20);
            assert!(w.bins.iter().all(|&b| b == 0.0));
            assert!(serde_json::to_string(&m).is_ok());
        }
        Err(e) => assert!(e.key.starts_with("err."), "{e:?}"),
    }
}
