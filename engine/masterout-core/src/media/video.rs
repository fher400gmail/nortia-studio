//! Vídeo: Spotify Canvas, vídeo de YouTube con imagen fija y motion artwork de Apple.
//!
//! Las líneas de comando se construyen de forma pura (`build_*_args`, probadas con
//! vectores golden) y se ejecutan con [`run_ffmpeg`] a través del sidecar.
//!
//! Restricciones del sidecar (`tools/ffmpeg/common.sh`, `--disable-everything`): solo
//! están los filtros `scale,crop,zoompan,gblur,overlay,format,fps,setsar,trim,atrim,
//! aresample,aformat,anull,null,color,loop,pad` (`boxblur` es solo GPL: no está).
//! Hoy no hay `split` ni `setpts`: la duración se limita con `-t`, el bucle con
//! `-stream_loop`/`-loop`, y el fondo desenfocado de YouTube lee la portada dos veces como
//! dos entradas (se mantiene así aunque `setpts`/`split`/`fade` se añadan al sidecar).
//!
//! Color: todo el vídeo sale en Rec.709 de rango limitado, con conversión real
//! (`scale=out_color_matrix=bt709:out_range=tv`) y etiquetas `bt709`.
//!
//! Codificador por plataforma (ver `encoder_args`):
//! * H.264 macOS: `h264_videotoolbox -allow_sw 1 -profile:v high`
//! * H.264 Windows: `h264_mf -profile:v 100` (la constante `high` no existe en h264_mf; 100 = High)
//! * HEVC (Apple motion 3840×3840): macOS `hevc_videotoolbox -allow_sw 1 -profile:v main -tag:v hvc1`;
//!   Windows `hevc_mf -profile:v 1 -tag:v hvc1` (1 = Main, numérico por la misma razón)
//! * Linux (solo pruebas/CI con ffmpeg del sistema, nunca se distribuye): `libx264 -profile:v high`
//!   también para "HEVC" (no hay codificador HEVC sin GPL)
//!
//! `run_ffmpeg` escribe las salidas `.mp4`/`.mov` a `<salida>.part` y renombra al terminar bien.

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
        Some("jpg" | "jpeg" | "png" | "webp" | "bmp" | "tiff" | "tif" | "gif" | "heic" | "heif")
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

/// ¿La fuente es una imagen fija (por su extensión)? Es lo que deciden los `build_*_args`
/// cuando no reciben la sonda.
pub fn is_still_source(src: &VideoSource) -> bool {
    is_still(src, None)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Codec {
    H264,
    Hevc,
}

/// Codificador del sistema, con sus flags por plataforma (incluye `-profile:v`).
fn encoder_args(platform: Platform, codec: Codec) -> Vec<String> {
    let v: &[&str] = match (platform, codec) {
        (Platform::MacOS, Codec::H264) => &[
            "-c:v",
            "h264_videotoolbox",
            "-allow_sw",
            "1",
            "-profile:v",
            "high",
        ],
        (Platform::MacOS, Codec::Hevc) => &[
            "-c:v",
            "hevc_videotoolbox",
            "-allow_sw",
            "1",
            "-profile:v",
            "main",
            "-tag:v",
            "hvc1",
        ],
        (Platform::Windows, Codec::H264) => &["-c:v", "h264_mf", "-profile:v", "100"],
        (Platform::Windows, Codec::Hevc) => {
            &["-c:v", "hevc_mf", "-profile:v", "1", "-tag:v", "hvc1"]
        }
        (Platform::Linux, _) => &["-c:v", "libx264", "-profile:v", "high"],
    };
    v.iter().map(|x| s(x)).collect()
}

/// Conversión real a Rec.709 de rango limitado, YUV 4:2:0.
const BT709: &str = "scale=out_color_matrix=bt709:out_range=tv,format=yuv420p";

fn color_tags() -> Vec<String> {
    [
        "-color_primaries",
        "bt709",
        "-color_trc",
        "bt709",
        "-colorspace",
        "bt709",
    ]
    .map(s)
    .to_vec()
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
        "{fit},zoompan=z='1+{zoom}*on/{last}':x='iw/2-(iw/zoom/2)':y='ih/2-(ih/zoom/2)':d={n}:s={w}x{h}:fps={fps},{BT709}"
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
            "crop='trunc(min(iw,ih*{w}/{h})/2)*2':'trunc(min(ih,iw*{h}/{w})/2)*2',scale={w}:{h},setsar=1,{BT709}"
        );
    }
    a.extend([s("-vf"), vf, s("-an"), s("-r"), spec.fps.to_string()]);
    if let Some(t) = t {
        a.extend([s("-t"), fmt_secs(t)]);
    }
    a.extend(encoder_args(platform, Codec::H264));
    a.extend(["-b:v", "5M", "-pix_fmt", "yuv420p"].map(s));
    a.extend(color_tags());
    a.extend(["-movflags", "+faststart"].map(s));
    a.push(out.to_string_lossy().into_owned());
    a
}

