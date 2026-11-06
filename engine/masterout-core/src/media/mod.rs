//! Medios (imagen, vídeo, sonda) y localización del sidecar ffmpeg/ffprobe.

pub mod adm;
pub mod image;
pub mod mp3;
pub mod probe;
pub mod video;

use crate::audio::AudioError;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

/// Plataforma de destino de los codificadores de vídeo. `Linux` existe SOLO para
/// pruebas locales y de CI con un ffmpeg del sistema (`libx264`): nunca se distribuye.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Platform {
    MacOS,
    Windows,
    Linux,
}

impl Platform {
    pub fn current() -> Platform {
        if cfg!(target_os = "macos") {
            Platform::MacOS
        } else if cfg!(target_os = "windows") {
            Platform::Windows
        } else {
            Platform::Linux
        }
    }
}

/// Codificadores de vídeo que funcionan de verdad en este equipo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MediaCaps {
    pub h264: bool,
    pub hevc: bool,
}

impl MediaCaps {
    /// Todo disponible (para planes sin sonda).
    pub const ALL: MediaCaps = MediaCaps {
        h264: true,
        hevc: true,
    };
}

type CapsCache = Mutex<HashMap<(PathBuf, Platform), MediaCaps>>;

/// Lado de la imagen de la prueba (algunos codificadores de hardware rechazan tamaños mínimos).
const PROBE_SIDE: u32 = 256;

/// Extensiones de imagen y vídeo que acepta una ficha de material propio.
pub const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "webp"];
pub const VIDEO_EXTS: &[&str] = &["mp4", "mov", "m4v", "mkv", "webm"];

/// ¿Es `path` un fichero regular existente con extensión admitida? Validación única de las
/// rutas de usuario (`media_inputs`), aplicada al resolver el plan (la usan plan y entrega).
pub fn usable_media_file(path: &Path, images: bool, videos: bool) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);
    let ok_ext = ext.is_some_and(|e| {
        (images && IMAGE_EXTS.contains(&e.as_str())) || (videos && VIDEO_EXTS.contains(&e.as_str()))
    });
    ok_ext && path.is_file()
}

/// Plazo de cada prueba de codificación: un codificador de hardware colgado no debe
/// bloquear nunca la aplicación.
const PROBE_DEADLINE: std::time::Duration = std::time::Duration::from_secs(8);

/// Resultado de una prueba: `Transient` (plazo vencido o fallo al lanzar) no se guarda.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tried {
    Yes,
    No,
    Transient,
}

/// Argumentos de la codificación de prueba de un fotograma. Usa la misma cadena de color que
/// los vídeos reales (`scale`, `format`, `setparams`): un sidecar al que le falte un filtro
/// que el motor necesita sale de la sonda como «sin H.264» y las fichas de vídeo se
/// deshabilitan, en vez de fallar luego en cada exportación.
pub fn build_probe_args(png: &Path, out: &Path, platform: Platform, hevc: bool) -> Vec<String> {
    let mut args: Vec<String> = ["-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i"]
        .map(String::from)
        .to_vec();
    args.push(png.to_string_lossy().into_owned());
    args.extend(["-frames:v", "1", "-vf", video::BT709].map(String::from));
    args.extend(video::probe_encoder_args(platform, hevc));
    args.push(out.to_string_lossy().into_owned());
    args
}

fn try_encode(
    sidecar: &SidecarPaths,
    platform: Platform,
    hevc: bool,
    cancel: &dyn Fn() -> bool,
) -> Result<Tried, AudioError> {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "masterout-caps-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    if std::fs::create_dir_all(&dir).is_err() {
        return Ok(Tried::Transient);
    }
    let (png, out) = (dir.join("in.png"), dir.join("out.mp4"));
    let r = if ::image::RgbImage::from_pixel(PROBE_SIDE, PROBE_SIDE, ::image::Rgb([90, 90, 90]))
        .save(&png)
        .is_err()
    {
        Ok(Tried::Transient)
    } else {
        let args = build_probe_args(&png, &out, platform, hevc);
        match video::run_ffmpeg_with_deadline(sidecar, &args, cancel, Some(PROBE_DEADLINE)) {
            Ok(()) => Ok(Tried::Yes),
            Err(AudioError::Cancelled) => Err(AudioError::Cancelled),
            Err(e) if e.key_and_detail().0 == "err.ffmpeg_timeout" => Ok(Tried::Transient),
            Err(_) => Ok(Tried::No),
        }
    };
    let _ = std::fs::remove_dir_all(&dir);
    r
}

