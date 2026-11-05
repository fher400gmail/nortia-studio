//! Análisis de un máster: medidas de loudness y forma de onda para la interfaz.

use crate::dto::{MasterDto, WaveformDto};
use crate::errors::{localize, BridgeError};
use masterout_core::export;
use masterout_core::i18n::Lang;
use masterout_core::naming;
use masterout_core::wav;
use std::path::Path;

/// Un valor no finito (silencio da -inf) no cabe en JSON: se acota a este suelo.
const FLOOR_DB: f64 = -200.0;

fn finite(v: f64) -> f64 {
    if v.is_finite() {
        v
    } else {
        FLOOR_DB
    }
}

/// `"Artista - Título"` -> (Some(artista), Some(título)); sin separador, solo título.
fn suggest(path: &Path) -> (Option<String>, Option<String>) {
    let base = naming::fallback_title("", path);
    for sep in [" - ", " – ", " — "] {
        if let Some((artist, title)) = base.split_once(sep) {
            let (artist, title) = (artist.trim(), title.trim());
            if !artist.is_empty() && !title.is_empty() {
                return (Some(artist.to_string()), Some(title.to_string()));
            }
        }
    }
    let base = base.trim();
    (None, (!base.is_empty()).then(|| base.to_string()))
}

/// Pico absoluto de todos los canales por tramo.
fn waveform(channels: &[Vec<f32>], bins: usize) -> Vec<f32> {
    let frames = channels.first().map_or(0, Vec::len);
    (0..bins)
        .map(|i| {
            let start = (i as u128 * frames as u128 / bins as u128) as usize;
            let end = (((i as u128 + 1) * frames as u128 / bins as u128) as usize)
                .max(start + 1)
                .min(frames);
            let peak = channels
                .iter()
                .flat_map(|c| c.get(start..end).unwrap_or(&[]).iter())
                .fold(0f32, |m, &x| m.max(x.abs()));
            peak.min(1.0)
        })
        .collect()
}

/// Lee y mide el máster y calcula `bins` picos para dibujar la onda.
pub fn analyze(
    path: &Path,
    lang: Lang,
    bins: usize,
) -> Result<(MasterDto, WaveformDto), BridgeError> {
    if path.is_dir() {
        return Err(BridgeError::from_key("err.not_wav", lang, None));
    }
    // Cabecera primero: rechaza rápido lo que no es WAV o supera 96 kHz sin cargar el audio.
    let probe = wav::probe(path).map_err(|e| localize(&e, lang))?;
    if probe.sample_rate > 96_000 {
        return Err(BridgeError::from_key("err.rate_over_96k", lang, None));
    }
    let (buf, info) = export::analyze(path).map_err(|e| localize(&e, lang))?;
    let l = &info.loudness;
    let (suggested_artist, suggested_title) = suggest(path);
    let master = MasterDto {
        path: path.to_string_lossy().into_owned(),
        file_name: path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default(),
        sample_rate: info.sample_rate,
        bits: info.bits,
        is_float: info.is_float,
        channels: info.channels,
        frames: info.frames,
        duration_secs: buf.duration_secs(),
        integrated_lufs: finite(l.integrated_lufs),
        true_peak_dbtp: finite(l.true_peak_dbtp),
        lra_lu: finite(l.lra_lu),
        overs_count: l.overs_count,
        suggested_artist,
        suggested_title,
    };
    let bins = waveform(&buf.channels, bins);
    Ok((master, WaveformDto { bins }))
}