// ---------- YouTube ----------

/// Fondo desenfocado de YouTube: la imagen a cubrir 1920×1080 con `gblur`, un solo fotograma
/// PNG. Se renderiza una vez y el vídeo lo lee como entrada (el desenfoque por fotograma
/// costaba minutos). Solo para fuentes de imagen fija.
pub fn build_youtube_bg_args(src: &VideoSource, spec: &YoutubeSpec, out_png: &Path) -> Vec<String> {
    let mut a = prefix();
    a.extend([s("-i"), src.path().to_string_lossy().into_owned()]);
    a.extend([
        s("-vf"),
        format!("{},gblur=sigma=40", cover_fit(spec.width, spec.height)),
        s("-frames:v"),
        s("1"),
        s("-update"),
        s("1"),
    ]);
    a.push(out_png.to_string_lossy().into_owned());
    a
}

/// Vídeo de YouTube: imagen fija con fondo desenfocado + portada 1000×1000 centrada y el máster a 48 kHz.
/// Dura lo que el audio (`-shortest`). `-g fps/2`, 2 B-frames, AAC-LC.
///
/// `bg`: fondo desenfocado ya renderizado ([`build_youtube_bg_args`]); sin él, el desenfoque
/// se calcula en el mismo grafo (lento). Con vídeo propio el fondo no se usa.
pub fn build_youtube_args(
    src: &VideoSource,
    bg: Option<&Path>,
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
        // primer plano y fondo son dos entradas (sin `split`, que el sidecar no trae)
        let bg_in = bg.map_or_else(|| img.clone(), |p| p.to_string_lossy().into_owned());
        for input in [bg_in, img.clone()] {
            a.extend([
                s("-framerate"),
                fps.clone(),
                s("-loop"),
                s("1"),
                s("-i"),
                input,
            ]);
        }
        a.extend([s("-i"), wav]);
        let bg_chain = if bg.is_some() {
            BT709.to_string()
        } else {
            format!("{},{BT709},gblur=sigma=40", cover_fit(w, h))
        };
        a.extend([
            s("-filter_complex"),
            format!(
                "[0:v]{bg_chain}[bg];[1:v]{},{BT709}[fg];[bg][fg]overlay=(W-w)/2:(H-h)/2[v]",
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
            format!("{},{BT709}", cover_fit(w, h)),
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
    a.extend(encoder_args(platform, Codec::H264));
    a.extend([s("-pix_fmt"), s("yuv420p")]);
    a.extend(color_tags());
    a.extend([s("-b:v"), format!("{}k", spec.video_kbps)]);
    a.extend([
        s("-c:a"),
        s("aac"),
        s("-b:a"),
        format!("{}k", spec.audio_kbps),
        s("-ar"),
        spec.audio_rate.to_string(),
    ]);
    // sin edit lists (YouTube): los B-frames se compensan con offsets CTS negativos
    a.extend(
        [
            "-use_editlist",
            "0",
            "-movflags",
            "+faststart+negative_cts_offsets",
            "-shortest",
        ]
        .map(s),
    );
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
        // prescale 2× en el vertical (zoompan redondea a píxel); el cuadrado 3840² va a 1×
        let k = if spec.square { 1 } else { 2 };
        zoom_chain(w * k, h * k, w, h, frames(spec.seconds, spec.fps), spec.fps)
    } else {
        a.extend([
            s("-stream_loop"),
            s("-1"),
            s("-i"),
            src.path().to_string_lossy().into_owned(),
        ]);
        format!("{},{BT709}", cover_fit(w, h))
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
    // 3840×3840 supera el nivel máximo de H.264: HEVC; 2048×2732 sigue en H.264
    let codec = if spec.square {
        Codec::Hevc
    } else {
        Codec::H264
    };
    a.extend(encoder_args(platform, codec));
    a.extend([
        s("-b:v"),
        format!("{}M", spec.mbps),
        s("-pix_fmt"),
        s("yuv420p"),
    ]);
    a.extend(color_tags());
    a.extend(["-movflags", "+faststart"].map(s));
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
        "no such encoder",
        "not supported by this device",
        "not supported by this platform",
        "not supported on this platform",
    ]
    .iter()
    .any(|k| l.contains(k))
}

fn tail(stderr: &str) -> String {
    let lines: Vec<&str> = stderr.lines().filter(|l| !l.trim().is_empty()).collect();
    let t = lines[lines.len().saturating_sub(8)..].join(" | ");
    t.chars().take(800).collect()
}

/// Clasifica un fallo de ffmpeg por su stderr. Solo "el codificador no existe / no está
/// soportado en este equipo" es `err.encoder_unavailable`; el resto, `err.ffmpeg_failed`
/// (con el detalle del stderr, p. ej. "Error while opening encoder").
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

/// Máximo de stderr que se conserva (los últimos bytes).
const STDERR_KEEP: usize = 16 * 1024;

/// Drena el stderr conservando solo los últimos `STDERR_KEEP` bytes.
fn drain_tail(mut pipe: impl Read) -> String {
    let mut keep: Vec<u8> = Vec::new();
    let mut buf = [0u8; 4096];
    while let Ok(n) = pipe.read(&mut buf) {
        if n == 0 {
            break;
        }
        keep.extend_from_slice(&buf[..n]);
        if keep.len() > 2 * STDERR_KEEP {
            keep.drain(..keep.len() - STDERR_KEEP);
        }
    }
    if keep.len() > STDERR_KEEP {
        keep.drain(..keep.len() - STDERR_KEEP);
    }
    String::from_utf8_lossy(&keep).into_owned()
}

/// Si el último argumento es una salida de vídeo (`.mp4`/`.mov`/`.m4v`), devuelve su ruta.
fn video_output(args: &[String]) -> Option<PathBuf> {
    let p = Path::new(args.last()?);
    let ext = p.extension()?.to_str()?.to_ascii_lowercase();
    matches!(ext.as_str(), "mp4" | "mov" | "m4v").then(|| p.to_path_buf())
}

fn part_path(out: &Path) -> PathBuf {
    let mut s = out.as_os_str().to_owned();
    s.push(".part");
    PathBuf::from(s)
}

/// Ejecuta ffmpeg con `args`. Si el último argumento es una salida `.mp4`/`.mov`, se codifica a
/// `<salida>.part` (con `-f mp4`) y se renombra al terminar bien; en error o cancelación el
/// `.part` se borra, así que nunca queda un archivo a medias.
/// Si `cancel()` pasa a `true` (se consulta cada 100 ms) mata el proceso y devuelve
/// `AudioError::Cancelled`. Un fallo es por trabajo: `err.encoder_unavailable` o
/// `err.ffmpeg_failed`; sidecar inexistente (NotFound): `err.sidecar_missing`; otros fallos
/// al lanzarlo: `err.ffmpeg_failed`.
pub fn run_ffmpeg(
    sidecar: &SidecarPaths,
    args: &[String],
    cancel: &dyn Fn() -> bool,
) -> Result<(), AudioError> {
    let target = video_output(args);
    let mut real: Vec<String> = args.to_vec();
    let part = target.as_deref().map(part_path);
    if let Some(part) = &part {
        let n = real.len();
        real[n - 1] = part.to_string_lossy().into_owned();
        real.insert(n - 1, "mp4".into());
        real.insert(n - 1, "-f".into());
    }
    let cleanup = || {
        if let Some(p) = &part {
            let _ = std::fs::remove_file(p);
        }
    };
    let mut cmd = Command::new(&sidecar.ffmpeg);
    cmd.args(&real)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    no_window(&mut cmd);
    let mut child = cmd.spawn().map_err(|e| {
        let key = if e.kind() == std::io::ErrorKind::NotFound {
            "err.sidecar_missing"
        } else {
            "err.ffmpeg_failed"
        };
        AudioError::UnsupportedFormat(format!("{key}: {}: {e}", sidecar.ffmpeg.display()))
    })?;
    let pipe = child.stderr.take();
    let reader = thread::spawn(move || pipe.map(drain_tail).unwrap_or_default());
    let status = loop {
        if cancel() {
            kill(&mut child);
            let _ = reader.join();
            cleanup();
            return Err(AudioError::Cancelled);
        }
        match child.try_wait() {
            Ok(Some(st)) => break st,
            Ok(None) => thread::sleep(Duration::from_millis(100)),
            Err(e) => {
                kill(&mut child);
                let _ = reader.join();
                cleanup();
                return Err(AudioError::Io(e));
            }
        }
    };
    let stderr = reader.join().unwrap_or_default();
    if !status.success() {
        cleanup();
        return Err(classify_failure(&stderr));
    }
    if let (Some(part), Some(out)) = (&part, &target) {
        if let Err(e) = std::fs::rename(part, out) {
            cleanup();
            return Err(AudioError::Io(e));
        }
    }
    Ok(())
}

pub use super::probe::probe;
