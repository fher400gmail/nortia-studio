//! Resolución del plan de entrega: qué archivo se produce para cada destino activo.

use crate::loudness::{self, LoudnessReport, Verdict};
use crate::naming;
use crate::presets::{self, BitsPolicy, Container, Preset, RatePolicy};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct MasterInfo {
    pub sample_rate: u32,
    pub bits: u16,
    pub is_float: bool,
    pub channels: u16,
    pub frames: u64,
    pub loudness: LoudnessReport,
}

#[derive(Debug, Clone, Default)]
pub struct ReleaseMeta {
    pub artist: String,
    pub title: String,
    pub version: Option<String>,
    pub featuring: Option<String>,
    pub album: Option<String>,
    pub year: Option<u16>,
    pub isrc: Option<String>,
    pub composer: Option<String>,
    pub label: Option<String>,
    pub copyright: Option<String>,
    pub cover: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub preset_id: String,
    pub subfolder: String,
    pub file_name: String,
    pub container: Container,
    pub bits: u16,
    pub sample_rate: u32,
    pub needs_resample: bool,
    pub needs_dither: bool,
    pub is_upsampling: bool,
    pub embed_metadata: bool,
    pub verdict: Verdict,
}

#[derive(Debug, Clone)]
pub enum Resolved {
    Job(Job),
    Disabled {
        preset_id: String,
        reason_key: String,
    },
}

fn disabled(preset: &Preset, key: &str) -> Resolved {
    Resolved::Disabled {
        preset_id: preset.id.clone(),
        reason_key: key.to_string(),
    }
}

fn resolve_one(master: &MasterInfo, meta: &ReleaseMeta, p: &Preset) -> Resolved {
    if matches!(p.id.as_str(), "cover" | "canvas" | "youtube_video") {
        return disabled(p, "not_in_this_build");
    }
    // Bits efectivos del máster: un float de 32 bit se entrega como mucho a 24.
    let master_bits = if master.is_float { 32 } else { master.bits };
    let native_bits = master_bits.min(24);

    if matches!(p.bits, BitsPolicy::Native) {
        if let Some(min) = p.min_bits {
            if native_bits < min {
                let key = p
                    .min_bits_reason_key
                    .clone()
                    .unwrap_or_else(|| "min_bits_not_met".into());
                return disabled(p, &key);
            }
        }
    }

    let bits = match p.bits {
        BitsPolicy::Native => native_bits,
        BitsPolicy::Fixed(b) => b,
    };
    let sample_rate = match p.rate {
        RatePolicy::Native => master.sample_rate,
        RatePolicy::Fixed(r) => r,
    };
    let is_audio_pcm = matches!(p.container, Container::Wav | Container::Flac);
    let needs_resample = sample_rate != master.sample_rate;
    let needs_dither = is_audio_pcm && bits < master_bits;
    let is_upsampling = sample_rate > master.sample_rate || (is_audio_pcm && bits > master_bits);

    let file_name = if p.container == Container::Mp3 {
        naming::media_file_name(
            &meta.artist,
            &meta.title,
            &p.tag,
            "",
            p.container.extension(),
        )
    } else {
        naming::audio_file_name(
            &meta.artist,
            &meta.title,
            &p.tag,
            bits,
            sample_rate,
            p.container.extension(),
        )
    };
    let verdict = loudness::verdict(
        &master.loudness,
        p.target_lufs,
        p.max_tp_dbtp.unwrap_or(0.0),
    );
    Resolved::Job(Job {
        preset_id: p.id.clone(),
        subfolder: p.subfolder.clone(),
        file_name,
        container: p.container,
        bits,
        sample_rate,
        needs_resample,
        needs_dither,
        is_upsampling,
        embed_metadata: p.embed_metadata,
        verdict,
    })
}

/// Resuelve los destinos activos, en el orden de las subcarpetas.
pub fn resolve(master: &MasterInfo, meta: &ReleaseMeta, active_ids: &[&str]) -> Vec<Resolved> {
    presets::all()
        .iter()
        .filter(|p| active_ids.contains(&p.id.as_str()))
        .map(|p| resolve_one(master, meta, p))
        .collect()
}

/// Tamaño estimado del archivo de salida en bytes. WAV exacto (sin cabecera);
/// FLAC aproximado (0,62 x WAV); MP3 320 kbps = 40 kB/s.
pub fn estimate_bytes(job: &Job, frames: u64, channels: u16) -> u64 {
    let wav = frames * u64::from(channels) * u64::from(job.bits) / 8;
    match job.container {
        Container::Wav => wav,
        Container::Flac => (wav as f64 * 0.62) as u64,
        Container::Mp3 => frames * 40_000 / u64::from(job.sample_rate.max(1)),
        _ => 0,
    }
}
