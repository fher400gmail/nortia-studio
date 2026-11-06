//! ffprobe (JSON) → `MediaInfo`. El análisis del JSON es puro y se prueba sin sidecar.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::process::{Command, Stdio};

use serde_json::Value;

use super::SidecarPaths;
use crate::audio::AudioError;

#[derive(Debug, Clone, PartialEq)]
pub struct MediaInfo {
    /// Segundos; 0 si el contenedor no declara duración (imagen).
    pub duration: f64,
    /// Dimensiones ya con la rotación aplicada (vídeo de móvil girado).
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub video_codec: Option<String>,
    pub video_profile: Option<String>,
    /// `color_space` de ffprobe (p. ej. `bt709`).
    pub color_space: Option<String>,
    pub audio_codec: Option<String>,
    pub audio_sample_rate: Option<u32>,
    pub audio_bitrate: Option<u32>,
    pub has_video: bool,
    pub has_audio: bool,
    pub nb_streams: u32,
    /// Imagen fija (PNG/JPG/WebP…), no vídeo.
    pub is_still: bool,
    /// `Some(true)` si en un MP4/MOV `moov` precede a `mdat` (faststart); `None` si no aplica.
    pub moov_first: Option<bool>,
}

fn unreadable(detail: impl std::fmt::Display) -> AudioError {
    AudioError::UnsupportedFormat(format!("err.media_unreadable: {detail}"))
}

fn rate(s: &str) -> f64 {
    match s.split_once('/') {
        Some((n, d)) => {
            let (n, d) = (
                n.parse::<f64>().unwrap_or(0.0),
                d.parse::<f64>().unwrap_or(0.0),
            );
            if d > 0.0 {
                n / d
            } else {
                0.0
            }
        }
        None => s.parse().unwrap_or(0.0),
    }
}

fn str_u32(v: &Value) -> Option<u32> {
    v.as_str()?.parse().ok()
}

/// Rotación en grados (0/90/180/270) de un stream de vídeo, de `side_data_list` o de la etiqueta `rotate`.
fn rotation(stream: &Value) -> i64 {
    let from_side = stream["side_data_list"]
        .as_array()
        .and_then(|l| l.iter().find_map(|d| d["rotation"].as_f64()));
    let from_tag = stream["tags"]["rotate"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok());
    let r = from_side.or(from_tag).unwrap_or(0.0).round() as i64;
    r.rem_euclid(360)
}

/// Analiza la salida de `ffprobe -show_format -show_streams -of json`.
pub fn parse_probe_json(json: &str) -> Result<MediaInfo, AudioError> {
    let v: Value = serde_json::from_str(json).map_err(unreadable)?;
    let streams = v["streams"].as_array().cloned().unwrap_or_default();
    let video = streams.iter().find(|s| s["codec_type"] == "video");
    let audio = streams.iter().find(|s| s["codec_type"] == "audio");
    if video.is_none() && audio.is_none() {
        return Err(unreadable("sin streams"));
    }
    let format_name = v["format"]["format_name"].as_str().unwrap_or("");
    let is_still =
        video.is_some() && (format_name.contains("image2") || format_name.ends_with("_pipe"));
    let duration = v["format"]["duration"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|d| d.is_finite() && !is_still)
        .unwrap_or(0.0);
    let (mut w, mut h, mut fps) = (0, 0, 0.0);
    if let Some(s) = video {
        w = s["width"].as_u64().unwrap_or(0) as u32;
        h = s["height"].as_u64().unwrap_or(0) as u32;
        if rotation(s) % 180 == 90 {
            std::mem::swap(&mut w, &mut h);
        }
        fps = s["r_frame_rate"].as_str().map(rate).unwrap_or(0.0);
    }
    Ok(MediaInfo {
        duration,
        width: w,
        height: h,
        fps,
        video_codec: video
            .and_then(|s| s["codec_name"].as_str())
            .map(String::from),
        video_profile: video.and_then(|s| s["profile"].as_str()).map(String::from),
        color_space: video
            .and_then(|s| s["color_space"].as_str())
            .map(String::from),
        audio_codec: audio
            .and_then(|s| s["codec_name"].as_str())
            .map(String::from),
        audio_sample_rate: audio.and_then(|s| str_u32(&s["sample_rate"])),
        audio_bitrate: audio.and_then(|s| str_u32(&s["bit_rate"])),
        has_video: video.is_some(),
        has_audio: audio.is_some(),
        nb_streams: streams.len() as u32,
        is_still,
        moov_first: None,
    })
}

