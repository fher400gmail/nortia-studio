//! Documentos de la entrega: README, informe de loudness y ficha de lanzamiento.

use crate::i18n::{t, tf, Lang};
use crate::loudness::Verdict;
use crate::media::adm::{AdmReport, AdmStatus};
use crate::plan::{Job, MasterInfo, MediaKind, ReleaseMeta, Resolved};
use crate::presets::Container;

pub struct DeliverySummary<'a> {
    pub meta: &'a ReleaseMeta,
    pub master: &'a MasterInfo,
    pub jobs: &'a [Resolved],
    /// ISO local.
    pub created_at: String,
    pub app_version: &'a str,
}

fn localize(lang: Lang, s: String) -> String {
    match lang {
        Lang::En => s,
        Lang::Es => s.replace('.', ",").replace('-', "\u{2212}"),
    }
}

/// Un decimal; en español coma decimal y signo menos Unicode.
fn num1(lang: Lang, v: f64) -> String {
    let mut s = format!("{v:.1}");
    if s == "-0.0" {
        s = "0.0".into();
    }
    localize(lang, s)
}

fn khz(lang: Lang, rate: u32) -> String {
    localize(lang, format!("{}", f64::from(rate) / 1000.0))
}

fn preset_name(lang: Lang, id: &str) -> String {
    t(lang, &format!("preset.{id}"))
}

fn container_label(j: &Job) -> String {
    j.container.extension().to_uppercase()
}

/// Ficha técnica de un destino para el README (audio, MP3, imagen y vídeo).
fn job_spec(lang: Lang, j: &Job) -> String {
    match j.media {
        Some(MediaKind::Cover) => "JPG + PNG · 3000×3000 · sRGB".to_string(),
        Some(MediaKind::Canvas) => "MP4 · 1080×1920 · H.264".to_string(),
        Some(MediaKind::YoutubeVideo) => format!(
            "MP4 · 1080p · H.264 + AAC 384 kbps · {} kHz",
            khz(lang, j.sample_rate)
        ),
        Some(MediaKind::AppleMotion) => "MP4 · 3840×3840 (HEVC) + 2048×2732 (H.264)".to_string(),
        None if j.container == Container::Mp3 => "MP3 320 kbps".to_string(),
        None => format!(
            "{} bit · {} kHz · {}",
            j.bits,
            khz(lang, j.sample_rate),
            container_label(j)
        ),
    }
}

fn verdict_text(lang: Lang, v: &Verdict) -> String {
    match v {
        Verdict::Ok => t(lang, "verdict.ok"),
        Verdict::Warn(k) | Verdict::Fail(k) => t(lang, k),
    }
}

fn verdict_prefix(lang: Lang, v: &Verdict) -> String {
    t(
        lang,
        match v {
            Verdict::Ok => "verdict.tag.ok",
            Verdict::Warn(_) => "verdict.tag.warn",
            Verdict::Fail(_) => "verdict.tag.fail",
        },
    )
}

fn header(s: &DeliverySummary) -> String {
    format!("{} - {}", s.meta.artist, s.meta.title)
}

pub fn readme(lang: Lang, s: &DeliverySummary) -> String {
    let mut out = String::new();
    out.push_str(&header(s));
    out.push_str("\n\n");
    out.push_str(&t(lang, "readme.folders"));
    out.push('\n');
    for r in s.jobs {
        match r {
            Resolved::Job(j) => {
                let spec = job_spec(lang, j);
                out.push_str(&format!("{}/  →  {}  ({})", j.subfolder, j.file_name, spec));
                if j.is_upsampling {
                    out.push_str(&format!("  ⚠ {}", t(lang, "readme.upsampling")));
                }
                out.push('\n');
                for extra in &j.extra_files {
                    out.push_str(&format!("{}/  →  {}\n", j.subfolder, extra));
                }
                for w in &j.warnings {
                    out.push_str(&format!("    ⚠ {}\n", t(lang, w)));
                }
            }
            Resolved::Disabled {
                preset_id,
                reason_key,
            } => {
                out.push_str(&format!(
                    "{}  →  {}: {}\n",
                    preset_name(lang, preset_id),
                    t(lang, "readme.disabled"),
                    t(lang, reason_key)
                ));
            }
        }
    }
    out.push('\n');
    out.push_str(&t(lang, "readme.master_untouched"));
    out.push('\n');
    out.push_str(&tf(
        lang,
        "readme.generated",
        &[("version", s.app_version), ("date", &s.created_at)],
    ));
    out.push('\n');
    out
}

