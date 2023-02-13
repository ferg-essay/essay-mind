use std::{thread, time, f32::consts::PI};
use ui_audio::AudioOut;
use cpal;
use fundsp::hacker::*;

fn main() {
    sub::<f32>();
}

fn sub<T>()
where
    T: cpal::Sample
{
    let mut audio = AudioOut::new();
    /*
    let host = cpal::default_host();

    // output device
    let device = host.default_output_device().expect("no output device");
    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config")
        .with_max_sample_rate();
    print!("{:?}", supported_config.sample_format());

    assert!(supported_config.sample_format() == SampleFormat::F32);

    let config: StreamConfig = supported_config.into();

    let device_in = host.default_input_device().expect("unable to get device");
    let config_in = device_in.default_input_config().expect("failed to get input");

    print!("\ninput {:?}\n", config_in);
    print!("\noutput {:?}\n", config);
    */

    let sample_rate = audio.config.sample_rate.0 as f64;
    let channels = audio.config.channels as usize;

    print!("sample: {}\n", sample_rate);

    let c = 0.2 * (organ_hz(midi_hz(57.0)));

    let mut c = c;
    /*
        >> (declick() | declick())
        >> (dcblock() | dcblock())
        >> limiter_stereo((1.0, 5.0));
    */
    c.reset(Some(sample_rate));

    let mut next_sample = move || c.get_mono() as f32; //.get_stereo();

    /*
    audio.open(move |data: &mut [])

    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        0.3 * (
          1.0 * (sample_clock * 128.0 * 2.0 * PI / sample_rate).sin()
          + 0.8 * (sample_clock * 132.0 * 2.0 * PI / sample_rate).sin()
          + 0.8 * (sample_clock * 200.0 * 2.0 * PI / sample_rate).sin()
          + 0.8 * (sample_clock * 228.0 * 2.0 * PI / sample_rate).sin()
          + 0.8 * (sample_clock * 230.0 * 2.0 * PI / sample_rate).sin()
          + 0.8 * (sample_clock * 235.0 * 2.0 * PI / sample_rate).sin()
          + 0.8 * (sample_clock * 240.0 * 2.0 * PI / sample_rate).sin()
          + 0.2 * (sample_clock * 330.0 * 2.0 * PI / sample_rate).sin()
          + 0.2 * (sample_clock * 335.0 * 2.0 * PI / sample_rate).sin()
          + 0.2 * (sample_clock * 340.0 * 2.0 * PI / sample_rate).sin()
          + 0.1 * (sample_clock * 430.0 * 2.0 * PI / sample_rate).sin()
          + 0.1 * (sample_clock * 435.0 * 2.0 * PI / sample_rate).sin()
          + 0.1 * (sample_clock * 440.0 * 2.0 * PI / sample_rate).sin()
        )
    };
    */

    print!("sample_rate {}\n", sample_rate);
    print!("channels {}\n", channels);

    audio.open(next_sample);

    // stream.play().unwrap();

    thread::sleep(time::Duration::from_millis(1000));
    
}

fn write_data(output: &mut [f32], channels: usize, next_sample: &mut dyn FnMut() -> (f64,f64))
{
    for frame in output.chunks_mut(channels) {
        let (left, right) = next_sample();
        //let left : T = cpal::Sample::from::<f32>(&(sample.0 as f32));
        //let right: T = cpal::Sample::from::<f32>(&(sample.1 as f32));

        for (channel, sample) in frame.iter_mut().enumerate() {
            if channel & 1 == 0 {
                *sample = left as f32;
            } else {
                *sample = right as f32;
            }
        }
    }
}