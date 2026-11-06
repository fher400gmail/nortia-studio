//! Gancho de humo sin ventana: `MASTERCHECK_SMOKE="<wav>;<carpeta_salida>"`.
//! Analiza, planifica y exporta por el puente; imprime el `DeliveryDto` en JSON.

use crate::analysis::analyze;
use crate::dto::{DeliveryDto, ExportRequestDto, ReleaseMetaDto};
use crate::errors::BridgeError;
use crate::exporting::export;
use crate::planning::plan;
use masterout_core::i18n::Lang;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

/// `"<wav>;<carpeta_salida>"` -> rutas. Se parte por el último `;` (las rutas pueden
/// llevar espacios y acentos; en Windows no pueden llevar `;` salvo casos raros).
pub fn parse_spec(spec: &str) -> Result<(PathBuf, PathBuf), String> {
    let (wav, out) = spec
        .rsplit_once(';')
        .ok_or_else(|| "MASTERCHECK_SMOKE debe ser \"<wav>;<carpeta_salida>\"".to_string())?;
    let (wav, out) = (wav.trim(), out.trim());
    if wav.is_empty() || out.is_empty() {
        return Err("MASTERCHECK_SMOKE: falta la ruta del WAV o la carpeta de salida".into());
    }
    Ok((PathBuf::from(wav), PathBuf::from(out)))
}

/// Destinos que ejercita el humo. El MP3 necesita el sidecar (ffmpeg); la portada, solo una
/// imagen; Canvas, el vídeo de YouTube y portada animada, las dos cosas (parten de la portada).
pub fn smoke_ids(has_sidecar: bool, has_cover: bool) -> Vec<String> {
    let mut ids: Vec<&str> = vec!["spotify", "apple_adm", "deezer", "distributor_std"];
    if has_sidecar {
        ids.push("mp3_promo");
    }
    if has_cover {
        ids.push("cover");
        if has_sidecar {
            ids.extend(["canvas", "youtube_video", "apple_motion"]);
        }
    }
    ids.into_iter().map(String::from).collect()
}

/// Opciones del humo. En la app real salen del entorno (`MASTERCHECK_SMOKE_COVER`).
#[derive(Clone, Debug, Default)]
pub struct SmokeOptions {
    /// Portada de la entrega (imagen); sin ella no se ejercitan portada ni vídeo.
    pub cover: Option<PathBuf>,
    /// Solo estos destinos (para pruebas rápidas); `None`: todos los que el entorno permita.
    pub only: Option<Vec<String>>,
}

impl SmokeOptions {
    pub fn from_env_with(env: &dyn Fn(&str) -> Option<std::ffi::OsString>) -> Self {
        SmokeOptions {
            cover: env("MASTERCHECK_SMOKE_COVER")
                .filter(|v| !v.is_empty())
                .map(PathBuf::from),
            only: None,
        }
    }

    pub fn from_env() -> Self {
        Self::from_env_with(&|k| std::env::var_os(k))
    }
}

/// Tiempo del sondeo de codificadores en frío (la primera llamada de la sesión).
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CapsTiming {
    pub h264: bool,
    pub hevc: bool,
    pub millis: u64,
}

/// Resultado del humo: la entrega más lo que CI necesita para decidir.
#[derive(Clone, Debug)]
pub struct SmokeReport {
    pub delivery: DeliveryDto,
    /// `None` si no hay sidecar (no hay sondeo).
    pub capabilities: Option<CapsTiming>,
    pub sidecar: Option<(PathBuf, PathBuf)>,
    /// (destino, motivo localizado) de lo que el plan dejó sin generar.
    pub disabled: Vec<(String, String)>,
    pub requested: Vec<String>,
}

impl SmokeReport {
    /// `DeliveryDto` con un campo extra `smoke` (los lectores del DTO lo ignoran).
    pub fn to_json(&self) -> serde_json::Value {
        let mut v = serde_json::to_value(&self.delivery).unwrap_or_default();
        let smoke = serde_json::json!({
            "capabilities": self.capabilities,
            "sidecar": self.sidecar.as_ref().map(|(f, p)| serde_json::json!({
                "ffmpeg": f.to_string_lossy(),
                "ffprobe": p.to_string_lossy(),
            })),
            "requested": self.requested,
            "disabled": self.disabled.iter()
                .map(|(id, why)| serde_json::json!({"preset_id": id, "reason": why}))
                .collect::<Vec<_>>(),
        });
        if let Some(o) = v.as_object_mut() {
            o.insert("smoke".into(), smoke);
        }
        v
    }
}

