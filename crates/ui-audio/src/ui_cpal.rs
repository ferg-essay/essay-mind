use std::{thread, time, f32::consts::PI};

use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}, SampleFormat, StreamConfig, Sample, FromSample, Device, Stream, SampleRate};

pub struct AudioOut {
    pub config: StreamConfig,
    pub device: Device,
    stream: Option<Stream>,
}

impl AudioOut {
    pub fn new() -> AudioOut {
        let host = cpal::default_host();
        // output device
        let device = host.default_output_device().expect("no output device");
        let mut supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");

        for cfg in supported_configs_range {
            print!("  {:?}\n", cfg);
        }

        let supported_config = device.supported_output_configs()
            .expect("config error")
            .next()
            .expect("no supported config")
            .with_max_sample_rate();
    
        assert!(supported_config.sample_format() == SampleFormat::F32);
    
        let config: StreamConfig = supported_config.into();

        Self {
            device: device,
            config: config,
            stream: None,
        }
    }

    pub fn open(
        &mut self,
        mut next_sample: impl FnMut() -> f32 + Send + 'static,
    ) {
        assert!(self.stream.is_none());

        let channels = self.config.channels as usize;

        let stream = self.device.build_output_stream(
            &self.config,
            move |output: &mut [f32], _| {
                for frame in output.chunks_mut(channels) {
                    let value = next_sample();
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            move |err| {
                panic!("error\n");
            },
            None
        ).unwrap();

        self.stream = Some(stream);
    }
}
