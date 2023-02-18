use audio::{BezierSpline, source::{spline_peaks, spline_gram}};
use gram::Gram;

#[test]
fn spline_line() {
    let spline = BezierSpline::new(&[
        (0., 0.),
        (0.5, 0.5),
        (0.5, 0.5),
        (1., 1.),
    ]);

    assert_feq(spline.eval(0.0).0, 0.0, 1e-3);
    assert_feq(spline.eval(0.0).1, 0.0, 1e-3);

    assert_feq(spline.eval(1.0).0, 1.0, 1e-3);
    assert_feq(spline.eval(1.0).1, 1.0, 1e-3);

    assert_feq(spline.eval(0.25).0, 0.30, 1e-2);
    assert_feq(spline.eval(0.25).1, 0.30, 1e-2);

    assert_feq(spline.eval(0.5).0, 0.5, 1e-2);
    assert_feq(spline.eval(0.5).1, 0.5, 1e-2);

    assert_feq(spline.eval(0.75).0, 0.70, 1e-2);
    assert_feq(spline.eval(0.75).1, 0.70, 1e-2);
}

#[test]
fn spline_line_2() {
    let spline = BezierSpline::new(&[
        (0., 0.),
        (0., 0.),
        (1., 1.),
        (1., 1.),
    ]);

    assert_feq(spline.eval(0.0).0, 0.0, 1e-3);
    assert_feq(spline.eval(0.0).1, 0.0, 1e-3);

    assert_feq(spline.eval(1.0).0, 1.0, 1e-3);
    assert_feq(spline.eval(1.0).1, 1.0, 1e-3);

    assert_feq(spline.eval(0.25).0, 0.16, 1e-2);
    assert_feq(spline.eval(0.25).1, 0.16, 1e-2);

    assert_feq(spline.eval(0.5).0, 0.5, 1e-2);
    assert_feq(spline.eval(0.5).1, 0.5, 1e-2);

    assert_feq(spline.eval(0.75).0, 0.84, 1e-2);
    assert_feq(spline.eval(0.75).1, 0.84, 1e-2);
}

#[test]
fn spline_line_mid() {
    let spline = BezierSpline::new(&[
        (0., 0.),
        (0.5, 0.5),
        (0.5, 0.5),
        (1., 1.),
    ]);

    assert_feq(spline.eval(0.0).0, 0.0, 1e-3);
    assert_feq(spline.eval(0.0).1, 0.0, 1e-3);

    assert_feq(spline.eval(1.0).0, 1.0, 1e-3);
    assert_feq(spline.eval(1.0).1, 1.0, 1e-3);

    assert_feq(spline.eval(0.25).0, 0.30, 1e-2);
    assert_feq(spline.eval(0.25).1, 0.30, 1e-2);

    assert_feq(spline.eval(0.5).0, 0.5, 1e-2);
    assert_feq(spline.eval(0.5).1, 0.5, 1e-2);

    assert_feq(spline.eval(0.75).0, 0.70, 1e-2);
    assert_feq(spline.eval(0.75).1, 0.70, 1e-2);
}

#[test]
fn spline_line_10() {
    let spline = BezierSpline::new(&[
        (10., 0.),
        (5., -5.),
        (5., -5.),
        (0., -10.),
    ]);

    assert_feq(spline.eval(0.0).0, 10.0, 1e-3);
    assert_feq(spline.eval(0.0).1, 0.0, 1e-3);

    assert_feq(spline.eval(1.0).0, 0.0, 1e-3);
    assert_feq(spline.eval(1.0).1, -10.0, 1e-3);

    assert_feq(spline.eval(0.25).0, 7.03, 1e-2);
    assert_feq(spline.eval(0.25).1, -2.97, 1e-2);

    assert_feq(spline.eval(0.5).0, 5., 1e-2);
    assert_feq(spline.eval(0.5).1, -5., 1e-2);

    assert_feq(spline.eval(0.75).0, 2.97, 1e-2);
    assert_feq(spline.eval(0.75).1, -7.03, 1e-2);
}

#[test]
fn spline_curve_4() {
    let spline = BezierSpline::new(&[
        (0., 1.),
        (0.5, 1.),
        (0.5, 0.),
        (1., 0.),
    ]);

    assert_feq(spline.eval(0.0).0, 0.0, 1e-3);
    assert_feq(spline.eval(0.0).1, 1.0, 1e-3);

    assert_feq(spline.eval(1.0).0, 1.0, 1e-3);
    assert_feq(spline.eval(1.0).1, 0.0, 1e-3);

    assert_feq(spline.eval(0.25).0, 0.30, 1e-2);
    assert_feq(spline.eval(0.25).1, 0.84, 1e-2);

    assert_feq(spline.eval(0.5).0, 0.5, 1e-2);
    assert_feq(spline.eval(0.5).1, 0.5, 1e-2);

    assert_feq(spline.eval(0.75).0, 0.84, 1e-2);
    assert_feq(spline.eval(0.75).1, 0.30, 1e-2);
}

#[test]
fn spline_fill_line() {
    let spline = BezierSpline::new(&[
        (0., 0.),
        (0.5, 0.5),
        (0.5, 0.5),
        (1., 1.),
    ]);

    let mut data = [0.0f32; 4];

    spline.eval_as_fn(&mut data);
    assert_array(&data, &[
        0.0,
        0.30,
        0.5,
        0.83,
    ], 1e-2);
}

#[test]
fn spline_fill_line_2() {
    let spline = BezierSpline::new(&[
        (0., 0.),
        (0., 0.),
        (1., 1.),
        (1., 1.),
    ]);

    let mut data = [0.0f32; 4];

    spline.eval_as_fn(&mut data);
    assert_array(&data, &[
        0.0,
        0.32,
        0.5,
        0.84,
    ], 1e-2);
}

#[test]
fn spline_fill_square() {
    let spline = BezierSpline::new(&[
        (0., 0.),
        (1., 0.),
        (0., 1.),
        (1., 1.),
    ]);

    let mut data = [0.0f32; 8];

    spline.eval_as_fn(&mut data);
    assert_array(&data, &[
        0.0,
        0.01,
        0.04,
        0.09,
        0.5,
        0.96,
        0.99,
        0.99,
    ], 1e-2);
}

//
// # spline source
//

#[test]
fn spline_source() {
    let sample = 8;

    let mut source = spline_peaks(1., &[
        (0.0, 1.0),
        (0.5, -1.0),
    ]);

    source.reset(Some(sample));

    let value: Vec<f32> = source.take(sample as usize).collect();

    assert_match(&value, &[
        1.0, 
        0.6875,
        0.0,
        -0.91,
        -1.,
        -0.6875,
        0.,
        0.91,
    ], 1e-2);
}

//
// # spline gram
//

#[test]
fn spline_gram_test() {
    let sample = 8;

    let mut source = spline_gram(1., Gram::from("7707_7707"), 16);

    source.reset(Some(sample));

    let value: Vec<f32> = source.take(sample as usize).collect();

    assert_match(&value, &[
        0.875, 
        0.60,
        0.0,
        -0.8,
        -0.875,
        -0.60,
        0.,
        0.8,
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
fn assert_feq(lhs: f32, rhs: f32, d: f32) {
    assert!((lhs - rhs).abs() < d, "{} == {}", lhs, rhs);
}
fn assert_array(lhs: &[f32], rhs: &[f32], d: f32) {
    assert_eq!(lhs.len(), rhs.len());

    for i in 0..lhs.len() {
        assert!((lhs[i] - rhs[i]).abs() < d, "{}: {} == {}", i, lhs[i], rhs[i]);
    }
}