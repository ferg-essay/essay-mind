use std::{f32::consts::PI, ops};

pub trait AudioComponent {
    fn next(&mut self) -> f32;
}

pub struct AudioSource {
    amplitude: f32,

    gen: Box<dyn AudioComponent>,
}

impl AudioSource {
    fn new(gen: Box<dyn AudioComponent>) -> Self {
        AudioSource {
            amplitude: 1.0,
            gen,
        }
    }

    pub fn resample(self, sample: usize) -> Self {
        resample(self, sample)
    }
}

impl AudioComponent for AudioSource {
    fn next(&mut self) -> f32 {
        self.amplitude * self.gen.next()
    }
}

//impl Clone for AudioSource {
//    fn clone(&self) -> Self {
//        Self { 
//            amplitude: self.amplitude.clone(), 
//            gen: self.gen.clone()
//        }
//    }
//}

impl ops::Mul<AudioSource> for f32 {
    type Output = AudioSource;

    fn mul(self, rhs: AudioSource) -> Self::Output {
        AudioSource {
            amplitude: self * rhs.amplitude,
            ..rhs
        }
    }
}

impl ops::Add<AudioSource> for AudioSource {
    type Output = AudioSource;

    fn add(self, rhs: AudioSource) -> Self::Output {
        AudioSource {
            amplitude: 1.0,
            gen: Box::new(AudioAdd {
                lhs: self,
                rhs: rhs,
            })
        }
    }
}

//
// # sine implementation
//


pub fn sine(freq: f32) -> AudioSource {
    AudioSource::new(Box::new(Sine {
        freq,
        step: 1.0f32 / 14410.0f32,
        time: 0.0
    }))
}

struct Sine {
    freq: f32,
    step: f32,
    time: f32,
}

impl AudioComponent for Sine {
    fn next(&mut self) -> f32 {
        let time = self.time;

        self.time += self.step;

        (time * self.freq * 2.0 * PI).sin()
    }
}

//
// # resample
//

///
/// resample the source
///
pub fn resample(source: AudioSource, sample: usize) -> AudioSource {
    AudioSource::new(Box::new(
        Resample {
            step: 1.0 / sample as f32,
            time: 0.0,

            source_step: 1.0 / 14400.0,
            source_time: 0.0,
            source_value: 0.0,
            source: source,
        }
    ))
}

struct Resample {
    step: f32,
    time: f32,

    source_step: f32,
    source_time: f32,
    source_value: f32,
    source: AudioSource,
}

impl AudioComponent for Resample {
    fn next(&mut self) -> f32 {
        if self.source_time <= self.time {
            self.source_time += self.source_step;

            self.source_value = self.source.next()
        }

        let value = self.source_value;

        self.time += self.step;

        while self.source_time < self.time {
            self.source_value = self.source.next();

            self.source_time += self.source_step;
        }

        value
    }
}

struct AudioAdd {
    lhs: AudioSource,
    rhs: AudioSource,
}

impl AudioComponent for AudioAdd {
    fn next(&mut self) -> f32 {
        self.lhs.next() + self.rhs.next()
    }
}
