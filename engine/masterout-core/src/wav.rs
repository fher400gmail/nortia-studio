//! Lectura y escritura de WAV.
//!
//! El lector recorre los chunks RIFF (con padding a par), tolera `JUNK` y cualquier
//! otro chunk antes de `fmt `, acepta `WAVE_FORMAT_EXTENSIBLE` y nunca carga el
//! archivo entero como bytes: los datos se leen por bloques de 1 MiB.

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::audio::{AudioBuffer, AudioError};

const TAG_PCM: u16 = 0x0001;
const TAG_FLOAT: u16 = 0x0003;
const TAG_EXTENSIBLE: u16 = 0xFFFE;
const READ_BLOCK_BYTES: usize = 1 << 20;

/// Información de cabecera de un WAV, sin leer las muestras.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WavInfo {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits: u16,
    pub is_float: bool,
    pub frames: u64,
    /// Tag original de `fmt ` (0xFFFE si es EXTENSIBLE).
    pub format_tag: u16,
    /// Ids de los chunks distintos de `fmt ` y `data` (p. ej. "JUNK", "bext").
    pub extra_chunks: Vec<String>,
}

struct Scan {
    info: WavInfo,
    data_offset: u64,
}

fn read_exact_or_eof<R: Read>(r: &mut R, buf: &mut [u8]) -> Result<bool, AudioError> {
    let mut filled = 0;
    while filled < buf.len() {
        let n = r.read(&mut buf[filled..])?;
        if n == 0 {
            return Ok(false);
        }
        filled += n;
    }
    Ok(true)
}

fn le16(b: &[u8]) -> u16 {
    u16::from_le_bytes([b[0], b[1]])
}

