//! Exportación de punta a punta: análisis del máster, plan, archivos de audio en paralelo
//! y documentos de la entrega. Cada archivo se escribe primero a un temporal `.part` en su
//! carpeta y se renombra al terminar bien, de modo que nunca quedan archivos a medias.

use crate::audio::{AudioBuffer, AudioError};
use crate::dither::{self, Dither};
use crate::flac::{self, FlacTags};
use crate::i18n::{t, Lang};
use crate::loudness;
use crate::media::image::{self as cover_image, CoverOptions, CoverOutput};
use crate::media::mp3::{self, Id3Tags};
use crate::media::video::{
    self, run_ffmpeg, AppleMotionSpec, CanvasSpec, VideoSource, YoutubeSpec,
};
use crate::media::{self, Platform, SidecarPaths};
use crate::naming;
use crate::plan::{self, Job, MasterInfo, MediaInputs, MediaKind, ReleaseMeta, Resolved};
use crate::presets::Container;
use crate::reports::{self, DeliverySummary};
use crate::resample;
use crate::wav::{self, WavMetadata, WavWriteOptions};
use rayon::prelude::*;
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct ExportRequest {
    pub master_path: PathBuf,
    pub meta: ReleaseMeta,
    pub active_ids: Vec<String>,
    pub out_root: PathBuf,
    pub lang: Lang,
    /// `None`: núcleos físicos.
    pub threads: Option<usize>,
    /// Si se activa, la exportación se detiene entre pasos: los archivos ya completos se
    /// conservan, no quedan temporales y se emite `Progress::Cancelled`.
    pub cancel: Option<Arc<AtomicBool>>,
    /// Portada y material propio por ficha (imagen o vídeo).
    pub media: MediaInputs,
    /// ffmpeg/ffprobe; `None` deshabilita las fichas de vídeo y el MP3 (el audio sale igual).
    pub sidecar: Option<SidecarPaths>,
    /// Codificadores de vídeo a usar (solo las pruebas cambian el de la plataforma real).
    pub platform: Platform,
}

impl ExportRequest {
    /// Petición con hilos automáticos y sin cancelación.
    pub fn new(
        master_path: PathBuf,
        meta: ReleaseMeta,
        active_ids: Vec<String>,
        out_root: PathBuf,
        lang: Lang,
    ) -> Self {
        ExportRequest {
            master_path,
            meta,
            active_ids,
            out_root,
            lang,
            threads: None,
            cancel: None,
            media: MediaInputs::default(),
            sidecar: default_sidecar(),
            platform: Platform::current(),
        }
    }

    pub fn with_media(mut self, media: MediaInputs) -> Self {
        self.media = media;
        self
    }

    pub fn with_sidecar(mut self, sidecar: Option<SidecarPaths>) -> Self {
        self.sidecar = sidecar;
        self
    }

    pub fn with_platform(mut self, platform: Platform) -> Self {
        self.platform = platform;
        self
    }

    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = Some(threads);
        self
    }

    pub fn with_cancel(mut self, cancel: Arc<AtomicBool>) -> Self {
        self.cancel = Some(cancel);
        self
    }
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
        /// Clave i18n del error (`err.*`); la interfaz la traduce.
        error: String,
        /// Detalle técnico sin localizar, si lo hay.
        detail: Option<String>,
    },
    /// Exportación cancelada; `files_kept` son los archivos de audio completos que se conservan.
    Cancelled {
        files_kept: u32,
    },
    Finished(Delivery),
}

#[derive(Debug, Clone)]
pub struct Delivery {
    pub folder: PathBuf,
    /// Todos los archivos escritos (audio y documentos) con su tamaño en bytes.
    pub files: Vec<(PathBuf, u64)>,
    /// `(preset_id, clave de error)` de los destinos que fallaron (`"documents"` para los documentos).
    pub failed: Vec<(String, String)>,
    /// `(preset_id, detalle técnico)` de los que tienen detalle.
    pub failed_details: Vec<(String, String)>,
    /// La exportación se canceló; `files` son los archivos completos que se conservan.
    pub cancelled: bool,
    pub elapsed_ms: u128,
}

