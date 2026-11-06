use masterout_core::wav::{self, WavMetadata, WavWriteOptions};

fn opts(bits: u16, metadata: Option<WavMetadata>) -> WavWriteOptions {
    WavWriteOptions { bits, metadata }
}

fn meta() -> WavMetadata {
    WavMetadata {
        title: "Canción de prueba".into(),
        artist: "Artista".into(),
        album: Some("Álbum".into()),
        year: Some(2026),
        bext_description: "Master Out test".into(),
        bext_originator: "ferboix. studio".into(),
        origination: Some((2026, 11, 5, 10, 30, 15)),
    }
}

/// Recorre los chunks de un WAV: (id, offset del cuerpo, tamaño).
fn chunks(bytes: &[u8]) -> Vec<(String, usize, usize)> {
    let mut out = Vec::new();
    let mut pos = 12;
    while pos + 8 <= bytes.len() {
        let id = String::from_utf8_lossy(&bytes[pos..pos + 4]).into_owned();
        let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap()) as usize;
        out.push((id, pos + 8, size));
        pos += 8 + size + (size & 1);
    }
    out
}

fn tmp() -> (tempfile::TempDir, std::path::PathBuf) {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("o.wav");
    (d, p)
}

#[test]
fn writes_pcm24_header_0x0001_fmt16() {
    let (_d, p) = tmp();
    wav::write(&p, &[vec![1, 2, 3], vec![4, 5, 6]], 48000, &opts(24, None)).unwrap();
    let b = std::fs::read(&p).unwrap();
    assert_eq!(&b[0..4], b"RIFF");
    assert_eq!(&b[8..16], b"WAVEfmt ");
    assert_eq!(u32::from_le_bytes(b[16..20].try_into().unwrap()), 16);
    assert_eq!(u16::from_le_bytes(b[20..22].try_into().unwrap()), 0x0001);
    assert_eq!(u16::from_le_bytes(b[22..24].try_into().unwrap()), 2);
    assert_eq!(u32::from_le_bytes(b[24..28].try_into().unwrap()), 48000);
    assert_eq!(u32::from_le_bytes(b[28..32].try_into().unwrap()), 48000 * 6);
    assert_eq!(u16::from_le_bytes(b[32..34].try_into().unwrap()), 6);
    assert_eq!(u16::from_le_bytes(b[34..36].try_into().unwrap()), 24);
}

#[test]
fn roundtrip_pcm16_and_pcm24() {
    let (_d, p) = tmp();
    let l: Vec<i32> = vec![0, 1, -1, 32767, -32768, 1234];
    let r: Vec<i32> = vec![5, -5, 100, -100, 0, 7];
    wav::write(&p, &[l.clone(), r.clone()], 44100, &opts(16, None)).unwrap();
    let a = wav::read(&p).unwrap();
    assert_eq!((a.source_bits, a.sample_rate), (16, 44100));
    for (c, ints) in [&l, &r].iter().enumerate() {
        let back: Vec<i32> = a.channels[c].iter().map(|v| (v * 32768.0) as i32).collect();
        assert_eq!(&back, *ints);
    }

    let l: Vec<i32> = vec![0, 1, -1, 8_388_607, -8_388_608, 123_456];
    let r: Vec<i32> = vec![-3, 3, 99_999, -99_999, 0, 42];
    wav::write(&p, &[l.clone(), r.clone()], 96000, &opts(24, None)).unwrap();
    let a = wav::read(&p).unwrap();
    assert_eq!((a.source_bits, a.sample_rate), (24, 96000));
    for (c, ints) in [&l, &r].iter().enumerate() {
        let back: Vec<i32> = a.channels[c]
            .iter()
            .map(|v| (v * 8_388_608.0) as i32)
            .collect();
        assert_eq!(&back, *ints);
    }
}

