use std::{f32::consts::PI, sync::Arc};

use rustfft::{num_complex::Complex, FftPlanner, Fft};

///
/// Processes a windowed FFT transformation. The windowing is a Hann
/// window, used to reduce noise from signal boundary issues.
/// 
/// # Examples
/// 
/// ```
/// let fft = FftWindow::new(512);
/// fft.process(&input, &mut output);
/// ```
/// 
pub struct FftWindow {
    len: usize,
    
    fft: Arc<dyn Fft<f32>>,
    window: Vec<f32>,
}

impl FftWindow {
    ///
    /// Creates a new windowed FFT processor.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let fft = FftWindow::new(512);
    /// fft.process(&input, &output);
    /// ```
    /// 
    /// # Panics
    /// 
    /// * The length must be a power of 2.
    /// 
    pub fn new(len: usize) -> FftWindow {
        assert!(len.count_ones() == 1, "len must be a power of 2");

        let mut planner = FftPlanner::<f32>::new();
        let fft_fwd = planner.plan_fft_forward(len);

        let is_hann = true;

        let window = if is_hann { 
            hann_window(len) 
        } else {
            unit_window(len)
        };

        Self {
            len: len,
            fft: fft_fwd,
            window: window,
        }
    }

    ///
    /// Process a windowed FFT transform.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let fft = FftWindow::new(512);
    /// fft.process(&input, &mut out);
    /// ```
    /// 
    /// # Panics
    /// * Both the input and output must equal the prepared length.
    /// 
    pub fn process(&self, input: &Vec<f32>, output: &mut Vec<f32>) {
        assert!(input.len() == self.len);
        assert!(output.len() == self.len);

        let mut buffer = Vec::<Complex<f32>>::new();

        let window = &self.window;

        for (i, item) in input.iter().enumerate() {
            buffer.push(Complex { re: *item * window[i], im: 0.0 });
        }

        self.fft.process(&mut buffer);

        for (i, value) in buffer.iter().enumerate() {
            let v = (value.re * value.re + value.im * value.im).sqrt();

            output[i] = v;
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

fn unit_window(len: usize) -> Vec<f32> {
    assert!(len.count_ones() == 1);

    let mut window = Vec::<f32>::new();

    for _ in 0..len {
        window.push(1.0);
    }

    window
}