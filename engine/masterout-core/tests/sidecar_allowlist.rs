//! Ata los argumentos de ffmpeg que construye el motor a la lista blanca del sidecar LGPL
//! (`tools/ffmpeg/common.sh`): filtros, codificadores, muxers y bitstream filters. Sin
//! sidecar ni ffmpeg: es una prueba pura. Habría cazado `setparams` (6a6848a), que el motor
//! usaba y el sidecar publicado no traía, porque los tests de ejecución corren con el ffmpeg
//! del sistema (con todos los filtros).

#[path = "allowlist/mod.rs"]
mod allowlist;

use allowlist::{filter_names, load, violations, Allow, Set};
use masterout_core::media::mp3::build_mp3_args;
use masterout_core::media::probe::MediaInfo;
use masterout_core::media::video::{
    build_apple_motion_args, build_canvas_args, build_youtube_args, build_youtube_bg_args,
    AppleMotionSpec, CanvasSpec, VideoSource, YoutubeSpec,
};
use masterout_core::media::{build_probe_args, Platform};
use std::path::Path;

fn info(duration: f64, is_still: bool) -> MediaInfo {
    MediaInfo {
        duration,
        width: 3840,
        height: 2160,
        fps: 30.0,
        video_codec: Some("h264".into()),
        video_profile: None,
        color_space: None,
        audio_codec: None,
        audio_sample_rate: None,
        audio_bitrate: None,
        has_video: true,
        has_audio: false,
        nb_streams: 1,
        is_still,
        moov_first: None,
    }
}

fn encoders<'a>(a: &'a Allow, p: Platform) -> &'a Set {
    match p {
        Platform::MacOS => &a.encoders_macos,
        Platform::Windows => &a.encoders_windows,
        Platform::Linux => unreachable!("Linux no se distribuye"),
    }
}

/// Todas las líneas de ffmpeg que el motor puede construir, con su etiqueta.
fn all_command_lines(p: Platform) -> Vec<(String, Vec<String>)> {
    let cover = VideoSource::FromCover("/t/cover.png".into());
    let img = VideoSource::UserMedia("/t/own.webp".into());
    let png = VideoSource::UserMedia("/t/own.png".into());
    let mp4 = VideoSource::UserMedia("/t/own.mp4".into());
    let mov = VideoSource::UserMedia("/t/own.mov".into());
    let out = Path::new("/t/out.mp4");
    let wav = Path::new("/t/in48.wav");
    let canvas = CanvasSpec::default();
    let yt = YoutubeSpec::default();
    let mut v: Vec<(String, Vec<String>)> = Vec::new();
    let mut add = |n: &str, a: Vec<String>| v.push((format!("{n} [{p:?}]"), a));

    // Canvas: portada, imagen propia, vídeo propio corto/medio/largo/sin sonda
    add("canvas portada", build_canvas_args(&cover, None, &canvas, out, p));
    for (n, s) in [("webp", &img), ("png", &png)] {
        add(&format!("canvas imagen {n}"), build_canvas_args(s, Some(&info(0.0, true)), &canvas, out, p));
    }
    for (n, d) in [("corto", 2.0), ("medio", 5.0), ("largo", 30.0)] {
        add(&format!("canvas vídeo {n}"), build_canvas_args(&mp4, Some(&info(d, false)), &canvas, out, p));
    }
    add("canvas vídeo mov", build_canvas_args(&mov, Some(&info(5.0, false)), &canvas, out, p));
    add("canvas vídeo sin sonda", build_canvas_args(&mp4, None, &canvas, out, p));

    // YouTube: portada con y sin fondo prerenderizado, imagen propia, vídeo propio
    let bg = Path::new("/t/bg.png");
    add("youtube portada con fondo", build_youtube_args(&cover, Some(bg), wav, &yt, out, p));
    add("youtube portada sin fondo", build_youtube_args(&cover, None, wav, &yt, out, p));
    add("youtube imagen propia", build_youtube_args(&img, Some(bg), wav, &yt, out, p));
    add("youtube vídeo propio", build_youtube_args(&mp4, None, wav, &yt, out, p));
    add("youtube vídeo propio mov", build_youtube_args(&mov, None, wav, &yt, out, p));
    add("youtube fondo (png)", build_youtube_bg_args(&cover, &yt, bg));

    // Portada animada: cuadrada (HEVC) y vertical (H.264), de portada y de vídeo propio
    for square in [true, false] {
        let spec = AppleMotionSpec { square, ..Default::default() };
        let tag = if square { "3840 HEVC" } else { "2048x2732 H.264" };
        add(&format!("portada animada {tag} portada"), build_apple_motion_args(&cover, &spec, out, p));
        add(&format!("portada animada {tag} vídeo"), build_apple_motion_args(&mp4, &spec, out, p));
        add(&format!("portada animada {tag} mov"), build_apple_motion_args(&mov, &spec, out, p));
    }

    // Sonda de capacidades (H.264 y HEVC)
    for hevc in [false, true] {
        add(
            &format!("sonda de capacidades hevc={hevc}"),
            build_probe_args(Path::new("/t/in.png"), out, p, hevc),
        );
    }
    v
}

