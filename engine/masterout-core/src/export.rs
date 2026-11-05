//! Exportación de punta a punta: análisis del máster, plan, archivos de audio en paralelo
//! y documentos de la entrega. Cada archivo se escribe primero a un temporal `.part` en su
//! carpeta y se renombra al terminar bien, de modo que nunca quedan archivos a medias.

use crate::audio::{AudioBuffer, AudioError};
use crate::dither;
use crate::flac::{self, FlacTags};
use crate::i18n::Lang;
use crate::loudness;
use crate::mp3::{self, Id3Tags};
use crate::naming;
use crate::plan::{self, Job, MasterInfo, ReleaseMeta, Resolved};
use crate::presets::Container;
use crate::reports::{self, DeliverySummary};
use crate::resample;
use crate::wav::{self, WavMetadata, WavWriteOptions};
use rayon::prelude::*;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct ExportRequest {
    pub master_path: PathBuf,
    pub meta: ReleaseMeta,
    pub active_ids: Vec<String>,
    pub out_root: PathBuf,
    pub lang: Lang,
    /// `None`: núcleos físicos.
    pub threads: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum Progress {
    Analyzed(MasterInfo),
    Planned(Vec<Resolved>),
    JobStarted {
        preset_id: String,
    },
    JobProgress {
        preset_id: String,
        fraction: f32,
    },
    JobDone {
        preset_id: String,
        path: PathBuf,
        bytes: u64,
    },
    JobFailed {
        preset_id: String,
        error: String,
    },
    Finished(Delivery),
}

#[derive(Debug, Clone)]
pub struct Delivery {
    pub folder: PathBuf,
    /// Todos los archivos escritos (audio y documentos) con su tamaño en bytes.
    pub files: Vec<(PathBuf, u64)>,
    /// `(preset_id, mensaje de error)` de los destinos que fallaron.
    pub failed: Vec<(String, String)>,
    pub elapsed_ms: u128,
}

fn unsupported(msg: impl Into<String>) -> AudioError {
    AudioError::UnsupportedFormat(msg.into())
}

/// Lee el máster y mide su loudness.
pub fn analyze(master_path: &Path) -> Result<(AudioBuffer, MasterInfo), AudioError> {
    let buf = wav::read(master_path)?;
    let loudness = loudness::measure(&buf)?;
    let info = MasterInfo {
        sample_rate: buf.sample_rate,
        bits: buf.source_bits,
        is_float: buf.source_float,
        channels: buf.num_channels() as u16,
        frames: buf.frames() as u64,
        loudness,
    };
    Ok((buf, info))
}

/// Fecha y hora UTC `(año, mes, día, hora, minuto, segundo)` del instante actual.
fn now_utc() -> (u16, u8, u8, u8, u8, u8) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs()) as i64;
    let (days, rem) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    // Días desde 1970-01-01 a fecha civil (algoritmo de H. Hinnant).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    (
        y as u16,
        m as u8,
        d as u8,
        (rem / 3600) as u8,
        (rem % 3600 / 60) as u8,
        (rem % 60) as u8,
    )
}

fn iso(t: (u16, u8, u8, u8, u8, u8)) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        t.0, t.1, t.2, t.3, t.4, t.5
    )
}

/// Crea `root/base`, o `root/base (2)`, `(3)`… si ya existe.
fn create_delivery_folder(root: &Path, base: &str) -> Result<PathBuf, AudioError> {
    std::fs::create_dir_all(root)?;
    for n in 1u32.. {
        let name = if n == 1 {
            base.to_string()
        } else {
            format!("{base} ({n})")
        };
        let path = root.join(name);
        match std::fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e.into()),
        }
    }
    unreachable!()
}

/// Escribe con `f` a `final_path.part` y lo renombra si todo fue bien; si no, lo borra.
fn atomic(
    final_path: &Path,
    f: impl FnOnce(&Path) -> Result<(), AudioError>,
) -> Result<u64, AudioError> {
    let mut tmp = final_path.as_os_str().to_owned();
    tmp.push(".part");
    let tmp = PathBuf::from(tmp);
    let r = f(&tmp).and_then(|()| std::fs::rename(&tmp, final_path).map_err(Into::into));
    if r.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    r?;
    Ok(std::fs::metadata(final_path)?.len())
}

