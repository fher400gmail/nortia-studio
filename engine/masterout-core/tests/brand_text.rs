//! Marcas de terceros (D-31, opción B): ningún texto visible del motor implica aval, certificación
//! o un programa ajeno, y la grafía de las plataformas es la oficial. Base: docs/legal/marcas-plataformas.md.

use masterout_core::i18n::{keys, t, Lang};
use masterout_core::presets;
use std::collections::BTreeMap;

/// Cadenas prohibidas en cualquier texto visible (es/en).
const FORBIDDEN: [&str; 12] = [
    "Digital Masters",
    "certificado",
    "certificada",
    "certified",
    "aprobado por",
    "aprobada por",
    "approved by",
    "optimizado para",
    "optimizada para",
    "optimized for",
    "Mastered for",
    "Ready",
];

fn visible_texts() -> Vec<(String, String)> {
    let mut out = vec![];
    for lang in [Lang::Es, Lang::En] {
        for k in keys(lang) {
            out.push((format!("i18n {lang:?} {k}"), t(lang, &k)));
        }
    }
    for p in presets::all() {
        out.push((format!("preset {} name", p.id), p.name.clone()));
        out.push((format!("preset {} tag", p.id), p.tag.clone()));
        out.push((
            format!("preset {} subfolder", p.id),
            p.subfolder_key.clone(),
        ));
    }
    out
}

#[test]
fn no_visible_text_implies_endorsement_or_certification() {
    for (what, text) in visible_texts() {
        for bad in FORBIDDEN {
            // «Ready» distingue mayúsculas (no debe saltar con «already»)
            let hit = if bad == "Ready" {
                text.contains(bad)
            } else {
                text.to_lowercase().contains(&bad.to_lowercase())
            };
            assert!(!hit, "{what}: «{text}» contiene «{bad}»");
        }
    }
}

#[test]
fn i18n_and_presets_use_the_official_spelling() {
    for lang in [Lang::Es, Lang::En] {
        assert_eq!(t(lang, "preset.tidal"), "TIDAL");
        assert_eq!(t(lang, "preset.apple_adm"), "Apple Music");
        assert_eq!(t(lang, "preset.amazon"), "Amazon Music");
    }
    assert_eq!(t(Lang::Es, "preset.youtube_video"), "YouTube (vídeo)");
    assert_eq!(t(Lang::En, "preset.youtube_video"), "YouTube (video)");
    assert_eq!(
        t(Lang::Es, "preset.apple_motion"),
        "Portada animada (Apple Music)"
    );
    assert_eq!(
        t(Lang::En, "preset.apple_motion"),
        "Motion artwork (Apple Music)"
    );
    assert_eq!(
        t(Lang::Es, "apple_needs_24bit"),
        "Apple Music pide un máster de 24 bit o más"
    );
    assert_eq!(
        t(Lang::En, "apple_needs_24bit"),
        "Apple Music asks for a 24-bit master or higher"
    );
    let by_id: BTreeMap<_, _> = presets::all()
        .into_iter()
        .map(|p| (p.id.clone(), p))
        .collect();
    assert_eq!(by_id["apple_adm"].name, "Apple Music");
    assert_eq!(by_id["apple_adm"].tag, "Apple Music");
    assert_eq!(by_id["tidal"].name, "TIDAL");
}

#[test]
fn trademark_notice_is_in_both_languages() {
    assert_eq!(
        t(Lang::Es, "readme.trademarks"),
        "Spotify, Apple Music, Amazon Music, TIDAL, Deezer, YouTube y TuneCore son marcas de sus titulares. master check no está afiliado ni avalado por ellos."
    );
    assert_eq!(
        t(Lang::En, "readme.trademarks"),
        "Spotify, Apple Music, Amazon Music, TIDAL, Deezer, YouTube and TuneCore are trademarks of their respective owners. master check is not affiliated with or endorsed by them."
    );
}
