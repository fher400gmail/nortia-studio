### Task 9: i18n y documentos (README, informe, ficha)

**Files:**
- Create: `engine/masterout-core/i18n/es.json`, `engine/masterout-core/i18n/en.json`, `engine/masterout-core/src/i18n.rs`, `engine/masterout-core/src/reports.rs`
- Test: `engine/masterout-core/tests/reports.rs`

**Interfaces:**
- Produces:
  ```rust
  #[derive(Clone, Copy)] pub enum Lang { Es, En }
  impl Lang { pub fn from_tag(tag: &str) -> Lang /* "es-ES"→Es, "en"→En, otro→En */; pub fn from_system() -> Lang /* LANG/LC_ALL; en Mac/Windows lo pasará la app */ }
  pub fn i18n::t(lang: Lang, key: &str) -> String  // falta la clave → devuelve "⟦key⟧" y lo registra; test exige que es.json y en.json tengan EXACTAMENTE las mismas claves
  pub fn i18n::tf(lang: Lang, key: &str, args: &[(&str, &str)]) -> String // sustituye {name}
  pub struct DeliverySummary<'a> { pub meta: &'a ReleaseMeta, pub master: &'a MasterInfo, pub jobs: &'a [Resolved], pub created_at: String /* ISO local */, pub app_version: &'a str }
  pub fn reports::readme(lang: Lang, s: &DeliverySummary) -> String
  pub fn reports::loudness_report(lang: Lang, s: &DeliverySummary) -> String
  pub fn reports::release_sheet_csv(s: &DeliverySummary) -> String   // cabeceras en inglés fijas (las distribuidoras las leen): artist,title,version,featuring,album,year,isrc,composer,label,copyright,duration_sec,sample_rate,bits,integrated_lufs,true_peak_dbtp,lra_lu
  pub fn reports::release_sheet_txt(lang: Lang, s: &DeliverySummary) -> String
  ```
  Contenido fijado: el README empieza con la línea `{Artista} - {Título}` y después, por cada subcarpeta, una línea `01 Spotify/  →  …flac  (24 bit · 96 kHz · FLAC)` y la frase de i18n `readme.master_untouched` ("Tu máster no se ha modificado. Master Out mide y avisa."). El informe incluye I, LRA, TP, sample peak, overs y una línea por destino con el veredicto. Nada de frases largas.

- [ ] **Step 1: Tests que fallan**:
  ```rust
  #[test] fn es_and_en_have_identical_key_sets()
  #[test] fn missing_key_is_visible_not_panic() // t(Es,"nope") == "⟦nope⟧"
  #[test] fn lang_from_tag() // "es-MX"→Es, "EN_us"→En, "fr"→En
  #[test] fn readme_lists_each_active_job_once_in_order() // 3 jobs → 3 líneas con "01 Spotify/", "05 Deezer/", "08 Promo/" en ese orden; contiene t(lang,"readme.master_untouched")
  #[test] fn readme_marks_upsampling_and_disabled() // tunecore → contiene t("readme.upsampling"); apple Disabled → contiene t("readme.disabled") + motivo
  #[test] fn loudness_report_has_numbers_with_comma_in_es_and_dot_in_en() // "−9,1 LUFS" en es, "-9.1 LUFS" en en
  #[test] fn csv_has_fixed_header_and_one_row() // primera línea exacta; campos vacíos permitidos; comas escapadas
  #[test] fn release_sheet_txt_in_both_languages_no_missing_keys() // no contiene "⟦"
  ```
- [ ] **Steps 2–5.** Commit `feat(core): i18n es/en y documentos de entrega`.

---

