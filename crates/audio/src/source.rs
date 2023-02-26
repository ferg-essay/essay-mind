use core::{f32::consts::PI};
use core::ops;
use crate::{AudioBuffer, BezierSpline};
use rand::Rng;
use crate::ui_symphonia::{AudioReader};

pub trait AudioSource: Iterator<Item = f32> + Send {
    fn reset(&mut self, sample: Option<u32>);
}

pub trait AudioFilter: Send {
    fn reset(&mut self, sample: Option<u32>);

    fn next(&mut self, data: f32) -> Option<f32>;
}

const DEFAULT_SAMPLES : u32 = 44100;

//
// # float multiply implementation
//

impl ops::Mul<Box<dyn AudioSource>> for f32 {
    type Output = Box<dyn AudioSource>;

    fn mul(self, rhs: Box<dyn AudioSource>) -> Self::Output {
        Box::new(MulSource {
            amplitude: self,
            source: rhs,
        })
    }
}

struct MulSource {
    amplitude: f32,
    source: Box<dyn AudioSource>,
}

impl AudioSource for MulSource {
    fn reset(&mut self, sample: Option<u32>) {
        self.source.reset(sample);
    }
}

impl Iterator for MulSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        match self.source.next() {
            Some(value) => Some(self.amplitude * value),
            None => None,
        }
    }
}

//
// # add implementation
//

impl ops::Add<Box<dyn AudioSource>> for Box<dyn AudioSource> {
    type Output = Box<dyn AudioSource>;

    fn add(self, rhs: Box<dyn AudioSource>) -> Self::Output {
        Box::new(AddSource {
            lhs: self,
            rhs: rhs,
        })
    }
}

struct AddSource {
    lhs: Box<dyn AudioSource>,
    rhs: Box<dyn AudioSource>,
}

impl AudioSource for AddSource {
    fn reset(&mut self, sample: Option<u32>) {
        self.lhs.reset(sample);
        self.rhs.reset(sample);
    }
}

impl Iterator for AddSource {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        match self.lhs.next() {
            Some(left_value) => {
                match self.rhs.next() {
                    Some(right_value) => {
                        Some(left_value + right_value)
                    },
                    None => None,
                }
            },
            None => None,
        }
    }
}

//
// # sine implementation
//

pub fn sine(freq: f32) -> Box<dyn AudioSource> {
    Box::new(TimeFunction::new(move |time| {
        (time * freq * 2.0 * PI).sin()
    }))
}

pub fn sine_phase(freq: f32, phase: f32) -> Box<dyn AudioSource> {
    Box::new(TimeFunction::new(move |time| {
        ((time * freq + phase) * 2.0 * PI).sin()
    }))
}

//
// # square
//

pub fn square(freq: f32) -> Box<dyn AudioSource> {
    Box::new(TimeFunction::new(move |time| {
        let sq_time = (time * freq) % 1.0;

        if sq_time <= 0.5 { 1.0 } else { -1.0 }
    }))
}

//
// # white
//

pub fn white() -> Box<dyn AudioSource> {
    Box::new(TimeFunction::new(move |_| {
        let mut rng = rand::thread_rng();

        let mut v: f32 = 0.;

        for _ in 0..12 {
            v += rng.gen::<f32>();
        }

        (1. / 6.) * v - 1.
    }))
}

//
// # time function iterators
//

struct TimeFunction {
    step: f32,
    time: f32,

    fun: Box<dyn Fn(f32) -> f32 + Send>,
}

impl TimeFunction {
    fn new(fun: impl Fn(f32) -> f32 + Send + 'static) -> Self {
        let sample = DEFAULT_SAMPLES;

        TimeFunction {
            step: 1.0 / sample as f32,
            time: 0.0,
            fun: Box::new(fun),
        }
    }
}

impl Iterator for TimeFunction {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let time = self.time;

        self.time += self.step;

        Some((self.fun)(time))
    }
}

impl AudioSource for TimeFunction {
    fn reset(&mut self, sample: Option<u32>) {
        self.time = 0.0;

        if let Some(sample) = sample {
            self.step = 1.0 / sample as f32;
        };
    }
}

//
// # file
//

pub fn file(path: &str) -> Result<Box<dyn AudioSource>,String> {
    let buffer = AudioReader::read(path);
   
    Ok(Box::new(FileBuffer {
        buffer: buffer,
        file_samples: DEFAULT_SAMPLES,
        time: 0,
    }))
}

struct FileBuffer {
    buffer: AudioBuffer,
    file_samples: u32,
    time: usize,
}

