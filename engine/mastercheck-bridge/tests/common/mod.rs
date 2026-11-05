#![allow(dead_code)]

use std::path::Path;

/// Escribe un WAV PCM entero a mano. `channels[c][i]` es el entero con signo de la muestra.
pub fn write_wav(path: &Path, sr: u32, bits: u16, channels: &[Vec<i32>]) {
    let ch = channels.len() as u16;
    let frames = channels[0].len();
    let bps = usize::from(bits / 8);
    let mut data = Vec::with_capacity(frames * channels.len() * bps);
    for i in 0..frames {
        for c in channels {
            data.extend_from_slice(&c[i].to_le_bytes()[..bps]);
        }
    }
    let block = ch * bits / 8;
    let mut fmt = Vec::new();
    fmt.extend_from_slice(&1u16.to_le_bytes());
    fmt.extend_from_slice(&ch.to_le_bytes());
    fmt.extend_from_slice(&sr.to_le_bytes());
    fmt.extend_from_slice(&(sr * u32::from(block)).to_le_bytes());
    fmt.extend_from_slice(&block.to_le_bytes());
    fmt.extend_from_slice(&bits.to_le_bytes());
    let mut body = Vec::new();
    body.extend_from_slice(b"WAVE");
    for (id, b) in [(b"fmt ", &fmt), (b"data", &data)] {
        body.extend_from_slice(id);
        body.extend_from_slice(&(b.len() as u32).to_le_bytes());
        body.extend_from_slice(b);
        if b.len() % 2 == 1 {
            body.push(0);
        }
    }
    let mut out = Vec::from(*b"RIFF");
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    std::fs::write(path, out).unwrap();
}

/// Seno estéreo de `secs` segundos con amplitud `amp` (0..1).
pub fn sine_stereo(sr: u32, bits: u16, secs: usize, amp: f64) -> Vec<Vec<i32>> {
    let n = sr as usize * secs;
    let full = f64::from((1u32 << (bits - 1)) - 1);
    let mk = |f: f64| -> Vec<i32> {
        (0..n)
            .map(|i| {
                let t = i as f64 / f64::from(sr);
                (amp * (std::f64::consts::TAU * f * t).sin() * full) as i32
            })
            .collect()
    };
    vec![mk(220.0), mk(330.0)]
}