/// Veredicto de un destino con el resultado de Apple Digital Masters: si el AAC tiene clipping,
/// el destino `apple_adm` pasa a `Warn("adm_clipping")` (un `Fail` previo se conserva).
pub fn verdict_with_adm(preset_id: &str, v: &Verdict, adm: Option<&AdmReport>) -> Verdict {
    match adm {
        Some(a)
            if preset_id == "apple_adm"
                && a.status == AdmStatus::Clipping
                && !matches!(v, Verdict::Fail(_)) =>
        {
            Verdict::Warn("adm_clipping".into())
        }
        _ => v.clone(),
    }
}

/// Línea del informe para el resultado de Apple Digital Masters.
pub fn adm_line(lang: Lang, a: &AdmReport) -> String {
    match &a.status {
        AdmStatus::Unavailable { reason_key } => {
            // minúscula inicial: va detrás de los dos puntos
            let reason = t(lang, reason_key);
            let mut cs = reason.chars();
            let reason = match cs.next() {
                Some(c) => c.to_lowercase().chain(cs).collect(),
                None => reason,
            };
            tf(lang, "report.adm_unavailable", &[("reason", &reason)])
        }
        _ => tf(
            lang,
            if a.clipped_samples == 1 {
                "report.adm_clipping_one"
            } else {
                "report.adm_clipping"
            },
            &[("n", &a.clipped_samples.to_string())],
        ),
    }
}

pub fn loudness_report(lang: Lang, s: &DeliverySummary) -> String {
    loudness_report_with_adm(lang, s, None)
}

/// Informe de loudness con el resultado de la validación Apple Digital Masters (si se hizo).
pub fn loudness_report_with_adm(
    lang: Lang,
    s: &DeliverySummary,
    adm: Option<&AdmReport>,
) -> String {
    let l = &s.master.loudness;
    let mut out = String::new();
    out.push_str(&format!("{}\n{}\n\n", t(lang, "report.title"), header(s)));
    out.push_str(&format!(
        "{}: {} LUFS\n",
        t(lang, "report.integrated"),
        num1(lang, l.integrated_lufs)
    ));
    out.push_str(&format!(
        "{}: {} LU\n",
        t(lang, "report.lra"),
        num1(lang, l.lra_lu)
    ));
    out.push_str(&format!(
        "{}: {} dBTP\n",
        t(lang, "report.true_peak"),
        num1(lang, l.true_peak_dbtp)
    ));
    out.push_str(&format!(
        "{}: {} dBFS\n",
        t(lang, "report.sample_peak"),
        num1(lang, l.sample_peak_dbfs)
    ));
    out.push_str(&format!(
        "{}: {}\n\n",
        t(lang, "report.overs"),
        l.overs_count
    ));
    out.push_str(&t(lang, "report.destinations"));
    out.push('\n');
    for r in s.jobs {
        match r {
            // La imagen y el vídeo sin audio no tienen veredicto de loudness.
            Resolved::Job(j)
                if matches!(
                    j.media,
                    Some(MediaKind::Cover | MediaKind::Canvas | MediaKind::AppleMotion)
                ) => {}
            Resolved::Job(j) => out.push_str(&format!(
                "{}: [{}] {}\n",
                preset_name(lang, &j.preset_id),
                verdict_prefix(lang, &verdict_with_adm(&j.preset_id, &j.verdict, adm)),
                verdict_text(lang, &verdict_with_adm(&j.preset_id, &j.verdict, adm))
            )),
            Resolved::Disabled {
                preset_id,
                reason_key,
            } => out.push_str(&format!(
                "{}: {}\n",
                preset_name(lang, preset_id),
                t(lang, reason_key)
            )),
        }
    }
    if let Some(a) = adm {
        out.push('\n');
        out.push_str(&adm_line(lang, a));
        out.push('\n');
    }
    out
}