impl Iterator for FileBuffer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let time = self.time;

        self.time += 1;

        if time < self.buffer.len() {
            Some(self.buffer[time])
        } else {
            None
        }
    }
}

impl AudioSource for FileBuffer {
    fn reset(&mut self, sample: Option<u32>) {
        self.time = 0;

        if let Some(sample) = sample {
            assert!(self.file_samples == sample);
        };
    }
}

//
// # splines
//

pub fn spline_peaks(freq: f32, points: &[(f32, f32)]) -> Box<dyn AudioSource> {
    assert!(freq > 0.);
    assert!(points.len() > 1);
    assert!(points[0].0 == 0.);
    assert!(points[points.len() - 1].0 < 1.);

    let mut splines = Vec::<SplinePoint>::new();

    for (i, point) in points.iter().skip(1).enumerate() {
        let prev = &points[i];

        assert!(prev.0 < point.0);

        let spline = BezierSpline::new(&[
            (0.0, prev.1),
            (0.5, prev.1),
            (0.5, point.1),
            (1.0, point.1),
        ]);

        splines.push(SplinePoint {
            x0: prev.0,
            x1: point.0,
            spline: spline,
        })
    }

    let spline = BezierSpline::new(&[
        (0.0, points[points.len() - 1].1),
        (0.5, points[points.len() - 1].1),
        (0.5, points[0].1),
        (1.0, points[0].1),
    ]);

    splines.push(SplinePoint {
        x0: points[points.len() - 1].0,
        x1: 1.,
        spline: spline,
    });

    let mut source = Box::new(SplineSource {
        freq: freq,
        splines: splines,
        buffer: Vec::<f32>::new(),
        time: 0,
    });

    source.reset(Some(DEFAULT_SAMPLES));

    source
}

pub fn spline_shape(freq: f32, points: &[(f32, f32)]) -> Box<dyn AudioSource> {
    assert!(freq > 0.);
    assert!(points.len() > 1);
    assert!(points.len() % 2 == 0);
    assert!(points[0].0 == 0.);
    assert!(points[points.len() - 2].0 < 1.);

    let mut splines = Vec::<SplinePoint>::new();

    let mut i = 0;
    while i < points.len() {
        let prev = &points[i];
        let shape = &points[i + 1];
        let next = &points[(i + 2) % points.len()];

        assert!(prev.0 < next.0 || i + 2 == points.len());

        let spline = BezierSpline::new(&[
            (0.0, prev.1),
            (shape.0, prev.1),
            (1. - shape.1, next.1),
            (1.0, next.1),
        ]);

        splines.push(SplinePoint {
            x0: prev.0,
            x1: if prev.0 <= next.0 { next.0 } else { 1.0 },
            spline: spline,
        });

        i += 2;
    }

    let mut source = Box::new(SplineSource {
        freq: freq,
        splines: splines,
        buffer: Vec::<f32>::new(),
        time: 0,
    });

    source.reset(Some(DEFAULT_SAMPLES));

    source
}

struct SplineSource {
    freq: f32,

    splines: Vec<SplinePoint>,

    buffer: Vec<f32>,
    time: usize,
}

struct SplinePoint {
    x0: f32,
    x1: f32,
    spline: BezierSpline,
}

impl Iterator for SplineSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let time = self.time;

        self.time = (self.time + 1) % self.buffer.len();

        Some(self.buffer[time])
    }
}

impl SplineSource {
    fn fill_buffer(&mut self, sample: u32) {
        let wave_len = (sample as f32 / self.freq) as usize;
        assert!(wave_len > 1);

        let wave = &mut self.buffer;
        wave.resize(wave_len, 0.0);

        for spline in &self.splines {
            let i0 = (spline.x0 * wave_len as f32) as usize;
            let i1 = (spline.x1 * wave_len as f32) as usize;

            spline.spline.eval_as_fn(&mut wave[i0..i1]);
        }
    }
}

impl AudioSource for SplineSource {
    fn reset(&mut self, sample: Option<u32>) {
        self.time = 0;

        if let Some(sample) = sample {
            self.fill_buffer(sample);
        };
    }
}

//
// # filters
//

impl ops::Shr<Box<dyn AudioFilter>> for Box<dyn AudioSource> {
    type Output = Box<dyn AudioSource>;

    fn shr(self, filter: Box<dyn AudioFilter>) -> Self::Output {
        Box::new(Filter {
            source: self,
            filter: filter,
        })
    }
}

struct Filter {
    source: Box<dyn AudioSource>,
    filter: Box<dyn AudioFilter>,
}

impl Iterator for Filter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            Some(data) => self.filter.next(data),
            None => None,
        }
    }
}

