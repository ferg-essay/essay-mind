use audio::gen::{sine, AudioComponent};
use audio::analyze::power_msq;

#[test]
fn power_sq_sine_test() {
    let sample = 14410;
    let mut source = sine(1.0);

    let vec : Vec<f32> = (0..sample).map(|_| source.next()).collect();

    assert_feq(power_msq(&vec), 0.5, 1e-2);

    assert_feq(2.0 / power_msq(&vec).sqrt(), 2.828, 1e-2);
}

#[test]
fn power_sq_half_sine_test() {
    let sample = 14410;
    let mut source = sine(1.0);

    let vec : Vec<f32> = (0..sample).map(|_| source.next()).collect();

    assert_feq(power_msq(&vec[0..sample / 2]), 0.5, 1e-2);
    assert_feq(power_msq(&vec[0..sample / 4]), 0.5, 1e-2);
    assert_feq(power_msq(&vec[0..sample / 8]), 0.1815, 1e-2);
}

fn assert_feq(lhs: f32, rhs: f32, delta: f32) {
    assert!(
        (lhs - rhs).abs() <= delta, 
        "{} != {}", lhs, rhs
    );
}