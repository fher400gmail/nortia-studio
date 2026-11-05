//! Vídeo: Spotify Canvas, vídeo de YouTube con imagen fija y motion artwork de Apple.
//!
//! Las líneas de comando se construyen de forma pura (`build_*_args`, probadas con
//! vectores golden) y se ejecutan con [`run_ffmpeg`] a través del sidecar.
//!
//! Restricciones del sidecar (`tools/ffmpeg/common.sh`, `--disable-everything`): solo
//! están los filtros `scale,crop,zoompan,gblur,boxblur,overlay,format,fps,setsar,trim,
//! atrim,aresample,aformat,anull,null,color,loop,pad`. Por eso NO se usan `split` ni
//! `setpts`: la duración se limita con `-t`, el bucle con `-stream_loop`/`-loop`, y el
//! fondo desenfocado de YouTube lee la portada dos veces como dos entradas.
//!
//! Codificador H.264 por plataforma (ver `encoder_args`):
//! * macOS: `h264_videotoolbox -allow_sw 1 -profile:v high`
//! * Windows: `h264_mf -profile:v 100` (la constante `high` no existe en h264_mf; 100 = High)
//! * Linux (solo pruebas/CI con ffmpeg del sistema): `libx264 -profile:v high`

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use super::probe::MediaInfo;
use super::{Platform, SidecarPaths};
use crate::audio::AudioError;

pub const CANVAS_MIN_SECONDS: f32 = 3.0;
pub const CANVAS_MAX_SECONDS: f32 = 8.0;
/// Zoom final de las animaciones fabricadas desde la portada (inicial 1,0).
const ZOOM_END: f32 = 1.08;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoSource {
    /// Portada ya conformada (imagen) para fabricar el vídeo.
    FromCover(PathBuf),
    /// Material propio del usuario: imagen o vídeo.
    UserMedia(PathBuf),
}

