//! Gancho de humo sin ventana: `MASTERCHECK_SMOKE="<wav>;<carpeta_salida>"`.
//! Analiza, planifica y exporta por el puente; imprime el `DeliveryDto` en JSON.

use super::lame::resolve_lame_path;
use crate::analysis::analyze;
use crate::dto::{DeliveryDto, ExportRequestDto, ReleaseMetaDto};
use crate::errors::BridgeError;
use crate::exporting::export;
use crate::planning::plan;
use masterout_core::i18n::Lang;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

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

/// Destinos que ejercita el humo. MP3 solo si hay `lame` junto al ejecutable o en `PATH`.
fn smoke_ids(exe_dir: &Path) -> Vec<String> {
    let mut ids: Vec<String> = ["spotify", "apple_adm", "deezer", "distributor_std"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    if resolve_lame_path(exe_dir).is_some() {
        ids.push("mp3_promo".into());
    }
    ids
}

/// Núcleo del humo: analizar, planificar y exportar. Falla si algo falla o no se escribe nada.
pub fn run_smoke(spec: &str, lang: Lang, exe_dir: &Path) -> Result<DeliveryDto, BridgeError> {
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
        ..Default::default()
    };
    let ids = smoke_ids(exe_dir);
    let planned = plan(&master, &meta, &ids, lang);
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
    Ok(delivery)
}

/// Ejecuta el humo e informa: JSON del `DeliveryDto` por stdout (y en
/// `<salida>/smoke-result.json`, porque en Windows la app GUI no tiene consola).
/// Devuelve el código de salida: 0 si todo va bien, 1 si no.
pub fn run_and_report(spec: &str, lang: Lang, exe_dir: &Path) -> i32 {
    let result = run_smoke(spec, lang, exe_dir);
    let (json, code) = match &result {
        Ok(d) => (serde_json::to_string_pretty(d).unwrap_or_default(), 0),
        Err(e) => (serde_json::to_string_pretty(e).unwrap_or_default(), 1),
    };
    println!("{json}");
    if let Ok((_, out)) = parse_spec(spec) {
        let _ = std::fs::create_dir_all(&out);
        let _ = std::fs::write(out.join("smoke-result.json"), &json);
    }
    code
}
