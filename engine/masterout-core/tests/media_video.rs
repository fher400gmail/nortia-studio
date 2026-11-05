//! Vídeo: vectores golden (puros, sin ffmpeg) y pruebas de ejecución contra un ffmpeg real
//! (sidecar o del sistema; si no hay ninguno, las de ejecución se saltan con un aviso).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use masterout_core::audio::AudioError;
use masterout_core::media::probe::{probe, MediaInfo};
use masterout_core::media::video::*;
use masterout_core::media::{discover, Platform, SidecarPaths};

fn v(a: &[&str]) -> Vec<String> {
    a.iter().map(|x| x.to_string()).collect()
}

fn cat(parts: &[Vec<String>]) -> Vec<String> {
    parts.concat()
}

fn cover() -> VideoSource {
    VideoSource::FromCover(PathBuf::from("cover.png"))
}

const PRE: [&str; 5] = ["-hide_banner", "-loglevel", "error", "-nostdin", "-y"];

fn enc(p: Platform) -> Vec<String> {
    match p {
        Platform::MacOS => v(&[
            "-c:v",
            "h264_videotoolbox",
            "-allow_sw",
            "1",
            "-profile:v",
            "high",
        ]),
        Platform::Windows => v(&["-c:v", "h264_mf", "-profile:v", "100"]),
        Platform::Linux => v(&["-c:v", "libx264", "-profile:v", "high"]),
    }
}

fn user_video_info(duration: f64) -> MediaInfo {
    MediaInfo {
        duration,
        width: 3840,
        height: 2160,
        fps: 30.0,
        video_codec: Some("h264".into()),
        video_profile: None,
        audio_codec: Some("aac".into()),
        audio_sample_rate: Some(48000),
        audio_bitrate: None,
        has_video: true,
        has_audio: true,
        nb_streams: 2,
        is_still: false,
        moov_first: None,
    }
}

// ------------------------------------------------------------------ golden

const CANVAS_COVER_VF: &str = "scale=2160:3840:force_original_aspect_ratio=increase,crop=2160:3840,setsar=1,zoompan=z='1+0.08*on/179':x='iw/2-(iw/zoom/2)':y='ih/2-(ih/zoom/2)':d=180:s=1080x1920:fps=30";

#[test]
fn canvas_args_from_cover_golden() {
    for p in [Platform::MacOS, Platform::Windows, Platform::Linux] {
        let got = build_canvas_args(
            &cover(),
            None,
            &CanvasSpec::default(),
            Path::new("o.mp4"),
            p,
        );
        let want = cat(&[
            v(&PRE),
            v(&[
                "-i",
                "cover.png",
                "-vf",
                CANVAS_COVER_VF,
                "-an",
                "-r",
                "30",
                "-t",
                "6",
            ]),
            enc(p),
            v(&[
                "-b:v",
                "5M",
                "-pix_fmt",
                "yuv420p",
                "-movflags",
                "+faststart",
                "o.mp4",
            ]),
        ]);
        assert_eq!(got, want, "{p:?}");
    }
}

#[test]
fn canvas_args_from_user_video_trims_to_8s() {
    let src = VideoSource::UserMedia(PathBuf::from("in.mp4"));
    let info = user_video_info(20.0);
    let got = build_canvas_args(
        &src,
        Some(&info),
        &CanvasSpec::default(),
        Path::new("o.mp4"),
        Platform::Linux,
    );
    let want = cat(&[
        v(&PRE),
        v(&[
            "-i",
            "in.mp4",
            "-vf",
            "crop='trunc(min(iw,ih*1080/1920)/2)*2':'trunc(min(ih,iw*1920/1080)/2)*2',scale=1080:1920,setsar=1",
            "-an",
            "-r",
            "30",
            "-t",
            "8",
        ]),
        enc(Platform::Linux),
        v(&["-b:v", "5M", "-pix_fmt", "yuv420p", "-movflags", "+faststart", "o.mp4"]),
    ]);
    assert_eq!(got, want);
    // 3..=8 s: sin recorte ni bucle; < 3 s: bucle hasta 3 s
    let mid = build_canvas_args(
        &src,
        Some(&user_video_info(5.0)),
        &CanvasSpec::default(),
        Path::new("o.mp4"),
        Platform::Linux,
    );
    assert!(!mid.contains(&"-t".to_string()) && !mid.contains(&"-stream_loop".to_string()));
    let short = build_canvas_args(
        &src,
        Some(&user_video_info(1.5)),
        &CanvasSpec::default(),
        Path::new("o.mp4"),
        Platform::Linux,
    );
    assert_eq!(&short[5..7], &v(&["-stream_loop", "-1"])[..]);
    let t = short.iter().position(|a| a == "-t").unwrap();
    assert_eq!(short[t + 1], "3");
}

