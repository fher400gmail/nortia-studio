use mastercheck_bridge::app_support::is_media_path;
use mastercheck_bridge::dto::{ExportRequestDto, JobDto, MasterDto, ReleaseMetaDto};
use mastercheck_bridge::planning::{plan_with, to_media_inputs};
use mastercheck_bridge::thumbnail::{base64, media_thumbnail};
use masterout_core::i18n::Lang;
use masterout_core::media::MediaCaps;

const ALL: Option<MediaCaps> = Some(MediaCaps::ALL);
use std::collections::HashMap;
use std::path::Path;

fn master() -> MasterDto {
    MasterDto {
        path: "/música/Vera Alcón - Luz.wav".into(),
        file_name: "Vera Alcón - Luz.wav".into(),
        sample_rate: 44_100,
        bits: 24,
        is_float: false,
        channels: 2,
        frames: 44_100 * 60,
        duration_secs: 60.0,
        integrated_lufs: -14.0,
        true_peak_dbtp: -1.5,
        lra_lu: 5.0,
        overs_count: 0,
        suggested_artist: Some("Vera Alcón".into()),
        suggested_title: Some("Luz".into()),
    }
}

fn meta(cover: Option<&Path>) -> ReleaseMetaDto {
    ReleaseMetaDto {
        artist: "Vera Alcón".into(),
        title: "Luz".into(),
        cover_path: cover.map(|p| p.to_string_lossy().into_owned()),
        ..Default::default()
    }
}

fn ids(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[test]
fn export_request_dto_roundtrips_media_inputs_and_defaults_when_absent() {
    let req = ExportRequestDto {
        master_path: "/m.wav".into(),
        out_root: "/o".into(),
        meta: ReleaseMetaDto::default(),
        active_ids: ids(&["canvas"]),
        media_inputs: HashMap::from([("canvas".to_string(), "/v/mio.mp4".to_string())]),
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"media_inputs\":{\"canvas\":\"/v/mio.mp4\"}"));
    assert_eq!(
        serde_json::from_str::<ExportRequestDto>(&json).unwrap(),
        req
    );
    // una interfaz anterior que no envía `media_inputs` sigue funcionando
    let old = r#"{"master_path":"/m.wav","out_root":"/o","meta":{"artist":"","title":"","version":null,"featuring":null,"album":null,"year":null,"isrc":null,"composer":null,"label":null,"copyright":null,"cover_path":null},"active_ids":[]}"#;
    assert!(serde_json::from_str::<ExportRequestDto>(old)
        .unwrap()
        .media_inputs
        .is_empty());
}

#[test]
fn job_dto_roundtrips_warnings() {
    let j = JobDto {
        preset_id: "cover".into(),
        subfolder: "09 Imagen".into(),
        file_name: "a.jpg".into(),
        container: "jpg".into(),
        bits: 0,
        sample_rate: 0,
        verdict: "ok".into(),
        verdict_text: String::new(),
        estimated_bytes: 1,
        enabled: true,
        disabled_reason: None,
        warnings: vec!["aviso".into()],
        extra_files: vec!["a.png".into()],
    };
    let json = serde_json::to_string(&j).unwrap();
    assert_eq!(serde_json::from_str::<JobDto>(&json).unwrap(), j);
}

#[test]
fn media_inputs_conversion_ignores_blank_paths() {
    let mi = to_media_inputs(
        &meta(Some(Path::new("/c.png"))),
        &HashMap::from([
            ("canvas".to_string(), " /v.mp4 ".to_string()),
            ("apple_motion".to_string(), "  ".to_string()),
        ]),
    );
    assert_eq!(mi.cover.as_deref(), Some(Path::new("/c.png")));
    assert_eq!(mi.per_preset.len(), 1);
    assert_eq!(mi.per_preset["canvas"], Path::new("/v.mp4"));
}

