use mastercheck_bridge::dto::{MasterDto, ReleaseMetaDto};
use mastercheck_bridge::planning::plan;
use masterout_core::i18n::Lang;

fn master(bits: u16) -> MasterDto {
    MasterDto {
        path: "/música/Vera Alcón - Luz de Noviembre (master v3).wav".into(),
        file_name: "Vera Alcón - Luz de Noviembre (master v3).wav".into(),
        sample_rate: 96_000,
        bits,
        is_float: false,
        channels: 2,
        frames: 96_000 * 60,
        duration_secs: 60.0,
        integrated_lufs: -14.0,
        true_peak_dbtp: -1.5,
        lra_lu: 5.0,
        overs_count: 0,
        suggested_artist: Some("Vera Alcón".into()),
        suggested_title: Some("Luz de Noviembre".into()),
    }
}

fn meta() -> ReleaseMetaDto {
    ReleaseMetaDto {
        artist: "Vera Alcón".into(),
        title: "Luz de Noviembre".into(),
        year: Some("2026".into()),
        ..Default::default()
    }
}

fn ids(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[test]
fn plan_dto_marks_disabled_apple_for_16bit_with_reason() {
    let p = plan(
        &master(16),
        &meta(),
        &ids(&["spotify", "apple_adm"]),
        &Default::default(),
        Lang::Es,
    );
    let apple = p.jobs.iter().find(|j| j.preset_id == "apple_adm").unwrap();
    assert!(!apple.enabled);
    assert!(apple.disabled_reason.as_deref().unwrap().contains("24 bit"));
    assert_eq!(apple.subfolder, "02 Apple Music");
    let spotify = p.jobs.iter().find(|j| j.preset_id == "spotify").unwrap();
    assert!(spotify.enabled && spotify.disabled_reason.is_none());
    assert_eq!(spotify.container, "flac");
    assert_eq!(spotify.verdict, "ok");
    assert_eq!(spotify.verdict_text, "Correcto");
    assert!(spotify.estimated_bytes > 0);
    assert_eq!(p.delivery_folder, "Vera Alcón - Luz de Noviembre");
    // 1 destino habilitado + 4 documentos.
    assert_eq!(p.total_files, 5);
}

#[test]
fn plan_dto_totals_count_documents() {
    let p = plan(
        &master(24),
        &meta(),
        &ids(&["spotify", "apple_adm", "deezer"]),
        &Default::default(),
        Lang::Es,
    );
    assert_eq!(p.jobs.len(), 3);
    assert!(p.jobs.iter().all(|j| j.enabled));
    assert_eq!(p.total_files, 7);
    assert_eq!(
        p.total_bytes,
        p.jobs.iter().map(|j| j.estimated_bytes).sum::<u64>()
    );
}

#[test]
fn plan_dto_english_names_use_english_contract() {
    let p = plan(
        &master(24),
        &meta(),
        &ids(&["distributor_std"]),
        &Default::default(),
        Lang::En,
    );
    let j = &p.jobs[0];
    assert_eq!(j.subfolder, "07 Distributor");
    assert!(j.file_name.contains("[Distributor]"), "{}", j.file_name);
    let es = plan(
        &master(24),
        &meta(),
        &ids(&["distributor_std"]),
        &Default::default(),
        Lang::Es,
    );
    assert_eq!(es.jobs[0].subfolder, "07 Distribuidora");
    assert!(es.jobs[0].file_name.contains("[Distribuidora]"));
}

#[test]
fn plan_dto_verdict_is_localized_and_fail_marked() {
    let mut m = master(24);
    m.true_peak_dbtp = 0.5;
    let p = plan(
        &m,
        &meta(),
        &ids(&["spotify"]),
        &Default::default(),
        Lang::En,
    );
    assert_eq!(p.jobs[0].verdict, "fail");
    assert!(p.jobs[0].verdict_text.contains("peak"));
}

#[test]
fn plan_dto_untitled_uses_suggested_title_without_duplicating_artist() {
    let mut mt = meta();
    mt.title = "  ".into();
    let p = plan(
        &master(24),
        &mt,
        &ids(&["deezer"]),
        &Default::default(),
        Lang::Es,
    );
    assert_eq!(p.delivery_folder, "Vera Alcón - Luz de Noviembre");
}
