//! Comandos Tauri: capa finísima sobre `mastercheck-bridge`.

use mastercheck_bridge::app_support::{self, busy_error, is_image_path, AppInfo};
use mastercheck_bridge::dto::{
    DeliveryDto, ExportRequestDto, MasterDto, PlanDto, ReleaseMetaDto, WaveformDto,
};
use mastercheck_bridge::errors::BridgeError;
use mastercheck_bridge::{analysis, exporting, planning};
use masterout_core::i18n::Lang;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State, WebviewWindow};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

/// Nombre del evento de progreso (el frontend lo escucha con `listen`).
pub const PROGRESS_EVENT: &str = "mastercheck://progress";

/// Idioma de los mensajes del motor (lo fija el sistema y puede forzarlo la interfaz).
pub struct LangState(pub Mutex<Lang>);

/// Bandera de cancelación de la exportación en curso.
#[derive(Default)]
pub struct ExportState {
    pub cancel: Mutex<Option<Arc<AtomicBool>>>,
}

#[derive(Serialize)]
pub struct AnalyzeResponse {
    pub master: MasterDto,
    pub waveform: WaveformDto,
}

fn current_lang(state: &State<'_, LangState>) -> Lang {
    *state.0.lock().unwrap_or_else(|e| e.into_inner())
}

fn internal(lang: Lang, detail: String) -> BridgeError {
    BridgeError::from_key("err.internal", lang, Some(detail))
}

#[tauri::command]
pub async fn analyze(
    app: AppHandle,
    lang: State<'_, LangState>,
    path: String,
    bins: u32,
) -> Result<AnalyzeResponse, BridgeError> {
    let lang = current_lang(&lang);
    let res = tauri::async_runtime::spawn_blocking(move || {
        analysis::analyze(std::path::Path::new(&path), lang, bins as usize)
            .map(|(master, waveform)| AnalyzeResponse { master, waveform })
    })
    .await
    .map_err(|e| internal(lang, e.to_string()))??;
    // El máster analizado se puede reproducir (asset protocol); el ámbito empieza vacío.
    let _ = app.asset_protocol_scope().allow_file(&res.master.path);
    Ok(res)
}

#[tauri::command]
pub fn plan(
    lang: State<'_, LangState>,
    master: MasterDto,
    meta: ReleaseMetaDto,
    active_ids: Vec<String>,
) -> PlanDto {
    planning::plan(&master, &meta, &active_ids, current_lang(&lang))
}

#[tauri::command]
pub async fn export(
    app: AppHandle,
    lang: State<'_, LangState>,
    state: State<'_, ExportState>,
    req: ExportRequestDto,
) -> Result<DeliveryDto, BridgeError> {
    let lang = current_lang(&lang);
    let cancel = Arc::new(AtomicBool::new(false));
    {
        // Una sola exportación a la vez: comprobar y reservar bajo el mismo cerrojo.
        let mut slot = state.cancel.lock().unwrap_or_else(|e| e.into_inner());
        if slot.is_some() {
            return Err(busy_error(lang));
        }
        *slot = Some(cancel.clone());
    }
    let result = tauri::async_runtime::spawn_blocking(move || {
        exporting::export(
            &req,
            lang,
            &|p| {
                let _ = app.emit(PROGRESS_EVENT, p);
            },
            cancel,
        )
    })
    .await
    .map_err(|e| internal(lang, e.to_string()))
    .and_then(|r| r);
    *state.cancel.lock().unwrap_or_else(|e| e.into_inner()) = None;
    result
}

#[tauri::command]
pub fn cancel_export(state: State<'_, ExportState>) {
    if let Some(flag) = state
        .cancel
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_ref()
    {
        flag.store(true, Ordering::SeqCst);
    }
}

fn to_string_path(p: tauri_plugin_dialog::FilePath) -> Option<String> {
    p.into_path().ok().map(|p| p.to_string_lossy().into_owned())
}