const YT_FILTER: &str = "[0:v]scale=1920:1080:force_original_aspect_ratio=increase,crop=1920:1080,setsar=1,gblur=sigma=40[bg];[1:v]scale=1000:1000:force_original_aspect_ratio=increase,crop=1000:1000,setsar=1[fg];[bg][fg]overlay=(W-w)/2:(H-h)/2[v]";

fn youtube_golden(p: Platform) -> Vec<String> {
    cat(&[
        v(&PRE),
        v(&[
            "-framerate",
            "30",
            "-loop",
            "1",
            "-i",
            "cover.png",
            "-framerate",
            "30",
            "-loop",
            "1",
            "-i",
            "cover.png",
            "-i",
            "m.wav",
        ]),
        v(&[
            "-filter_complex",
            YT_FILTER,
            "-map",
            "[v]",
            "-map",
            "2:a:0",
            "-r",
            "30",
            "-g",
            "15",
            "-bf",
            "2",
        ]),
        enc(p),
        v(&[
            "-pix_fmt", "yuv420p", "-b:v", "8000k", "-c:a", "aac", "-b:a", "384k", "-ar", "48000",
        ]),
        v(&["-movflags", "+faststart", "-shortest", "o.mp4"]),
    ])
}

#[test]
fn youtube_args_golden_mac() {
    let got = build_youtube_args(
        &cover(),
        Path::new("m.wav"),
        &YoutubeSpec::default(),
        Path::new("o.mp4"),
        Platform::MacOS,
    );
    assert_eq!(got, youtube_golden(Platform::MacOS));
    assert!(got
        .join(" ")
        .contains("-c:v h264_videotoolbox -allow_sw 1 -profile:v high"));
}

#[test]
fn youtube_args_golden_win() {
    let got = build_youtube_args(
        &cover(),
        Path::new("m.wav"),
        &YoutubeSpec::default(),
        Path::new("o.mp4"),
        Platform::Windows,
    );
    assert_eq!(got, youtube_golden(Platform::Windows));
    // h264_mf no conoce "high": perfil numérico
    assert!(got.join(" ").contains("-c:v h264_mf -profile:v 100"));
}

#[test]
fn youtube_args_golden_linux() {
    let got = build_youtube_args(
        &cover(),
        Path::new("m.wav"),
        &YoutubeSpec::default(),
        Path::new("o.mp4"),
        Platform::Linux,
    );
    assert_eq!(got, youtube_golden(Platform::Linux));
}

#[test]
fn apple_motion_args_golden() {
    let vf_sq = "scale=3840:3840:force_original_aspect_ratio=increase,crop=3840:3840,setsar=1,zoompan=z='1+0.08*on/299':x='iw/2-(iw/zoom/2)':y='ih/2-(ih/zoom/2)':d=300:s=3840x3840:fps=30";
    let vf_pt = "scale=2048:2732:force_original_aspect_ratio=increase,crop=2048:2732,setsar=1,zoompan=z='1+0.08*on/299':x='iw/2-(iw/zoom/2)':y='ih/2-(ih/zoom/2)':d=300:s=2048x2732:fps=30";
    for p in [Platform::MacOS, Platform::Windows, Platform::Linux] {
        for (square, vf) in [(true, vf_sq), (false, vf_pt)] {
            let spec = AppleMotionSpec {
                square,
                ..AppleMotionSpec::default()
            };
            let got = build_apple_motion_args(&cover(), &spec, Path::new("o.mp4"), p);
            let want = cat(&[
                v(&PRE),
                v(&["-i", "cover.png", "-vf", vf, "-an", "-r", "30", "-t", "10"]),
                enc(p),
                v(&[
                    "-b:v",
                    "60M",
                    "-pix_fmt",
                    "yuv420p",
                    "-color_primaries",
                    "bt709",
                    "-color_trc",
                    "bt709",
                    "-colorspace",
                    "bt709",
                ]),
                v(&["-movflags", "+faststart", "o.mp4"]),
            ]);
            assert_eq!(got, want, "{p:?} square={square}");
        }
    }
}

