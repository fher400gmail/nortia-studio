//! Pico de memoria de una exportación completa. Solo Linux (lee VmHWM de /proc).
#![cfg(target_os = "linux")]

use masterout_core::export::{self, ExportRequest};
use masterout_core::i18n::Lang;
use masterout_core::plan::ReleaseMeta;
use std::io::Write;
use std::path::Path;

fn vm_hwm_kb() -> u64 {
    let s = std::fs::read_to_string("/proc/self/status").unwrap();
    s.lines()
        .find(|l| l.starts_with("VmHWM:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|n| n.parse().ok())
        .unwrap()
}

/// Intenta reiniciar el pico (Linux >= 4.0); si no se puede, el delta sigue siendo válido.
fn reset_hwm() {
    let _ = std::fs::write("/proc/self/clear_refs", "5");
}

/// WAV 96 kHz / 24 bit estéreo escrito en streaming (sin cargar el audio en memoria).
fn write_streaming_master(path: &Path, secs: usize) {
    let sr = 96_000usize;
    let frames = sr * secs;
    let data_len = (frames * 6) as u32;
    let mut w = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    w.write_all(b"RIFF").unwrap();
    w.write_all(&(36 + data_len).to_le_bytes()).unwrap();
    w.write_all(b"WAVEfmt ").unwrap();
    w.write_all(&16u32.to_le_bytes()).unwrap();
    w.write_all(&1u16.to_le_bytes()).unwrap();
    w.write_all(&2u16.to_le_bytes()).unwrap();
    w.write_all(&(sr as u32).to_le_bytes()).unwrap();
    w.write_all(&(sr as u32 * 6).to_le_bytes()).unwrap();
    w.write_all(&6u16.to_le_bytes()).unwrap();
    w.write_all(&24u16.to_le_bytes()).unwrap();
    w.write_all(b"data").unwrap();
    w.write_all(&data_len.to_le_bytes()).unwrap();
    for i in 0..frames {
        let t = i as f64 / sr as f64;
        for (f, ph) in [(220.0, 0.0), (330.0, 0.5)] {
            let v = 0.25 * (std::f64::consts::TAU * f * t + ph).sin()
                + 0.1 * (std::f64::consts::TAU * f * 3.1 * t).sin();
            let s = (v * 8_388_607.0) as i32;
            w.write_all(&s.to_le_bytes()[..3]).unwrap();
        }
    }
    w.flush().unwrap();
}

#[test]
fn peak_memory_under_4x_master() {
    let tmp = tempfile::tempdir().unwrap();
    let master = tmp.path().join("peso (master v1).wav");
    write_streaming_master(&master, 60);
    // Tamaño del máster en f32: 60 s x 96000 x 2 canales x 4 bytes = 46 MB.
    let master_f32_kb = 60 * 96_000 * 2 * 4 / 1024;

    let ids: Vec<String> = [
        "spotify",
        "apple_adm",
        "amazon",
        "tidal",
        "deezer",
        "youtube_music",
        "distributor_std",
        "distributor_hires",
        "tunecore",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let meta = ReleaseMeta {
        artist: "Artista".into(),
        title: "Memoria".into(),
        ..Default::default()
    };
    let req =
        ExportRequest::new(master, meta, ids, tmp.path().join("out"), Lang::Es).with_threads(4);

    reset_hwm();
    let before = vm_hwm_kb();
    let d = export::run(&req, &|_| {}).unwrap();
    let after = vm_hwm_kb();
    assert!(d.failed.is_empty(), "{:?}", d.failed);
    let delta = after.saturating_sub(before);
    eprintln!(
        "VmHWM antes {before} kB, después {after} kB, delta {delta} kB, límite {} kB",
        4 * master_f32_kb
    );
    assert!(
        delta < 4 * master_f32_kb,
        "pico {delta} kB >= 4 x máster ({master_f32_kb} kB)"
    );
}
