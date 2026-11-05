//! `lame::resolve_lame_path` desde el crate de la app (sin Tauri: `--no-default-features`).

use mastercheck_app_lib::lame::{resolve_lame_in, resolve_lame_path};
use std::path::Path;

fn fake_lame(dir: &Path) -> std::path::PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let p = dir.join(if cfg!(windows) { "lame.exe" } else { "lame" });
    std::fs::write(&p, b"fake").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    p
}

#[test]
fn resolve_lame_path_prefers_bundle_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let exe_dir = tmp.path().join("Carpeta con espacios y acentos ñ");
    let lame = fake_lame(&exe_dir);
    assert_eq!(resolve_lame_path(&exe_dir), Some(lame));
}

#[test]
fn resolve_lame_path_falls_back_to_path_env() {
    let tmp = tempfile::tempdir().unwrap();
    let exe_dir = tmp.path().join("vacio");
    std::fs::create_dir_all(&exe_dir).unwrap();
    let lame = fake_lame(&tmp.path().join("bin"));
    let path = std::env::join_paths([tmp.path().join("bin")]).unwrap();
    assert_eq!(resolve_lame_in(&exe_dir, Some(&path)), Some(lame));
}

#[test]
fn system_lang_maps_locale() {
    use mastercheck_app_lib::locale::map_locale;
    assert_eq!(map_locale("es-ES"), "es");
    assert_eq!(map_locale("en-US"), "en");
    assert_eq!(map_locale(""), "en");
}