impl VideoSource {
    pub fn path(&self) -> &Path {
        match self {
            VideoSource::FromCover(p) | VideoSource::UserMedia(p) => p,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasSpec {
    pub width: u32,
    pub height: u32,
    /// Duración al fabricar desde imagen. Con vídeo propio: `min(len, 8)`, y `3` si dura menos (bucle).
    pub seconds: f32,
    pub fps: u32,
}
impl Default for CanvasSpec {
    fn default() -> Self {
        CanvasSpec {
            width: 1080,
            height: 1920,
            seconds: 6.0,
            fps: 30,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct YoutubeSpec {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub video_kbps: u32,
    pub audio_kbps: u32,
    pub audio_rate: u32,
}
impl Default for YoutubeSpec {
    fn default() -> Self {
        YoutubeSpec {
            width: 1920,
            height: 1080,
            fps: 30,
            video_kbps: 8000,
            audio_kbps: 384,
            audio_rate: 48000,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppleMotionSpec {
    /// `true`: 3840×3840; `false`: 2048×2732.
    pub square: bool,
    pub seconds: f32,
    pub fps: u32,
    pub mbps: u32,
}
impl Default for AppleMotionSpec {
    fn default() -> Self {
        AppleMotionSpec {
            square: true,
            seconds: 10.0,
            fps: 30,
            mbps: 60,
        }
    }
}
impl AppleMotionSpec {
    pub fn size(&self) -> (u32, u32) {
        if self.square {
            (3840, 3840)
        } else {
            (2048, 2732)
        }
    }
}

// ---------- utilidades puras ----------

fn s(v: &str) -> String {
    v.to_string()
}

fn fmt_secs(x: f32) -> String {
    format!("{x}")
}

/// Prefijo común: silencioso, sin stdin, sobrescribe la salida.
fn prefix() -> Vec<String> {
    ["-hide_banner", "-loglevel", "error", "-nostdin", "-y"]
        .map(s)
        .to_vec()
}

fn looks_like_image(p: &Path) -> bool {
    matches!(
        p.extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("jpg" | "jpeg" | "png" | "webp")
    )
}

/// ¿La fuente es una imagen fija? Con `info` manda la sonda; sin ella, la extensión.
fn is_still(src: &VideoSource, info: Option<&MediaInfo>) -> bool {
    match src {
        VideoSource::FromCover(_) => true,
        VideoSource::UserMedia(p) => info
            .map(|i| i.is_still)
            .unwrap_or_else(|| looks_like_image(p)),
    }
}

/// Codificador H.264 del sistema, con sus flags por plataforma (incluye `-profile:v`).
fn encoder_args(platform: Platform) -> Vec<String> {
    let v: &[&str] = match platform {
        Platform::MacOS => &[
            "-c:v",
            "h264_videotoolbox",
            "-allow_sw",
            "1",
            "-profile:v",
            "high",
        ],
        Platform::Windows => &["-c:v", "h264_mf", "-profile:v", "100"],
        Platform::Linux => &["-c:v", "libx264", "-profile:v", "high"],
    };
    v.iter().map(|x| s(x)).collect()
}

/// Escala a cubrir `w×h` y recorta centrado (nunca deforma).
fn cover_fit(w: u32, h: u32) -> String {
    format!("scale={w}:{h}:force_original_aspect_ratio=increase,crop={w}:{h},setsar=1")
}

/// Zoom lento 1,0 → 1,08 en `n` fotogramas sobre una imagen ya ajustada a `pw×ph`.
fn zoom_chain(pw: u32, ph: u32, w: u32, h: u32, n: u32, fps: u32) -> String {
    let last = n.saturating_sub(1).max(1);
    let zoom = format!("{:.2}", ZOOM_END - 1.0);
    let fit = cover_fit(pw, ph);
    format!(
        "{fit},zoompan=z='1+{zoom}*on/{last}':x='iw/2-(iw/zoom/2)':y='ih/2-(ih/zoom/2)':d={n}:s={w}x{h}:fps={fps}"
    )
}

fn frames(seconds: f32, fps: u32) -> u32 {
    ((seconds * fps as f32).round() as u32).max(2)
}

// ---------- Canvas ----------

/// Spotify Canvas 9:16. Desde portada/imagen: `zoompan` lento sobre recorte 9:16 centrado.
/// Desde vídeo propio: recorte 9:16 centrado + escala, máx. 8 s (`-t`), bucle si < 3 s. Sin audio.
pub fn build_canvas_args(
    src: &VideoSource,
    info: Option<&MediaInfo>,
    spec: &CanvasSpec,
    out: &Path,
    platform: Platform,
) -> Vec<String> {
    let mut a = prefix();
    let (w, h) = (spec.width, spec.height);
    let vf;
    let mut t: Option<f32> = None;
    if is_still(src, info) {
        a.push(s("-i"));
        a.push(src.path().to_string_lossy().into_owned());
        let n = frames(spec.seconds, spec.fps);
        vf = zoom_chain(w * 2, h * 2, w, h, n, spec.fps);
        t = Some(spec.seconds);
    } else {
        let len = info.map(|i| i.duration as f32).filter(|d| *d > 0.0);
        match len {
            Some(l) if l < CANVAS_MIN_SECONDS => {
                a.extend([s("-stream_loop"), s("-1")]);
                t = Some(CANVAS_MIN_SECONDS);
            }
            Some(l) if l <= CANVAS_MAX_SECONDS => {}
            _ => t = Some(CANVAS_MAX_SECONDS),
        }
        a.push(s("-i"));
        a.push(src.path().to_string_lossy().into_owned());
        // recorte 9:16 centrado (dimensiones pares) y escala final
        vf = format!(
            "crop='trunc(min(iw,ih*{w}/{h})/2)*2':'trunc(min(ih,iw*{h}/{w})/2)*2',scale={w}:{h},setsar=1"
        );
    }
    a.extend([s("-vf"), vf, s("-an"), s("-r"), spec.fps.to_string()]);
    if let Some(t) = t {
        a.extend([s("-t"), fmt_secs(t)]);
    }
    a.extend(encoder_args(platform));
    a.extend(
        [
            "-b:v",
            "5M",
            "-pix_fmt",
            "yuv420p",
            "-movflags",
            "+faststart",
        ]
        .map(s),
    );
    a.push(out.to_string_lossy().into_owned());
    a
}

// ---------- YouTube ----------

/// Vídeo de YouTube: imagen fija con fondo desenfocado + portada 1000×1000 centrada y el máster a 48 kHz.
/// Dura lo que el audio (`-shortest`). `-g fps/2`, 2 B-frames, AAC-LC.
pub fn build_youtube_args(
    src: &VideoSource,
    master_wav_48k: &Path,
    spec: &YoutubeSpec,
    out: &Path,
    platform: Platform,
) -> Vec<String> {
    let mut a = prefix();
    let (w, h) = (spec.width, spec.height);
    let img = src.path().to_string_lossy().into_owned();
    let wav = master_wav_48k.to_string_lossy().into_owned();
    let fps = spec.fps.to_string();
    if is_still(src, None) {
        // la portada entra dos veces: fondo y primer plano (sin `split`, que el sidecar no trae)
        for _ in 0..2 {
            a.extend([
                s("-framerate"),
                fps.clone(),
                s("-loop"),
                s("1"),
                s("-i"),
                img.clone(),
            ]);
        }
        a.extend([s("-i"), wav]);
        a.extend([
            s("-filter_complex"),
            format!(
                "[0:v]{},gblur=sigma=40[bg];[1:v]{}[fg];[bg][fg]overlay=(W-w)/2:(H-h)/2[v]",
                cover_fit(w, h),
                cover_fit(1000, 1000)
            ),
            s("-map"),
            s("[v]"),
            s("-map"),
            s("2:a:0"),
        ]);
    } else {
        // vídeo propio a pantalla completa, en bucle hasta que acabe el audio
        a.extend([s("-stream_loop"), s("-1"), s("-i"), img, s("-i"), wav]);
        a.extend([
            s("-vf"),
            cover_fit(w, h),
            s("-map"),
            s("0:v:0"),
            s("-map"),
            s("1:a:0"),
        ]);
    }
    a.extend([
        s("-r"),
        fps,
        s("-g"),
        (spec.fps / 2).to_string(),
        s("-bf"),
        s("2"),
    ]);
    a.extend(encoder_args(platform));
    a.extend([
        s("-pix_fmt"),
        s("yuv420p"),
        s("-b:v"),
        format!("{}k", spec.video_kbps),
    ]);
    a.extend([
        s("-c:a"),
        s("aac"),
        s("-b:a"),
        format!("{}k", spec.audio_kbps),
        s("-ar"),
        spec.audio_rate.to_string(),
    ]);
    a.extend(["-movflags", "+faststart", "-shortest"].map(s));
    a.push(out.to_string_lossy().into_owned());
    a
}

// ---------- Apple motion artwork ----------

/// Motion artwork de Apple Music: zoom lento (o vídeo propio ajustado y en bucle/recortado a
/// `seconds`), sin audio, Rec.709. 3840×3840 o 2048×2732.
pub fn build_apple_motion_args(
    src: &VideoSource,
    spec: &AppleMotionSpec,
    out: &Path,
    platform: Platform,
) -> Vec<String> {
    let mut a = prefix();
    let (w, h) = spec.size();
    let vf = if is_still(src, None) {
        a.extend([s("-i"), src.path().to_string_lossy().into_owned()]);
        zoom_chain(w, h, w, h, frames(spec.seconds, spec.fps), spec.fps)
    } else {
        a.extend([
            s("-stream_loop"),
            s("-1"),
            s("-i"),
            src.path().to_string_lossy().into_owned(),
        ]);
        cover_fit(w, h)
    };
    a.extend([
        s("-vf"),
        vf,
        s("-an"),
        s("-r"),
        spec.fps.to_string(),
        s("-t"),
        fmt_secs(spec.seconds),
    ]);
    a.extend(encoder_args(platform));
    a.extend([
        s("-b:v"),
        format!("{}M", spec.mbps),
        s("-pix_fmt"),
        s("yuv420p"),
    ]);
    a.extend(
        [
            "-color_primaries",
            "bt709",
            "-color_trc",
            "bt709",
            "-colorspace",
            "bt709",
            "-movflags",
            "+faststart",
        ]
        .map(s),
    );
    a.push(out.to_string_lossy().into_owned());
    a
}

// ---------- ejecución ----------

#[cfg(windows)]
pub(crate) fn no_window(cmd: &mut Command) {
    use std::os::windows::process::CommandExt;
    cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
}
#[cfg(not(windows))]
pub(crate) fn no_window(_cmd: &mut Command) {}

fn is_encoder_unavailable(stderr: &str) -> bool {
    let l = stderr.to_ascii_lowercase();
    [
        "unknown encoder",
        "encoder not found",
        "error while opening encoder",
        "videotoolbox",
        "h264_mf",
        "mfplat",
        "mediafoundation",
        "mft",
    ]
    .iter()
    .any(|k| l.contains(k))
}

fn tail(stderr: &str) -> String {
    let lines: Vec<&str> = stderr.lines().filter(|l| !l.trim().is_empty()).collect();
    let t = lines[lines.len().saturating_sub(8)..].join(" | ");
    t.chars().take(800).collect()
}

/// Clasifica un fallo de ffmpeg por su stderr.
pub fn classify_failure(stderr: &str) -> AudioError {
    if is_encoder_unavailable(stderr) {
        AudioError::UnsupportedFormat(format!("err.encoder_unavailable: {}", tail(stderr)))
    } else {
        AudioError::UnsupportedFormat(format!("err.ffmpeg_failed: {}", tail(stderr)))
    }
}

fn kill(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

/// Ejecuta ffmpeg con `args`. Si `cancel()` pasa a `true` (se consulta cada 100 ms) mata el
/// proceso y devuelve `AudioError::Cancelled`. Un fallo es por trabajo: `err.encoder_unavailable`
/// o `err.ffmpeg_failed`; sidecar inexistente o no ejecutable: `err.sidecar_missing`.
pub fn run_ffmpeg(
    sidecar: &SidecarPaths,
    args: &[String],
    cancel: &dyn Fn() -> bool,
) -> Result<(), AudioError> {
    let mut cmd = Command::new(&sidecar.ffmpeg);
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    no_window(&mut cmd);
    let mut child = cmd.spawn().map_err(|e| {
        AudioError::UnsupportedFormat(format!(
            "err.sidecar_missing: {}: {e}",
            sidecar.ffmpeg.display()
        ))
    })?;
    let mut pipe = child.stderr.take();
    let reader = thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(p) = pipe.as_mut() {
            let _ = p.read_to_end(&mut buf);
        }
        String::from_utf8_lossy(&buf).into_owned()
    });
    let status = loop {
        if cancel() {
            kill(&mut child);
            let _ = reader.join();
            return Err(AudioError::Cancelled);
        }
        match child.try_wait() {
            Ok(Some(st)) => break st,
            Ok(None) => thread::sleep(Duration::from_millis(100)),
            Err(e) => {
                kill(&mut child);
                let _ = reader.join();
                return Err(AudioError::Io(e));
            }
        }
    };
    let stderr = reader.join().unwrap_or_default();
    if status.success() {
        Ok(())
    } else {
        Err(classify_failure(&stderr))
    }
}

pub use super::probe::probe;
