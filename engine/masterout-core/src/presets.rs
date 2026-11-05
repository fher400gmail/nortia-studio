//! Presets de destinos, cargados de `presets.json` (cada uno con su fuente).

use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Container {
    Wav,
    Flac,
    Mp3,
    Mp4,
    Jpg,
    Png,
}

impl Container {
    pub fn extension(self) -> &'static str {
        match self {
            Container::Wav => "wav",
            Container::Flac => "flac",
            Container::Mp3 => "mp3",
            Container::Mp4 => "mp4",
            Container::Jpg => "jpg",
            Container::Png => "png",
        }
    }
    fn parse(s: &str) -> Container {
        match s {
            "wav" => Container::Wav,
            "flac" => Container::Flac,
            "mp3" => Container::Mp3,
            "mp4" => Container::Mp4,
            "jpg" => Container::Jpg,
            "png" => Container::Png,
            other => panic!("contenedor desconocido en presets.json: {other}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitsPolicy {
    Native,
    Fixed(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatePolicy {
    Native,
    Fixed(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Confirmed,
    Recommendation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub subfolder: String,
    pub tag: String,
    pub container: Container,
    pub bits: BitsPolicy,
    pub rate: RatePolicy,
    pub target_lufs: Option<f64>,
    pub max_tp_dbtp: Option<f64>,
    pub embed_metadata: bool,
    pub status: Status,
    pub source_url: String,
    /// "streaming" | "distributor" | "extras"
    pub group: String,
    pub default_on: bool,
    /// Bits mínimos del máster para usar este destino (con política nativa).
    pub min_bits: Option<u16>,
    /// Clave i18n del motivo cuando el máster no llega a `min_bits`.
    pub min_bits_reason_key: Option<String>,
}

fn s(v: &Value, k: &str) -> String {
    v[k].as_str()
        .unwrap_or_else(|| panic!("presets.json: falta '{k}'"))
        .to_string()
}

fn parse(v: &Value) -> Preset {
    let bits = match &v["bits"] {
        Value::Number(n) => BitsPolicy::Fixed(n.as_u64().unwrap() as u16),
        _ => BitsPolicy::Native,
    };
    let rate = match &v["rate"] {
        Value::Number(n) => RatePolicy::Fixed(n.as_u64().unwrap() as u32),
        _ => RatePolicy::Native,
    };
    Preset {
        id: s(v, "id"),
        name: s(v, "name"),
        subfolder: s(v, "subfolder"),
        tag: s(v, "tag"),
        container: Container::parse(&s(v, "container")),
        bits,
        rate,
        target_lufs: v["target_lufs"].as_f64(),
        max_tp_dbtp: v["max_tp_dbtp"].as_f64(),
        embed_metadata: v["embed_metadata"].as_bool().unwrap_or(false),
        status: if s(v, "status") == "confirmed" {
            Status::Confirmed
        } else {
            Status::Recommendation
        },
        source_url: s(v, "source_url"),
        group: s(v, "group"),
        default_on: v["default_on"].as_bool().unwrap_or(false),
        min_bits: v["min_bits"].as_u64().map(|b| b as u16),
        min_bits_reason_key: v["min_bits_reason_key"].as_str().map(str::to_string),
    }
}

/// Todos los presets, en el orden de `presets.json` (orden de las subcarpetas).
pub fn all() -> Vec<Preset> {
    let root: Value = serde_json::from_str(include_str!("../presets.json"))
        .expect("presets.json debe ser JSON válido");
    let mut v: Vec<Preset> = root
        .as_array()
        .expect("presets.json debe ser una lista")
        .iter()
        .map(parse)
        .collect();
    v.sort_by(|a, b| a.subfolder.cmp(&b.subfolder));
    v
}
