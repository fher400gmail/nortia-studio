//! Tipos de intercambio con la interfaz. Los nombres de campo son los de Rust (snake_case).

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MasterDto {
    pub path: String,
    pub file_name: String,
    pub sample_rate: u32,
    pub bits: u16,
    pub is_float: bool,
    pub channels: u16,
    pub frames: u64,
    pub duration_secs: f64,
    pub integrated_lufs: f64,
    pub true_peak_dbtp: f64,
    pub lra_lu: f64,
    pub overs_count: u64,
    pub suggested_artist: Option<String>,
    pub suggested_title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WaveformDto {
    /// Pico absoluto por tramo, en 0..=1; tantos como se pidieron.
    pub bins: Vec<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ReleaseMetaDto {
    pub artist: String,
    pub title: String,
    pub version: Option<String>,
    pub featuring: Option<String>,
    pub album: Option<String>,
    pub year: Option<String>,
    pub isrc: Option<String>,
    pub composer: Option<String>,
    pub label: Option<String>,
    pub copyright: Option<String>,
    pub cover_path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct JobDto {
    pub preset_id: String,
    pub subfolder: String,
    pub file_name: String,
    /// "wav" | "flac" | "mp3" | "mp4" | "jpg" | "png"
    pub container: String,
    pub bits: u16,
    pub sample_rate: u32,
    /// "ok" | "warn" | "fail"
    pub verdict: String,
    /// Localizado.
    pub verdict_text: String,
    pub estimated_bytes: u64,
    pub enabled: bool,
    /// Localizado.
    pub disabled_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlanDto {
    pub delivery_folder: String,
    pub jobs: Vec<JobDto>,
    /// Destinos habilitados + 4 documentos.
    pub total_files: u32,
    pub total_bytes: u64,
}
