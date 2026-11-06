//! Exportación con progreso como DTOs, cancelación y errores localizados.

use crate::analysis::suggest;
use crate::dto::{DeliveryDto, ExportRequestDto, ProgressDto, VerdictDto};
use crate::errors::{localize, BridgeError};
use crate::planning::{to_media_inputs, to_release_meta, verdict_parts};
use masterout_core::export::{self, default_sidecar, Delivery, ExportRequest, Progress};
use masterout_core::i18n::{t, Lang};
use masterout_core::media::SidecarPaths;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

fn path_str(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

fn delivery_dto(d: &Delivery, lang: Lang) -> DeliveryDto {
    DeliveryDto {
        // Tras cancelar sin archivos completos la carpeta se borra: se informa vacía.
        folder: if d.folder.exists() {
            path_str(&d.folder)
        } else {
            String::new()
        },
        cancelled: d.cancelled,
        files: d.files.iter().map(|(p, b)| (path_str(p), *b)).collect(),
        failed: d
            .failed
            .iter()
            .map(|(id, key)| (id.clone(), t(lang, key)))
            .collect(),
        failed_keys: d.failed.clone(),
        failed_details: d.failed_details.clone(),
        verdicts: d
            .verdicts
            .iter()
            .map(|(id, v)| {
                let (verdict, verdict_text) = verdict_parts(v, lang);
                VerdictDto {
                    preset_id: id.clone(),
                    verdict: verdict.to_string(),
                    verdict_text,
                }
            })
            .collect(),
        elapsed_ms: u64::try_from(d.elapsed_ms).unwrap_or(u64::MAX),
    }
}

fn progress_dto(p: Progress, lang: Lang) -> ProgressDto {
    let kind = |k: &str| ProgressDto {
        kind: k.to_string(),
        ..Default::default()
    };
    match p {
        Progress::Analyzed(_) => kind("analyzed"),
        Progress::Planned(_) => kind("planned"),
        Progress::Preparing { fraction } => ProgressDto {
            fraction: Some(fraction),
            ..kind("preparing")
        },
        Progress::JobStarted { preset_id } => ProgressDto {
            preset_id: Some(preset_id),
            ..kind("job_started")
        },
        Progress::JobProgress {
            preset_id,
            fraction,
        } => ProgressDto {
            preset_id: Some(preset_id),
            fraction: Some(fraction),
            ..kind("job_progress")
        },
        Progress::JobDone {
            preset_id,
            path,
            bytes,
        } => ProgressDto {
            preset_id: Some(preset_id),
            path: Some(path_str(&path)),
            bytes: Some(bytes),
            ..kind("job_done")
        },
        Progress::JobFailed {
            preset_id,
            error,
            detail,
        } => ProgressDto {
            preset_id: Some(preset_id),
            error: Some(t(lang, &error)),
            error_key: Some(error),
            error_detail: detail,
            ..kind("job_failed")
        },
        Progress::Cancelled { .. } => kind("cancelled"),
        Progress::Finished(d) => ProgressDto {
            delivery: Some(delivery_dto(&d, lang)),
            ..kind("finished")
        },
    }
}

/// Exporta la entrega. `cancel` se puede activar desde otro hilo: los archivos ya completos
/// se conservan (aparecen en `DeliveryDto::files`), no quedan temporales y se emite "cancelled".
pub fn export(
    req: &ExportRequestDto,
    lang: Lang,
    on_progress: &(dyn Fn(ProgressDto) + Sync),
    cancel: Arc<AtomicBool>,
) -> Result<DeliveryDto, BridgeError> {
    export_with(req, lang, on_progress, cancel, default_sidecar())
}

/// Como [`export`], con el sidecar de ffmpeg indicado (`None`: sin vídeo ni MP3).
pub fn export_with(
    req: &ExportRequestDto,
    lang: Lang,
    on_progress: &(dyn Fn(ProgressDto) + Sync),
    cancel: Arc<AtomicBool>,
    sidecar: Option<SidecarPaths>,
) -> Result<DeliveryDto, BridgeError> {
    export_custom(req, lang, on_progress, cancel, sidecar, None)
}

/// Como [`export_with`], con las herramientas de Apple Music (AAC 256, afclip) indicadas
/// `(afconvert, afclip)` en vez de buscarlas en el sistema (pruebas con herramientas falsas).
pub fn export_custom(
    req: &ExportRequestDto,
    lang: Lang,
    on_progress: &(dyn Fn(ProgressDto) + Sync),
    cancel: Arc<AtomicBool>,
    sidecar: Option<SidecarPaths>,
    adm_tools: Option<(PathBuf, PathBuf)>,
) -> Result<DeliveryDto, BridgeError> {
    let master_path = PathBuf::from(&req.master_path);
    let mut meta = to_release_meta(&req.meta);
    // Título vacío: el sugerido por el nombre de archivo ("Artista - Título" se separa, sin
    // duplicar el artista); el artista vacío toma el sugerido.
    if meta.title.is_empty() {
        let (artist, title) = suggest(&master_path);
        if let Some(title) = title {
            meta.title = title;
        }
        if meta.artist.is_empty() {
            meta.artist = artist.unwrap_or_default();
        }
    }
    let request = ExportRequest::new(
        master_path,
        meta,
        req.active_ids.clone(),
        PathBuf::from(&req.out_root),
        lang,
    )
    .with_cancel(cancel)
    .with_sidecar(sidecar)
    .with_media(to_media_inputs(&req.meta, &req.media_inputs));
    let request = match adm_tools {
        Some((afconvert, afclip)) => request.with_adm_tools(afconvert, afclip),
        None => request,
    };
    export::run(&request, &|p| on_progress(progress_dto(p, lang)))
        .map(|d| delivery_dto(&d, lang))
        .map_err(|e| localize(&e, lang))
}