impl AudioSource for Filter {
    fn reset(&mut self, sample: Option<u32>) {
        self.source.reset(sample);
        self.filter.reset(sample);
    }
}

//
// # lowpass chebyshev
//

pub fn lowpass(freq: f32) -> Box<dyn AudioFilter> {
    let epsilon = 0.05;
    
    let mut filter = LowPassChebyshev::<2>::new(freq, epsilon);

    filter.reset(Some(DEFAULT_SAMPLES));

    Box::new(filter)
}

pub fn lowpass_2(freq: f32) -> Box<dyn AudioFilter> {
    let epsilon = 0.05;
    
    let mut filter = LowPassChebyshev::<1>::new(freq, epsilon);

    filter.reset(Some(DEFAULT_SAMPLES));

    Box::new(filter)
}

pub fn lowpass_4(freq: f32) -> Box<dyn AudioFilter> {
    let epsilon = 0.05;
    
    let mut filter = LowPassChebyshev::<2>::new(freq, epsilon);

    filter.reset(Some(DEFAULT_SAMPLES));

    Box::new(filter)
}

pub fn lowpass_8(freq: f32) -> Box<dyn AudioFilter> {
    let epsilon = 0.05;
    
    let mut filter = LowPassChebyshev::<4>::new(freq, epsilon);

    filter.reset(Some(DEFAULT_SAMPLES));

    Box::new(filter)
}

//#[derive(Default)]
struct LowPassChebyshev<const N: usize> {
    freq: f32,
    epsilon: f32,

    a: [f32; N],
    d1: [f32; N],
    d2: [f32; N],

    // w0: [f32; N],
    w1: [f32; N],
    w2: [f32; N],
}

impl<const N: usize> LowPassChebyshev<N> {
    fn new(freq: f32, epsilon: f32) -> Self {
        Self {
            freq,
            epsilon,
    
            a: [0.; N],
            d1: [0.; N],
            d2: [0.; N],
    
            // w0: [0.; N],
            w1: [0.; N],
            w2: [0.; N],
        }
    }
}

impl<const N: usize> AudioFilter for LowPassChebyshev<N> {
    fn next(&mut self, x: f32) -> Option<f32> {
        let a = &self.a;
        let d1 = &self.d1;
        let d2 = &self.d2;

        // let w0 = &mut self.w0;
        let w1 = &mut self.w1;
        let w2 = &mut self.w2;

        let mut x = x;

        for i in 0..N {
            let w0_i = d1[i] * w1[i] + d2[i] * w2[i] + x;
            x = a[i] * (w0_i + 2. * w1[i] + w2[i]);
            w2[i] = w1[i];
            w1[i] = w0_i;
        }

        Some(x * 2. / self.epsilon)
    }

    fn reset(&mut self, sample: Option<u32>) {
        if let Some(sample) = sample {
            // n = filter order
            let n_f = 2. * N as f32;

            let a = (PI * self.freq / sample as f32).tan();
            let a_sq = a * a;

            let eps = self.epsilon;
            let u = ((1. + (1. + eps * eps).sqrt()) / eps).ln();

            let sinh_u = (u / n_f).sinh();
            let cosh_u = (u / n_f).cosh();

            for i in 0..N {
                let i_f = i as f32;
                let b = (PI * (2. * i_f + 1.) / (2. * n_f)).sin() * sinh_u;
                let c = (PI * (2. * i_f + 1.) / (2. * n_f)).cos() * cosh_u;
    
                let c_sq = b * b + c * c;
                let s = a_sq * c_sq + 2. * a * b + 1.;

                self.a[i] = a_sq / (4. * s);
                self.d1[i] = 2. * (1. - a_sq * c) / s;
                self.d2[i] = - (a_sq * c - 2. * a * b + 1.) / s;
            }
        }

        for i in 0..N {
            // self.w0[i] = 0.;
            self.w1[i] = 0.;
            self.w2[i] = 0.;
        }
    }
}


//
// # highpass chebyshev
//

pub fn highpass(freq: f32) -> Box<dyn AudioFilter> {
    let epsilon = 0.05;
    
    let mut filter = HighPassChebyshev::<2>::new(freq, epsilon);

    filter.reset(Some(DEFAULT_SAMPLES));

    Box::new(filter)
}

pub fn highpass_2(freq: f32) -> Box<dyn AudioFilter> {
    let epsilon = 0.05;
    
    let mut filter = HighPassChebyshev::<1>::new(freq, epsilon);

    filter.reset(Some(DEFAULT_SAMPLES));

    Box::new(filter)
}