/// Tope de tamaño del material propio (el mismo que el de la portada, `image::MAX_SIDE` y
/// `image::MAX_PIXELS`): ffmpeg decodificaría un WebP de 16383² (≈1 GB en RGBA) o un vídeo
/// enorme sin límite. `err.media_too_large` con las dimensiones en el detalle.
pub fn check_size(info: &MediaInfo) -> Result<(), AudioError> {
    use super::image::{MAX_PIXELS, MAX_SIDE};
    if info.width.max(info.height) > MAX_SIDE
        || u64::from(info.width) * u64::from(info.height) > MAX_PIXELS
    {
        return Err(AudioError::UnsupportedFormat(format!(
            "err.media_too_large: {}x{}",
            info.width, info.height
        )));
    }
    Ok(())
}

/// Recorre los átomos de primer nivel de un MP4/MOV: `Some(true)` si `moov` va antes de `mdat`.
pub fn moov_before_mdat(path: &Path) -> Option<bool> {
    let mut f = File::open(path).ok()?;
    let total = f.seek(SeekFrom::End(0)).ok()?;
    f.seek(SeekFrom::Start(0)).ok()?;
    let mut pos = 0u64;
    let (mut moov, mut mdat) = (None, None);
    for i in 0..64 {
        if pos + 8 > total {
            break;
        }
        f.seek(SeekFrom::Start(pos)).ok()?;
        let mut h = [0u8; 8];
        f.read_exact(&mut h).ok()?;
        let mut size = u32::from_be_bytes([h[0], h[1], h[2], h[3]]) as u64;
        let typ = &h[4..8];
        if size == 1 {
            let mut b = [0u8; 8];
            f.read_exact(&mut b).ok()?;
            size = u64::from_be_bytes(b);
        } else if size == 0 {
            size = total - pos;
        }
        match typ {
            b"moov" => moov = moov.or(Some(i)),
            b"mdat" => mdat = mdat.or(Some(i)),
            _ => {}
        }
        if size < 8 {
            break;
        }
        pos += size;
    }
    match (moov, mdat) {
        (Some(a), Some(b)) => Some(a < b),
        _ => None,
    }
}

/// Plazo de ffprobe: un archivo corrupto o un disco de red no deben bloquear la aplicación.
const PROBE_DEADLINE: std::time::Duration = std::time::Duration::from_secs(15);

