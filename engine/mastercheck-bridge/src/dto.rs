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

/// Petición de exportación.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExportRequestDto {
    pub master_path: String,
    pub out_root: String,
    pub meta: ReleaseMetaDto,
    pub active_ids: Vec<String>,
}

/// Evento de progreso. Según `kind`, solo se rellenan algunos campos:
/// "analyzed" | "planned" | "job_started" | "job_progress" (`preset_id`, `fraction`) |
/// "job_done" (`preset_id`, `path`, `bytes`) |
/// "job_failed" (`preset_id`, `error` localizado, `error_key` estable, `error_detail` técnico) |
/// "cancelled" | "finished" (`delivery`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ProgressDto {
    pub kind: String,
    pub preset_id: Option<String>,
    pub fraction: Option<f32>,
    pub path: Option<String>,
    pub bytes: Option<u64>,
    pub error: Option<String>,
    /// Clave estable del error (`err.*`).
    pub error_key: Option<String>,
    /// Detalle técnico sin localizar.
    pub error_detail: Option<String>,
    pub delivery: Option<DeliveryDto>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DeliveryDto {
    /// Carpeta de la entrega; vacía si se canceló y se borró por no tener archivos completos.
    pub folder: String,
    /// La exportación se canceló; `files` son los archivos completos que se conservan.
    pub cancelled: bool,
    /// (ruta, bytes) de cada archivo escrito, audio y documentos.
    pub files: Vec<(String, u64)>,
    /// (destino, mensaje localizado) de lo que falló.
    pub failed: Vec<(String, String)>,
    /// (destino, clave estable `err.*`) de lo que falló, en el mismo orden que `failed`.
    pub failed_keys: Vec<(String, String)>,
    /// (destino, detalle técnico sin localizar).
    pub failed_details: Vec<(String, String)>,
    pub elapsed_ms: u64,
}
