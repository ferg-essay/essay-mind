use audio::{sine, Harmonic, analyze::power_msq};

//
// # power
//

#[test]
fn power_sine_256() {
    let sample = 8192;
    let mut source = sine(256.0);
    source.reset(Some(sample));
    let vec : Vec::<f32> = source.take(sample as usize).collect();

    let power_msq_wave = power_msq(&vec);

    let harmonics = Harmonic::harmonics_wave(&vec, sample);

    assert_feq(harmonics.power_rms, power_msq_wave * 2.0, 1e-2);
    assert_feq(harmonics.power_rms, 1.0, 1e-2);
}

#[test]
fn power_sine_512() {
    let sample = 8192;
    let mut source = sine(512.0);
    source.reset(Some(sample));
    let vec : Vec::<f32> = source.take(sample as usize).collect();

    let power_msq_wave = power_msq(&vec);

    let harmonics = Harmonic::harmonics_wave(&vec, sample);

    assert_feq(harmonics.power_rms, power_msq_wave * 2.0, 1e-2);
    assert_feq(harmonics.power_rms, 1.0, 1e-2);
}

#[test]
fn power_sine_2x_256() {
    let sample = 8192;
    let mut source = 2.0 * sine(256.0);
    source.reset(Some(sample));
    let vec : Vec::<f32> = source.take(sample as usize).collect();

    let power_msq_wave = power_msq(&vec);

    let harmonics = Harmonic::harmonics_wave(&vec, sample);

    assert_feq(harmonics.power_rms, power_msq_wave * 2.0, 1e-2);
    assert_feq(harmonics.power_rms, 2.0, 1e-2);
}

#[test]
fn power_sine_223() {
    let sample = 8192;
    let mut source = sine(223.0);
    source.reset(Some(sample));
    let vec : Vec::<f32> = source.take(sample as usize).collect();

    let power_msq_wave = power_msq(&vec);

    let harmonics = Harmonic::harmonics_wave(&vec, sample);

    assert_feq(harmonics.power_rms, power_msq_wave * 2.0, 1e-2);
    assert_feq(harmonics.power_rms, 1.0, 1e-2);
}

//
// # fundamental(base) frequency
//


#[test]
fn base_sine_256() {
    let sample = 8192;
    let mut source = sine(256.0);
    source.reset(Some(sample));
    let vec : Vec::<f32> = source.take(sample as usize).collect();

    let harmonics = Harmonic::harmonics_wave(&vec, sample);

    assert_eq!(harmonics.freqs.len(), 1);
    assert_feq(harmonics.freqs[0].freq, 256.0, 1e-3);
    assert_feq(harmonics.freqs[0].power, 1.0, 1e-2);
    assert_eq!(harmonics.freqs[0].harmonics.len(), 1);
    assert_feq(harmonics.freqs[0].harmonics[0], 1.0, 1e-2);
}

#[test]
fn base_sine_256_400() {
    let sample = 8192;
    let mut source = sine(256.0) + 0.5 * sine(400.);
    source.reset(Some(sample));
    let vec : Vec::<f32> = source.take(sample as usize).collect();

    let harmonics = Harmonic::harmonics_wave(&vec, sample);

    assert_eq!(harmonics.freqs.len(), 2);
    assert_feq(harmonics.freqs[0].freq, 256.0, 1e-3);
    assert_feq(harmonics.freqs[0].power, 0.67, 1e-2);
    assert_eq!(harmonics.freqs[0].harmonics.len(), 1);
    assert_feq(harmonics.freqs[0].harmonics[0], 0.67, 1e-2);

    assert_feq(harmonics.freqs[1].freq, 400.0, 1e-3);
    assert_feq(harmonics.freqs[1].power, 0.33, 1e-2);
    assert_eq!(harmonics.freqs[1].harmonics.len(), 1);
    assert_feq(harmonics.freqs[1].harmonics[0], 0.33, 1e-2);
}

#[test]
fn base_sine_256_512() {
    let sample = 8192;
    let mut source =
        sine(256.0)
        + 0.5 * sine(512.0);

    source.reset(Some(sample));
    let vec : Vec::<f32> = source.take(sample as usize).collect();

    let harmonics = Harmonic::harmonics_wave(&vec, sample);

    assert_eq!(harmonics.freqs.len(), 1);
    assert_feq(harmonics.freqs[0].freq, 256.0, 1e-3);
    assert_feq(harmonics.freqs[0].power, 1.0, 1e-2);
    assert_eq!(harmonics.freqs[0].harmonics.len(), 2);
    assert_feq(harmonics.freqs[0].harmonics[0], 0.67, 1e-2);
    assert_feq(harmonics.freqs[0].harmonics[1], 0.33, 1e-2);
}

fn assert_feq(lhs: f32, rhs: f32, delta: f32) {
    assert!(
        (lhs - rhs).abs() <= delta, 
        "{} != {}", lhs, rhs
    );
}