#[tauri::command]
pub async fn pick_folder(app: AppHandle) -> Option<String> {
    app.dialog()
        .file()
        .blocking_pick_folder()
        .and_then(to_string_path)
}

/// Selector de archivo (el `<input type=file>` del webview no da la ruta real).
/// `kind`: `"audio"` (WAV) o `"image"`.
#[tauri::command]
pub async fn pick_file(app: AppHandle, kind: String) -> Option<String> {
    let dialog = app.dialog().file();
    let image = kind == "image";
    let dialog = if image {
        dialog.add_filter("image", &["jpg", "jpeg", "png"])
    } else {
        dialog.add_filter("audio", &["wav", "wave"])
    };
    let path = dialog.blocking_pick_file().and_then(to_string_path)?;
    // Lo elegido en el selector se puede mostrar o reproducir; el resto del disco, no.
    let _ = app.asset_protocol_scope().allow_file(&path);
    Some(path)
}

/// Autoriza ver una imagen arrastrada (portada). Solo `.png`, `.jpg` y `.jpeg`.
#[tauri::command]
pub fn allow_asset(
    app: AppHandle,
    lang: State<'_, LangState>,
    path: String,
) -> Result<(), BridgeError> {
    if !is_image_path(&path) {
        return Err(BridgeError::from_key(
            "err.generic",
            current_lang(&lang),
            Some(format!("tipo de archivo no permitido: {path}")),
        ));
    }
    app.asset_protocol_scope()
        .allow_file(&path)
        .map_err(|e| BridgeError::from_key("err.io", current_lang(&lang), Some(e.to_string())))
}

/// Revela la carpeta (o archivo) en el explorador del sistema.
#[tauri::command]
pub fn open_path(
    app: AppHandle,
    lang: State<'_, LangState>,
    path: String,
) -> Result<(), BridgeError> {
    app.opener()
        .reveal_item_in_dir(&path)
        .map_err(|e| BridgeError::from_key("err.io", current_lang(&lang), Some(e.to_string())))
}

#[tauri::command]
pub fn system_lang(lang: State<'_, LangState>) -> String {
    match current_lang(&lang) {
        Lang::Es => "es".into(),
        Lang::En => "en".into(),
    }
}

/// La interfaz fija el idioma (p. ej. `?lang=` o el del sistema) para que los mensajes
/// del motor coincidan con los suyos.
#[tauri::command]
pub fn set_lang(lang: State<'_, LangState>, tag: String) {
    *lang.0.lock().unwrap_or_else(|e| e.into_inner()) =
        mastercheck_bridge::lang::Lang::from_locale(&tag);
}

#[tauri::command]
pub fn app_info(app: AppHandle) -> AppInfo {
    let default_out = app
        .path()
        .desktop_dir()
        .ok()
        .map(|d| d.join("master check").to_string_lossy().into_owned());
    app_support::app_info(
        &app.package_info().version.to_string(),
        &crate::exe_dir(),
        default_out,
    )
}

/// El frontend avisa cuando ha pintado: entonces se muestra la ventana (nace oculta).
#[tauri::command]
pub fn frontend_ready(window: WebviewWindow) {
    let _ = window.show();
}

pub fn run_app() {
    crate::prepare_lame_path(&crate::exe_dir());
    let locale = tauri_plugin_os::locale().unwrap_or_default();
    let lang = mastercheck_bridge::lang::Lang::from_locale(&locale);
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .manage(LangState(Mutex::new(lang)))
        .manage(ExportState::default())
        .setup(|app| {
            // Red de seguridad: si el frontend no avisa, la ventana se muestra igualmente.
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(4));
                if let Some(w) = handle.get_webview_window("main") {
                    let _ = w.show();
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            analyze,
            plan,
            export,
            cancel_export,
            pick_folder,
            pick_file,
            allow_asset,
            open_path,
            system_lang,
            set_lang,
            app_info,
            frontend_ready,
        ])
        .run(tauri::generate_context!())
        .expect("error al iniciar master check");
}
