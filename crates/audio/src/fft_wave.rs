use crate::{FftInverse, buffer::AudioBuffer};

pub struct FftWave {
    len: usize,
    sample_rate: f32,
    data: Vec<f32>,
}

impl FftWave {
    pub fn new(len: usize) -> Self {
        assert!(len.count_ones() == 1);

        let mut data = Vec::<f32>::new();
        data.resize(len, 0.0);

        Self {
            len,
            sample_rate: 16000 as f32,
            data: data,
        }
    }

    pub fn sample_rate(&mut self, rate: i32) -> &mut Self {
        self.sample_rate = rate as f32;

        self
    }

    pub fn frequency(&mut self, frequency: f32, amplitude: f32) {
        let freq_i = self.freq_to_index(frequency);
        
        self.data[freq_i] += amplitude;
    }

    pub fn frequency_2(&mut self, frequency: f32, amplitude: f32) {
        let freq_i = self.freq_to_index(frequency);
        
        self.data[freq_i - 1] += 0.25 * amplitude;
        self.data[freq_i] += 0.5 * amplitude;
        self.data[freq_i + 2] += 0.5 * amplitude;
    }

    pub fn white(&mut self, amplitude: f32) {
        let start = 1;
        let end = self.data.len();
        assert!(start < end);
        let value = amplitude / (end - start) as f32;
        // let mut rng = rand::thread_rng();

        for i in start..end {
            let v = rand::random::<f32>();
            self.data[i] += value * (v - 0.5);
        }
    }

    pub fn freq_drop(&mut self, frequency: f32, amplitude: f32) {
        self.frequency_2(frequency, amplitude * 0.8);
        self.frequency_2(2.0 * frequency, amplitude * 0.3);
        self.frequency_2(4.0 * frequency, amplitude * 0.1);
        //self.frequency(6.0 * frequency, amplitude * 0.05);
        //self.frequency(8.0 * frequency, amplitude * 0.1);
    }

    pub fn blur_flat(&mut self, freq_min: f32, freq_max: f32, amplitude: f32) {
        let i_min = self.freq_to_index(freq_min);
        let i_max = self.freq_to_index(freq_max);

        let value = amplitude / (i_max - i_min + 1) as f32;

        for i in i_min..=i_max {
            self.data[i] += value;
        }
    }

    pub fn blur_log(&mut self, freq: f32, freq_width: f32, amplitude: f32) {
        // let freq_min = freq;
        let freq_max = freq * (1.0 + freq_width);

        let factor = (2.0f32).powf(1.0 / 12.0);
        //let factor = 3.0 / 2.0;
        let mut values = Vec::<f32>::new();
        let mut f = freq;

        while f <= freq_max {
            values.push(f);

            f *= factor;
        }

        let value = amplitude / values.len() as f32;

        for f in values {
            let i = self.freq_to_index(f);

            self.data[i] += value;
        }
    }

    fn freq_to_index(&self, frequency: f32) -> usize {
        (frequency * self.len as f32 / self.sample_rate) as usize
    }
    
    pub fn build(mut self) -> AudioBuffer {
        let fft = FftInverse::new(self.len);

        let mut result: Vec<f32>  = Vec::new();
        result.resize(self.len, 0.0);

        fft.process(&mut self.data, &mut result);

        /*
        for (i, val) in result.iter().enumerate() {
            print!("\n  {} {}\n", i, val);

        }
        */
        /*
        for (i, val) in result.iter().enumerate() {
            print!("  {} {}\n", i, val);
        }
        */

        AudioBuffer::new(result)
    }
}