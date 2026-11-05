### Task 7: Contrato de nombres

**Files:**
- Create: `engine/masterout-core/src/naming.rs`
- Test: `engine/masterout-core/tests/naming.rs`

**Interfaces:**
- Produces:
  ```rust
  pub fn sanitize_component(s: &str) -> String   // reemplaza / \ : * ? " < > | y controles por '-', colapsa espacios, recorta espacios y puntos finales, limita a 200 bytes sin partir un carácter
  pub fn khz_label(sample_rate: u32) -> String   // 44100→"44kHz", 48000→"48kHz", 88200→"88kHz", 96000→"96kHz", 176400→"176kHz", 192000→"192kHz"
  pub fn delivery_folder(artist: &str, title: &str, version: Option<&str>) -> String   // "Artista - Título" o "Artista - Título (Versión)"
  pub fn audio_file_name(artist: &str, title: &str, tag: &str, bits: u16, sample_rate: u32, ext: &str) -> String  // "Artista - Título [Tag] 24bit-96kHz.flac"
  pub fn media_file_name(artist: &str, title: &str, tag: &str, suffix: &str, ext: &str) -> String  // "Artista - Título [Portada] 3000x3000.jpg", "[YouTube] 1080p.mp4", "[MP3 320].mp3" (suffix vacío → sin espacio extra)
  ```
- [ ] **Step 1: Tests que fallan** (valores exactos del contrato):
  ```rust
  #[test] fn spotify_example_matches_spec() // ("Vera Alcón","Luz de Noviembre","Spotify",24,96000,"flac") == "Vera Alcón - Luz de Noviembre [Spotify] 24bit-96kHz.flac"
  #[test] fn deezer_example_44k() // == "Vera Alcón - Luz de Noviembre [Deezer] 16bit-44kHz.wav"
  #[test] fn folder_with_version() // ("Vera Alcón","Luz de Noviembre",Some("Acústico")) == "Vera Alcón - Luz de Noviembre (Acústico)"
  #[test] fn sanitizes_path_separators() // artist "AC/DC", title "¿Qué?: remix" → "AC-DC - ¿Qué-- remix [Tidal] 24bit-48kHz.flac" (regla: cada carácter prohibido → '-', sin colapsar guiones)
  #[test] fn trims_trailing_dots_and_spaces() // "Título..." → "Título"
  #[test] fn keeps_accents_and_non_latin() // "Björk – Jóga 東京" se conserva
  #[test] fn limits_length_to_200_bytes_without_splitting_utf8()
  #[test] fn empty_title_falls_back_to_file_stem() // naming::fallback_title("", Path("Mi Cancion (master v3).wav")) == "Mi Cancion"  (quita sufijos entre paréntesis que contengan "master", "mix", "v\d+")
  #[test] fn media_examples() // "[Portada] 3000x3000.jpg", "[Canvas] 1080x1920.mp4", "[MP3 320].mp3"
  ```
- [ ] **Steps 2–5.** Commit `feat(core): contrato de nombres de carpeta y archivos`.

---

