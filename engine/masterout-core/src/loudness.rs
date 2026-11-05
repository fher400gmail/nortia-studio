//! Medición de loudness según ITU-R BS.1770-4 / EBU R128 (crate `ebur128`).
//! Solo mide: nunca modifica el audio.

use crate::audio::{AudioBuffer, AudioError};
use ebur128::{EbuR128, Mode};

/// Margen (LU) alrededor del objetivo dentro del cual no se avisa.
const TARGET_TOLERANCE_LU: f64 = 1.0;

/// Resultado de una medición.
#[derive(Debug, Clone, PartialEq)]
pub struct LoudnessReport {
    pub integrated_lufs: f64,
    pub lra_lu: f64,
    pub true_peak_dbtp: f64,
    pub sample_peak_dbfs: f64,
    pub momentary_max_lufs: f64,
    pub short_term_max_lufs: f64,
    /// Número de tramos de 100 ms cuyo true peak supera 0 dBTP (no de muestras).
    pub overs_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Ok,
    /// Clave i18n del aviso.
    Warn(String),
    /// Clave i18n del fallo.
    Fail(String),
}

fn to_db(linear: f64) -> f64 {
    if linear > 0.0 {
        20.0 * linear.log10()
    } else {
        f64::NEG_INFINITY
    }
}

fn map_err(e: ebur128::Error) -> AudioError {
    AudioError::UnsupportedFormat(format!("ebur128: {e}"))
}

/// Mide loudness integrado, LRA, picos y máximos momentáneo y a corto plazo.
///
/// El audio se alimenta en tramos de 100 ms; `overs_count` cuenta los tramos de
/// 100 ms cuyo true peak (máximo de los canales) supera 0 dBTP.
pub fn measure(buf: &AudioBuffer) -> Result<LoudnessReport, AudioError> {
    let ch = buf.num_channels();
    if ch == 0 || ch > 2 {
        return Err(AudioError::UnsupportedFormat(format!(
            "{ch} canales (solo mono y estéreo)"
        )));
    }
    let mode = Mode::I | Mode::LRA | Mode::TRUE_PEAK | Mode::SAMPLE_PEAK | Mode::M | Mode::S;
    let mut ebu = EbuR128::new(ch as u32, buf.sample_rate, mode).map_err(map_err)?;

    let frames = buf.frames();
    let step = ((buf.sample_rate / 10) as usize).max(1);
    let mut momentary_max = f64::NEG_INFINITY;
    let mut short_max = f64::NEG_INFINITY;
    let mut overs = 0u64;

    let mut pos = 0;
    while pos < frames {
        let end = (pos + step).min(frames);
        let slices: Vec<&[f32]> = buf.channels.iter().map(|c| &c[pos..end]).collect();
        ebu.add_frames_planar_f32(&slices).map_err(map_err)?;
        if let Ok(m) = ebu.loudness_momentary() {
            momentary_max = momentary_max.max(m);
        }
        if let Ok(s) = ebu.loudness_shortterm() {
            short_max = short_max.max(s);
        }
        let mut chunk_tp = 0.0f64;
        for c in 0..ch {
            chunk_tp = chunk_tp.max(ebu.prev_true_peak(c as u32).map_err(map_err)?);
        }
        if chunk_tp > 1.0 {
            overs += 1;
        }
        pos = end;
    }

    let mut tp = 0.0f64;
    let mut sp = 0.0f64;
    for c in 0..ch {
        tp = tp.max(ebu.true_peak(c as u32).map_err(map_err)?);
        sp = sp.max(ebu.sample_peak(c as u32).map_err(map_err)?);
    }

    Ok(LoudnessReport {
        integrated_lufs: ebu.loudness_global().map_err(map_err)?,
        lra_lu: ebu.loudness_range().map_err(map_err)?,
        true_peak_dbtp: to_db(tp),
        sample_peak_dbfs: to_db(sp),
        momentary_max_lufs: momentary_max,
        short_term_max_lufs: short_max,
        overs_count: overs,
    })
}

/// Veredicto frente a un destino.
///
/// - Con objetivo: true peak por encima de `max_tp_dbtp` => `Fail("tp_over_platform")`;
///   si no, más de 1 LU por encima/debajo del objetivo => `Warn("louder_than_target")` /
///   `Warn("quieter_than_target")`; si no, `Ok`.
/// - Sin objetivo (`None`): true peak > 0 dBTP => `Warn("tp_over_zero")`; si no, `Ok`.
pub fn verdict(report: &LoudnessReport, target_lufs: Option<f64>, max_tp_dbtp: f64) -> Verdict {
    match target_lufs {
        None => {
            if report.true_peak_dbtp > 0.0 {
                Verdict::Warn("tp_over_zero".into())
            } else {
                Verdict::Ok
            }
        }
        Some(target) => {
            if report.true_peak_dbtp > max_tp_dbtp {
                Verdict::Fail("tp_over_platform".into())
            } else if report.integrated_lufs > target + TARGET_TOLERANCE_LU {
                Verdict::Warn("louder_than_target".into())
            } else if report.integrated_lufs < target - TARGET_TOLERANCE_LU {
                Verdict::Warn("quieter_than_target".into())
            } else {
                Verdict::Ok
            }
        }
    }
}