#[test]
fn user_image_uses_the_still_path_and_user_video_does_not_zoom() {
    let img = VideoSource::UserMedia(PathBuf::from("a.JPG"));
    let a = build_canvas_args(
        &img,
        None,
        &CanvasSpec::default(),
        Path::new("o.mp4"),
        Platform::Linux,
    );
    assert!(a.iter().any(|x| x.contains("zoompan")));
    let vid = VideoSource::UserMedia(PathBuf::from("a.mov"));
    let a = build_apple_motion_args(
        &vid,
        &AppleMotionSpec::default(),
        Path::new("o.mp4"),
        Platform::Linux,
    );
    assert!(!a.iter().any(|x| x.contains("zoompan")));
    assert!(a.join(" ").contains("-stream_loop -1 -i a.mov"));
    let y = build_youtube_args(
        &vid,
        Path::new("m.wav"),
        &YoutubeSpec::default(),
        Path::new("o.mp4"),
        Platform::Linux,
    );
    assert!(y.join(" ").contains("-stream_loop -1 -i a.mov -i m.wav"));
}

#[test]
fn failure_classification() {
    let key = |e: AudioError| e.key_and_detail().0;
    assert_eq!(
        key(classify_failure("Unknown encoder 'h264_mf'")),
        "err.encoder_unavailable"
    );
    assert_eq!(
        key(classify_failure(
            "[h264_videotoolbox @ 0x1] cannot create compression session"
        )),
        "err.encoder_unavailable"
    );
    assert_eq!(
        key(classify_failure(
            "in.mp4: Invalid data found when processing input"
        )),
        "err.ffmpeg_failed"
    );
}

#[test]
fn missing_sidecar_is_err_sidecar_missing() {
    let sc = SidecarPaths {
        ffmpeg: PathBuf::from("/no/such/ffmpeg"),
        ffprobe: PathBuf::from("/no/such/ffprobe"),
    };
    let e = run_ffmpeg(&sc, &v(&["-version"]), &|| false).unwrap_err();
    assert_eq!(e.key_and_detail().0, "err.sidecar_missing");
    let e = probe(&sc, Path::new("x.mp4")).unwrap_err();
    assert_eq!(e.key_and_detail().0, "err.sidecar_missing");
}

// ------------------------------------------------------------------ ejecución

fn sidecar() -> Option<SidecarPaths> {
    let sc = discover(Path::new("/nonexistent-exe-dir"));
    if sc.is_none() {
        eprintln!("SKIP: no hay ffmpeg/ffprobe (sidecar, MASTERCHECK_FFMPEG o PATH)");
    }
    sc
}

/// WAV PCM16 estéreo de seno a 48 kHz.
fn write_wav(path: &Path, secs: u32) {
    let sr = 48000u32;
    let n = sr * secs;
    let mut d = Vec::with_capacity(n as usize * 4);
    for i in 0..n {
        let s = ((i as f32 * 440.0 * std::f32::consts::TAU / sr as f32).sin() * 8000.0) as i16;
        d.extend_from_slice(&s.to_le_bytes());
        d.extend_from_slice(&s.to_le_bytes());
    }
    let mut w = Vec::new();
    w.extend_from_slice(b"RIFF");
    w.extend_from_slice(&(36 + d.len() as u32).to_le_bytes());
    w.extend_from_slice(b"WAVEfmt ");
    w.extend_from_slice(&16u32.to_le_bytes());
    w.extend_from_slice(&1u16.to_le_bytes());
    w.extend_from_slice(&2u16.to_le_bytes());
    w.extend_from_slice(&sr.to_le_bytes());
    w.extend_from_slice(&(sr * 4).to_le_bytes());
    w.extend_from_slice(&4u16.to_le_bytes());
    w.extend_from_slice(&16u16.to_le_bytes());
    w.extend_from_slice(b"data");
    w.extend_from_slice(&(d.len() as u32).to_le_bytes());
    w.extend_from_slice(&d);
    std::fs::write(path, w).unwrap();
}

/// Imagen PNG 1600×900 (con ffmpeg: fuente `color`, un fotograma).
fn make_png(sc: &SidecarPaths, path: &Path, size: &str) {
    run_ffmpeg(
        sc,
        &v(&[
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-f",
            "lavfi",
            "-i",
            &format!("color=c=0x3060c0:s={size}"),
            "-frames:v",
            "1",
            path.to_str().unwrap(),
        ]),
        &|| false,
    )
    .unwrap();
}