/// Qué codificadores H.264 y HEVC funcionan en este equipo: una codificación mínima de un
/// fotograma por códec, con plazo. Se guarda el resultado por sidecar y plataforma, salvo
/// si alguna prueba venció el plazo (se reintentará en la siguiente llamada).
pub fn capabilities(sidecar: &SidecarPaths, platform: Platform) -> MediaCaps {
    capabilities_cancellable(sidecar, platform, &|| false).unwrap_or(MediaCaps {
        h264: false,
        hevc: false,
    })
}

/// Como `capabilities`, pero `cancel` interrumpe la sonda (`None`).
pub fn capabilities_cancellable(
    sidecar: &SidecarPaths,
    platform: Platform,
    cancel: &dyn Fn() -> bool,
) -> Option<MediaCaps> {
    static CACHE: OnceLock<CapsCache> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let key = (sidecar.ffmpeg.clone(), platform);
    if let Some(c) = cache.lock().unwrap_or_else(|e| e.into_inner()).get(&key) {
        return Some(*c);
    }
    let h = try_encode(sidecar, platform, false, cancel).ok()?;
    // sin H.264 no se prueba HEVC: no habría vídeo de ningún modo
    let v = if h == Tried::Yes {
        try_encode(sidecar, platform, true, cancel).ok()?
    } else {
        Tried::No
    };
    let caps = MediaCaps {
        h264: h == Tried::Yes,
        hevc: v == Tried::Yes,
    };
    if h != Tried::Transient && v != Tried::Transient {
        cache
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(key, caps);
    }
    Some(caps)
}

/// Rutas de los ejecutables del sidecar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidecarPaths {
    pub ffmpeg: PathBuf,
    pub ffprobe: PathBuf,
}

const EXE_SUFFIX: &str = if cfg!(windows) { ".exe" } else { "" };

/// ¿Se admiten ffmpeg/ffprobe de fuera del paquete (`MASTERCHECK_FFMPEG`, `MASTERCHECK_FFPROBE`,
/// PATH)? Solo en compilaciones de depuración (los tests del motor usan el ffmpeg del sistema)
/// o con la feature `dev-sidecar`. Una app de producción solo ejecuta el sidecar LGPL que
/// viene junto al ejecutable: un `ffmpeg` del PATH puede ser GPL, un shim o un binario
/// manipulado, y un antivirus que ponga en cuarentena el empaquetado no debe hacer que se use
/// otro sin avisar.
pub const EXTERNAL_SIDECAR: bool = cfg!(any(debug_assertions, feature = "dev-sidecar"));

fn find_one(
    name: &str,
    env_var: &str,
    exe_dir: &Path,
    env: &dyn Fn(&str) -> Option<OsString>,
    path_var: Option<OsString>,
    allow_external: bool,
) -> Option<PathBuf> {
    let file = format!("{name}{EXE_SUFFIX}");
    // 1) junto al ejecutable (Tauri externalBin, sin sufijo de target en el bundle)
    let next_to_exe = exe_dir.join(&file);
    if next_to_exe.is_file() {
        return Some(next_to_exe);
    }
    if !allow_external {
        return None;
    }
    // 2) variable de entorno
    if let Some(v) = env(env_var).filter(|v| !v.is_empty()) {
        let p = PathBuf::from(v);
        if p.is_file() {
            return Some(p);
        }
    }
    // 3) PATH
    let path_var = path_var?;
    std::env::split_paths(&path_var)
        .map(|d| d.join(&file))
        .find(|p| p.is_file())
}

/// Versión pura de [`discover`] (entorno, PATH y política inyectados) para poder probarla.
/// Con `allow_external = false` solo cuenta lo que hay junto al ejecutable.
pub fn discover_policy(
    exe_dir: &Path,
    env: &dyn Fn(&str) -> Option<OsString>,
    path_var: Option<OsString>,
    allow_external: bool,
) -> Option<SidecarPaths> {
    Some(SidecarPaths {
        ffmpeg: find_one(
            "ffmpeg",
            "MASTERCHECK_FFMPEG",
            exe_dir,
            env,
            path_var.clone(),
            allow_external,
        )?,
        ffprobe: find_one(
            "ffprobe",
            "MASTERCHECK_FFPROBE",
            exe_dir,
            env,
            path_var,
            allow_external,
        )?,
    })
}

/// [`discover_policy`] con la política de esta compilación ([`EXTERNAL_SIDECAR`]).
pub fn discover_with(
    exe_dir: &Path,
    env: &dyn Fn(&str) -> Option<OsString>,
    path_var: Option<OsString>,
) -> Option<SidecarPaths> {
    discover_policy(exe_dir, env, path_var, EXTERNAL_SIDECAR)
}

