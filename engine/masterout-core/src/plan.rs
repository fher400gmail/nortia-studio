//! Resolución del plan de entrega: qué archivo se produce para cada destino activo.

use crate::i18n::Lang;
use crate::loudness::{self, LoudnessReport, Verdict};
use crate::media::image as cover_image;
use crate::media::MediaCaps;
use crate::naming;
use crate::presets::{self, BitsPolicy, Container, Preset, RatePolicy};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct MasterInfo {
    pub sample_rate: u32,
    pub bits: u16,
    pub is_float: bool,
    pub channels: u16,
    pub frames: u64,
    pub loudness: LoudnessReport,
}

#[derive(Debug, Clone, Default)]
pub struct ReleaseMeta {
    pub artist: String,
    pub title: String,
    pub version: Option<String>,
    pub featuring: Option<String>,
    pub album: Option<String>,
    pub year: Option<u16>,
    pub isrc: Option<String>,
    pub composer: Option<String>,
    pub label: Option<String>,
    pub copyright: Option<String>,
    pub cover: Option<PathBuf>,
}

/// Material propio del usuario: la portada de la entrega y, por destino, el archivo
/// (imagen o vídeo) que sustituye al que se fabricaría desde la portada.
#[derive(Debug, Clone, Default)]
pub struct MediaInputs {
    pub cover: Option<PathBuf>,
    pub per_preset: HashMap<String, PathBuf>,
}

/// Tipo de destino que no es audio PCM/MP3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    /// JPG + PNG 3000×3000 sRGB.
    Cover,
    /// Spotify Canvas 1080×1920.
    Canvas,
    /// Vídeo de YouTube 1080p con el máster a 48 kHz.
    YoutubeVideo,
    /// Motion artwork de Apple Music: 3840×3840 y 2048×2732.
    AppleMotion,
}

