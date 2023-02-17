use core::{f32::consts::PI};
use core::ops;
use crate::AudioBuffer;
use crate::ui_symphonia::{AudioReader};

pub trait AudioSource: Iterator<Item = f32> + Send {
    fn reset(&mut self, _sample: Option<u32>) { }
}

pub trait AudioFilter {
    fn next(&mut self, source: dyn AudioSource) -> Option<f32>;
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
