use mastercheck_bridge::errors::localize;
use masterout_core::audio::AudioError;
use masterout_core::i18n::Lang;

#[test]
fn localize_maps_every_variant_to_stable_keys() {
    let io = AudioError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "nope"));
    let e = localize(&io, Lang::Es);
    assert_eq!(e.key, "err.io");
    assert!(e.detail.as_deref().unwrap().contains("nope"));

    let e = localize(&AudioError::NotRiffWave, Lang::En);
    assert_eq!(e.key, "err.not_wav");
    assert!(e.message.contains("WAV"));

    let e = localize(
        &AudioError::UnsupportedFormat("err.rate_over_96k".into()),
        Lang::Es,
    );
    assert_eq!(e.key, "err.rate_over_96k");
    assert!(e.message.contains("96 kHz"));

    let e = localize(
        &AudioError::UnsupportedFormat("ebur128: x".into()),
        Lang::Es,
    );
    assert_eq!(e.key, "err.generic");
    assert_eq!(e.detail.as_deref(), Some("ebur128: x"));

    for (err, key) in [
        (AudioError::MissingFmt, "err.not_wav"),
        (AudioError::MissingData, "err.not_wav"),
        (AudioError::TooManyChannels(6), "err.too_many_channels"),
        (AudioError::Rf64Unsupported, "err.rf64_unsupported"),
    ] {
        for lang in [Lang::Es, Lang::En] {
            let e = localize(&err, lang);
            assert_eq!(e.key, key);
            assert!(!e.message.starts_with('⟦'), "{}", e.message);
        }
    }
}

#[test]
fn bridge_error_round_trips_through_json() {
    let e = localize(&AudioError::NotRiffWave, Lang::Es);
    let json = serde_json::to_string(&e).unwrap();
    assert!(json.contains("\"key\":\"err.not_wav\""));
    assert_eq!(
        serde_json::from_str::<mastercheck_bridge::errors::BridgeError>(&json).unwrap(),
        e
    );
}
