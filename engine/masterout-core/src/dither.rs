//! Cuantización f32 -> entero con dither TPDF (solo cuando se baja de profundidad).
//!
//! No se aplica ninguna ganancia: la escala es exactamente 2^(bits-1) y las muestras
//! fuera de rango se recortan al límite del entero.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Semilla fija de `quantize`: mismo audio de entrada, mismo archivo de salida.
const DEFAULT_SEED: u64 = 0x4D41_5354_4552_4F55;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dither {
    None,
    /// TPDF: suma de dos uniformes en [-0,5, 0,5) LSB (pico +-1 LSB), RNG determinista
    /// por canal derivado de `seed`.
    Tpdf {
        seed: u64,
    },
}

/// Elige el dither y cuantiza. TPDF solo si `target_bits < source_bits` o la fuente es float.
pub fn quantize(
    channels: &[Vec<f32>],
    target_bits: u16,
    source_bits: u16,
    source_float: bool,
) -> (Vec<Vec<i32>>, Dither) {
    let dither = if target_bits < source_bits || source_float {
        Dither::Tpdf { seed: DEFAULT_SEED }
    } else {
        Dither::None
    };
    (quantize_with(channels, target_bits, dither), dither)
}

/// Cuantizador de un canal: redondeo al más cercano, dither opcional y clamp.
struct Quantizer {
    scale: f64,
    lo: f64,
    hi: f64,
    rng: Option<StdRng>,
}

impl Quantizer {
    fn new(target_bits: u16, dither: Dither, channel: usize) -> Self {
        assert!(
            (2..=32).contains(&target_bits),
            "target_bits fuera de rango: {target_bits}"
        );
        let scale = 2f64.powi(i32::from(target_bits) - 1);
        let rng = match dither {
            Dither::None => None,
            // seed_from_u64 mezcla la semilla (SplitMix64): semillas consecutivas
            // dan flujos independientes.
            Dither::Tpdf { seed } => Some(StdRng::seed_from_u64(seed.wrapping_add(channel as u64))),
        };
        Quantizer {
            scale,
            lo: -scale,
            hi: scale - 1.0,
            rng,
        }
    }

    fn q(&mut self, x: f32) -> i32 {
        let mut y = f64::from(x) * self.scale;
        if let Some(r) = self.rng.as_mut() {
            y += r.gen_range(-0.5..0.5) + r.gen_range(-0.5..0.5);
        }
        y.round().clamp(self.lo, self.hi) as i32
    }
}

/// Cuantiza con el dither indicado: redondeo al más cercano y clamp a
/// [-2^(b-1), 2^(b-1)-1]. `target_bits` debe estar entre 2 y 32.
pub fn quantize_with(channels: &[Vec<f32>], target_bits: u16, dither: Dither) -> Vec<Vec<i32>> {
    channels
        .iter()
        .enumerate()
        .map(|(c, samples)| {
            let mut q = Quantizer::new(target_bits, dither, c);
            samples.iter().map(|&x| q.q(x)).collect()
        })
        .collect()
}

/// Igual que `quantize_with` (mismo resultado, bit a bit) pero consume los canales
/// para reutilizar su memoria: no hay copia f32 + i32 a la vez.
pub fn quantize_owned(channels: Vec<Vec<f32>>, target_bits: u16, dither: Dither) -> Vec<Vec<i32>> {
    quantize_owned_cancellable(channels, target_bits, dither, &|| false)
        .expect("sin cancelación no hay None")
}

/// Como `quantize_owned`, comprobando `should_cancel` cada 64 Ki muestras; `None` si se cancela.
pub fn quantize_owned_cancellable(
    channels: Vec<Vec<f32>>,
    target_bits: u16,
    dither: Dither,
    should_cancel: &dyn Fn() -> bool,
) -> Option<Vec<Vec<i32>>> {
    const CHECK_EVERY: usize = 1 << 16;
    let stopped = std::cell::Cell::new(false);
    let mut out = Vec::with_capacity(channels.len());
    for (c, samples) in channels.into_iter().enumerate() {
        let mut q = Quantizer::new(target_bits, dither, c);
        let mut n = 0usize;
        // `into_iter().map().collect()` reutiliza la memoria del Vec<f32> (mismo tamaño).
        let v: Vec<i32> = samples
            .into_iter()
            .map(|x| {
                if n.is_multiple_of(CHECK_EVERY) && !stopped.get() && should_cancel() {
                    stopped.set(true);
                }
                n += 1;
                if stopped.get() {
                    0
                } else {
                    q.q(x)
                }
            })
            .collect();
        if stopped.get() {
            return None;
        }
        out.push(v);
    }
    Some(out)
}

/// Como `quantize_with` pero comprobando `should_cancel` cada 64 Ki muestras; `None` si se cancela.
pub fn quantize_cancellable(
    channels: &[Vec<f32>],
    target_bits: u16,
    dither: Dither,
    should_cancel: &dyn Fn() -> bool,
) -> Option<Vec<Vec<i32>>> {
    const CHECK_EVERY: usize = 1 << 16;
    let mut out = Vec::with_capacity(channels.len());
    for (c, samples) in channels.iter().enumerate() {
        let mut q = Quantizer::new(target_bits, dither, c);
        let mut v = Vec::with_capacity(samples.len());
        for chunk in samples.chunks(CHECK_EVERY) {
            if should_cancel() {
                return None;
            }
            v.extend(chunk.iter().map(|&x| q.q(x)));
        }
        out.push(v);
    }
    Some(out)
}