/// Sidecar junto al ejecutable actual, en `MASTERCHECK_FFMPEG`/`MASTERCHECK_FFPROBE` o en PATH.
pub fn default_sidecar() -> Option<SidecarPaths> {
    let dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_default();
    media::discover(&dir)
}

/// Semilla fija del dither TPDF: mismo audio de entrada, mismo archivo de salida.
const DITHER_SEED: u64 = 0x4D41_5354_4552_4F55;

fn unsupported(msg: impl Into<String>) -> AudioError {
    AudioError::UnsupportedFormat(msg.into())
}

/// Lee el máster y mide su loudness.
pub fn analyze(master_path: &Path) -> Result<(AudioBuffer, MasterInfo), AudioError> {
    let buf = wav::read(master_path)?;
    if buf.sample_rate > 96_000 {
        // Clave i18n `err.rate_over_96k`: la interfaz la traduce.
        return Err(unsupported("err.rate_over_96k"));
    }
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

/// Fecha y hora UTC (sin base de datos de zonas horarias; los documentos y el `bext` van en UTC).
///
///  `(año, mes, día, hora, minuto, segundo)` del instante actual.
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

/// Borra el archivo al soltarse salvo que se desarme: así un error, una cancelación o un
/// panic nunca dejan temporales.
struct TempGuard<'a> {
    path: &'a Path,
    armed: bool,
}

impl Drop for TempGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            let _ = std::fs::remove_file(self.path);
        }
    }
}

/// Escribe con `f` a `final_path.part` y lo renombra si todo fue bien; si no (error,
/// cancelación o panic), lo borra.
fn atomic(
    final_path: &Path,
    f: impl FnOnce(&Path) -> Result<(), AudioError>,
) -> Result<u64, AudioError> {
    let mut tmp = final_path.as_os_str().to_owned();
    tmp.push(".part");
    let tmp = PathBuf::from(tmp);
    let mut guard = TempGuard {
        path: &tmp,
        armed: true,
    };
    f(&tmp)?;
    std::fs::rename(&tmp, final_path)?;
    guard.armed = false;
    Ok(std::fs::metadata(final_path)?.len())
}

/// Punto de inyección de fallos para pruebas: si el id de destino coincide, escribe el
/// temporal y entra en pánico (una sola vez).
#[doc(hidden)]
pub static TEST_PANIC_PRESET: Mutex<Option<String>> = Mutex::new(None);

fn inject_fault(job: &Job, tmp: &Path) {
    let hit = {
        let mut g = TEST_PANIC_PRESET.lock().unwrap_or_else(|e| e.into_inner());
        if g.as_deref() == Some(job.preset_id.as_str()) {
            g.take().is_some()
        } else {
            false
        }
    };
    if hit {
        let _ = std::fs::write(tmp, b"parcial");
        panic!("fallo inyectado en {}", job.preset_id);
    }
}

/// Clave de la caché de muestras: (bits, frecuencia, dither). El dither es un bool porque la
/// semilla TPDF (`DITHER_SEED`) es constante: dos destinos con la misma clave producen
/// exactamente las mismas muestras. Si algún día la semilla dependiera del destino, habría
/// que meterla en la clave.
type SampleKey = (u16, u32, bool);
type Shared = Arc<Vec<Vec<i32>>>;

struct CacheEntry {
    data: Result<Shared, String>,
    /// Destinos que aún tienen que usar esta combinación; al llegar a 0 se libera.
    remaining: usize,
}

type Cache = HashMap<SampleKey, CacheEntry>;