fn csv_field(v: &str) -> String {
    if v.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", v.replace('"', "\"\""))
    } else {
        v.to_string()
    }
}

fn dot1(v: f64) -> String {
    format!("{v:.1}")
}

pub fn release_sheet_csv(s: &DeliverySummary) -> String {
    let m = s.meta;
    let ms = s.master;
    let opt = |o: &Option<String>| o.clone().unwrap_or_default();
    let duration = if ms.sample_rate == 0 {
        0.0
    } else {
        ms.frames as f64 / f64::from(ms.sample_rate)
    };
    let fields = [
        m.artist.clone(),
        m.title.clone(),
        opt(&m.version),
        opt(&m.featuring),
        opt(&m.album),
        m.year.map(|y| y.to_string()).unwrap_or_default(),
        opt(&m.isrc),
        opt(&m.composer),
        opt(&m.label),
        opt(&m.copyright),
        dot1(duration),
        ms.sample_rate.to_string(),
        ms.bits.to_string(),
        dot1(ms.loudness.integrated_lufs),
        dot1(ms.loudness.true_peak_dbtp),
        dot1(ms.loudness.lra_lu),
    ];
    let row: Vec<String> = fields.iter().map(|f| csv_field(f)).collect();
    format!(
        "artist,title,version,featuring,album,year,isrc,composer,label,copyright,duration_sec,sample_rate,bits,integrated_lufs,true_peak_dbtp,lra_lu\n{}\n",
        row.join(",")
    )
}

pub fn release_sheet_txt(lang: Lang, s: &DeliverySummary) -> String {
    let m = s.meta;
    let ms = s.master;
    let l = &ms.loudness;
    let opt = |o: &Option<String>| o.clone().unwrap_or_default();
    let duration = if ms.sample_rate == 0 {
        0.0
    } else {
        ms.frames as f64 / f64::from(ms.sample_rate)
    };
    let rows: Vec<(String, String)> = vec![
        (t(lang, "sheet.artist"), m.artist.clone()),
        (t(lang, "sheet.track_title"), m.title.clone()),
        (t(lang, "sheet.version"), opt(&m.version)),
        (t(lang, "sheet.featuring"), opt(&m.featuring)),
        (t(lang, "sheet.album"), opt(&m.album)),
        (
            t(lang, "sheet.year"),
            m.year.map(|y| y.to_string()).unwrap_or_default(),
        ),
        (t(lang, "sheet.isrc"), opt(&m.isrc)),
        (t(lang, "sheet.composer"), opt(&m.composer)),
        (t(lang, "sheet.label"), opt(&m.label)),
        (t(lang, "sheet.copyright"), opt(&m.copyright)),
        (
            t(lang, "sheet.duration"),
            format!(
                "{} {}",
                localize(lang, format!("{duration:.1}")),
                t(lang, "sheet.seconds")
            ),
        ),
        (
            t(lang, "sheet.master"),
            format!(
                "{} bit · {} kHz · {} LUFS · {} dBTP · {} LU",
                ms.bits,
                khz(lang, ms.sample_rate),
                num1(lang, l.integrated_lufs),
                num1(lang, l.true_peak_dbtp),
                num1(lang, l.lra_lu)
            ),
        ),
    ];
    let mut out = format!("{}\n{}\n\n", t(lang, "sheet.title"), header(s));
    for (k, v) in rows {
        out.push_str(&format!("{k}: {v}\n"));
    }
    out
}
