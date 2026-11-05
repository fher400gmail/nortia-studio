use masterout_core::dither::{quantize, quantize_with, Dither};

#[test]
fn same_bits_no_dither_is_exact() {
    let vals: Vec<f32> = [0, 1, -1, 8_388_607, -8_388_608, 12345]
        .iter()
        .map(|&k| k as f32 / 8_388_608.0)
        .collect();
    let (q, d) = quantize(&[vals.clone(), vals], 24, 24, false);
    assert_eq!(d, Dither::None);
    assert_eq!(q[0], vec![0, 1, -1, 8_388_607, -8_388_608, 12345]);
}

#[test]
fn downgrade_24_to_16_uses_tpdf() {
    let (_, d) = quantize(&[vec![0.1, 0.2]], 16, 24, false);
    assert!(matches!(d, Dither::Tpdf { .. }));
    // float de origen también dithea al bajar a cualquier profundidad entera
    let (_, d) = quantize(&[vec![0.1, 0.2]], 24, 32, true);
    assert!(matches!(d, Dither::Tpdf { .. }));
}

#[test]
fn upgrade_16_to_24_no_dither() {
    let vals: Vec<f32> = [0, 1, -1, 32767, -32768, 777]
        .iter()
        .map(|&k| k as f32 / 32768.0)
        .collect();
    let (q, d) = quantize(&[vals], 24, 16, false);
    assert_eq!(d, Dither::None);
    assert_eq!(
        q[0],
        vec![0, 256, -256, 32767 * 256, -32768 * 256, 777 * 256]
    );
}

#[test]
fn tpdf_noise_statistics() {
    // Error = salida - ideal, en LSB de 16 bit. Con TPDF de +-1 LSB (suma de dos uniformes
    // en [-0,5, 0,5), varianza 2 * 1/12 = 1/6) y redondeo al entero más cercano (error
    // uniforme, varianza 1/12, no correlacionado con el dither gracias a la TPDF), la
    // varianza total es 1/6 + 1/12 = 1/4, es decir RMS teórico = 0,50 LSB y media 0.
    // Con 200k muestras el error estándar de la RMS es ~0,001, así que [0,45, 0,55] es
    // una ventana muy holgada que aun así descarta RPDF (0,41) o ausencia de dither (0).
    // 0,25 FS es 8192 LSB exactos; 0,2501 cae entre niveles para ejercitar el redondeo.
    for level in [0.25f32, 0.2501] {
        let n = 200_000;
        let ideal = f64::from(level) * 32768.0;
        let q = quantize_with(&[vec![level; n]], 16, Dither::Tpdf { seed: 1 });
        let errs: Vec<f64> = q[0].iter().map(|&v| f64::from(v) - ideal).collect();
        let mean = errs.iter().sum::<f64>() / n as f64;
        let rms = (errs.iter().map(|e| e * e).sum::<f64>() / n as f64).sqrt();
        assert!(mean.abs() <= 0.02, "media {mean} (nivel {level})");
        assert!((0.45..=0.55).contains(&rms), "rms {rms} (nivel {level})");
    }
}

#[test]
fn clamps_overs() {
    let q = quantize_with(&[vec![1.5, -1.5, 1.0, -1.0]], 16, Dither::None);
    assert_eq!(q[0], vec![32767, -32768, 32767, -32768]);
    // También con dither activo
    let q = quantize_with(&[vec![1.5, -1.5]], 16, Dither::Tpdf { seed: 3 });
    assert_eq!(q[0], vec![32767, -32768]);
}

#[test]
fn deterministic_with_seed() {
    let sig: Vec<f32> = (0..1000).map(|i| (i as f32 * 0.01).sin() * 0.3).collect();
    let a = quantize_with(&[sig.clone(), sig.clone()], 16, Dither::Tpdf { seed: 7 });
    let b = quantize_with(&[sig.clone(), sig.clone()], 16, Dither::Tpdf { seed: 7 });
    assert_eq!(a, b);
    let c = quantize_with(&[sig.clone(), sig], 16, Dither::Tpdf { seed: 8 });
    assert_ne!(a, c);
    // canales independientes: misma señal, ruido distinto
    assert_ne!(a[0], a[1]);
}
