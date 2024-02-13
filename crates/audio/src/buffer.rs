use std::ops::Index;


pub struct AudioBuffer {
    buffer: Vec<f32>,
    index: usize,
}

impl AudioBuffer {
    pub fn new(buffer: Vec<f32>) -> Self {
        Self {
            buffer,
            index: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, data: f32) {
        self.buffer.push(data);
    }

    pub fn extend(&mut self, next: AudioBuffer) {
        self.buffer.extend(&next.buffer);
    }

    #[inline]
    pub fn append(&mut self, next: &[f32]) {
        self.buffer.extend(next);
    }

    #[inline]
    pub fn next(&mut self) -> f32 {
        let i = self.index;

        self.index = (self.index + 1) % self.buffer.len();

        self.buffer[i]
    }

    pub fn as_vec(&self) -> Vec<f32> {
        self.buffer.clone()
    }

    pub fn get(&self, i: usize) -> f32 {
        self.buffer[i]
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

impl Clone for AudioBuffer {
    fn clone(&self) -> Self {
        Self { buffer: self.buffer.clone(), index: self.index.clone() }
    }
}

impl<I> Index<I> for AudioBuffer
where 
    I: std::slice::SliceIndex<[f32]>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.buffer[index]
    }
}

pub struct AudioWriter<'a> {
    buf: &'a mut AudioBuffer,
    offset: f32,
    rate: f32,
    sum: f32,
    source_count: usize,
    target_count: usize,
}

impl<'a> AudioWriter<'a> {
    pub const RATE : usize = 20000;

    pub fn new(buf: &'a mut AudioBuffer, rate: usize) -> Self {
        assert!(Self::RATE <= rate);

        Self {
            buf,
            rate: rate as f32 / Self::RATE as f32,
            offset: 0.,
            sum: 0.,
            source_count: 0,
            target_count: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, item: f32) {
        let offset = self.offset + self.rate;

        self.sum += item;
        self.source_count += 1;
        self.offset = offset;

        if 1. < offset + self.rate {
            if self.target_count < Self::RATE {
                self.buf.push(self.sum / self.source_count as f32);
                self.target_count += 1;
            }
            self.sum = 0.;
            self.source_count = 0;
            self.offset -= 1.;
        }
    }

    pub fn finish(&mut self) {
        if self.source_count > 0 {
            let value = self.sum / self.source_count as f32;
            while self.target_count < Self::RATE {
                self.buf.push(value);
                self.target_count += 1;
            }
            self.source_count = 0;
        }
    }
}