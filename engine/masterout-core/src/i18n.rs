//! Textos de la interfaz y de los documentos en español e inglés.

use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    Es,
    En,
}

impl Lang {
    /// "es-ES" -> Es, "en" -> En, cualquier otro -> En. Sin distinguir mayúsculas.
    pub fn from_tag(tag: &str) -> Lang {
        let primary = tag
            .split(['-', '_', '.'])
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        if primary == "es" {
            Lang::Es
        } else {
            Lang::En
        }
    }

    /// Idioma del sistema según `LC_ALL`, `LC_MESSAGES` o `LANG`.
    pub fn from_system() -> Lang {
        for var in ["LC_ALL", "LC_MESSAGES", "LANG"] {
            if let Ok(v) = std::env::var(var) {
                if !v.is_empty() {
                    return Lang::from_tag(&v);
                }
            }
        }
        Lang::En
    }
}

type Table = HashMap<String, String>;

fn load(src: &str) -> Table {
    serde_json::from_str(src).expect("i18n JSON inválido")
}

fn table(lang: Lang) -> &'static Table {
    static ES: OnceLock<Table> = OnceLock::new();
    static EN: OnceLock<Table> = OnceLock::new();
    match lang {
        Lang::Es => ES.get_or_init(|| load(include_str!("../i18n/es.json"))),
        Lang::En => EN.get_or_init(|| load(include_str!("../i18n/en.json"))),
    }
}

/// Claves de un idioma (para comprobar que es/en coinciden).
pub fn keys(lang: Lang) -> Vec<String> {
    let mut k: Vec<String> = table(lang).keys().cloned().collect();
    k.sort();
    k
}

/// Texto de una clave; si falta devuelve `⟦clave⟧` y lo registra en stderr.
pub fn t(lang: Lang, key: &str) -> String {
    match table(lang).get(key) {
        Some(v) => v.clone(),
        None => {
            eprintln!("i18n: falta la clave '{key}' ({lang:?})");
            format!("⟦{key}⟧")
        }
    }
}

/// Como `t`, sustituyendo `{nombre}` por su valor.
pub fn tf(lang: Lang, key: &str, args: &[(&str, &str)]) -> String {
    let mut s = t(lang, key);
    for (name, value) in args {
        s = s.replace(&format!("{{{name}}}"), value);
    }
    s
}
