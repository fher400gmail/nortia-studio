//! Escritor de WAV sintéticos a mano (independiente de `wav.rs`).
#![allow(dead_code)]

use std::path::Path;

#[derive(Default, Clone, Copy)]
pub struct Opts {
    pub junk_before_fmt: bool,
    pub extensible: bool,
}

fn chunk(out: &mut Vec<u8>, id: &[u8; 4], body: &[u8]) {
    out.extend_from_slice(id);
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(body);
    if body.len() % 2 == 1 {
        out.push(0);
    }
}

fn fmt_body(tag: u16, sr: u32, bits: u16, ch: u16, extensible: bool) -> Vec<u8> {
    let block = ch * bits / 8;
    let mut f = Vec::new();
    let wtag = if extensible { 0xFFFE } else { tag };
    f.extend_from_slice(&wtag.to_le_bytes());
    f.extend_from_slice(&ch.to_le_bytes());
    f.extend_from_slice(&sr.to_le_bytes());
    f.extend_from_slice(&(sr * u32::from(block)).to_le_bytes());
    f.extend_from_slice(&block.to_le_bytes());
    f.extend_from_slice(&bits.to_le_bytes());
    if extensible {
        f.extend_from_slice(&22u16.to_le_bytes());
        f.extend_from_slice(&bits.to_le_bytes()); // valid bits
        f.extend_from_slice(&3u32.to_le_bytes()); // channel mask
        f.extend_from_slice(&tag.to_le_bytes()); // GUID: formato real
        f.extend_from_slice(&[
            0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71,
        ]);
    }
    f
}

fn assemble(fmt: &[u8], data: &[u8], opts: Opts) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(b"WAVE");
    if opts.junk_before_fmt {
        chunk(&mut body, b"JUNK", &[0u8; 64]);
    }
    chunk(&mut body, b"fmt ", fmt);
    chunk(&mut body, b"data", data);
    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

/// Escribe un WAV PCM entero. `channels[c][i]` es el entero con signo de la muestra.
pub fn write_wav(path: &Path, sr: u32, bits: u16, channels: &[Vec<i32>], opts: Opts) {
    let ch = channels.len() as u16;
    let frames = channels[0].len();
    let mut data = Vec::new();
    for i in 0..frames {
        for c in channels {
            let b = c[i].to_le_bytes();
            data.extend_from_slice(&b[..usize::from(bits / 8)]);
        }
    }
    let fmt = fmt_body(1, sr, bits, ch, opts.extensible);
    std::fs::write(path, assemble(&fmt, &data, opts)).unwrap();
}

/// Escribe un WAV float32 (tag 0x0003).
pub fn write_wav_f32(path: &Path, sr: u32, channels: &[Vec<f32>], opts: Opts) {
    let ch = channels.len() as u16;
    let mut data = Vec::new();
    for i in 0..channels[0].len() {
        for c in channels {
            data.extend_from_slice(&c[i].to_le_bytes());
        }
    }
    let fmt = fmt_body(3, sr, 32, ch, opts.extensible);
    std::fs::write(path, assemble(&fmt, &data, opts)).unwrap();
}

/// Directorio de fixtures reales, o `None` (con aviso) si no está definido.
pub fn fixtures_dir() -> Option<std::path::PathBuf> {
    match std::env::var_os("MASTEROUT_FIXTURES") {
        Some(d) => Some(d.into()),
        None => {
            eprintln!("skip: MASTEROUT_FIXTURES no definido");
            None
        }
    }
}
