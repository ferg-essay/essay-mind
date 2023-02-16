use crate::{FftWindow, analyze::power_msq};


#[derive(Debug)]
pub struct Harmonic {
    pub power_rms: f32,
    pub freqs: Vec<HarmonicItem>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point(usize, f32);

#[derive(Debug, Clone)]
pub struct Freq {
    pub freq: f32,
    pub power: f32,
}

#[derive(Debug)]
pub struct HarmonicItem {
    pub freq: f32,
    pub power: f32,
    pub harmonics: Vec<f32>,
}

impl Harmonic {
    pub fn harmonics_wave(
        wave: &[f32],
        sample: usize
    ) -> Harmonic {
        assert!(wave.len().count_ones() == 1);

        let mut fft_out = Vec::<f32>::new();
        fft_out.resize(wave.len(), 0.0);

        let fft = FftWindow::new(wave.len());
        fft.process(&wave, &mut fft_out);

        let fft_len = fft_out.len();

        Self::harmonics(&mut fft_out[..fft_len / 2], sample)
    }

    const N_POINTS: usize = 16;

    pub fn harmonics(
        fft: &mut [f32], 
        sample: usize,
    ) -> Harmonic {
        let power_msq = Self::normalize(fft);

        let mut max_freqs = Self::max_freqs(fft);

        let freq_factor: f32 = sample as f32 / (2 * fft.len()) as f32;

        /*
        let exp_freqs: Vec<Freq> = max_freqs.iter().map(|p| {
            Freq {
                freq: p.0 as f32 * sample as f32 / (2 * fft.len()) as f32,
                power: p.1,
            }
        }).collect();
         */
        let harmonics = Self::fill_harmonic_items(&mut max_freqs, freq_factor);
        
        Harmonic {
            power_rms: power_msq.sqrt(),
            freqs: harmonics,
        }
    }

    fn fill_harmonic_items(
        points: &mut Vec<Point>, 
        freq_factor: f32
    ) -> Vec<HarmonicItem> {
        points.sort_by_key(|p| p.0);

        let mut items = Vec::<HarmonicItem>::new();

        while points.len() > 0 {
            let base = points.remove(0);

            let mut harmonics = Vec::<f32>::new();
            let freq = base.0 as f32;
            harmonics.push(base.1);

            let freq = Self::extract_harmonics(points, &mut harmonics, freq);

            items.push(HarmonicItem { 
                freq: freq * freq_factor,
                power: harmonics.iter().sum(),
                harmonics,
            });
        }

        items
    }

    fn extract_harmonics(
        points: &mut Vec<Point>,
        harmonics: &mut Vec<f32>,
        base: f32,
    ) -> f32 {
        let delta = 1.0 / base;

        let mut i = 0;
        while i < points.len() {
            let factor = points[i].0 as f32 / base;

            if (factor + delta) % 1.0 < 2.0 * delta {
                let factor = (factor + delta) as usize;

                while harmonics.len() + 1 < factor {
                    harmonics.push(0.0);
                }

                harmonics.push(points[i].1);

                points.remove(i);
            } else {
                i += 1;
            }
        }

        base
    }

    fn max_freqs(fft: &[f32]) -> Vec::<Point> {
        let mut max_freqs = [Point(0, 0.0); Self::N_POINTS];

        let mut triple_freqs: Vec::<Point> = Vec::new();
        triple_freqs.resize(3 * Self::N_POINTS, Point(0, 0.0));

        for (freq, power) in fft.iter().enumerate() {
            Self::add_point(&mut triple_freqs, Point(freq, *power));
        }

        Self::extract_from_triple(&mut triple_freqs, &mut max_freqs);

        let threshold = 1e-2 * max_freqs[0].1;
        let mut values = Vec::<Point>::new();

        for point in max_freqs {
            if threshold <= point.1 {
                values.push(point);
            }
        }

        values
    }

    fn extract_from_triple(triple_points: &mut Vec<Point>, points: &mut [Point]) {
        for point in points {
            let mut first = triple_points.remove(0);

            let mut count: usize = 1; 
            let mut value = first.1;
            let mut i = 0;
            
            while i < triple_points.len() {
                let p: Point = triple_points[i];

                if p.0 == first.0 + 1 || p.0 == first.0 - 1 {
                    value += p.1;
                    count += 1;
                    triple_points.remove(i);
                } else {
                    i += 1;
                }
            }

            //value = value / count as f32;

            first.1 = value;

            *point = first;
        }
    }

    fn add_point(points: &mut [Point], point: Point) {
        let tail = points.len() - 1;

        if points[tail].1 < point.1 {
            points[tail] = point;

            points.sort_by(|p, q| {
                let left: f32 = p.1;
                let right: f32 = q.1;

                right.partial_cmp(&left).unwrap()
            });
        }
    }

    fn normalize(fft: &mut [f32]) -> f32 {
        let power_sum: f32 = fft.iter().sum();
        let power_msq = power_sum / (2 * fft.len()) as f32;
        let power_sum_inv = 1.0 / power_sum;

        for freq in fft.iter_mut() {
            *freq *= power_sum_inv;
        }

        power_msq
    }
}