use std::{f32::consts::PI};

use wavelet::{FftWindow};

pub fn main() {
    let mut input: Vec<f32> = Vec::new();
    let len = 64;

    for i in 0..len {
        let x = i as f32 / len as f32;

        let mut v = 0.0;

        v += (32.0 * 2.0 * PI * x).sin();
        //v += (8.0 * 2.0 * PI * x).cos();

        input.push(v);
    }

    let fft = FftWindow::new(len);
    
    let mut out: Vec<f32> = Vec::new();
    out.resize(input.len(), 0.0);

    fft.process(&input, &mut out);

    print!("ffi: {:?}\n", input);
    print!("ffi_out: {:?}\n", out);
    for (i, x) in out.iter().enumerate() {
        print!("{} {}\n", i, x);
    }
}
