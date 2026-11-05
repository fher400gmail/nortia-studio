//! Plan de entrega para la interfaz: qué se va a producir y qué destinos no están disponibles.

use crate::dto::{JobDto, MasterDto, PlanDto, ReleaseMetaDto};
use masterout_core::i18n::{t, Lang};
use masterout_core::loudness::{LoudnessReport, Verdict};
use masterout_core::naming;
use masterout_core::plan::{self, MasterInfo, ReleaseMeta, Resolved};
use masterout_core::presets;
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

pub fn plan(
    master: &MasterDto,
    meta: &ReleaseMetaDto,
    active_ids: &[String],
    lang: Lang,
) -> PlanDto {
    let info = master_info(master);
    let mut rmeta = to_release_meta(meta);
    rmeta.title = naming::fallback_title(&rmeta.title, Path::new(&master.path));
    let ids: Vec<&str> = active_ids.iter().map(String::as_str).collect();
    let all = presets::all();

    let jobs: Vec<JobDto> = plan::resolve_in(&info, &rmeta, &ids, lang)
        .into_iter()
        .map(|r| match r {
            Resolved::Job(j) => {
                let (verdict, verdict_text) = verdict_parts(&j.verdict, lang);
                JobDto {
                    estimated_bytes: plan::estimate_bytes(&j, info.frames, info.channels),
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
                }
            }
        })
        .collect();

    let enabled = jobs.iter().filter(|j| j.enabled).count() as u32;
    PlanDto {
        delivery_folder: naming::delivery_folder(
            &rmeta.artist,
            &rmeta.title,
            rmeta.version.as_deref(),
        ),
        total_bytes: jobs.iter().map(|j| j.estimated_bytes).sum(),
        total_files: enabled + DOCUMENTS,
        jobs,
    }
}