#[test]
fn plan_dto_media_chips_follow_cover_and_sidecar() {
    let tmp = tempfile::tempdir().unwrap();
    let cover = tmp.path().join("c.png");
    image::RgbImage::from_pixel(2400, 1800, image::Rgb([1, 2, 3]))
        .save(&cover)
        .unwrap();
    let all = ids(&[
        "cover",
        "canvas",
        "youtube_video",
        "apple_motion",
        "mp3_promo",
    ]);
    let none = HashMap::new();
    // sin portada: imagen y vídeo deshabilitados con motivo
    let p = plan_with(&master(), &meta(None), &all, &none, Lang::Es, ALL);
    for j in p.jobs.iter().filter(|j| j.preset_id != "mp3_promo") {
        assert!(!j.enabled, "{}", j.preset_id);
        assert!(j.disabled_reason.as_deref().unwrap().contains("portada"));
    }
    // con portada y sidecar: todo habilitado, con avisos localizados en la portada
    let p = plan_with(&master(), &meta(Some(&cover)), &all, &none, Lang::Es, ALL);
    assert!(p.jobs.iter().all(|j| j.enabled), "{:?}", p.jobs);
    let c = p.jobs.iter().find(|j| j.preset_id == "cover").unwrap();
    assert_eq!(c.file_name, "Vera Alcón - Luz [Portada] 3000x3000.jpg");
    assert_eq!(
        c.extra_files,
        vec!["Vera Alcón - Luz [Portada] 3000x3000.png"]
    );
    assert_eq!(c.warnings.len(), 2);
    assert!(c.warnings[0].contains("recortado"), "{:?}", c.warnings);
    // 4 destinos de medios + MP3 + 2 archivos extra + 4 documentos
    assert_eq!(p.total_files, 5 + 2 + 4);
    let en = plan_with(&master(), &meta(Some(&cover)), &all, &none, Lang::En, ALL);
    let c = en.jobs.iter().find(|j| j.preset_id == "cover").unwrap();
    assert!(c.warnings[0].contains("cropped"), "{:?}", c.warnings);
    // sin sidecar: vídeo y MP3 deshabilitados; la portada sigue
    let p = plan_with(&master(), &meta(Some(&cover)), &all, &none, Lang::Es, None);
    for j in &p.jobs {
        assert_eq!(j.enabled, j.preset_id == "cover", "{}", j.preset_id);
    }
    // material propio: un vídeo propio habilita el Canvas aun sin portada
    let mio = tmp.path().join("mio.mp4");
    std::fs::write(&mio, b"x").unwrap();
    let own = HashMap::from([("canvas".to_string(), mio.to_string_lossy().into_owned())]);
    let p = plan_with(
        &master(),
        &meta(None),
        &ids(&["canvas", "cover"]),
        &own,
        Lang::Es,
        ALL,
    );
    assert!(
        p.jobs
            .iter()
            .find(|j| j.preset_id == "canvas")
            .unwrap()
            .enabled
    );
    // una ruta que no existe o que no es un vídeo deja la ficha deshabilitada con el motivo
    for bad in ["/v/no-existe.mp4", "/etc/hostname"] {
        let own = HashMap::from([("canvas".to_string(), bad.to_string())]);
        let p = plan_with(
            &master(),
            &meta(None),
            &ids(&["canvas"]),
            &own,
            Lang::Es,
            ALL,
        );
        let j = &p.jobs[0];
        assert!(!j.enabled, "{bad}");
        assert!(
            j.disabled_reason
                .as_deref()
                .is_some_and(|r| !r.starts_with("err.")),
            "{bad}: texto localizado"
        );
    }
    assert!(
        !p.jobs
            .iter()
            .find(|j| j.preset_id == "cover")
            .unwrap()
            .enabled
    );
}

#[test]
fn base64_matches_known_vectors() {
    assert_eq!(base64(b""), "");
    assert_eq!(base64(b"f"), "Zg==");
    assert_eq!(base64(b"fo"), "Zm8=");
    assert_eq!(base64(b"foo"), "Zm9v");
    assert_eq!(base64(b"foobar"), "Zm9vYmFy");
}