/// Vídeo sintético con audio AAC (para comprobar que el Canvas lo descarta).
fn make_video(sc: &SidecarPaths, path: &Path, size: &str, secs: u32, wav: &Path) {
    let codec: &[&str] = match Platform::current() {
        Platform::MacOS => &["-c:v", "h264_videotoolbox", "-allow_sw", "1"],
        Platform::Windows => &["-c:v", "h264_mf"],
        Platform::Linux => &["-c:v", "libx264", "-preset", "ultrafast"],
    };
    let mut a = v(&[
        "-hide_banner",
        "-loglevel",
        "error",
        "-y",
        "-f",
        "lavfi",
        "-i",
        &format!("color=c=0xc06030:s={size}:r=10"),
        "-i",
        wav.to_str().unwrap(),
        "-t",
        &secs.to_string(),
    ]);
    a.extend(v(codec));
    a.extend(v(&[
        "-pix_fmt",
        "yuv420p",
        "-c:a",
        "aac",
        path.to_str().unwrap(),
    ]));
    run_ffmpeg(sc, &a, &|| false).unwrap();
}

#[test]
fn canvas_from_user_video_is_conformed() {
    let Some(sc) = sidecar() else { return };
    let d = tempfile::tempdir().unwrap();
    let (wav, vin, out) = (
        d.path().join("a.wav"),
        d.path().join("in.mp4"),
        d.path().join("canvas.mp4"),
    );
    write_wav(&wav, 20);
    make_video(&sc, &vin, "3840x2160", 20, &wav);
    let src = VideoSource::UserMedia(vin.clone());
    let info = probe(&sc, &vin).unwrap();
    assert!(info.has_audio && (info.width, info.height) == (3840, 2160));
    let args = build_canvas_args(
        &src,
        Some(&info),
        &CanvasSpec::default(),
        &out,
        Platform::current(),
    );
    run_ffmpeg(&sc, &args, &|| false).unwrap();
    let o = probe(&sc, &out).unwrap();
    assert_eq!((o.width, o.height), (1080, 1920));
    assert!(
        o.duration <= 8.05 && o.duration > 7.5,
        "duración {}",
        o.duration
    );
    assert!(!o.has_audio && o.nb_streams == 1);
    assert_eq!(o.video_codec.as_deref(), Some("h264"));
}

#[test]
fn canvas_from_short_user_video_loops_to_3s_and_from_cover_is_6s() {
    let Some(sc) = sidecar() else { return };
    let d = tempfile::tempdir().unwrap();
    let (wav, vin, out) = (
        d.path().join("a.wav"),
        d.path().join("in.mp4"),
        d.path().join("c.mp4"),
    );
    write_wav(&wav, 1);
    make_video(&sc, &vin, "640x360", 1, &wav);
    let info = probe(&sc, &vin).unwrap();
    let args = build_canvas_args(
        &VideoSource::UserMedia(vin),
        Some(&info),
        &CanvasSpec::default(),
        &out,
        Platform::current(),
    );
    run_ffmpeg(&sc, &args, &|| false).unwrap();
    let o = probe(&sc, &out).unwrap();
    assert!((o.duration - 3.0).abs() < 0.2, "duración {}", o.duration);

    let png = d.path().join("cover.png");
    make_png(&sc, &png, "1600x1600");
    let args = build_canvas_args(
        &VideoSource::FromCover(png),
        None,
        &CanvasSpec::default(),
        &out,
        Platform::current(),
    );
    run_ffmpeg(&sc, &args, &|| false).unwrap();
    let o = probe(&sc, &out).unwrap();
    assert_eq!((o.width, o.height), (1080, 1920));
    assert!((o.duration - 6.0).abs() < 0.2 && !o.has_audio, "{o:?}");
    assert!((o.fps - 30.0).abs() < 0.01);
}