/// Busca `ffmpeg` y `ffprobe` junto al ejecutable. En depuración o con la feature
/// `dev-sidecar` sigue con `MASTERCHECK_FFMPEG` / `MASTERCHECK_FFPROBE` y PATH. `None` si
/// falta alguno de los dos (las fichas de vídeo quedan deshabilitadas con `reason.no_sidecar`).
pub fn discover(exe_dir: &Path) -> Option<SidecarPaths> {
    discover_with(exe_dir, &|k| std::env::var_os(k), std::env::var_os("PATH"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn touch(p: &Path) {
        std::fs::write(p, b"x").unwrap();
    }
    fn exe(n: &str) -> String {
        format!("{n}{EXE_SUFFIX}")
    }

    #[test]
    fn discover_prefers_exe_dir_then_env_then_path() {
        let exe_dir = tempfile::tempdir().unwrap();
        let env_dir = tempfile::tempdir().unwrap();
        let path_dir = tempfile::tempdir().unwrap();
        for d in [&env_dir, &path_dir] {
            touch(&d.path().join(exe("ffmpeg")));
            touch(&d.path().join(exe("ffprobe")));
        }
        let path_var = std::env::join_paths([path_dir.path()]).ok();
        let env_ff = env_dir.path().join(exe("ffmpeg"));
        let env_fp = env_dir.path().join(exe("ffprobe"));
        let env = |k: &str| match k {
            "MASTERCHECK_FFMPEG" => Some(env_ff.clone().into_os_string()),
            "MASTERCHECK_FFPROBE" => Some(env_fp.clone().into_os_string()),
            _ => None,
        };
        // PATH y entorno: gana el entorno
        let got = discover_policy(exe_dir.path(), &env, path_var.clone(), true).unwrap();
        assert_eq!(got.ffmpeg, env_ff);
        // sin entorno: gana PATH
        let got = discover_policy(exe_dir.path(), &|_| None, path_var.clone(), true).unwrap();
        assert_eq!(got.ffprobe, path_dir.path().join(exe("ffprobe")));
        // junto al ejecutable: gana sobre todo
        touch(&exe_dir.path().join(exe("ffmpeg")));
        touch(&exe_dir.path().join(exe("ffprobe")));
        let got = discover_policy(exe_dir.path(), &env, path_var, true).unwrap();
        assert_eq!(got.ffmpeg, exe_dir.path().join(exe("ffmpeg")));
    }

    #[test]
    fn production_discover_only_looks_next_to_the_executable() {
        let exe_dir = tempfile::tempdir().unwrap();
        let other = tempfile::tempdir().unwrap();
        touch(&other.path().join(exe("ffmpeg")));
        touch(&other.path().join(exe("ffprobe")));
        let path_var = std::env::join_paths([other.path()]).ok();
        let env = |k: &str| match k {
            "MASTERCHECK_FFMPEG" => Some(other.path().join(exe("ffmpeg")).into_os_string()),
            "MASTERCHECK_FFPROBE" => Some(other.path().join(exe("ffprobe")).into_os_string()),
            _ => None,
        };
        // entorno y PATH poblados, nada junto al ejecutable: sin sidecar
        assert!(discover_policy(exe_dir.path(), &env, path_var.clone(), false).is_none());
        // solo uno de los dos junto al ejecutable: tampoco se completa desde fuera
        touch(&exe_dir.path().join(exe("ffmpeg")));
        assert!(discover_policy(exe_dir.path(), &env, path_var.clone(), false).is_none());
        // los dos junto al ejecutable: se usan esos
        touch(&exe_dir.path().join(exe("ffprobe")));
        let got = discover_policy(exe_dir.path(), &env, path_var, false).unwrap();
        assert_eq!(got.ffmpeg, exe_dir.path().join(exe("ffmpeg")));
        assert_eq!(got.ffprobe, exe_dir.path().join(exe("ffprobe")));
    }

    #[cfg(any(debug_assertions, feature = "dev-sidecar"))]
    #[test]
    fn external_sidecar_is_enabled_for_the_engine_tests() {
        // los tests del motor corren en depuración con el ffmpeg del sistema
        assert!(EXTERNAL_SIDECAR);
    }

    #[cfg(not(any(debug_assertions, feature = "dev-sidecar")))]
    #[test]
    fn external_sidecar_is_disabled_in_production_builds() {
        assert!(!EXTERNAL_SIDECAR);
        let d = tempfile::tempdir().unwrap();
        // `discover` real: sin binarios junto al ejecutable no hay sidecar, aunque haya PATH/env
        assert!(discover(d.path()).is_none());
    }

    #[test]
    fn discover_none_when_either_binary_is_missing() {
        let d = tempfile::tempdir().unwrap();
        touch(&d.path().join(exe("ffmpeg")));
        assert!(discover_policy(d.path(), &|_| None, None, true).is_none());
        assert!(discover_policy(Path::new("/nonexistent-dir"), &|_| None, None, true).is_none());
    }
}