#[test]
fn data_is_last_chunk_and_sizes_consistent() {
    for m in [None, Some(meta())] {
        let (_d, p) = tmp();
        let ch: Vec<i32> = (0..1001).collect();
        wav::write(&p, &[ch.clone(), ch], 44100, &opts(24, m)).unwrap();
        let b = std::fs::read(&p).unwrap();
        let riff = u32::from_le_bytes(b[4..8].try_into().unwrap()) as usize;
        assert_eq!(riff, b.len() - 8);
        let cs = chunks(&b);
        let last = cs.last().unwrap();
        assert_eq!(last.0, "data");
        assert_eq!(last.2, 1001 * 6);
        assert_eq!(last.1 + last.2 + (last.2 & 1), b.len());
    }
}

#[test]
fn writes_bext_and_list_info_when_metadata() {
    let (_d, p) = tmp();
    wav::write(
        &p,
        &[vec![0; 10], vec![0; 10]],
        44100,
        &opts(16, Some(meta())),
    )
    .unwrap();
    let b = std::fs::read(&p).unwrap();
    let cs = chunks(&b);
    let ids: Vec<&str> = cs.iter().map(|c| c.0.as_str()).collect();
    assert_eq!(ids, ["fmt ", "bext", "LIST", "data"]);

    let (_, off, size) = cs[1].clone();
    assert_eq!(size, 602);
    let bext = &b[off..off + size];
    assert!(bext[..256].starts_with(b"Master Out test\0"));
    assert!(bext[15..256].iter().all(|&x| x == 0));
    assert!(bext[256..288].starts_with(b"ferboix. studio\0"));
    assert_eq!(&bext[320..330], b"2026-11-05");
    assert_eq!(&bext[330..338], b"10:30:15");
    assert_eq!(u16::from_le_bytes(bext[346..348].try_into().unwrap()), 1); // Version
    assert!(bext[348..].iter().all(|&x| x == 0));

    let (_, off, size) = cs[2].clone();
    let list = &b[off..off + size];
    assert_eq!(&list[0..4], b"INFO");
    let mut subs = Vec::new();
    let mut pos = 4;
    while pos + 8 <= list.len() {
        let id = String::from_utf8_lossy(&list[pos..pos + 4]).into_owned();
        let n = u32::from_le_bytes(list[pos + 4..pos + 8].try_into().unwrap()) as usize;
        let text = String::from_utf8_lossy(&list[pos + 8..pos + 8 + n])
            .trim_end_matches('\0')
            .to_string();
        subs.push((id, text));
        pos += 8 + n + (n & 1);
    }
    let get = |k: &str| subs.iter().find(|s| s.0 == k).map(|s| s.1.clone());
    assert_eq!(get("INAM").as_deref(), Some("Canción de prueba"));
    assert_eq!(get("IART").as_deref(), Some("Artista"));
    assert_eq!(get("ICRD").as_deref(), Some("2026"));
    assert_eq!(get("ISFT").as_deref(), Some("master check"));
    assert!(get("ISRC").is_none());
    assert!(subs.iter().all(|s| s.0 != "ISRC"));

    // El lector sigue funcionando y ve los chunks extra.
    let info = wav::probe(&p).unwrap();
    assert_eq!(info.extra_chunks, ["bext", "LIST"]);
}

#[test]
fn no_metadata_means_only_fmt_and_data() {
    let (_d, p) = tmp();
    wav::write(&p, &[vec![1, 2]], 44100, &opts(16, None)).unwrap();
    let b = std::fs::read(&p).unwrap();
    let ids: Vec<String> = chunks(&b).into_iter().map(|c| c.0).collect();
    assert_eq!(ids, ["fmt ", "data"]);
}

#[test]
fn rejects_unsupported_bits_and_channel_mismatch() {
    let (_d, p) = tmp();
    assert!(wav::write(&p, &[vec![0]], 44100, &opts(32, None)).is_err());
    assert!(wav::write(&p, &[vec![0, 1], vec![0]], 44100, &opts(16, None)).is_err());
}