fn le32(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

struct Fmt {
    tag: u16,
    real_tag: u16,
    channels: u16,
    sample_rate: u32,
    bits: u16,
}

fn parse_fmt(body: &[u8]) -> Result<Fmt, AudioError> {
    if body.len() < 16 {
        return Err(AudioError::UnsupportedFormat(
            "chunk fmt demasiado corto".into(),
        ));
    }
    let tag = le16(body);
    let mut real_tag = tag;
    if tag == TAG_EXTENSIBLE {
        // cbSize(2) + validBits(2) + mask(4) + GUID(16) tras los 16 bytes base.
        if body.len() < 40 {
            return Err(AudioError::UnsupportedFormat(
                "fmt EXTENSIBLE incompleto".into(),
            ));
        }
        real_tag = le16(&body[24..26]);
    }
    Ok(Fmt {
        tag,
        real_tag,
        channels: le16(&body[2..]),
        sample_rate: le32(&body[4..]),
        bits: le16(&body[14..]),
    })
}

fn scan(r: &mut BufReader<File>, file_len: u64) -> Result<Scan, AudioError> {
    let mut head = [0u8; 12];
    if !read_exact_or_eof(r, &mut head)? {
        return Err(AudioError::NotRiffWave);
    }
    match &head[0..4] {
        b"RF64" | b"BW64" => return Err(AudioError::Rf64Unsupported),
        b"RIFF" => {}
        _ => return Err(AudioError::NotRiffWave),
    }
    if &head[8..12] != b"WAVE" {
        return Err(AudioError::NotRiffWave);
    }

    let mut fmt: Option<Fmt> = None;
    let mut data: Option<(u64, u64)> = None; // (offset, len)
    let mut extra = Vec::new();
    let mut pos: u64 = 12;

    loop {
        r.seek(SeekFrom::Start(pos))?;
        let mut ch = [0u8; 8];
        if !read_exact_or_eof(r, &mut ch)? {
            break;
        }
        let id = [ch[0], ch[1], ch[2], ch[3]];
        let size = u64::from(le32(&ch[4..]));
        let body_start = pos + 8;
        match &id {
            b"fmt " => {
                if fmt.is_none() {
                    let n = usize::try_from(size.min(256)).unwrap_or(256);
                    let mut body = vec![0u8; n];
                    if !read_exact_or_eof(r, &mut body)? {
                        return Err(AudioError::UnsupportedFormat("fmt truncado".into()));
                    }
                    fmt = Some(parse_fmt(&body)?);
                }
            }
            b"data" => {
                if data.is_none() {
                    let remaining = file_len.saturating_sub(body_start);
                    let len = if size == 0 || size == 0xFFFF_FFFF || size > remaining {
                        remaining
                    } else {
                        size
                    };
                    data = Some((body_start, len));
                    if size == 0 || size == 0xFFFF_FFFF {
                        break; // los datos llegan hasta EOF
                    }
                }
            }
            _ => extra.push(String::from_utf8_lossy(&id).into_owned()),
        }
        pos = body_start + size + (size & 1);
        if pos >= file_len {
            break;
        }
    }

    let fmt = fmt.ok_or(AudioError::MissingFmt)?;
    let (data_offset, data_len) = data.ok_or(AudioError::MissingData)?;

    if fmt.channels == 0 || fmt.sample_rate == 0 {
        return Err(AudioError::UnsupportedFormat(
            "canales o frecuencia de muestreo a 0".into(),
        ));
    }
    if fmt.channels > 2 {
        return Err(AudioError::TooManyChannels(fmt.channels));
    }
    let is_float = match (fmt.real_tag, fmt.bits) {
        (TAG_PCM, 16 | 24 | 32) => false,
        (TAG_FLOAT, 32) => true,
        (t, b) => {
            return Err(AudioError::UnsupportedFormat(format!(
                "tag 0x{t:04X} con {b} bits"
            )))
        }
    };
    let block_align = u64::from(fmt.channels) * u64::from(fmt.bits / 8);

    Ok(Scan {
        info: WavInfo {
            sample_rate: fmt.sample_rate,
            channels: fmt.channels,
            bits: fmt.bits,
            is_float,
            frames: data_len / block_align,
            format_tag: fmt.tag,
            extra_chunks: extra,
        },
        data_offset,
    })
}

fn open_and_scan(path: &Path) -> Result<(BufReader<File>, Scan), AudioError> {
    let file = File::open(path)?;
    let len = file.metadata()?.len();
    let mut r = BufReader::new(file);
    let s = scan(&mut r, len)?;
    Ok((r, s))
}

/// Lee solo la cabecera.
pub fn probe(path: &Path) -> Result<WavInfo, AudioError> {
    Ok(open_and_scan(path)?.1.info)
}

/// Lee el WAV completo a `AudioBuffer` (f32, canales separados).
pub fn read(path: &Path) -> Result<AudioBuffer, AudioError> {
    let (mut r, s) = open_and_scan(path)?;
    let info = s.info;
    let nch = usize::from(info.channels);
    let bytes_per_sample = usize::from(info.bits / 8);
    let block = nch * bytes_per_sample;
    let total_frames = usize::try_from(info.frames)
        .map_err(|_| AudioError::UnsupportedFormat("archivo demasiado grande".into()))?;

    let mut channels: Vec<Vec<f32>> = (0..nch).map(|_| Vec::with_capacity(total_frames)).collect();
    r.seek(SeekFrom::Start(s.data_offset))?;

    let frames_per_block = (READ_BLOCK_BYTES / block).max(1);
    let mut buf = vec![0u8; frames_per_block * block];
    let mut remaining = total_frames;
    while remaining > 0 {
        let n = remaining.min(frames_per_block);
        let bytes = &mut buf[..n * block];
        r.read_exact(bytes)?;
        for frame in bytes.chunks_exact(block) {
            for (c, sample) in frame.chunks_exact(bytes_per_sample).enumerate() {
                channels[c].push(decode(sample, info.bits, info.is_float));
            }
        }
        remaining -= n;
    }

    Ok(AudioBuffer {
        sample_rate: info.sample_rate,
        channels,
        source_bits: info.bits,
        source_float: info.is_float,
    })
}

fn decode(b: &[u8], bits: u16, is_float: bool) -> f32 {
    match (bits, is_float) {
        (16, _) => f32::from(i16::from_le_bytes([b[0], b[1]])) / 32768.0,
        (24, _) => {
            let v = i32::from_le_bytes([0, b[0], b[1], b[2]]) >> 8;
            v as f32 / 8_388_608.0
        }
        (_, true) => f32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        _ => i32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f32 / 2_147_483_648.0,
    }
}

// ---------------------------------------------------------------------------
// Escritura
// ---------------------------------------------------------------------------

/// Metadatos opcionales: `bext` (EBU Tech 3285 v1) y `LIST/INFO`.
/// El ISRC no se escribe nunca en `LIST/INFO`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WavMetadata {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub year: Option<u16>,
    pub bext_description: String,
    /// Normalmente "ferboix. studio".
    pub bext_originator: String,
    /// Fecha y hora de origen (año, mes, día, hora, minuto, segundo), sin dependencia de chrono.
    pub origination: Option<(u16, u8, u8, u8, u8, u8)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WavWriteOptions {
    /// 16 o 24.
    pub bits: u16,
    pub metadata: Option<WavMetadata>,
}

/// Tamaño fijo del cuerpo del chunk `bext` v1 sin CodingHistory.
pub const BEXT_SIZE: usize = 602;
const WRITE_BLOCK_FRAMES: usize = 64 * 1024;

/// Texto ASCII relleno con ceros hasta `len` (trunca; lo no ASCII pasa a '?').
fn ascii_field(s: &str, len: usize) -> Vec<u8> {
    let mut v: Vec<u8> = s
        .chars()
        .map(|c| {
            if c.is_ascii() && c != '\0' {
                c as u8
            } else {
                b'?'
            }
        })
        .take(len)
        .collect();
    v.resize(len, 0);
    v
}

fn bext_body(m: &WavMetadata) -> Vec<u8> {
    let mut b = Vec::with_capacity(BEXT_SIZE);
    b.extend(ascii_field(&m.bext_description, 256));
    b.extend(ascii_field(&m.bext_originator, 32));
    b.extend(ascii_field("", 32)); // OriginatorReference
    let (date, time) = match m.origination {
        Some((y, mo, d, h, mi, s)) => (
            format!("{y:04}-{mo:02}-{d:02}"),
            format!("{h:02}:{mi:02}:{s:02}"),
        ),
        None => (String::new(), String::new()),
    };
    b.extend(ascii_field(&date, 10));
    b.extend(ascii_field(&time, 8));
    b.extend_from_slice(&0u64.to_le_bytes()); // TimeReference
    b.extend_from_slice(&1u16.to_le_bytes()); // Version
    b.extend_from_slice(&[0u8; 64]); // UMID
    b.extend_from_slice(&[0u8; 10]); // 5 x i16 de loudness
    b.extend_from_slice(&[0u8; 180]); // Reserved
    debug_assert_eq!(b.len(), BEXT_SIZE);
    b
}

fn info_sub(out: &mut Vec<u8>, id: &[u8; 4], text: &str) {
    let mut body = text.as_bytes().to_vec();
    body.push(0);
    out.extend_from_slice(id);
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    if body.len() % 2 == 1 {
        out.push(0);
    }
}

fn list_info_body(m: &WavMetadata) -> Vec<u8> {
    let mut b = b"INFO".to_vec();
    info_sub(&mut b, b"INAM", &m.title);
    info_sub(&mut b, b"IART", &m.artist);
    if let Some(album) = &m.album {
        info_sub(&mut b, b"IPRD", album);
    }
    if let Some(y) = m.year {
        info_sub(&mut b, b"ICRD", &y.to_string());
    }
    info_sub(&mut b, b"ISFT", "master check");
    b
}

fn put_chunk<W: Write>(w: &mut W, id: &[u8; 4], body: &[u8]) -> std::io::Result<()> {
    w.write_all(id)?;
    w.write_all(&(body.len() as u32).to_le_bytes())?;
    w.write_all(body)?;
    if body.len() % 2 == 1 {
        w.write_all(&[0])?;
    }
    Ok(())
}

/// Escribe un WAV PCM (tag 0x0001, `fmt ` de 16 bytes) con las muestras ya cuantizadas.
/// Orden de chunks: `fmt `, `bext`?, `LIST`?, `data`.
pub fn write(
    path: &Path,
    samples_i32: &[Vec<i32>],
    sample_rate: u32,
    opts: &WavWriteOptions,
) -> Result<(), AudioError> {
    if opts.bits != 16 && opts.bits != 24 {
        return Err(AudioError::UnsupportedFormat(format!(
            "escritura a {} bits",
            opts.bits
        )));
    }
    let nch = samples_i32.len();
    if nch == 0 || sample_rate == 0 {
        return Err(AudioError::UnsupportedFormat(
            "sin canales o frecuencia 0".into(),
        ));
    }
    if nch > 2 {
        return Err(AudioError::TooManyChannels(
            nch.try_into().unwrap_or(u16::MAX),
        ));
    }
    let frames = samples_i32[0].len();
    if samples_i32.iter().any(|c| c.len() != frames) {
        return Err(AudioError::UnsupportedFormat(
            "canales de distinta longitud".into(),
        ));
    }

    let bps = usize::from(opts.bits / 8);
    let block_align = nch * bps;
    let data_len = frames
        .checked_mul(block_align)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| AudioError::UnsupportedFormat("más de 4 GiB de datos".into()))?;

    let bext = opts.metadata.as_ref().map(bext_body);
    let list = opts.metadata.as_ref().map(list_info_body);
    let chunk_size = |body: usize| 8 + body + (body & 1);
    let riff_size = 4
        + chunk_size(16)
        + bext.as_ref().map_or(0, |b| chunk_size(b.len()))
        + list.as_ref().map_or(0, |b| chunk_size(b.len()))
        + chunk_size(data_len as usize);
    let riff_size = u32::try_from(riff_size)
        .map_err(|_| AudioError::UnsupportedFormat("más de 4 GiB de datos".into()))?;

    let mut w = BufWriter::new(File::create(path)?);
    w.write_all(b"RIFF")?;
    w.write_all(&riff_size.to_le_bytes())?;
    w.write_all(b"WAVE")?;

    let mut fmt = Vec::with_capacity(16);
    fmt.extend_from_slice(&TAG_PCM.to_le_bytes());
    fmt.extend_from_slice(&(nch as u16).to_le_bytes());
    fmt.extend_from_slice(&sample_rate.to_le_bytes());
    fmt.extend_from_slice(&(sample_rate * block_align as u32).to_le_bytes());
    fmt.extend_from_slice(&(block_align as u16).to_le_bytes());
    fmt.extend_from_slice(&opts.bits.to_le_bytes());
    put_chunk(&mut w, b"fmt ", &fmt)?;
    if let Some(b) = &bext {
        put_chunk(&mut w, b"bext", b)?;
    }
    if let Some(l) = &list {
        put_chunk(&mut w, b"LIST", l)?;
    }

    w.write_all(b"data")?;
    w.write_all(&data_len.to_le_bytes())?;
    let max = (1i64 << (opts.bits - 1)) - 1;
    let min = -(1i64 << (opts.bits - 1));
    let mut buf = Vec::with_capacity(WRITE_BLOCK_FRAMES * block_align);
    for start in (0..frames).step_by(WRITE_BLOCK_FRAMES) {
        let end = (start + WRITE_BLOCK_FRAMES).min(frames);
        buf.clear();
        for i in start..end {
            for c in samples_i32 {
                let v = i64::from(c[i]).clamp(min, max) as i32;
                buf.extend_from_slice(&v.to_le_bytes()[..bps]);
            }
        }
        w.write_all(&buf)?;
    }
    if data_len % 2 == 1 {
        w.write_all(&[0])?;
    }
    w.flush()?;
    Ok(())
}