/// Núcleo del humo: analizar, planificar y exportar. Falla si algo falla o no se escribe nada.
pub fn run_smoke(spec: &str, lang: Lang, exe_dir: &Path) -> Result<DeliveryDto, BridgeError> {
    run_smoke_with(spec, lang, exe_dir, &SmokeOptions::default()).map(|r| r.delivery)
}

pub fn run_smoke_with(
    spec: &str,
    lang: Lang,
    exe_dir: &Path,
    opts: &SmokeOptions,
) -> Result<SmokeReport, BridgeError> {
    let fail = |m: String| BridgeError {
        key: "err.generic".into(),
        message: m.clone(),
        detail: Some(m),
    };
    let (wav, out) = parse_spec(spec).map_err(fail)?;
    let (master, _wave) = analyze(&wav, lang, 600)?;
    let meta = ReleaseMetaDto {
        artist: master
            .suggested_artist
            .clone()
            .unwrap_or_else(|| "master check".into()),
        title: master
            .suggested_title
            .clone()
            .unwrap_or_else(|| "smoke".into()),
        cover_path: opts
            .cover
            .as_ref()
            .map(|c| c.to_string_lossy().into_owned()),
        ..Default::default()
    };
    let sidecar = masterout_core::media::discover(exe_dir);
    // Sondeo en frío ANTES de planificar (el plan lo reutiliza de la caché): es el tiempo que
    // vería la persona la primera vez. Objetivo < 3 s; el plazo de cada códec es 8 s.
    let capabilities = sidecar.as_ref().map(|sc| {
        let t0 = Instant::now();
        let caps =
            masterout_core::media::capabilities(sc, masterout_core::media::Platform::current());
        CapsTiming {
            h264: caps.h264,
            hevc: caps.hevc,
            millis: u64::try_from(t0.elapsed().as_millis()).unwrap_or(u64::MAX),
        }
    });
    let ids = opts
        .only
        .clone()
        .unwrap_or_else(|| smoke_ids(sidecar.is_some(), opts.cover.is_some()));
    let planned = plan(&master, &meta, &ids, &Default::default(), lang);
    let disabled: Vec<(String, String)> = planned
        .jobs
        .iter()
        .filter(|j| !j.enabled)
        .map(|j| {
            (
                j.preset_id.clone(),
                j.disabled_reason.clone().unwrap_or_default(),
            )
        })
        .collect();
    let active: Vec<String> = planned
        .jobs
        .iter()
        .filter(|j| j.enabled)
        .map(|j| j.preset_id.clone())
        .collect();
    let req = ExportRequestDto {
        master_path: master.path.clone(),
        out_root: out.to_string_lossy().into_owned(),
        meta,
        active_ids: active,
        media_inputs: Default::default(),
    };
    let delivery = export(&req, lang, &|_| {}, Arc::new(AtomicBool::new(false)))?;
    if !delivery.failed.is_empty() {
        let msg = delivery
            .failed
            .iter()
            .map(|(id, e)| format!("{id}: {e}"))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(fail(msg));
    }
    if delivery.files.is_empty() {
        return Err(fail("la entrega no contiene archivos".into()));
    }
    Ok(SmokeReport {
        delivery,
        capabilities,
        sidecar: sidecar.map(|s| (s.ffmpeg, s.ffprobe)),
        disabled,
        requested: ids,
    })
}

/// Ejecuta el humo e informa: JSON del `DeliveryDto` (más el bloque `smoke`) por stdout y en
/// `<salida>/smoke-result.json`, porque en Windows la app GUI no tiene consola.
/// Devuelve el código de salida: 0 si todo va bien, 1 si no.
pub fn run_and_report(spec: &str, lang: Lang, exe_dir: &Path) -> i32 {
    let result = run_smoke_with(spec, lang, exe_dir, &SmokeOptions::from_env());
    let (json, code) = match &result {
        Ok(r) => (
            serde_json::to_string_pretty(&r.to_json()).unwrap_or_default(),
            0,
        ),
        Err(e) => (serde_json::to_string_pretty(e).unwrap_or_default(), 1),
    };
    println!("{json}");
    if let Ok((_, out)) = parse_spec(spec) {
        let _ = std::fs::create_dir_all(&out);
        let _ = std::fs::write(out.join("smoke-result.json"), &json);
    }
    code
}
