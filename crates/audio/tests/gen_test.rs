use audio::gen::*;

#[test]
fn sine_4() {
    let sample = 4;

    let mut source = sine(1.0).resample(sample);

    let value: Vec<f32> = (0..sample).map(|_| { source.next() }).collect();

    assert_match(&value, &[
        0.0, 
        1.0, 
        0.0, 
        -1.0,
    ], 1e-2);
}

#[test]
fn sine_4_2hz() {
    let sample = 8;
    let freq = 2.0;

    let mut source = sine(freq).resample(sample);

    let value: Vec<f32> = (0..sample).map(|_| { source.next() }).collect();

    assert_match(&value, &[
        0.0, 
        1.0, 
        0.0, 
        -1.0,
        0.0, 
        1.0, 
        0.0, 
        -1.0,
    ], 1e-2);
}

#[test]
fn sine_4_3hz() {
    let sample = 12;
    let freq = 3.0;

    let mut source = sine(freq).resample(sample);

    let value: Vec<f32> = (0..sample).map(|_| { source.next() }).collect();

    assert_match(&value, &[
        0.0, 
        1.0, 
        0.0, 
        -1.0,
        0.0, 
        1.0, 
        0.0, 
        -1.0,
        0.0, 
        1.0, 
        0.0, 
        -1.0,
    ], 2e-2);
}

#[test]
fn sine_8() {
    let sample = 8;
    let mut source = sine(1.0).resample(sample);

    let value: Vec<f32> = (0..sample).map(|_| { source.next() }).collect();

    assert_match(&value, &[
        0.0, 
        0.707, 
        1.0, 
        0.707, 
        0.0, 
        -0.707, 
        -1.0
        -0.707, 
    ], 1e-2);
}

#[test]
fn sine_12() {
    let sample = 12;

    let mut source = sine(1.0).resample(sample);

    let value: Vec<f32> = (0..sample).map(|_| { source.next() }).collect();

    assert_match(&value, &[
        0.0, 
        0.5, 
        0.866, 
        1.0, 
        0.866, 
        0.5, 
        0.0,
        -0.5, 
        -0.866,
        -1.0, 
        -0.866,
        -0.5, 
    ], 1e-2);
}

#[test]
fn amplitude_x2() {
    let sample = 4;

    let source = 2.0 * sine(1.0);

    let mut source = source.resample(sample);

    let value: Vec<f32> = (0..sample).map(|_| { source.next() }).collect();

    assert_match(&value, &[
        0.0, 
        2.0, 
        0.0, 
        -2.0, 
    ], 1e-2);
}

#[test]
fn amplitude_x0_5() {
    let sample = 4;

    let source = 0.5 * sine(1.0);

    let mut source = source.resample(sample);

    let value: Vec<f32> = (0..sample).map(|_| { source.next() }).collect();

    assert_match(&value, &[
        0.0, 
        0.5, 
        0.0, 
        -0.5, 
    ], 1e-2);
}

#[test]
fn add_sine_2() {
    let sample = 8;

    let source = sine(1.0) + sine(2.0);

    let mut source = source.resample(sample);

    let value: Vec<f32> = (0..sample).map(|_| { source.next() }).collect();

    assert_match(&value, &[
        0.0, 
        1.0 + 0.707, 
        0.0 + 1.0, 
        0.707 - 1.0, 
        0.0, 
        1.0 - 0.707, 
        0.0 - 1.0, 
        -1.0 - 0.707, 
    ], 1e-2);
}

fn assert_match(lhs: &[f32], rhs: &[f32], delta: f32) {
    assert_eq!(lhs.len(), rhs.len());

    for i in 0..lhs.len() {
        assert!(
            (lhs[i] - rhs[i]).abs() <= delta, 
            "{}: {} != {}", i, lhs[i], rhs[i]
        );
    }
}