#[test]
fn video_args_only_use_components_in_the_sidecar_allowlist() {
    let allow = load();
    let mut bad = Vec::new();
    for p in [Platform::MacOS, Platform::Windows] {
        for (what, args) in all_command_lines(p) {
            bad.extend(violations(&what, &args, &allow, encoders(&allow, p)));
        }
    }
    bad.sort();
    bad.dedup();
    assert!(
        bad.is_empty(),
        "el motor usa componentes que el sidecar LGPL no trae (tools/ffmpeg/common.sh):\n  {}",
        bad.join("\n  ")
    );
}

#[test]
fn mp3_args_only_use_components_in_the_sidecar_allowlist() {
    let allow = load();
    let args = build_mp3_args(Path::new("/t/in.wav"), Path::new("/t/out.mp3.part"));
    // libmp3lame va en ALLOW_ENCODERS_BASE: igual en las dos plataformas
    let bad = violations("mp3", &args, &allow, &allow.encoders_macos);
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn allowlist_is_loaded_from_the_shell_scripts() {
    let a = load();
    for f in ["scale", "zoompan", "gblur", "overlay", "format", "abuffer", "hflip"] {
        assert!(a.filters.contains(f), "filtro {f} no leído de common.sh/accept.sh");
    }
    assert!(a.decoders.contains("h264") && a.decoders.contains("vp8"));
    assert!(a.muxers.contains("mp4") && a.muxers.contains("mp3"));
    assert!(a.bsfs.contains("aac_adtstoasc"));
    for e in ["aac", "libmp3lame", "png", "mjpeg"] {
        assert!(a.encoders_macos.contains(e) && a.encoders_windows.contains(e), "{e}");
    }
    assert!(a.encoders_macos.contains("hevc_videotoolbox") && !a.encoders_macos.contains("h264_mf"));
    assert!(a.encoders_windows.contains("hevc_mf") && !a.encoders_windows.contains("libx264"));
}

/// Cada extensión de vídeo propio que acepta la ficha tiene su decodificador en el sidecar
/// (un `.mov` de motion design suele ser ProRes) y cada imagen, el suyo.
#[test]
fn accepted_user_media_extensions_have_decoders() {
    use masterout_core::media::{IMAGE_EXTS, VIDEO_EXTS};
    let a = load();
    assert!(VIDEO_EXTS.contains(&"mov"), "los .mov (ProRes) se aceptan");
    for ext in VIDEO_EXTS {
        let need: &[&str] = match *ext {
            "mov" => &["prores", "h264", "hevc"],
            "mp4" | "m4v" => &["h264", "hevc"],
            "mkv" | "webm" => &["h264", "hevc", "vp9"],
            other => panic!("extensión de vídeo sin decodificadores declarados: {other}"),
        };
        for d in need {
            assert!(a.decoders.contains(*d), ".{ext} necesita el decodificador {d}");
        }
    }
    for ext in IMAGE_EXTS {
        let d = match *ext {
            "jpg" | "jpeg" => "mjpeg",
            "png" => "png",
            "webp" => "webp",
            other => panic!("extensión de imagen sin decodificador declarado: {other}"),
        };
        assert!(a.decoders.contains(d), ".{ext} necesita el decodificador {d}");
    }
}

#[test]
fn filter_name_extraction_handles_labels_quotes_and_nesting() {
    assert_eq!(
        filter_names("[0:v]setpts=PTS-STARTPTS,split=2[bg][fg];[bg]scale=1:2,gblur=sigma=3[b];[b][f]overlay=(W-w)/2:(H-h)/2[v]"),
        ["setpts", "split", "scale", "gblur", "overlay"]
    );
    assert_eq!(
        filter_names("crop='trunc(min(iw,ih*9/16)/2)*2':'trunc(min(ih,iw))',scale=1:2,setsar=1"),
        ["crop", "scale", "setsar"]
    );
    assert_eq!(
        filter_names("zoompan=z='min(zoom+0.0008,1.1)':d=1,setparams=range=tv"),
        ["zoompan", "setparams"]
    );
}

#[test]
fn violations_flag_unknown_components() {
    let a = load();
    let args: Vec<String> = ["-vf", "scale=1:2,nope=1", "-c:v", "libx264", "-bsf:v", "x", "-f", "avi", "o.avi"]
        .map(String::from)
        .to_vec();
    let bad = violations("t", &args, &a, &a.encoders_macos);
    assert_eq!(bad.len(), 4, "{bad:?}");
}
