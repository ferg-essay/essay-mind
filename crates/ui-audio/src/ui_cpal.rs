use cpal::{traits::{HostTrait, DeviceTrait}, SampleFormat, StreamConfig, Device, Stream, SampleRate};

pub struct AudioOut {
    config: StreamConfig,
    device: Device,
    stream: Option<Stream>,
}

impl AudioOut {
    pub fn new() -> AudioOut {
        let host = cpal::default_host();
        // output device
        let device = host.default_output_device().expect("no output device");
    
        let config: StreamConfig = AudioOut::select_config(&device);

        Self {
            device: device,
            config: config,
            stream: None,
        }
    }

    fn select_config(device: &Device) -> StreamConfig {
        let supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");

        for cfg in supported_configs_range {
            if cfg.sample_format() == SampleFormat::F32
                && cfg.max_sample_rate() == SampleRate(44100) {
                    return cfg.with_max_sample_rate().into();
            }
        }

        panic!("Unable to find F32 and 44100 config");
    }

    pub fn sample_rate(&self) -> u32 {
        self.config.sample_rate.0
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
            move |_err| {
                panic!("error\n");
            },
            None
        ).unwrap();

        self.stream = Some(stream);
    }
}
