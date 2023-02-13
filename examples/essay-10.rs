use std::{thread, time, f32::consts::PI};

use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}, SampleFormat, StreamConfig, Sample, FromSample};

fn main() {
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

    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

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

    print!("sample_rate {}\n", sample_rate);
    print!("channels {}\n", channels);

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            print!("callback {}\n", data.len());
            write_data(data, channels, &mut next_value);

        },
        move |err| {
            panic!("error\n");
        },
        None
    ).unwrap();

    stream.play().unwrap();

    thread::sleep(time::Duration::from_millis(1000));
    
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}