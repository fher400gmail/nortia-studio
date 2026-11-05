//! Exportación con progreso como DTOs, cancelación y errores localizados.

use crate::dto::{DeliveryDto, ExportRequestDto, ProgressDto};
use crate::errors::{localize, BridgeError};
use crate::planning::to_release_meta;
use masterout_core::export::{self, Delivery, ExportRequest, Progress};
use masterout_core::i18n::{t, Lang};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

fn path_str(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

/// Los errores por destino que son claves (`err.*`) se traducen; el resto va tal cual.
fn localize_message(msg: &str, lang: Lang) -> String {
    if msg.starts_with("err.") && !msg.contains(' ') {
        t(lang, msg)
    } else {
        msg.to_string()
    }
}

fn delivery_dto(d: &Delivery, lang: Lang) -> DeliveryDto {
    DeliveryDto {
        folder: path_str(&d.folder),
        files: d.files.iter().map(|(p, b)| (path_str(p), *b)).collect(),
        failed: d
            .failed
            .iter()
            .map(|(id, e)| (id.clone(), localize_message(e, lang)))
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
        Progress::JobFailed { preset_id, error } => ProgressDto {
            preset_id: Some(preset_id),
            error: Some(localize_message(&error, lang)),
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
    let master_path = PathBuf::from(&req.master_path);
    let request = ExportRequest::new(
        master_path,
        to_release_meta(&req.meta),
        req.active_ids.clone(),
        PathBuf::from(&req.out_root),
        lang,
    )
    .with_cancel(cancel);
    export::run(&request, &|p| on_progress(progress_dto(p, lang)))
        .map(|d| delivery_dto(&d, lang))
        .map_err(|e| localize(&e, lang))
}