struct Ctx<'a> {
    meta: &'a ReleaseMeta,
    folder: &'a Path,
    stamp: (u16, u8, u8, u8, u8, u8),
    lang: Lang,
    cancel: Option<Arc<AtomicBool>>,
    /// Muestras cuantizadas por combinación (bits, frecuencia, dither), calculadas una sola vez
    /// y compartidas entre destinos; se liberan cuando el último destino que las usa termina.
    cache: Mutex<Cache>,
    media: &'a MediaInputs,
    sidecar: Option<&'a SidecarPaths>,
    platform: Platform,
    /// Carpeta temporal de esta exportación (WAV intermedios, portada conformada, fondos).
    temp: &'a Path,
    /// Portadas ya conformadas, por ruta de origen.
    covers: Mutex<HashMap<PathBuf, Arc<Result<CoverOutput, AudioError>>>>,
}

/// Dither de un destino: lo decide el plan (`needs_dither`), nunca la detección automática.
///
/// Nota v1: con un máster float32 el plan marca `needs_dither` al bajar a 24 bit
/// (comparando con 32), y aquí se respeta tal cual.
fn dither_for(needs: bool) -> Dither {
    if needs {
        Dither::Tpdf { seed: DITHER_SEED }
    } else {
        Dither::None
    }
}

/// ¿Necesita muestras cuantizadas del máster? Audio PCM, MP3 y el audio del vídeo de YouTube.
fn is_sampled(job: &Job) -> bool {
    matches!(
        job.container,
        Container::Wav | Container::Flac | Container::Mp3
    ) || job.media == Some(MediaKind::YoutubeVideo)
}

/// Combinaciones distintas (bits, frecuencia, dither) que hay que calcular para estos destinos.
#[doc(hidden)]
pub fn plan_sample_keys(jobs: &[&Job]) -> Vec<(u16, u32, bool)> {
    let mut keys: Vec<SampleKey> = jobs
        .iter()
        .filter(|j| is_sampled(j))
        .map(|j| sample_key(j))
        .collect();
    keys.sort_unstable();
    keys.dedup();
    keys
}

/// Combinación de muestras que necesita un destino (el MP3 parte de la versión CD).
fn sample_key(job: &Job) -> SampleKey {
    if job.container == Container::Mp3 {
        (16, 44_100, job.needs_dither)
    } else {
        (job.bits, job.sample_rate, job.needs_dither)
    }
}

