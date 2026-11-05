fn main() {
    // Sin la feature `app` (pruebas de lógica pura) no se enlaza Tauri ni se validan sus permisos.
    #[cfg(feature = "app")]
    tauri_build::build();
}