#[test]
fn youtube_output_matches_spec() {
    let Some(sc) = sidecar() else { return };
    let d = tempfile::tempdir().unwrap();
    let (wav, png, out) = (
        d.path().join("m.wav"),
        d.path().join("cover.png"),
        d.path().join("yt.mp4"),
    );
    write_wav(&wav, 4);
    make_png(&sc, &png, "1600x1600");
    let args = build_youtube_args(
        &VideoSource::FromCover(png),
        &wav,
        &YoutubeSpec::default(),
        &out,
        Platform::current(),
    );
    run_ffmpeg(&sc, &args, &|| false).unwrap();
    let o = probe(&sc, &out).unwrap();
    assert_eq!((o.width, o.height), (1920, 1080));
    assert_eq!(o.video_codec.as_deref(), Some("h264"));
    assert_eq!(o.video_profile.as_deref(), Some("High"));
    assert!((o.fps - 30.0).abs() < 0.01, "fps {}", o.fps);
    assert_eq!(o.audio_codec.as_deref(), Some("aac"));
    assert_eq!(o.audio_sample_rate, Some(48000));
    // el aac nativo no es CBR estricto: se acota por arriba
    assert!(o.audio_bitrate.unwrap() <= 430_000, "{:?}", o.audio_bitrate);
    assert_eq!(o.nb_streams, 2);
    assert!((o.duration - 4.0).abs() < 0.2, "duración {}", o.duration);
    assert_eq!(o.moov_first, Some(true));
}

#[test]
fn apple_motion_output_matches_spec_portrait() {
    let Some(sc) = sidecar() else { return };
    let d = tempfile::tempdir().unwrap();
    let (png, out) = (d.path().join("cover.png"), d.path().join("am.mp4"));
    make_png(&sc, &png, "3000x3000");
    let spec = AppleMotionSpec {
        square: false,
        seconds: 2.0,
        ..AppleMotionSpec::default()
    };
    let args = build_apple_motion_args(
        &VideoSource::FromCover(png),
        &spec,
        &out,
        Platform::current(),
    );
    run_ffmpeg(&sc, &args, &|| false).unwrap();
    let o = probe(&sc, &out).unwrap();
    assert_eq!((o.width, o.height), (2048, 2732));
    assert!(!o.has_audio && (o.duration - 2.0).abs() < 0.2, "{o:?}");
}

#[test]
fn encoder_failure_is_per_job() {
    let Some(sc) = sidecar() else { return };
    let d = tempfile::tempdir().unwrap();
    let (wav, png, out) = (
        d.path().join("m.wav"),
        d.path().join("cover.png"),
        d.path().join("yt.mp4"),
    );
    write_wav(&wav, 1);
    make_png(&sc, &png, "1600x1600");
    let mut args = build_youtube_args(
        &VideoSource::FromCover(png),
        &wav,
        &YoutubeSpec::default(),
        &out,
        Platform::current(),
    );
    let i = args.iter().position(|a| a == "-c:v").unwrap();
    args[i + 1] = "encoder_que_no_existe".into();
    let e = run_ffmpeg(&sc, &args, &|| false).unwrap_err();
    assert_eq!(e.key_and_detail().0, "err.encoder_unavailable", "{e}");
}

#[test]
fn cancel_kills_ffmpeg() {
    let Some(sc) = sidecar() else { return };
    let d = tempfile::tempdir().unwrap();
    let out = d.path().join("long.mp4");
    // trabajo larguísimo (1 h de vídeo); solo termina si se mata el proceso
    let mut a = v(&[
        "-hide_banner",
        "-loglevel",
        "error",
        "-y",
        "-f",
        "lavfi",
        "-i",
        "color=c=black:s=1920x1080:r=30",
        "-t",
        "3600",
    ]);
    a.extend(match Platform::current() {
        Platform::Linux => v(&["-c:v", "libx264"]),
        Platform::MacOS => v(&["-c:v", "h264_videotoolbox", "-allow_sw", "1"]),
        Platform::Windows => v(&["-c:v", "h264_mf"]),
    });
    a.push(out.to_str().unwrap().into());
    let flag = AtomicBool::new(false);
    let t0 = Instant::now();
    let r = std::thread::scope(|s| {
        s.spawn(|| {
            std::thread::sleep(Duration::from_millis(500));
            flag.store(true, Ordering::SeqCst);
        });
        run_ffmpeg(&sc, &a, &|| flag.load(Ordering::SeqCst))
    });
    assert!(matches!(r, Err(AudioError::Cancelled)), "{r:?}");
    assert!(
        t0.elapsed() < Duration::from_secs(2),
        "tardó {:?}",
        t0.elapsed()
    );
}

#[test]
fn probe_unreadable_file_is_media_unreadable() {
    let Some(sc) = sidecar() else { return };
    let d = tempfile::tempdir().unwrap();
    let f = d.path().join("x.mp4");
    std::fs::write(&f, b"esto no es un video").unwrap();
    assert_eq!(
        probe(&sc, &f).unwrap_err().key_and_detail().0,
        "err.media_unreadable"
    );
}
