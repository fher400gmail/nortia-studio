//! Remuestreo de alta calidad con `rubato` (sinc, sin cambio de nivel).

use crate::audio::{AudioBuffer, AudioError};
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

const CHUNK: usize = 1024;

fn map_err(e: impl std::fmt::Display) -> AudioError {
    AudioError::UnsupportedFormat(format!("rubato: {e}"))
}

/// Remuestrea `buf` a `target_rate`. Si coincide con la tasa de origen devuelve un clon.
///
/// Usa `SincFixedIn` (sinc 256, corte 0,95, ventana Blackman-Harris 2) procesando en bloques,
/// descarta la latencia del filtro y ajusta la longitud a `round(frames * target / source)`.
/// No aplica ganancia: el nivel se conserva.
pub fn resample(buf: &AudioBuffer, target_rate: u32) -> Result<AudioBuffer, AudioError> {
    resample_cancellable(buf, target_rate, &|| false)
}

/// Como `resample`, comprobando `should_cancel` en cada bloque; si devuelve `true`
/// termina con `AudioError::Cancelled`.
pub fn resample_cancellable(
    buf: &AudioBuffer,
    target_rate: u32,
    should_cancel: &dyn Fn() -> bool,
) -> Result<AudioBuffer, AudioError> {
    if target_rate == buf.sample_rate {
        return Ok(buf.clone());
    }
    if target_rate == 0 || buf.sample_rate == 0 || buf.num_channels() == 0 {
        return Err(AudioError::UnsupportedFormat(
            "tasa de muestreo o canales no válidos".into(),
        ));
    }
    let ratio = f64::from(target_rate) / f64::from(buf.sample_rate);
    let frames = buf.frames();
    let expected = (frames as f64 * ratio).round() as usize;
    let nch = buf.num_channels();

    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        oversampling_factor: 256,
        interpolation: SincInterpolationType::Cubic,
        window: WindowFunction::BlackmanHarris2,
    };
    let mut rs = SincFixedIn::<f32>::new(ratio, 1.0, params, CHUNK, nch).map_err(map_err)?;
    let delay = rs.output_delay();
    let want = delay + expected;

    let mut out: Vec<Vec<f32>> = vec![Vec::with_capacity(want + CHUNK * 2); nch];
    let mut pos = 0;
    while frames - pos >= rs.input_frames_next() {
        if should_cancel() {
            return Err(AudioError::Cancelled);
        }
        let n = rs.input_frames_next();
        let chunk: Vec<&[f32]> = buf.channels.iter().map(|c| &c[pos..pos + n]).collect();
        let res = rs.process(&chunk, None).map_err(map_err)?;
        for (o, r) in out.iter_mut().zip(&res) {
            o.extend_from_slice(r);
        }
        pos += n;
    }
    // Resto (bloque parcial relleno con ceros) y vaciado del filtro hasta tener lo necesario.
    let mut tail: Option<Vec<&[f32]>> =
        (pos < frames).then(|| buf.channels.iter().map(|c| &c[pos..]).collect());
    while out[0].len() < want {
        if should_cancel() {
            return Err(AudioError::Cancelled);
        }
        let res = rs.process_partial(tail.as_deref(), None).map_err(map_err)?;
        tail = None;
        for (o, r) in out.iter_mut().zip(&res) {
            o.extend_from_slice(r);
        }
    }

    let channels = out
        .into_iter()
        .map(|mut c| {
            c.drain(..delay);
            c.truncate(expected);
            c
        })
        .collect();
    Ok(AudioBuffer {
        sample_rate: target_rate,
        channels,
        source_bits: buf.source_bits,
        source_float: buf.source_float,
    })
}