struct Ctx<'a> {
    buf: &'a AudioBuffer,
    meta: &'a ReleaseMeta,
    folder: &'a Path,
    stamp: (u16, u8, u8, u8, u8, u8),
    /// Versión 16 bit / 44,1 kHz, calculada una sola vez y compartida por Deezer,
    /// distribuidora estándar y MP3.
    cd: OnceLock<Result<Vec<Vec<i32>>, String>>,
}

impl Ctx<'_> {
    fn cd_samples(&self) -> Result<&[Vec<i32>], AudioError> {
        self.cd
            .get_or_init(|| {
                resample::resample(self.buf, 44_100)
                    .map(|r| {
                        dither::quantize(
                            &r.channels,
                            16,
                            self.buf.source_bits,
                            self.buf.source_float,
                        )
                        .0
                    })
                    .map_err(|e| e.to_string())
            })
            .as_ref()
            .map(Vec::as_slice)
            .map_err(|e| unsupported(e.clone()))
    }

    fn samples(&self, job: &Job) -> Result<Cow<'_, [Vec<i32>]>, AudioError> {
        if job.bits == 16 && job.sample_rate == 44_100 {
            return Ok(Cow::Borrowed(self.cd_samples()?));
        }
        let resampled;
        let channels = if job.needs_resample {
            resampled = resample::resample(self.buf, job.sample_rate)?;
            &resampled.channels
        } else {
            &self.buf.channels
        };
        let (q, _) = dither::quantize(
            channels,
            job.bits,
            self.buf.source_bits,
            self.buf.source_float,
        );
        Ok(Cow::Owned(q))
    }

    fn wav_metadata(&self) -> WavMetadata {
        WavMetadata {
            title: self.meta.title.clone(),
            artist: self.meta.artist.clone(),
            album: self.meta.album.clone(),
            year: self.meta.year,
            bext_description: format!("{} - {}", self.meta.artist, self.meta.title),
            bext_originator: "nortia. studio".into(),
            origination: Some(self.stamp),
        }
    }

    fn flac_tags(&self) -> FlacTags {
        FlacTags {
            title: self.meta.title.clone(),
            artist: self.meta.artist.clone(),
            album: self.meta.album.clone(),
            date: self.meta.year.map(|y| y.to_string()),
            isrc: self.meta.isrc.clone(),
            copyright: self.meta.copyright.clone(),
        }
    }

    fn id3_tags(&self) -> Id3Tags {
        Id3Tags {
            title: self.meta.title.clone(),
            artist: self.meta.artist.clone(),
            album: self.meta.album.clone(),
            year: self.meta.year,
            comment: Some("nortia. studio - master check".into()),
            isrc: self.meta.isrc.clone(),
            composer: self.meta.composer.clone(),
            label: self.meta.label.clone(),
            copyright: self.meta.copyright.clone(),
        }
    }

    fn render(&self, job: &Job, step: &dyn Fn(f32)) -> Result<(PathBuf, u64), AudioError> {
        let dir = self.folder.join(&job.subfolder);
        let path = dir.join(&job.file_name);
        let lame = if job.container == Container::Mp3 {
            Some(mp3::find_lame().ok_or_else(|| unsupported("lame not found"))?)
        } else {
            None
        };
        let samples = match job.container {
            Container::Wav | Container::Flac | Container::Mp3 => {
                if job.container == Container::Mp3 {
                    Cow::Borrowed(self.cd_samples()?)
                } else {
                    self.samples(job)?
                }
            }
            other => return Err(unsupported(format!("contenedor {}", other.extension()))),
        };
        step(0.6);
        std::fs::create_dir_all(&dir)?;
        let bytes = match job.container {
            Container::Wav => atomic(&path, |tmp| {
                let opts = WavWriteOptions {
                    bits: job.bits,
                    metadata: job.embed_metadata.then(|| self.wav_metadata()),
                };
                wav::write(tmp, &samples, job.sample_rate, &opts)
            })?,
            Container::Flac => atomic(&path, |tmp| {
                let tags = job.embed_metadata.then(|| self.flac_tags());
                flac::write(tmp, &samples, job.sample_rate, job.bits, tags.as_ref())
            })?,
            _ => atomic(&path, |tmp| {
                let mut wav_tmp = tmp.as_os_str().to_owned();
                wav_tmp.push(".wav");
                let wav_tmp = PathBuf::from(wav_tmp);
                let opts = WavWriteOptions {
                    bits: 16,
                    metadata: None,
                };
                let r = wav::write(&wav_tmp, &samples, 44_100, &opts).and_then(|()| {
                    step(0.8);
                    let lame = lame.as_deref().expect("lame comprobado");
                    mp3::encode_with(
                        lame,
                        &wav_tmp,
                        tmp,
                        &self.id3_tags(),
                        self.meta.cover.as_deref(),
                    )
                });
                let _ = std::fs::remove_file(&wav_tmp);
                r
            })?,
        };
        Ok((path, bytes))
    }
}

