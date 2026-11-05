//! master check · app de escritorio (Tauri 2). Comandos finos sobre `mastercheck-bridge`.
//!
//! La lógica que no necesita Tauri (lame, locale, gancho de humo) vive en
//! `mastercheck_bridge::app_support` y se prueba en cualquier sitio; los módulos de Tauri
//! están tras la feature `app` (activa por defecto).

pub mod lame;
pub mod locale;

#[cfg(feature = "app")]
pub mod commands;

use std::path::PathBuf;

/// Directorio del ejecutable actual.
pub fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_default()
}

/// Gancho de humo: con `MASTERCHECK_SMOKE="<wav>;<carpeta_salida>"` ejecuta analizar +
/// planificar + exportar por el puente, imprime el `DeliveryDto` en JSON y termina con
/// 0/1 sin abrir ventana.
fn smoke_hook() -> Option<i32> {
    let spec = std::env::var("MASTERCHECK_SMOKE").ok()?;
    let dir = exe_dir();
    // El motor busca `lame` junto al ejecutable y en PATH: añade también el empaquetado.
    prepare_lame_path(&dir);
    let tag = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LANG"))
        .unwrap_or_default();
    let lang = mastercheck_bridge::lang::Lang::from_locale(&tag);
    Some(mastercheck_bridge::app_support::smoke::run_and_report(
        &spec, lang, &dir,
    ))
}

/// Pone la carpeta donde está `lame` al principio de `PATH` para que el motor la encuentre
/// (macOS lo empaqueta en `Contents/Resources`).
pub fn prepare_lame_path(exe_dir: &std::path::Path) {
    if let Some(lame) = lame::resolve_lame_path(exe_dir) {
        if let Some(dir) = lame.parent() {
            let path = lame::path_with_dir_first(dir, std::env::var_os("PATH"));
            std::env::set_var("PATH", path);
        }
    }
}

pub fn run() {
    if let Some(code) = smoke_hook() {
        std::process::exit(code);
    }
    #[cfg(feature = "app")]
    commands::run_app();
    #[cfg(not(feature = "app"))]
    eprintln!("master check: compilado sin la feature `app`");
}