/// Ejecuta ffprobe sobre `path`. La ruta se pasa canonicalizada (absoluta), así que nunca
/// puede empezar por `-` y leerse como opción.
pub fn probe(sidecar: &SidecarPaths, path: &Path) -> Result<MediaInfo, AudioError> {
    let path = std::fs::canonicalize(path).map_err(|e| unreadable(e.to_string()))?;
    let path = path.as_path();
    let mut cmd = Command::new(&sidecar.ffprobe);
    cmd.args([
        "-v",
        "error",
        "-show_format",
        "-show_streams",
        "-of",
        "json",
    ])
    .arg(path)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
    super::video::no_window(&mut cmd);
    let spawn_err = |e: std::io::Error| {
        let key = if e.kind() == std::io::ErrorKind::NotFound {
            "err.sidecar_missing"
        } else {
            "err.ffmpeg_failed"
        };
        AudioError::UnsupportedFormat(format!("{key}: {}: {e}", sidecar.ffprobe.display()))
    };
    let mut child = cmd.spawn().map_err(spawn_err)?;
    // las dos tuberías se drenan en hilos para que un archivo grande no bloquee a ffprobe
    let mut so = child.stdout.take();
    let mut se = child.stderr.take();
    let t_out = std::thread::spawn(move || {
        let mut v = Vec::new();
        if let Some(p) = so.as_mut() {
            let _ = std::io::Read::read_to_end(p, &mut v);
        }
        v
    });
    let t_err = std::thread::spawn(move || {
        let mut v = Vec::new();
        if let Some(p) = se.as_mut() {
            let _ = std::io::Read::read_to_end(p, &mut v);
        }
        v
    });
    let started = std::time::Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(st)) => break st,
            Ok(None) if started.elapsed() >= PROBE_DEADLINE => {
                super::video::kill(&mut child);
                return Err(AudioError::UnsupportedFormat(format!(
                    "err.ffmpeg_timeout: {}",
                    sidecar.ffprobe.display()
                )));
            }
            Ok(None) => std::thread::sleep(std::time::Duration::from_millis(20)),
            Err(e) => {
                super::video::kill(&mut child);
                return Err(AudioError::Io(e));
            }
        }
    };
    let out = std::process::Output {
        status,
        stdout: t_out.join().unwrap_or_default(),
        stderr: t_err.join().unwrap_or_default(),
    };
    if !out.status.success() {
        let msg = String::from_utf8_lossy(&out.stderr);
        return Err(unreadable(msg.trim()));
    }
    let mut info = parse_probe_json(&String::from_utf8_lossy(&out.stdout))?;
    if info.has_video && !info.is_still {
        info.moov_first = moov_before_mdat(path);
    }
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MP4: &str = r#"{"streams":[
      {"codec_type":"video","codec_name":"h264","profile":"High","width":1920,"height":1080,"r_frame_rate":"30/1","color_space":"bt709"},
      {"codec_type":"audio","codec_name":"aac","sample_rate":"48000","bit_rate":"232000"}],
      "format":{"format_name":"mov,mp4,m4a,3gp,3g2,mj2","duration":"8.020000"}}"#;

    #[test]
    fn parses_video_and_audio() {
        let i = parse_probe_json(MP4).unwrap();
        assert_eq!((i.width, i.height), (1920, 1080));
        assert!((i.fps - 30.0).abs() < 1e-9 && (i.duration - 8.02).abs() < 1e-9);
        assert_eq!(i.video_profile.as_deref(), Some("High"));
        assert_eq!(i.color_space.as_deref(), Some("bt709"));
        assert_eq!(i.audio_sample_rate, Some(48000));
        assert!(i.has_audio && i.has_video && !i.is_still);
        assert_eq!(i.nb_streams, 2);
    }

    #[test]
    fn rotation_swaps_dimensions() {
        let j = r#"{"streams":[{"codec_type":"video","codec_name":"h264","width":1920,"height":1080,
          "r_frame_rate":"30000/1001","side_data_list":[{"side_data_type":"Display Matrix","rotation":-90}]}],
          "format":{"format_name":"mov,mp4","duration":"3.0"}}"#;
        let i = parse_probe_json(j).unwrap();
        assert_eq!((i.width, i.height), (1080, 1920));
        assert!((i.fps - 29.97).abs() < 0.01);
    }

    #[test]
    fn still_image_has_no_duration() {
        let j = r#"{"streams":[{"codec_type":"video","codec_name":"png","width":3000,"height":3000,"r_frame_rate":"25/1"}],
          "format":{"format_name":"png_pipe","duration":"0.040000"}}"#;
        let i = parse_probe_json(j).unwrap();
        assert!(i.is_still && i.duration == 0.0 && !i.has_audio);
    }

    #[test]
    fn check_size_applies_the_cover_limits() {
        let mut i = parse_probe_json(MP4).unwrap();
        assert!(check_size(&i).is_ok());
        (i.width, i.height) = (16_385, 16);
        assert_eq!(
            check_size(&i).unwrap_err().key_and_detail().0,
            "err.media_too_large"
        );
        (i.width, i.height) = (12_000, 12_000); // 144 MP
        assert_eq!(
            check_size(&i).unwrap_err().key_and_detail().0,
            "err.media_too_large"
        );
        (i.width, i.height) = (10_000, 12_000); // 120 MP exactos
        assert!(check_size(&i).is_ok());
    }

    #[test]
    fn garbage_is_media_unreadable() {
        let e = parse_probe_json("no json").unwrap_err();
        assert_eq!(e.key_and_detail().0, "err.media_unreadable");
        let e = parse_probe_json(r#"{"streams":[],"format":{}}"#).unwrap_err();
        assert_eq!(e.key_and_detail().0, "err.media_unreadable");
    }
}
