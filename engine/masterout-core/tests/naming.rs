use masterout_core::naming;
use masterout_core::naming::*;
use std::path::Path;

#[test]
fn spotify_example_matches_spec() {
    assert_eq!(
        audio_file_name(
            "Vera Alcón",
            "Luz de Noviembre",
            "Spotify",
            24,
            96000,
            "flac"
        ),
        "Vera Alcón - Luz de Noviembre [Spotify] 24bit-96kHz.flac"
    );
}

#[test]
fn deezer_example_44k() {
    assert_eq!(
        audio_file_name("Vera Alcón", "Luz de Noviembre", "Deezer", 16, 44100, "wav"),
        "Vera Alcón - Luz de Noviembre [Deezer] 16bit-44kHz.wav"
    );
}

#[test]
fn folder_with_version() {
    assert_eq!(
        delivery_folder("Vera Alcón", "Luz de Noviembre", Some("Acústico")),
        "Vera Alcón - Luz de Noviembre (Acústico)"
    );
    assert_eq!(delivery_folder("A", "B", None), "A - B");
}

#[test]
fn sanitizes_path_separators() {
    assert_eq!(
        audio_file_name("AC/DC", "¿Qué?: remix", "TIDAL", 24, 48000, "flac"),
        "AC-DC - ¿Qué-- remix [TIDAL] 24bit-48kHz.flac"
    );
}

#[test]
fn trims_trailing_dots_and_spaces() {
    assert_eq!(sanitize_component("Título..."), "Título");
    assert_eq!(sanitize_component("  Título . "), "Título");
}

#[test]
fn keeps_accents_and_non_latin() {
    assert_eq!(sanitize_component("Björk – Jóga 東京"), "Björk – Jóga 東京");
}

#[test]
fn limits_length_to_200_bytes_without_splitting_utf8() {
    let long = "東".repeat(200);
    let s = sanitize_component(&long);
    assert!(s.len() <= 200);
    assert!(s.chars().all(|c| c == '東'));
    let f = audio_file_name(&long, &long, "Spotify", 24, 96000, "flac");
    assert!(f.len() <= 200);
    assert!(f.ends_with(" [Spotify] 24bit-96kHz.flac"));
    assert!(delivery_folder(&long, &long, Some("Acústico")).len() <= 200);
}

#[test]
fn empty_title_falls_back_to_file_stem() {
    assert_eq!(
        fallback_title("", Path::new("Mi Cancion (master v3).wav")),
        "Mi Cancion"
    );
    assert_eq!(fallback_title("X", Path::new("a.wav")), "X");
}

#[test]
fn khz_labels() {
    assert_eq!(khz_label(44100), "44kHz");
    assert_eq!(khz_label(88200), "88kHz");
    assert_eq!(khz_label(176400), "176kHz");
    assert_eq!(khz_label(192000), "192kHz");
}

#[test]
fn media_examples() {
    assert_eq!(
        media_file_name("Vera Alcón", "Luz", "Portada", "3000x3000", "jpg"),
        "Vera Alcón - Luz [Portada] 3000x3000.jpg"
    );
    assert_eq!(
        media_file_name("Vera Alcón", "Luz", "Canvas", "1080x1920", "mp4"),
        "Vera Alcón - Luz [Canvas] 1080x1920.mp4"
    );
    assert_eq!(
        media_file_name("Vera Alcón", "Luz", "MP3 320", "", "mp3"),
        "Vera Alcón - Luz [MP3 320].mp3"
    );
}

#[test]
fn long_version_tag_and_suffix_never_panic_or_exceed_200() {
    let long = "x".repeat(300);
    assert!(delivery_folder("Artista", "Título", Some(&long)).len() <= 200);
    assert!(delivery_folder(&long, &long, Some(&long)).len() <= 200);
    let f = audio_file_name(&long, &long, &long, 24, 96000, "flac");
    assert!(f.len() <= 200 && f.ends_with(" 24bit-96kHz.flac"));
    let m = media_file_name(&long, &long, &long, &long, "jpg");
    assert!(m.len() <= 200 && m.ends_with(".jpg"));
}

#[test]
fn english_contract_folder_and_document_names() {
    use masterout_core::i18n::Lang;
    assert_eq!(naming::subfolder("distributor", Lang::En), "07 Distributor");
    assert_eq!(naming::subfolder("artwork", Lang::En), "09 Artwork");
    assert_eq!(naming::subfolder("spotify", Lang::En), "01 Spotify");
    assert_eq!(naming::subfolder("youtube", Lang::En), "06 YouTube");
    let d = naming::document_names(Lang::En);
    assert_eq!(d.readme, "README.txt");
    assert_eq!(d.loudness_report, "loudness-report.txt");
    assert_eq!(d.sheet_csv, "release-sheet.csv");
    assert_eq!(d.sheet_txt, "release-sheet.txt");
    let cover = masterout_core::presets::all()
        .into_iter()
        .find(|p| p.id == "cover")
        .unwrap();
    assert_eq!(naming::preset_tag(&cover, Lang::En), "Cover");
    assert_eq!(naming::preset_tag(&cover, Lang::Es), "Portada");
}

#[test]
fn spanish_contract_unchanged() {
    use masterout_core::i18n::Lang;
    assert_eq!(
        naming::subfolder("distributor", Lang::Es),
        "07 Distribuidora"
    );
    assert_eq!(naming::subfolder("artwork", Lang::Es), "09 Imagen");
    assert_eq!(naming::subfolder("promo", Lang::Es), "08 Promo");
    let d = naming::document_names(Lang::Es);
    assert_eq!(d.readme, "README.txt");
    assert_eq!(d.loudness_report, "informe-loudness.txt");
    assert_eq!(d.sheet_csv, "ficha-lanzamiento.csv");
    assert_eq!(d.sheet_txt, "ficha-lanzamiento.txt");
}

#[test]
fn official_spelling_in_tags_and_folders_in_both_languages() {
    use masterout_core::{i18n::Lang, presets};
    let all = presets::all();
    let get = |id: &str| all.iter().find(|p| p.id == id).unwrap();
    assert_eq!(preset_tag(get("tidal"), Lang::Es), "TIDAL");
    assert_eq!(preset_tag(get("tidal"), Lang::En), "TIDAL");
    assert_eq!(subfolder("tidal", Lang::Es), "04 TIDAL");
    assert_eq!(subfolder("tidal", Lang::En), "04 TIDAL");
    assert_eq!(preset_tag(get("apple_motion"), Lang::Es), "Portada animada");
    assert_eq!(preset_tag(get("apple_motion"), Lang::En), "Motion artwork");
    assert_eq!(preset_tag(get("apple_adm"), Lang::En), "Apple Music");
}
