//! Plan de entrega para la interfaz: qué se va a producir y qué destinos no están disponibles.

use crate::dto::{JobDto, MasterDto, PlanDto, ReleaseMetaDto};
use masterout_core::export::default_sidecar;
use masterout_core::i18n::{t, Lang};
use masterout_core::loudness::{LoudnessReport, Verdict};
use masterout_core::naming;
use masterout_core::plan::{self, MasterInfo, MediaInputs, ReleaseMeta, Resolved};
use masterout_core::presets;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Documentos de cada entrega (README, informe, ficha CSV y TXT).
const DOCUMENTS: u32 = 4;

fn non_empty(o: &Option<String>) -> Option<String> {
    o.as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Metadatos del motor desde el DTO (un año no numérico se ignora).
pub fn to_release_meta(meta: &ReleaseMetaDto) -> ReleaseMeta {
    ReleaseMeta {
        artist: meta.artist.trim().to_string(),
        title: meta.title.trim().to_string(),
        version: non_empty(&meta.version),
        featuring: non_empty(&meta.featuring),
        album: non_empty(&meta.album),
        year: non_empty(&meta.year).and_then(|y| y.parse().ok()),
        isrc: non_empty(&meta.isrc),
        composer: non_empty(&meta.composer),
        label: non_empty(&meta.label),
        copyright: non_empty(&meta.copyright),
        cover: non_empty(&meta.cover_path).map(PathBuf::from),
    }
}

/// Material propio del motor desde el DTO: la portada de los metadatos y las rutas por destino
/// (las vacías se ignoran).
pub fn to_media_inputs(meta: &ReleaseMetaDto, inputs: &HashMap<String, String>) -> MediaInputs {
    MediaInputs {
        cover: non_empty(&meta.cover_path).map(PathBuf::from),
        per_preset: inputs
            .iter()
            .filter(|(_, p)| !p.trim().is_empty())
            .map(|(id, p)| (id.clone(), PathBuf::from(p.trim())))
            .collect(),
    }
}

fn master_info(m: &MasterDto) -> MasterInfo {
    // El veredicto solo usa loudness integrado y true peak; el resto no viaja en el DTO.
    MasterInfo {
        sample_rate: m.sample_rate,
        bits: m.bits,
        is_float: m.is_float,
        channels: m.channels,
        frames: m.frames,
        loudness: LoudnessReport {
            integrated_lufs: m.integrated_lufs,
            lra_lu: m.lra_lu,
            true_peak_dbtp: m.true_peak_dbtp,
            sample_peak_dbfs: m.true_peak_dbtp,
            momentary_max_lufs: m.integrated_lufs,
            short_term_max_lufs: m.integrated_lufs,
            overs_count: m.overs_count,
        },
    }
}

fn verdict_parts(v: &Verdict, lang: Lang) -> (&'static str, String) {
    match v {
        Verdict::Ok => ("ok", t(lang, "verdict.ok")),
        Verdict::Warn(k) => ("warn", t(lang, k)),
        Verdict::Fail(k) => ("fail", t(lang, k)),
    }
}

/// Plan de la entrega. Comprueba si el sidecar de ffmpeg está disponible en este equipo.
pub fn plan(
    master: &MasterDto,
    meta: &ReleaseMetaDto,
    active_ids: &[String],
    media_inputs: &HashMap<String, String>,
    lang: Lang,
) -> PlanDto {
    plan_with(
        master,
        meta,
        active_ids,
        media_inputs,
        lang,
        default_sidecar().is_some(),
    )
}

/// Como [`plan`], con la disponibilidad del sidecar indicada (para probar sin ffmpeg).
pub fn plan_with(
    master: &MasterDto,
    meta: &ReleaseMetaDto,
    active_ids: &[String],
    media_inputs: &HashMap<String, String>,
    lang: Lang,
    sidecar: bool,
) -> PlanDto {
    let info = master_info(master);
    let mut rmeta = to_release_meta(meta);
    if rmeta.title.is_empty() {
        // Igual que `exporting::export`: el título sugerido, sin repetir el artista.
        rmeta.title = match &master.suggested_title {
            Some(t) => t.clone(),
            None => naming::fallback_title("", Path::new(&master.path)),
        };
        if rmeta.artist.is_empty() {
            rmeta.artist = master.suggested_artist.clone().unwrap_or_default();
        }
    }
    let ids: Vec<&str> = active_ids.iter().map(String::as_str).collect();
    let all = presets::all();

    let media = to_media_inputs(meta, media_inputs);
    let jobs: Vec<JobDto> = plan::resolve_in(&info, &rmeta, &ids, &media, lang, sidecar)
        .into_iter()
        .map(|r| match r {
            Resolved::Job(j) => {
                let (verdict, verdict_text) = verdict_parts(&j.verdict, lang);
                JobDto {
                    estimated_bytes: plan::estimate_bytes(
                        &j,
                        info.frames,
                        info.channels,
                        info.sample_rate,
                    ),
                    warnings: j.warnings.iter().map(|w| t(lang, w)).collect(),
                    extra_files: j.extra_files.clone(),
                    preset_id: j.preset_id,
                    subfolder: j.subfolder,
                    file_name: j.file_name,
                    container: j.container.extension().to_string(),
                    bits: j.bits,
                    sample_rate: j.sample_rate,
                    verdict: verdict.to_string(),
                    verdict_text,
                    enabled: true,
                    disabled_reason: None,
                }
            }
            // Sin archivo: bits y frecuencia a 0, veredicto "ok" sin texto; manda `disabled_reason`.
            Resolved::Disabled {
                preset_id,
                reason_key,
            } => {
                let preset = all.iter().find(|p| p.id == preset_id);
                JobDto {
                    subfolder: preset
                        .map(|p| naming::subfolder(&p.subfolder_key, lang))
                        .unwrap_or_default(),
                    file_name: String::new(),
                    container: preset
                        .map(|p| p.container.extension().to_string())
                        .unwrap_or_default(),
                    preset_id,
                    bits: 0,
                    sample_rate: 0,
                    verdict: "ok".into(),
                    verdict_text: String::new(),
                    estimated_bytes: 0,
                    enabled: false,
                    disabled_reason: Some(t(lang, &reason_key)),
                    warnings: Vec::new(),
                    extra_files: Vec::new(),
                }
            }
        })
        .collect();

    let enabled = jobs.iter().filter(|j| j.enabled).count() as u32;
    let extra_files = jobs.iter().map(|j| j.extra_files.len() as u32).sum::<u32>();
    PlanDto {
        delivery_folder: naming::delivery_folder(
            &rmeta.artist,
            &rmeta.title,
            rmeta.version.as_deref(),
        ),
        total_bytes: jobs.iter().map(|j| j.estimated_bytes).sum(),
        total_files: enabled + extra_files + DOCUMENTS,
        jobs,
    }
}
