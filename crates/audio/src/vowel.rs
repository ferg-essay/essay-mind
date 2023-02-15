use std::cmp;

use mind::{Gram, Digit::Med};

pub fn analyze_vowel(
    waves: &[f32], 
    fft: &[f32], 
    samples: usize,
    fft_len: usize
) -> String {
    let mut harmonics = [0.0; 12];

    let base = analyze_harmonics(fft, &mut harmonics);

    let mut gram = Gram::new();
    // the constant frequency is a marker for noise
    if fft[0] < 0.05 {
        gram.push(mind::Digit::Nil);
    } else {
        gram.push(mind::Digit::try_from_unit(fft[0], 16));
    }
    //gram.push(Med(u8::try_from(max_harmonic(&harmonics)).expect("can't convert")));
    for (i, value) in harmonics[0..6].iter().enumerate() {
        let value = fmin(*value, 0.999);

        if value < 0.05 {
            gram.push(mind::Digit::Nil);
        } else {
            gram.push(mind::Digit::try_from_unit(value, 16));
        }
    }

    analyze_wave(&mut gram, &waves, &harmonics, base, samples, fft_len);

    String::from(gram)
}

fn analyze_wave(
    gram: &mut Gram, 
    wave: &[f32], 
    harmonics: &[f32],
    base: usize,
    samples: usize,
    fft_len: usize
) {
    let expected_length = fft_len as f32 / base as f32;

    if wave.len() < 2 * expected_length as usize {
        return;
    }
    /*
    print!("len={} samples={} fft={} base={}\n", 
        expected_length,
        samples, fft_len, base
    );
    */
    let (max, start, end) = wave_bounds(&wave, expected_length as usize);
    print!("analyze max:{} start:{}[{}] end:{}[{}] exp={} samp={} base={} fft_len={}\n",
        max, start, wave[start], end, wave[end],
        expected_length, samples, base, fft_len);

    let segments  = harmonic_segments(&harmonics);
    let step = (end - start) as f32 / segments as f32;
    let range = (1.25 * 0.5 * step) as i32;

    for i in 0..segments {
        let pos = start as i32 + (i as f32 * step) as i32;

        let value = if i % 2 == 0 {
            segment_max(&wave, pos, range)
        } else {
            - segment_min(&wave, pos, range)
        };

        let radix = 16;
        let value = fmax(0.0, fmin(0.99, (0.5 + value / (2.01 * max))));

        if is_ramp(wave, pos) && false {
            gram.push(mind::Digit::Nil);
            /*
        } else if -1.0 <= value && value <= 1.0 {
            gram.push(mind::Digit::Nil);
            */
        } else {
            gram.push(mind::Digit::try_from_unit(value, radix));
        }
    }

    //print!("analyze max:{} start:{} end:{}", max, start, end);
}

fn harmonic_segments(harmonics: &[f32]) -> usize {
    let max = max_harmonic(harmonics);

    let limit = 0.4f32;

    if max == 3 && harmonics[3] > limit {
        return 12;
    } else if max == 3 && harmonics[2] > limit {
        return 6;
    } else if max == 4 && harmonics[2] > limit {
        return 12;
    } else if max == 2 && harmonics[2] > limit {
        return 6;
    } else if max == 2 && harmonics[3] > limit {
        return 8;
    } else {
        return 2 * max
    }
}

fn segment_min(wave: &[f32], pos: i32, range: i32) -> f32 {
    let min = cmp::max(0, pos - range) as usize;
    let max = cmp::min(wave.len() as i32 - 1, pos + range) as usize;

    let mut min_value = 1.0e3f32;

    for i in min..=max {
        min_value = fmin(min_value, wave[i]);
    }

    min_value
}

fn segment_max(wave: &[f32], pos: i32, range: i32) -> f32 {
    let min = cmp::max(0, pos - range) as usize;
    let max = cmp::min(wave.len() as i32 - 1, pos + range) as usize;

    let mut max_value = -1.0e3f32;

    for i in min..=max {
        max_value = fmax(max_value, wave[i]);
    }

    max_value
}

fn is_ramp(wave: &[f32], pos: i32) -> bool {
    if pos < 2 || pos > wave.len() as i32 - 2 {
        return false;
    }

    let pos = pos as usize;

    if wave[pos - 1] < wave[pos] && wave[pos] < wave[pos + 1] {
        true
    } else if wave[pos + 1] < wave[pos] && wave[pos] < wave[pos - 1] {
        true
    } else {
        false
    }
}