#[test]
fn media_path_filter_accepts_images_and_videos_only() {
    for ok in [
        "a.PNG", "a.jpg", "a.jpeg", "a.mp4", "a.MOV", "a.m4v", "a.webm",
    ] {
        assert!(is_media_path(ok), "{ok}");
    }
    for no in ["a.wav", "a.txt", "a", "/etc/passwd", "a.exe"] {
        assert!(!is_media_path(no), "{no}");
    }
}

fn decode_data_url(url: &str) -> image::DynamicImage {
    let (head, body) = url.split_once(";base64,").unwrap();
    assert!(head.starts_with("data:image/"));
    // decodificación mínima de base64 para la prueba
    let val = |c: u8| match c {
        b'A'..=b'Z' => c - b'A',
        b'a'..=b'z' => c - b'a' + 26,
        b'0'..=b'9' => c - b'0' + 52,
        b'+' => 62,
        _ => 63,
    };
    let mut out = Vec::new();
    for ch in body.as_bytes().chunks(4) {
        let n = ch.iter().fold(0u32, |a, &c| {
            (a << 6) | if c == b'=' { 0 } else { u32::from(val(c)) }
        });
        out.push((n >> 16) as u8);
        if ch[2] != b'=' {
            out.push((n >> 8) as u8);
        }
        if ch[3] != b'=' {
            out.push(n as u8);
        }
    }
    image::load_from_memory(&out).unwrap()
}

#[test]
fn thumbnail_of_an_image_is_a_160px_data_url_that_keeps_the_aspect() {
    let tmp = tempfile::tempdir().unwrap();
    let p = tmp.path().join("a.png");
    image::RgbImage::from_pixel(1000, 500, image::Rgb([200, 50, 20]))
        .save(&p)
        .unwrap();
    let url = media_thumbnail(&p.to_string_lossy(), Lang::Es).unwrap();
    assert!(url.starts_with("data:image/jpeg;base64,"));
    let img = decode_data_url(&url);
    assert_eq!((img.width(), img.height()), (160, 80));
}

#[test]
fn thumbnail_of_a_video_is_its_first_frame() {
    let Some(sc) = masterout_core::export::default_sidecar() else {
        eprintln!("skip: no hay ffmpeg");
        return;
    };
    let tmp = tempfile::tempdir().unwrap();
    let v = tmp.path().join("v.mp4");
    let st = std::process::Command::new(&sc.ffmpeg)
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-f",
            "lavfi",
            "-i",
            "testsrc2=s=640x360:r=30:d=1",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
        ])
        .arg(&v)
        .status()
        .unwrap();
    assert!(st.success());
    let url = media_thumbnail(&v.to_string_lossy(), Lang::En).unwrap();
    assert!(url.starts_with("data:image/png;base64,"));
    let img = decode_data_url(&url);
    assert_eq!((img.width(), img.height()), (160, 90));
}

#[test]
fn thumbnail_errors_are_localized_keys() {
    let e = media_thumbnail("/etc/passwd", Lang::Es).unwrap_err();
    assert_eq!(e.key, "err.media_unreadable");
    let tmp = tempfile::tempdir().unwrap();
    let bad = tmp.path().join("roto.png");
    std::fs::write(&bad, b"no soy una imagen").unwrap();
    let e = media_thumbnail(&bad.to_string_lossy(), Lang::En).unwrap_err();
    assert_eq!(e.key, "err.media_unreadable");
    assert!(e.message.contains("could not be read"));
}

#[test]
fn plan_dto_reports_missing_hevc_in_the_apple_motion_warnings() {
    let tmp = tempfile::tempdir().unwrap();
    let cover = tmp.path().join("c.png");
    image::RgbImage::from_pixel(3000, 3000, image::Rgb([1, 2, 3]))
        .save(&cover)
        .unwrap();
    let caps = Some(MediaCaps {
        h264: true,
        hevc: false,
    });
    let p = plan_with(
        &master(),
        &meta(Some(&cover)),
        &ids(&["apple_motion"]),
        &HashMap::new(),
        Lang::Es,
        caps,
    );
    let j = &p.jobs[0];
    assert!(j.enabled && j.extra_files.is_empty());
    assert!(
        j.file_name.ends_with("[Apple Motion] 2048x2732.mp4"),
        "{}",
        j.file_name
    );
    assert!(j.warnings[0].contains("HEVC"), "{:?}", j.warnings);
    let no264 = Some(MediaCaps {
        h264: false,
        hevc: false,
    });
    let p = plan_with(
        &master(),
        &meta(Some(&cover)),
        &ids(&["apple_motion", "cover"]),
        &HashMap::new(),
        Lang::En,
        no264,
    );
    assert!(
        !p.jobs
            .iter()
            .find(|j| j.preset_id == "apple_motion")
            .unwrap()
            .enabled
    );
    assert!(
        p.jobs
            .iter()
            .find(|j| j.preset_id == "cover")
            .unwrap()
            .enabled
    );
}