pub fn highpass_4(freq: f32) -> Box<dyn AudioFilter> {
    let epsilon = 0.05;
    
    let mut filter = HighPassChebyshev::<2>::new(freq, epsilon);

    filter.reset(Some(DEFAULT_SAMPLES));

    Box::new(filter)
}

pub fn highpass_8(freq: f32) -> Box<dyn AudioFilter> {
    let epsilon = 0.05;
    
    let mut filter = HighPassChebyshev::<4>::new(freq, epsilon);

    filter.reset(Some(DEFAULT_SAMPLES));

    Box::new(filter)
}

//#[derive(Default)]
struct HighPassChebyshev<const N: usize> {
    freq: f32,
    epsilon: f32,

    a: [f32; N],
    d1: [f32; N],
    d2: [f32; N],

    // w0: [f32; N],
    w1: [f32; N],
    w2: [f32; N],
}

impl<const N: usize> HighPassChebyshev<N> {
    fn new(freq: f32, epsilon: f32) -> Self {
        Self {
            freq,
            epsilon,
    
            a: [0.; N],
            d1: [0.; N],
            d2: [0.; N],
    
            // w0: [0.; N],
            w1: [0.; N],
            w2: [0.; N],
        }
    }
}

impl<const N: usize> AudioFilter for HighPassChebyshev<N> {
    fn next(&mut self, x: f32) -> Option<f32> {
        let a = &self.a;
        let d1 = &self.d1;
        let d2 = &self.d2;

        // let w0 = &mut self.w0;
        let w1 = &mut self.w1;
        let w2 = &mut self.w2;

        let mut x = x;

        for i in 0..N {
            let w0_i = d1[i] * w1[i] + d2[i] * w2[i] + x;
            x = a[i] * (w0_i + 2. * w1[i] + w2[i]);
            w2[i] = w1[i];
            w1[i] = w0_i;
        }

        Some(x * 2. / self.epsilon)
    }

    fn reset(&mut self, sample: Option<u32>) {
        if let Some(sample) = sample {
            // n = filter order
            let n_f = 2. * N as f32;

            let a = (PI * self.freq / sample as f32).tan();
            let a_sq = a * a;

            let eps = self.epsilon;
            let u = ((1. + (1. + eps * eps).sqrt()) / eps).ln();

            let sinh_u = (u / n_f).sinh();
            let cosh_u = (u / n_f).cosh();

            for i in 0..N {
                let i_f = i as f32;
                let sr = (PI * (2. * i_f + 1.) / (2. * n_f)).sin() * sinh_u;
                let cr = (PI * (2. * i_f + 1.) / (2. * n_f)).cos() * cosh_u;
    
                let c_sq = sr * sr + cr * cr;
                let s = a_sq + 2. * a * sr + c_sq;

                self.a[i] = 1. / (4. * s);
                self.d1[i] = 2. * (c_sq - a_sq) / s;
                self.d2[i] = - (a_sq - 2. * a * sr + c_sq) / s;
            }
        }

        for i in 0..N {
            // self.w0[i] = 0.;
            self.w1[i] = 0.;
            self.w2[i] = 0.;
        }
    }
}


//
// # bandpass chebyshev
//

pub fn bandpass(freq_min: f32, freq_max: f32) -> Box<dyn AudioFilter> {
    // assert!(freq_min < freq_max);
    bandpass_4n::<1>(freq_min, freq_max)
}

pub fn bandpass_4(freq_min: f32, freq_max: f32) -> Box<dyn AudioFilter> {
    // assert!(freq_min < freq_max);
    bandpass_4n::<1>(freq_min, freq_max)
}

pub fn bandpass_8(freq_min: f32, freq_max: f32) -> Box<dyn AudioFilter> {
    // assert!(freq_min < freq_max);
    bandpass_4n::<2>(freq_min, freq_max)
}

pub fn bandpass_16(freq_min: f32, freq_max: f32) -> Box<dyn AudioFilter> {
    // assert!(freq_min < freq_max);
    bandpass_4n::<4>(freq_min, freq_max)
}

pub fn bandpass_4n<const N: usize>(freq_min: f32, freq_max: f32) -> Box<dyn AudioFilter> {
        // assert!(freq_min < freq_max);
    
    let epsilon = 0.05;
    
    let mut filter = BandPassChebyshev::<N>::new(
        freq_min,
        freq_max,
        epsilon
    );

    filter.reset(Some(DEFAULT_SAMPLES));

    Box::new(filter)
}

struct BandPassChebyshev<const N: usize> {
    freq_min: f32,
    freq_max: f32,
    epsilon: f32,