fn write_text(path: &Path, text: &str) -> Result<u64, AudioError> {
    atomic(path, |tmp| std::fs::write(tmp, text).map_err(Into::into))
}

/// Ejecuta la exportación completa y devuelve la entrega.
pub fn run(
    req: &ExportRequest,
    on_progress: &(dyn Fn(Progress) + Sync),
) -> Result<Delivery, AudioError> {
    let t0 = Instant::now();
    let (buf, info) = analyze(&req.master_path)?;
    on_progress(Progress::Analyzed(info.clone()));

    let mut meta = req.meta.clone();
    meta.title = naming::fallback_title(&meta.title, &req.master_path);
    let ids: Vec<&str> = req.active_ids.iter().map(String::as_str).collect();
    let plan = plan::resolve(&info, &meta, &ids);
    on_progress(Progress::Planned(plan.clone()));

    let folder = create_delivery_folder(
        &req.out_root,
        &naming::delivery_folder(&meta.artist, &meta.title, meta.version.as_deref()),
    )?;
    let stamp = now_utc();
    let ctx = Ctx {
        buf: &buf,
        meta: &meta,
        folder: &folder,
        stamp,
        cd: OnceLock::new(),
    };

    let jobs: Vec<&Job> = plan
        .iter()
        .filter_map(|r| match r {
            Resolved::Job(j) => Some(j),
            Resolved::Disabled { .. } => None,
        })
        .collect();
    let threads = req.threads.unwrap_or_else(num_cpus::get_physical).max(1);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .map_err(|e| unsupported(format!("rayon: {e}")))?;
    let results: Vec<Result<(PathBuf, u64), String>> = pool.install(|| {
        jobs.par_iter()
            .map(|job| {
                let id = job.preset_id.clone();
                on_progress(Progress::JobStarted {
                    preset_id: id.clone(),
                });
                let step = |fraction: f32| {
                    on_progress(Progress::JobProgress {
                        preset_id: id.clone(),
                        fraction,
                    })
                };
                step(0.0);
                match ctx.render(job, &step) {
                    Ok((path, bytes)) => {
                        step(1.0);
                        on_progress(Progress::JobDone {
                            preset_id: id,
                            path: path.clone(),
                            bytes,
                        });
                        Ok((path, bytes))
                    }
                    Err(e) => {
                        let error = e.to_string();
                        on_progress(Progress::JobFailed {
                            preset_id: id,
                            error: error.clone(),
                        });
                        Err(error)
                    }
                }
            })
            .collect()
    });

    let mut files = Vec::new();
    let mut failed = Vec::new();
    for (job, r) in jobs.iter().zip(results) {
        match r {
            Ok(f) => files.push(f),
            Err(e) => failed.push((job.preset_id.clone(), e)),
        }
    }
    // Sin subcarpetas vacías (p. ej. si todos los jobs de una carpeta fallaron).
    for job in &jobs {
        let _ = std::fs::remove_dir(folder.join(&job.subfolder));
    }

    let summary = DeliverySummary {
        meta: &meta,
        master: &info,
        jobs: &plan,
        created_at: iso(stamp),
        app_version: env!("CARGO_PKG_VERSION"),
    };
    for (name, text) in [
        ("README.txt", reports::readme(req.lang, &summary)),
        (
            "informe-loudness.txt",
            reports::loudness_report(req.lang, &summary),
        ),
        (
            "ficha-lanzamiento.csv",
            reports::release_sheet_csv(&summary),
        ),
        (
            "ficha-lanzamiento.txt",
            reports::release_sheet_txt(req.lang, &summary),
        ),
    ] {
        let path = folder.join(name);
        let bytes = write_text(&path, &text)?;
        files.push((path, bytes));
    }

    let delivery = Delivery {
        folder,
        files,
        failed,
        elapsed_ms: t0.elapsed().as_millis(),
    };
    on_progress(Progress::Finished(delivery.clone()));
    Ok(delivery)
}
