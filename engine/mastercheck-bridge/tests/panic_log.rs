//! El gancho de panics es global al proceso: va en su propio binario de pruebas.

use mastercheck_bridge::app_support::panic_log::{install, LOG_FILE, MAX_LOG_BYTES};

#[test]
fn a_panic_is_logged_to_the_file_and_the_log_stays_bounded() {
    let dir = tempfile::tempdir().unwrap();
    let log = dir.path().join("subcarpeta").join(LOG_FILE);
    install(dir.path().join("subcarpeta"));

    let r = std::panic::catch_unwind(|| panic!("fallo de prueba {}", 7));
    assert!(r.is_err());
    let text = std::fs::read_to_string(&log).expect("el panic se registra");
    assert!(text.contains("fallo de prueba 7"), "{text}");
    assert!(text.contains("panic_log.rs"), "lleva el lugar: {text}");
    assert_eq!(text.lines().count(), 1);

    // un payload que no es texto también se registra
    let r = std::panic::catch_unwind(|| std::panic::panic_any(42u8));
    assert!(r.is_err());
    assert_eq!(std::fs::read_to_string(&log).unwrap().lines().count(), 2);

    // el registro no crece sin límite: pasado el tope se reinicia
    std::fs::write(&log, vec![b'x'; MAX_LOG_BYTES as usize + 10]).unwrap();
    let _ = std::panic::catch_unwind(|| panic!("otra"));
    let len = std::fs::metadata(&log).unwrap().len();
    assert!(len < 1000, "{len}");
    assert!(std::fs::read_to_string(&log).unwrap().contains("otra"));
}