fn max_harmonic(harmonics: &[f32]) -> usize {
    let mut max_harmonic = 0;
    let mut max_value = 0.0f32;

    for (i, value) in harmonics.iter().enumerate() {
        if max_value < *value {
            max_harmonic = i;
            max_value = *value;
        }
    }

    max_harmonic + 1
}

fn fmin(v1: f32, v2: f32) -> f32 {
    if v1 < v2 { v1 } else { v2 }
}

fn fmax(v1: f32, v2: f32) -> f32 {
    if v1 < v2 { v2 } else { v1 }
}

fn wave_bounds(wave: &[f32], expected_length: usize) -> (f32, usize, usize) {
    let limit = wave.len() - wave.len() / 4;
    let start = wave_start(wave, 0, limit);

    let expect_end = start + expected_length;
    let expect_range = expected_length / 4;
    let mut end = wave_start(
        wave,
        expect_end - expect_range, 
        expect_end + expect_range);

    if end == 0 {
        end = expect_end as usize;
    }

    let max = wave_max(wave, start, end);

    let start = cmp::min(start, wave.len() - 1);
    let end = cmp::min(end, wave.len() - 1);

    (max, start, end)
}

///
/// returns the starting point for the wave analysis
/// 
fn wave_start(wave: &[f32], min: usize, limit: usize) -> usize {
    let mut start_index: usize = 0;
    let mut best_delta = 0.0f32;

    let mut last_pos_value = 0.0f32;
    let mut pos_candidate_value: f32 = 0.0f32;
    let mut pos_index = 0 as usize;

    let mut last_neg_value = 0.0f32;

    let min = cmp::max(0, min);
    let limit = cmp::min(wave.len() - 1, limit);

    for i in min..limit {
        let value = wave[i];

        if value > 0.0 {
            last_neg_value = 0.0;

            if last_pos_value < value {
                last_pos_value = value;
                pos_candidate_value = value;
                pos_index = i;
            }
        } else {
            last_pos_value = 0.0;

            if value < last_neg_value {
                last_neg_value = value;
                let delta = pos_candidate_value - last_neg_value;

                if best_delta < delta {
                    best_delta = delta;
                    start_index = pos_index;
                }
            }
        }
    }

    start_index
}

///
/// returns the max absolute value
/// 
fn wave_max(wave: &[f32], min: usize, limit: usize) -> f32 {
    let limit = cmp::min(wave.len() - 1, limit);

    let mut max_value = 0.0f32;

    for i in min..limit {
        let value = wave[i];

        max_value = fmax(max_value, value.abs());
    }

    max_value
}

struct Spike {
    index: usize,
    value: f32,
}

fn top_spikes(fft: &[f32], spikes: &mut [f32]) {
    for (i, value) in fft.iter().enumerate() {

    }

    
}
///
/// analyze the fft to find the base frequency and fill in the first
/// few harmonics
/// 
fn analyze_harmonics(fft: &[f32], harmonics: &mut [f32]) -> usize {
    let mut max_value = 0.0f32;
    let mut max_index: usize = 1;

    for (index, value) in fft.iter().enumerate() {
        let value = *value;
        
        if max_value < value {
            max_value = value;
            max_index = index;
        }
    }

    harmonics[0] = max_value;

    if max_index <= 8 {
        return cmp::max(1, max_index);
    }

    let mut best_base = max_index;
    let mut base_max = 0.0f32;

    for i in 1..8 {
        let base = (max_index / i) as usize;

        let base_value = fft_base(fft, max_index, i);

        if base_max < base_value {
            base_max = base_value;
            best_base = base;
            let step = max_index as f32 / i as f32;

            for j in 0..harmonics.len() {
                let mut freq = (step * (j as f32 + 1.0)).round() as usize;

                if i == j + 1 {
                    freq = max_index;
                }

                if freq + 1 < fft.len() {
                    harmonics[j] = fft_spike(fft, freq);
                }
            }
        }
    }

    best_base
}

fn fft_base(fft: &[f32], main: usize, factor: usize) -> f32 {
    let step = main as f32 / factor as f32;

    let mut value = 0f32;

    for i in 1..=8 {
        let index = (step * i as f32).round() as usize;

        value += fft_spike(fft, index);
    }

    value
}

fn fft_spike(fft: &[f32], freq: usize) -> f32 {
    if fft.len() * 3 / 4 <= freq {
        return 0.0;
    }
    
    let mut fft_max = fft[freq];
    let range = if freq > 8 { 2 } else { 1 };

    for i in freq - range..=cmp::min(freq + range, fft.len() - 1) {
        fft_max = fmax(fft_max, fft[i]);
    }

    if range > 1 && (fft_max == fft[freq - range] || fft_max == fft[freq + range]) {
        0.0
    } else {
        fft_max
    }
}