impl Ctx<'_> {
    fn cancelled(&self) -> bool {
        self.cancel
            .as_ref()
            .is_some_and(|c| c.load(Ordering::Relaxed))
    }

    fn cache_lock(&self) -> std::sync::MutexGuard<'_, Cache> {
        self.cache.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Muestras cuantizadas de un destino, de la caché compartida.
    fn samples(&self, job: &Job) -> Result<Shared, AudioError> {
        match self.cache_lock().get(&sample_key(job)) {
            Some(e) => e.data.clone().map_err(unsupported),
            None => Err(unsupported("err.internal")),
        }
    }

    /// Un destino ha terminado (bien, mal o saltado): libera la combinación si ya nadie la usa.
    fn release(&self, job: &Job) {
        let key = sample_key(job);
        let mut cache = self.cache_lock();
        if let Some(e) = cache.get_mut(&key) {
            e.remaining = e.remaining.saturating_sub(1);
            if e.remaining == 0 {
                cache.remove(&key);
            }
        }
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
            comment: Some(t(self.lang, "mp3.comment")),
            isrc: self.meta.isrc.clone(),
            composer: self.meta.composer.clone(),
            label: self.meta.label.clone(),
            copyright: self.meta.copyright.clone(),
        }
    }

    /// Portada conformada (3000×3000 sRGB, JPG y PNG) a partir de `src`, una sola vez por origen.
    fn conformed_cover(&self, src: &Path) -> Result<CoverOutput, AudioError> {
        let mut covers = self.covers.lock().unwrap_or_else(|e| e.into_inner());
        let entry = match covers.get(src) {
            Some(e) => e.clone(),
            None => {
                let n = covers.len();
                let jpg = self.temp.join(format!("cover-{n}.jpg"));
                let png = self.temp.join(format!("cover-{n}.png"));
                let r = Arc::new(cover_image::conform_cover(
                    src,
                    &jpg,
                    &png,
                    &CoverOptions::default(),
                    self.lang,
                ));
                covers.insert(src.to_path_buf(), r.clone());
                r
            }
        };
        match &*entry {
            Ok(c) => Ok(c.clone()),
            Err(e) => Err(rekey(e)),
        }
    }

    /// Portada de la entrega (la de `media`, o la de los metadatos).
    fn release_cover(&self) -> Option<PathBuf> {
        self.media.cover.clone().or_else(|| self.meta.cover.clone())
    }

    fn sidecar(&self) -> Result<&SidecarPaths, AudioError> {
        self.sidecar
            .ok_or_else(|| unsupported("err.sidecar_missing"))
    }

    /// Fuente de un vídeo: el material propio del destino o, si no hay, la portada conformada.
    fn video_source(&self, job: &Job) -> Result<VideoSource, AudioError> {
        if let Some(p) = &job.own_media {
            return Ok(VideoSource::UserMedia(p.clone()));
        }
        let cover = self
            .release_cover()
            .ok_or_else(|| unsupported("err.media_unreadable"))?;
        Ok(VideoSource::FromCover(self.conformed_cover(&cover)?.png))
    }

    /// Escribe el audio cuantizado de un destino a un WAV temporal (para ffmpeg).
    fn temp_wav(
        &self,
        job: &Job,
        samples: &[Vec<i32>],
        bits: u16,
        rate: u32,
    ) -> Result<(PathBuf, TempFile), AudioError> {
        let path = self.temp_path(&job.preset_id, "wav");
        let guard = TempFile(path.clone());
        let opts = WavWriteOptions {
            bits,
            metadata: None,
        };
        wav::write(&path, samples, rate, &opts)?;
        Ok((path, guard))
    }

    fn temp_path(&self, stem: &str, ext: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        self.temp.join(format!(
            "{stem}-{}.{ext}",
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ))
    }

    /// Imagen/vídeo del destino: devuelve los archivos escritos (el principal primero).
    fn render_media(
        &self,
        kind: MediaKind,
        job: &Job,
        dir: &Path,
        step: &dyn Fn(f32),
    ) -> Result<Vec<(PathBuf, u64)>, AudioError> {
        let path = dir.join(&job.file_name);
        let cancel = || self.cancelled();
        let size = |p: &Path| -> Result<u64, AudioError> { Ok(std::fs::metadata(p)?.len()) };
        match kind {
            MediaKind::Cover => {
                let src = job
                    .own_media
                    .clone()
                    .or_else(|| self.release_cover())
                    .ok_or_else(|| unsupported("err.media_unreadable"))?;
                let c = self.conformed_cover(&src)?;
                std::fs::create_dir_all(dir)?;
                let mut out = Vec::new();
                let png_path = dir.join(&job.extra_files[0]);
                for (from, to) in [(&c.jpg, &path), (&c.png, &png_path)] {
                    let r = atomic(to, |tmp| {
                        std::fs::copy(from, tmp).map(|_| ()).map_err(Into::into)
                    });
                    match r {
                        Ok(b) => out.push((to.clone(), b)),
                        Err(e) => {
                            for (p, _) in &out {
                                let _ = std::fs::remove_file(p);
                            }
                            return Err(e);
                        }
                    }
                }
                Ok(out)
            }
            MediaKind::Canvas => {
                let sc = self.sidecar()?;
                let src = self.video_source(job)?;
                let info = match &src {
                    VideoSource::UserMedia(p) => Some(media::probe::probe(sc, p)?),
                    VideoSource::FromCover(_) => None,
                };
                step(0.1);
                std::fs::create_dir_all(dir)?;
                let args = video::build_canvas_args(
                    &src,
                    info.as_ref(),
                    &CanvasSpec::default(),
                    &path,
                    self.platform,
                );
                run_ffmpeg(sc, &args, &cancel)?;
                Ok(vec![(path.clone(), size(&path)?)])
            }
            MediaKind::AppleMotion => {
                let sc = self.sidecar()?;
                let src = self.video_source(job)?;
                std::fs::create_dir_all(dir)?;
                // (cuadrado?, ruta): sin HEVC solo el vertical, en `file_name`
                let targets: Vec<(bool, PathBuf)> = if job.hevc_skipped {
                    vec![(false, path.clone())]
                } else {
                    vec![(true, path.clone()), (false, dir.join(&job.extra_files[0]))]
                };
                let mut out: Vec<(PathBuf, u64)> = Vec::new();
                for (i, (square, to)) in targets.iter().enumerate() {
                    step(if i == 0 { 0.1 } else { 0.55 });
                    let spec = AppleMotionSpec {
                        square: *square,
                        ..AppleMotionSpec::default()
                    };
                    let args = video::build_apple_motion_args(&src, &spec, to, self.platform);
                    let r = run_ffmpeg(sc, &args, &cancel).and_then(|()| size(to));
                    match r {
                        Ok(b) => out.push((to.clone(), b)),
                        Err(e) => {
                            // un destino es todo o nada
                            for (p, _) in &out {
                                let _ = std::fs::remove_file(p);
                            }
                            return Err(e);
                        }
                    }
                }
                Ok(out)
            }
            MediaKind::YoutubeVideo => {
                let sc = self.sidecar()?;
                let samples = self.samples(job)?;
                let src = self.video_source(job)?;
                step(0.2);
                // El audio es el WAV de 48 kHz que ya tiene el motor, nunca el original.
                let (wav48, _wav_guard) =
                    self.temp_wav(job, &samples, job.bits, job.sample_rate)?;
                let spec = YoutubeSpec::default();
                let (bg_path, _bg_guard);
                let bg = if video::is_still_source(&src) {
                    bg_path = self.temp_path("yt-bg", "png");
                    _bg_guard = TempFile(bg_path.clone());
                    let args = video::build_youtube_bg_args(&src, &spec, &bg_path);
                    run_ffmpeg(sc, &args, &cancel)?;
                    Some(bg_path.as_path())
                } else {
                    None
                };
                step(0.4);
                std::fs::create_dir_all(dir)?;
                let args = video::build_youtube_args(&src, bg, &wav48, &spec, &path, self.platform);
                run_ffmpeg(sc, &args, &cancel)?;
                Ok(vec![(path.clone(), size(&path)?)])
            }
        }
    }

    /// Archivos escritos por un destino; vacío (`None`): cancelado antes de escribir.
    fn render(
        &self,
        job: &Job,
        step: &dyn Fn(f32),
    ) -> Result<Option<Vec<(PathBuf, u64)>>, AudioError> {
        if self.cancelled() {
            return Ok(None);
        }
        let dir = self.folder.join(&job.subfolder);
        let path = dir.join(&job.file_name);
        if let Some(kind) = job.media {
            let files = self.render_media(kind, job, &dir, step)?;
            return Ok(Some(files));
        }
        if !matches!(
            job.container,
            Container::Wav | Container::Flac | Container::Mp3
        ) {
            return Err(unsupported(format!(
                "err.container: {}",
                job.container.extension()
            )));
        }
        let sidecar = if job.container == Container::Mp3 {
            Some(self.sidecar()?)
        } else {
            None
        };
        let samples = self.samples(job)?;
        step(0.6);
        if self.cancelled() {
            return Ok(None);
        }
        std::fs::create_dir_all(&dir)?;
        let bytes = match job.container {
            Container::Wav => atomic(&path, |tmp| {
                let opts = WavWriteOptions {
                    bits: job.bits,
                    metadata: job.embed_metadata.then(|| self.wav_metadata()),
                };
                inject_fault(job, tmp);
                wav::write(tmp, &samples, job.sample_rate, &opts)
            })?,
            Container::Flac => atomic(&path, |tmp| {
                let tags = job.embed_metadata.then(|| self.flac_tags());
                inject_fault(job, tmp);
                flac::write_cancellable(
                    tmp,
                    &samples,
                    job.sample_rate,
                    job.bits,
                    tags.as_ref(),
                    &|| self.cancelled(),
                )
            })?,
            _ => atomic(&path, |tmp| {
                let Some(sidecar) = sidecar else {
                    return Err(unsupported("err.sidecar_missing"));
                };
                // La portada conformada va incrustada (APIC); si no se puede conformar, el
                // MP3 sale sin ella.
                let cover_jpg = self
                    .release_cover()
                    .and_then(|c| self.conformed_cover(&c).ok())
                    .map(|c| c.jpg);
                // WAV intermedio fuera de la carpeta de entrega.
                let (wav_tmp, _wav_guard) = self.temp_wav(job, &samples, 16, 44_100)?;
                inject_fault(job, tmp);
                step(0.8);
                mp3::encode(
                    sidecar,
                    self.lang,
                    &wav_tmp,
                    tmp,
                    &self.id3_tags(),
                    cover_jpg.as_deref(),
                    &|| self.cancelled(),
                )
            })?,
        };
        Ok(Some(vec![(path, bytes)]))
    }
}

