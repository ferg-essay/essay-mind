use std::{f32::consts::PI, sync::Arc};

use rustfft::{num_complex::Complex, FftPlanner, Fft};

pub struct FftWindow {
    len: usize,
    
    fft: Arc<dyn Fft<f32>>,
    window: Vec<f32>,
}

impl FftWindow {
    pub fn new(len: usize) -> FftWindow {
        assert!(len.count_ones() == 1, "len must be a power of 2");

        let mut planner = FftPlanner::<f32>::new();
        let fft_fwd = planner.plan_fft_forward(len);

        Self {
            len: len,
            fft: fft_fwd,
            window: hann_window(len),
        }
    }

    pub fn process(&self, input: &Vec<f32>, out: &mut Vec<f32>) {
        assert!(input.len() == self.len);
        assert!(out.len() == self.len);

        let mut buffer = Vec::<Complex<f32>>::new();

        let window = &self.window;

        for (i, item) in input.iter().enumerate() {
            buffer.push(Complex { re: *item * window[i], im: 0.0 });
        }

        self.fft.process(&mut buffer);

        for (i, value) in buffer.iter().enumerate() {
            let v = (value.re * value.re + value.im * value.im).sqrt();

            out[i] = v;
        }
    }
}

fn hann_window(len: usize) -> Vec<f32> {
    assert!(len.count_ones() == 1);

    let mut window = Vec::<f32>::new();

    let step : f32 = PI / len as f32;

    for i in 0..len {
        let tmp = (step * i as f32).sin();

        window.push(tmp * tmp);
    }

    window
}