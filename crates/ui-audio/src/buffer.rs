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

    pub fn push(&mut self, data: f32) {
        self.buffer.push(data);
    }

    pub fn extend(&mut self, next: AudioBuffer) {
        self.buffer.extend(&next.buffer);
    }

    pub fn append(&mut self, next: &[f32]) {
        self.buffer.extend(next);
    }

    pub fn next(&mut self) -> f32 {
        let i = self.index;

        self.index = (self.index + 1) % self.buffer.len();

        self.buffer[i]
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