    a: [f32; N],

    d1: [f32; N],
    d2: [f32; N],
    d3: [f32; N],
    d4: [f32; N],

    // w0: [f32; N],
    w1: [f32; N],
    w2: [f32; N],
    w3: [f32; N],
    w4: [f32; N],
}

impl<const N: usize> BandPassChebyshev<N> {
    fn new(freq_min: f32, freq_max: f32, epsilon: f32) -> Self {
        Self {
            freq_min,
            freq_max,
            epsilon,
    
            a: [0.; N],
            d1: [0.; N],
            d2: [0.; N],
            d3: [0.; N],
            d4: [0.; N],
    
            // w0: [0.; N],
            w1: [0.; N],
            w2: [0.; N],
            w3: [0.; N],
            w4: [0.; N],
        }
    }
}

impl<const N: usize> AudioFilter for BandPassChebyshev<N> {
    fn next(&mut self, x: f32) -> Option<f32> {
        let a = &self.a;
        let d1 = &self.d1;
        let d2 = &self.d2;
        let d3 = &self.d3;
        let d4 = &self.d4;

        //let w0 = &mut self.w0;
        let w1 = &mut self.w1;
        let w2 = &mut self.w2;
        let w3 = &mut self.w3;
        let w4 = &mut self.w4;

        let mut x = x;

        for i in 0..N {
            let w0_i = d1[i] * w1[i] + d2[i] * w2[i] + d3[i] * w3[i] + d4[i] * w4[i] + x;
            x = a[i] * (w0_i - 2. * w2[i] + w4[i]);
            w4[i] = w3[i];
            w3[i] = w2[i];
            w2[i] = w1[i];
            w1[i] = w0_i;
        }

        Some(x * 2.0 / self.epsilon)
        // Some(x)
    }

    fn reset(&mut self, sample: Option<u32>) {
        if let Some(sample) = sample {
            let n_f = 4. * N as f32;

            let s_f = sample as f32;

            let f1 = self.freq_max;
            let f2 = self.freq_min;
            // let (f1, f2) = (self.freq_max, self.freq_min);
            //let (f2, f1) = (self.freq_max, self.freq_min);

            let a =
                (PI * (f1 + f2) / s_f).cos() /
                (PI * (f1 - f2) / s_f).cos();
            let a_sq = a * a;

            let b = (PI * (f1 - f2) / s_f).tan();
            let b_sq = b * b;

            let eps = self.epsilon;
            let u = ((1. + (1. + eps * eps).sqrt()) / eps).ln();

            let sinh_u = (2. * u / n_f).sinh();
            let cosh_u = (2. * u / n_f).cosh();

            for i in 0..N {
                let i_f = i as f32;

                let sr = (PI * (2. * i_f + 1.) / n_f).sin() * sinh_u;
                let cr = (PI * (2. * i_f + 1.) / n_f).cos() * cosh_u;
    
                let c_sq = sr * sr + cr * cr;
                let s = b_sq * c_sq + 2. * b * sr + 1.;

                self.a[i] = b_sq / (4. * s);
                self.d1[i] = 4. * a * (1.0 + b * sr) / s;
                self.d2[i] = 2. * (b_sq * c_sq - 2. * a_sq - 1.) / s;
                self.d3[i] = 4. * a * (1. - b * sr) / s;
                self.d4[i] = -(b_sq * c_sq - 2. * b * sr + 1.) / s;
            }
        }

        for i in 0..N {
            // self.w0[i] = 0.;
            self.w1[i] = 0.;
            self.w2[i] = 0.;
            self.w3[i] = 0.;
            self.w4[i] = 0.;
        }
    }
}

//
// # jitter
//

struct Jitter {
    sample: usize,
    update_period: usize,

    _frequency_jitter: f32,
    _amplitude_jitter: f32,

    sample_step: f32,
    jitter_step: f32,
    amplitude: f32,

    time: usize,
    source_time: usize,
    next_update: usize,
}

impl AudioFilter for Jitter {
    fn reset(&mut self, sample: Option<u32>) {
        if let Some(sample) = sample {
            self.update_period = sample as usize / 8;
        }

        self.time = 0;
        self.next_update = self.time + self.sample as usize / 8;

        self.sample_step = 1. / self.sample as f32;
        self.jitter_step = 1. / self.sample as f32;
        self.amplitude = 1.;
    }

    fn next(&mut self, data: f32) -> Option<f32> {
        self.time += 1;
        while self.source_time < self.time {
            self.source_time += 1;
        }

        Some(data)
    }
}