impl MediaKind {
    pub fn of(preset_id: &str) -> Option<MediaKind> {
        match preset_id {
            "cover" => Some(MediaKind::Cover),
            "canvas" => Some(MediaKind::Canvas),
            "youtube_video" => Some(MediaKind::YoutubeVideo),
            "apple_motion" => Some(MediaKind::AppleMotion),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Job {
    pub preset_id: String,
    pub subfolder: String,
    pub file_name: String,
    pub container: Container,
    pub bits: u16,
    pub sample_rate: u32,
    pub needs_resample: bool,
    pub needs_dither: bool,
    pub is_upsampling: bool,
    pub embed_metadata: bool,
    pub verdict: Verdict,
    /// `Some` para portada, Canvas, YouTube y Apple Motion.
    pub media: Option<MediaKind>,
    /// Archivo propio del usuario para este destino (en vez de fabricarlo desde la portada).
    pub own_media: Option<PathBuf>,
    /// Nombres de los archivos adicionales del mismo destino (el PNG de la portada, el
    /// segundo tamaño de Apple Motion), en la misma subcarpeta.
    pub extra_files: Vec<String>,
    /// Apple Motion sin codificador HEVC: solo sale el archivo de 2048×2732 (H.264) y el de
    /// 3840×3840 se omite (el aviso `reason.hevc_unavailable` va en `warnings`).
    pub hevc_skipped: bool,
    /// Claves `warn.*` que se mostrarán en la interfaz y en el README.
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Resolved {
    Job(Job),
    Disabled {
        preset_id: String,
        reason_key: String,
    },
}

fn disabled(preset: &Preset, key: &str) -> Resolved {
    Resolved::Disabled {
        preset_id: preset.id.clone(),
        reason_key: key.to_string(),
    }
}

/// Resultado (memoizado por resolución) de mirar la cabecera de la portada.
type CoverCheck = Result<Vec<String>, String>;

struct Ctx<'a> {
    media: &'a MediaInputs,
    cover: Option<PathBuf>,
    caps: Option<MediaCaps>,
    cover_check: Option<CoverCheck>,
}

impl Ctx<'_> {
    fn check_cover(&mut self) -> CoverCheck {
        if self.cover_check.is_none() {
            let r = match &self.cover {
                Some(p) => cover_image::inspect_cover(p, &cover_image::CoverOptions::default())
                    .map_err(|e| e.key_and_detail().0),
                None => Ok(Vec::new()),
            };
            self.cover_check = Some(r);
        }
        self.cover_check.clone().unwrap_or(Ok(Vec::new()))
    }
}

fn resolve_media(
    master: &MasterInfo,
    meta: &ReleaseMeta,
    p: &Preset,
    kind: MediaKind,
    cx: &mut Ctx,
    lang: Lang,
) -> Resolved {
    let own = cx.media.per_preset.get(&p.id).cloned();
    // Sin sidecar no hay vídeo; la portada se hace con el crate `image`.
    if kind != MediaKind::Cover {
        match cx.caps {
            None => return disabled(p, "reason.no_sidecar"),
            Some(c) if !c.h264 => return disabled(p, "reason.encoder_unavailable"),
            Some(_) => {}
        }
    }
    let mut warnings = Vec::new();
    // La portada de la entrega solo hace falta si el destino no trae material propio.
    // En la ficha de portada, el material propio ES la portada.
    let cover_src: Option<PathBuf> = if kind == MediaKind::Cover {
        own.clone().or_else(|| cx.cover.clone())
    } else {
        None
    };
    if kind == MediaKind::Cover {
        match cover_src {
            None => return disabled(p, "reason.no_cover"),
            Some(path) => {
                match cover_image::inspect_cover(&path, &cover_image::CoverOptions::default()) {
                    Ok(w) => warnings = w,
                    Err(e) => return disabled(p, &e.key_and_detail().0),
                }
            }
        }
    } else if own.is_none() {
        if cx.cover.is_none() {
            return disabled(p, "reason.no_cover");
        }
        if let Err(key) = cx.check_cover() {
            return disabled(p, &key);
        }
    }

    let tag = naming::preset_tag(p, lang);
    let name = |suffix: &str, ext: &str| {
        naming::media_file_name(&meta.artist, &meta.title, &tag, suffix, ext)
    };
    let (file_name, extra_files) = match kind {
        MediaKind::Cover => (name("3000x3000", "jpg"), vec![name("3000x3000", "png")]),
        MediaKind::Canvas => (name("1080x1920", "mp4"), vec![]),
        MediaKind::YoutubeVideo => (name("1080p", "mp4"), vec![]),
        MediaKind::AppleMotion => (name("3840x3840", "mp4"), vec![name("2048x2732", "mp4")]),
    };
    // sin HEVC solo sale el tamaño vertical (H.264); el cuadrado se omite con aviso
    let hevc_skipped = kind == MediaKind::AppleMotion && cx.caps.is_some_and(|c| !c.hevc);
    let (file_name, extra_files) = if hevc_skipped {
        warnings.push("reason.hevc_unavailable".to_string());
        (name("2048x2732", "mp4"), Vec::new())
    } else {
        (file_name, extra_files)
    };
    let (bits, sample_rate, needs_resample, needs_dither, is_upsampling, verdict) =
        if kind == MediaKind::YoutubeVideo {
            // El audio del vídeo es el máster remuestreado a 48 kHz (con dither por el plan).
            let master_bits = if master.is_float { 32 } else { master.bits };
            let bits = master_bits.min(24);
            let rate = match p.rate {
                RatePolicy::Fixed(r) => r,
                RatePolicy::Native => master.sample_rate,
            };
            let resample = rate != master.sample_rate;
            let dither = bits < master_bits || master.is_float || (resample && bits <= 24);
            (
                bits,
                rate,
                resample,
                dither,
                rate > master.sample_rate,
                loudness::verdict(
                    &master.loudness,
                    p.target_lufs,
                    p.max_tp_dbtp.unwrap_or(0.0),
                ),
            )
        } else {
            (0, 0, false, false, false, Verdict::Ok)
        };
    Resolved::Job(Job {
        preset_id: p.id.clone(),
        subfolder: naming::subfolder(&p.subfolder_key, lang),
        file_name,
        container: p.container,
        bits,
        sample_rate,
        needs_resample,
        needs_dither,
        is_upsampling,
        embed_metadata: false,
        verdict,
        media: Some(kind),
        own_media: own,
        extra_files,
        hevc_skipped,
        warnings,
    })
}

fn resolve_one(
    master: &MasterInfo,
    meta: &ReleaseMeta,
    p: &Preset,
    cx: &mut Ctx,
    lang: Lang,
) -> Resolved {
    if let Some(kind) = MediaKind::of(&p.id) {
        return resolve_media(master, meta, p, kind, cx, lang);
    }
    // MP3 e imagen/vídeo comparten el sidecar (ffmpeg): sin él, solo el audio PCM sale.
    if p.container == Container::Mp3 && cx.caps.is_none() {
        return disabled(p, "reason.no_sidecar");
    }
    // Bits efectivos del máster: un float de 32 bit se entrega como mucho a 24.
    let master_bits = if master.is_float { 32 } else { master.bits };
    let native_bits = master_bits.min(24);

    if matches!(p.bits, BitsPolicy::Native) {
        if let Some(min) = p.min_bits {
            if native_bits < min {
                let key = p
                    .min_bits_reason_key
                    .clone()
                    .unwrap_or_else(|| "min_bits_not_met".into());
                return disabled(p, &key);
            }
        }
    }

    let bits = match p.bits {
        BitsPolicy::Native => native_bits,
        BitsPolicy::Fixed(b) => b,
    };
    let sample_rate = match p.rate {
        RatePolicy::Native => master.sample_rate,
        RatePolicy::Fixed(r) => r,
    };
    let container = p.container;
    let is_audio_pcm = matches!(container, Container::Wav | Container::Flac);
    let needs_resample = sample_rate != master.sample_rate;
    // Regla de dither: bits destino < bits origen, origen float, o remuestreo a <= 24 bit.
    // Dither si baja de bits, si el máster es float, o si se remuestrea a <= 24 bit
    // (el audio remuestreado tiene más precisión que un entero).
    // El MP3 parte de la versión de 16 bit / 44,1 kHz con dither, igual que un PCM de 16 bit,
    // y comparte con ellos la misma entrada de caché.
    let dithers = is_audio_pcm || container == Container::Mp3;
    let needs_dither =
        dithers && (bits < master_bits || master.is_float || (needs_resample && bits <= 24));
    let is_upsampling = sample_rate > master.sample_rate || (is_audio_pcm && bits > master_bits);

    let tag = naming::preset_tag(p, lang);
    let file_name = if container == Container::Mp3 {
        naming::media_file_name(&meta.artist, &meta.title, &p.tag, "", container.extension())
    } else {
        naming::audio_file_name(
            &meta.artist,
            &meta.title,
            &tag,
            bits,
            sample_rate,
            container.extension(),
        )
    };
    let verdict = loudness::verdict(
        &master.loudness,
        p.target_lufs,
        p.max_tp_dbtp.unwrap_or(0.0),
    );
    Resolved::Job(Job {
        preset_id: p.id.clone(),
        subfolder: naming::subfolder(&p.subfolder_key, lang),
        file_name,
        container,
        bits,
        sample_rate,
        needs_resample,
        needs_dither,
        is_upsampling,
        embed_metadata: p.embed_metadata,
        verdict,
        media: None,
        own_media: None,
        extra_files: Vec::new(),
        hevc_skipped: false,
        warnings: Vec::new(),
    })
}

/// Resuelve los destinos activos con los nombres del contrato en español y con el sidecar
/// de ffmpeg disponible.
pub fn resolve(
    master: &MasterInfo,
    meta: &ReleaseMeta,
    active_ids: &[&str],
    media: &MediaInputs,
) -> Vec<Resolved> {
    resolve_in(
        master,
        meta,
        active_ids,
        media,
        Lang::Es,
        Some(MediaCaps::ALL),
    )
}

/// Resuelve los destinos activos, en el orden de las subcarpetas, con los nombres
/// de carpeta y etiquetas del idioma pedido. `caps`: `None` si no hay ffmpeg/ffprobe (se
/// deshabilitan, con motivo, las fichas de vídeo y el MP3; el resto sale igual); si hay, los
/// codificadores que funcionan (sin H.264 no hay vídeo; sin HEVC, Apple Motion omite 3840×3840).
pub fn resolve_in(
    master: &MasterInfo,
    meta: &ReleaseMeta,
    active_ids: &[&str],
    media: &MediaInputs,
    lang: Lang,
    caps: Option<MediaCaps>,
) -> Vec<Resolved> {
    let mut cx = Ctx {
        media,
        cover: media.cover.clone().or_else(|| meta.cover.clone()),
        caps,
        cover_check: None,
    };
    presets::all()
        .iter()
        .filter(|p| active_ids.contains(&p.id.as_str()))
        .map(|p| resolve_one(master, meta, p, &mut cx, lang))
        .collect()
}

/// Tamaño estimado del archivo de salida en bytes. WAV exacto (sin cabecera);
/// FLAC aproximado (0,62 x WAV); MP3 320 kbps = 40 kB/s; imagen y vídeo, orden de magnitud
/// según la tasa de cada destino. `frames` y `master_rate` son los del máster.
pub fn estimate_bytes(job: &Job, frames: u64, channels: u16, master_rate: u32) -> u64 {
    let secs = frames / u64::from(master_rate.max(1));
    match job.media {
        // JPG 92 % + PNG 8 bit de 3000×3000
        Some(MediaKind::Cover) => return 12_000_000,
        // 6 s a 5 Mbps
        Some(MediaKind::Canvas) => return 4_000_000,
        // 2 archivos de 10 s a 60 Mbps
        Some(MediaKind::AppleMotion) => return 150_000_000,
        // vídeo a 8 Mbps + audio AAC a 384 kbps
        Some(MediaKind::YoutubeVideo) => return secs * (8_000_000 + 384_000) / 8,
        None => {}
    }
    // fotogramas del archivo de salida (puede estar remuestreado)
    let out_frames = frames * u64::from(job.sample_rate) / u64::from(master_rate.max(1));
    let wav = out_frames * u64::from(channels) * u64::from(job.bits) / 8;
    match job.container {
        Container::Wav => wav,
        Container::Flac => (wav as f64 * 0.62) as u64,
        Container::Mp3 => secs * 40_000,
        _ => 0,
    }
}
