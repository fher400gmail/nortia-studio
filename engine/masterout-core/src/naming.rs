//! Contrato de nombres de carpeta y archivos de entrega.

use std::path::Path;

const MAX_BYTES: usize = 200;

fn is_forbidden(c: char) -> bool {
    matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') || c.is_control()
}

fn truncate_bytes(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

fn trim_tail(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_whitespace())
        .trim_end_matches(|c: char| c == '.' || c.is_whitespace())
}

fn sanitize_max(s: &str, max: usize) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_space = false;
    for c in s.chars() {
        if is_forbidden(c) {
            out.push('-');
            last_space = false;
        } else if c.is_whitespace() {
            if !last_space {
                out.push(' ');
            }
            last_space = true;
        } else {
            out.push(c);
            last_space = false;
        }
    }
    trim_tail(truncate_bytes(trim_tail(&out), max)).to_string()
}

/// Sanea un componente de nombre: caracteres prohibidos y de control a `-`,
/// espacios colapsados, sin espacios/puntos finales, máximo 200 bytes.
pub fn sanitize_component(s: &str) -> String {
    sanitize_max(s, MAX_BYTES)
}

/// `44100` -> `"44kHz"`, `176400` -> `"176kHz"` (sin decimales).
pub fn khz_label(sample_rate: u32) -> String {
    format!("{}kHz", sample_rate / 1000)
}

fn base_name(artist: &str, title: &str) -> String {
    format!(
        "{} - {}",
        sanitize_component(artist),
        sanitize_component(title)
    )
}

/// `"Artista - Título"` o `"Artista - Título (Versión)"`.
pub fn delivery_folder(artist: &str, title: &str, version: Option<&str>) -> String {
    let mut name = base_name(artist, title);
    let suffix = match version.map(sanitize_component) {
        Some(v) if !v.is_empty() => format!(" ({v})"),
        _ => String::new(),
    };
    name = format!(
        "{}{}",
        sanitize_max(&name, MAX_BYTES - suffix.len()),
        suffix
    );
    name
}

fn compose(artist: &str, title: &str, tail: &str) -> String {
    let budget = MAX_BYTES.saturating_sub(tail.len());
    let base = sanitize_max(&base_name(artist, title), budget);
    format!("{base}{tail}")
}

/// `"Artista - Título [Tag] 24bit-96kHz.flac"`.
pub fn audio_file_name(
    artist: &str,
    title: &str,
    tag: &str,
    bits: u16,
    sample_rate: u32,
    ext: &str,
) -> String {
    let tail = format!(
        " [{}] {}bit-{}.{}",
        sanitize_component(tag),
        bits,
        khz_label(sample_rate),
        ext
    );
    compose(artist, title, &tail)
}

/// `"Artista - Título [Portada] 3000x3000.jpg"`; con `suffix` vacío no hay espacio extra.
pub fn media_file_name(artist: &str, title: &str, tag: &str, suffix: &str, ext: &str) -> String {
    let suffix = sanitize_component(suffix);
    let sep = if suffix.is_empty() { "" } else { " " };
    let tail = format!(" [{}]{}{}.{}", sanitize_component(tag), sep, suffix, ext);
    compose(artist, title, &tail)
}

fn is_noise_group(inner: &str) -> bool {
    let l = inner.to_lowercase();
    if l.contains("master") || l.contains("mix") {
        return true;
    }
    let b = l.as_bytes();
    (0..b.len().saturating_sub(1)).any(|i| b[i] == b'v' && b[i + 1].is_ascii_digit())
}

/// Título de reserva: el propio si no está vacío; si no, el nombre de archivo sin
/// extensión y sin sufijos entre paréntesis tipo "(master v3)".
pub fn fallback_title(title: &str, path: &Path) -> String {
    if !title.trim().is_empty() {
        return title.trim().to_string();
    }
    let mut stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().trim().to_string())
        .unwrap_or_default();
    loop {
        let t = stem.trim_end();
        if t.ends_with(')') {
            if let Some(open) = t.rfind('(') {
                if is_noise_group(&t[open + 1..t.len() - 1]) {
                    stem = t[..open].trim_end().to_string();
                    continue;
                }
            }
        }
        break;
    }
    stem.trim().to_string()
}