/// Borra el archivo temporal al soltarse (también si hay un panic).
struct TempFile(PathBuf);

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Carpeta temporal de la exportación; se borra entera al terminar.
struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Result<TempDir, AudioError> {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let p = std::env::temp_dir().join(format!(
            "masterout-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&p)?;
        Ok(TempDir(p))
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Copia de un error por su clave y detalle (`AudioError` no es `Clone`).
fn rekey(e: &AudioError) -> AudioError {
    match e {
        AudioError::Cancelled => AudioError::Cancelled,
        other => {
            let (k, d) = other.key_and_detail();
            unsupported(match d {
                Some(d) => format!("{k}: {d}"),
                None => k,
            })
        }
    }
}

fn write_text(path: &Path, text: &str) -> Result<u64, AudioError> {
    atomic(path, |tmp| std::fs::write(tmp, text).map_err(Into::into))
}

/// Calcula una sola vez las muestras de cada combinación (bits, frecuencia, dither) que piden
/// los destinos y libera el máster f32 en cuanto no hace falta: la última combinación a
/// frecuencia nativa lo consume para reutilizar su memoria. Pico: máster + una combinación
/// en curso + las ya calculadas (en i32), nunca varias copias remuestreadas a la vez.
fn build_cache(mut buf: AudioBuffer, jobs: &[&Job], cancelled: &dyn Fn() -> bool) -> Cache {
    let mut cache: Cache = HashMap::new();
    for key in plan_sample_keys(jobs) {
        let remaining = jobs
            .iter()
            .filter(|j| is_sampled(j) && sample_key(j) == key)
            .count();
        cache.insert(
            key,
            CacheEntry {
                data: Err("err.internal".to_string()),
                remaining,
            },
        );
    }
    let native = buf.sample_rate;
    // Primero las remuestreadas (transitorios pequeños), al final las nativas.
    let mut keys: Vec<SampleKey> = cache.keys().copied().collect();
    keys.sort_by_key(|&(bits, rate, dither)| (rate == native, rate, bits, dither));
    let last = keys.len().saturating_sub(1);
    for (i, key) in keys.into_iter().enumerate() {
        if cancelled() {
            // Los destinos se saltan; no se calcula nada más.
            cache.remove(&key);
            continue;
        }
        let (bits, rate, dither) = key;
        let data = catch_unwind(AssertUnwindSafe(|| -> Result<Shared, String> {
            let d = dither_for(dither);
            let stop = || "err.cancelled".to_string();
            if rate == native {
                if i == last {
                    let channels = std::mem::take(&mut buf.channels);
                    dither::quantize_owned_cancellable(channels, bits, d, cancelled)
                        .map(Arc::new)
                        .ok_or_else(stop)
                } else {
                    dither::quantize_cancellable(&buf.channels, bits, d, cancelled)
                        .map(Arc::new)
                        .ok_or_else(stop)
                }
            } else {
                let resampled =
                    resample::resample_cancellable(&buf, rate, cancelled).map_err(|e| {
                        match e.key_and_detail() {
                            (k, Some(d)) => format!("{k}: {d}"),
                            (k, None) => k,
                        }
                    })?;
                dither::quantize_owned_cancellable(resampled.channels, bits, d, cancelled)
                    .map(Arc::new)
                    .ok_or_else(stop)
            }
        }))
        .unwrap_or_else(|_| Err("err.internal".to_string()));
        if let Some(e) = cache.get_mut(&key) {
            e.data = data;
        }
    }
    cache
}

/// Resultado de un destino dentro de la exportación paralela.
enum Outcome {
    Done(Vec<(PathBuf, u64)>),
    /// (clave de error, detalle técnico)
    Failed(String, Option<String>),
    /// Cancelado antes de escribir.
    Skipped,
}

/// Ejecuta la exportación completa y devuelve la entrega.
///
/// Ningún panic de un destino ni del callback de progreso aborta la exportación: un destino
/// que entra en pánico se informa como `JobFailed { error: "err.internal" }`. Los errores de
/// destino son siempre claves `err.*` (nunca texto de un idioma); el detalle va aparte.
pub fn run(
    req: &ExportRequest,
    on_progress: &(dyn Fn(Progress) + Sync),
) -> Result<Delivery, AudioError> {
    let t0 = Instant::now();
    let emit = |p: Progress| {
        let _ = catch_unwind(AssertUnwindSafe(|| on_progress(p)));
    };
    let (buf, info) = analyze(&req.master_path)?;
    emit(Progress::Analyzed(info.clone()));

    let mut meta = req.meta.clone();
    meta.title = naming::fallback_title(&meta.title, &req.master_path);
    let ids: Vec<&str> = req.active_ids.iter().map(String::as_str).collect();
    let plan = plan::resolve_in(
        &info,
        &meta,
        &ids,
        &req.media,
        req.lang,
        req.sidecar
            .as_ref()
            .map(|sc| media::capabilities(sc, req.platform)),
    );
    emit(Progress::Planned(plan.clone()));

    let threads = req.threads.unwrap_or_else(num_cpus::get_physical).max(1);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .map_err(|e| unsupported(format!("err.threadpool: {e}")))?;
    let folder = create_delivery_folder(
        &req.out_root,
        &naming::delivery_folder(&meta.artist, &meta.title, meta.version.as_deref()),
    )?;
    let stamp = now_utc();

    let jobs: Vec<&Job> = plan
        .iter()
        .filter_map(|r| match r {
            Resolved::Job(j) => Some(j),
            Resolved::Disabled { .. } => None,
        })
        .collect();
    let cache = build_cache(buf, &jobs, &|| {
        req.cancel
            .as_ref()
            .is_some_and(|c| c.load(Ordering::Relaxed))
    });
    let temp = TempDir::new()?;
    let ctx = Ctx {
        meta: &meta,
        folder: &folder,
        stamp,
        lang: req.lang,
        cancel: req.cancel.clone(),
        cache: Mutex::new(cache),
        media: &req.media,
        sidecar: req.sidecar.as_ref(),
        platform: req.platform,
        temp: &temp.0,
        covers: Mutex::new(HashMap::new()),
    };

    let outcomes: Vec<Outcome> = pool.install(|| {
        jobs.par_iter()
            .map(|job| {
                if ctx.cancelled() {
                    ctx.release(job);
                    return Outcome::Skipped;
                }
                let id = job.preset_id.clone();
                emit(Progress::JobStarted {
                    preset_id: id.clone(),
                });
                let step = |fraction: f32| {
                    emit(Progress::JobProgress {
                        preset_id: id.clone(),
                        fraction,
                    })
                };
                step(0.0);
                let r = catch_unwind(AssertUnwindSafe(|| ctx.render(job, &step)));
                ctx.release(job);
                match r {
                    Ok(Ok(Some(files))) if !files.is_empty() => {
                        step(1.0);
                        emit(Progress::JobDone {
                            preset_id: id,
                            path: files[0].0.clone(),
                            bytes: files.iter().map(|f| f.1).sum(),
                        });
                        Outcome::Done(files)
                    }
                    Ok(Ok(_)) => Outcome::Skipped,
                    Ok(Err(AudioError::Cancelled)) => Outcome::Skipped,
                    Ok(Err(e)) => {
                        let (error, detail) = e.key_and_detail();
                        emit(Progress::JobFailed {
                            preset_id: id,
                            error: error.clone(),
                            detail: detail.clone(),
                        });
                        Outcome::Failed(error, detail)
                    }
                    Err(_) => {
                        let error = "err.internal".to_string();
                        emit(Progress::JobFailed {
                            preset_id: id,
                            error: error.clone(),
                            detail: None,
                        });
                        Outcome::Failed(error, None)
                    }
                }
            })
            .collect()
    });
    drop(ctx);

    let mut files = Vec::new();
    let mut failed = Vec::new();
    let mut failed_details = Vec::new();
    for (job, r) in jobs.iter().zip(outcomes) {
        match r {
            Outcome::Done(fs) => files.extend(fs),
            Outcome::Failed(e, d) => {
                failed.push((job.preset_id.clone(), e));
                if let Some(d) = d {
                    failed_details.push((job.preset_id.clone(), d));
                }
            }
            Outcome::Skipped => {}
        }
    }
    // Sin subcarpetas vacías (p. ej. si todos los jobs de una carpeta fallaron o se canceló).
    for job in &jobs {
        let _ = std::fs::remove_dir(folder.join(&job.subfolder));
    }

    let cancelled = req
        .cancel
        .as_ref()
        .is_some_and(|c| c.load(Ordering::Relaxed));
    if cancelled {
        // Se conservan los archivos completos; sin ninguno, la carpeta parcial se elimina.
        let _ = std::fs::remove_dir(&folder);
        emit(Progress::Cancelled {
            files_kept: files.len() as u32,
        });
        return Ok(Delivery {
            folder,
            files,
            failed,
            failed_details,
            cancelled: true,
            elapsed_ms: t0.elapsed().as_millis(),
        });
    }

    let summary = DeliverySummary {
        meta: &meta,
        master: &info,
        jobs: &plan,
        created_at: iso(stamp),
        app_version: env!("CARGO_PKG_VERSION"),
    };
    let names = naming::document_names(req.lang);
    let docs = [
        (names.readme, reports::readme(req.lang, &summary)),
        (
            names.loudness_report,
            reports::loudness_report(req.lang, &summary),
        ),
        (names.sheet_csv, reports::release_sheet_csv(&summary)),
        (
            names.sheet_txt,
            reports::release_sheet_txt(req.lang, &summary),
        ),
    ];
    // Un fallo en los documentos no aborta: se informa en `failed` y `Finished` siempre se emite.
    for (name, text) in docs {
        let path = folder.join(&name);
        // Un panic al escribir un documento tampoco impide que `Finished` se emita.
        let r = catch_unwind(AssertUnwindSafe(|| write_text(&path, &text)))
            .unwrap_or_else(|_| Err(unsupported("err.internal")));
        match r {
            Ok(bytes) => files.push((path, bytes)),
            Err(e) => {
                failed.push(("documents".to_string(), "err.documents".to_string()));
                let detail = match e.key_and_detail() {
                    (_, Some(d)) => d,
                    (k, None) => k,
                };
                failed_details.push(("documents".to_string(), format!("{name}: {detail}")));
            }
        }
    }

    let delivery = Delivery {
        folder,
        files,
        failed,
        failed_details,
        cancelled: false,
        elapsed_ms: t0.elapsed().as_millis(),
    };
    emit(Progress::Finished(delivery.clone()));
    Ok(delivery)
}
