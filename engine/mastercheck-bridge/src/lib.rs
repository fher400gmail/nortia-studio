//! Puente entre el motor `masterout-core` y la interfaz: DTOs serde estables (campos en
//! snake_case, tal cual los de Rust), análisis con forma de onda, plan, exportación con
//! progreso y errores localizados. Sin dependencias de Tauri: se prueba en cualquier sitio.

pub mod analysis;
pub mod dto;
pub mod errors;
pub mod exporting;
pub mod lang;
pub mod planning;

/// Idioma del motor, para quien use el puente sin depender de `masterout-core`.
pub use masterout_core::i18n::Lang as EngineLang;