/// webp (lo decodifica ffmpeg, no el crate `image`): sus dimensiones se leen con ffprobe ANTES de
/// decodificar y por encima del tope de la portada se rechaza con `err.cover_too_large`.
#[cfg(unix)]
mod webp_cap {
    use super::*;
    use mastercheck_bridge::thumbnail::media_thumbnail_with;
    use masterout_core::media::SidecarPaths;
    use std::os::unix::fs::PermissionsExt;

    fn script(dir: &Path, name: &str, body: &str) -> std::path::PathBuf {
        let p = dir.join(name);
        std::fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
        p
    }

    /// ffprobe falso con las dimensiones dadas; ffmpeg falso que deja una marca si se llega a usar.
    fn sidecar(dir: &Path, w: u32, h: u32) -> (SidecarPaths, std::path::PathBuf) {
        let marker = dir.join("ffmpeg-llamado");
        let probe = script(
            dir,
            "ffprobe",
            &format!(
                "echo '{{\"streams\":[{{\"codec_type\":\"video\",\"codec_name\":\"webp\",\"width\":{w},\"height\":{h}}}],\"format\":{{\"format_name\":\"webp_pipe\"}}}}'"
            ),
        );
        let ffmpeg = script(
            dir,
            "ffmpeg",
            &format!("touch '{}'; exit 1", marker.display()),
        );
        (
            SidecarPaths {
                ffmpeg,
                ffprobe: probe,
            },
            marker,
        )
    }

    #[test]
    fn a_huge_webp_is_rejected_before_ffmpeg_decodes_it() {
        let tmp = tempfile::tempdir().unwrap();
        let webp = tmp.path().join("enorme.webp");
        std::fs::write(&webp, b"RIFFxxxxWEBP").unwrap();
        for (w, h) in [(20_000, 100), (12_000, 12_000)] {
            let (sc, marker) = sidecar(tmp.path(), w, h);
            let e = media_thumbnail_with(&webp.to_string_lossy(), Lang::Es, Some(sc)).unwrap_err();
            assert_eq!(e.key, "err.cover_too_large", "{w}x{h}");
            assert!(e.message.contains("demasiado grande"), "{}", e.message);
            assert!(!marker.exists(), "ffmpeg no debe decodificar {w}x{h}");
        }
    }

    #[test]
    fn a_normal_webp_goes_on_to_ffmpeg() {
        let tmp = tempfile::tempdir().unwrap();
        let webp = tmp.path().join("normal.webp");
        std::fs::write(&webp, b"RIFFxxxxWEBP").unwrap();
        let (sc, marker) = sidecar(tmp.path(), 3000, 3000);
        // el ffmpeg falso falla: lo que importa es que se llegó a él
        let e = media_thumbnail_with(&webp.to_string_lossy(), Lang::En, Some(sc)).unwrap_err();
        assert_eq!(e.key, "err.media_unreadable");
        assert!(marker.exists());
    }

    #[test]
    fn a_video_is_not_subject_to_the_image_cap() {
        let tmp = tempfile::tempdir().unwrap();
        let mp4 = tmp.path().join("v.mp4");
        std::fs::write(&mp4, b"x").unwrap();
        let (sc, marker) = sidecar(tmp.path(), 20_000, 20_000);
        let _ = media_thumbnail_with(&mp4.to_string_lossy(), Lang::En, Some(sc));
        assert!(marker.exists());
    }
}
