use std::{thread, time};
use ui_audio::{AudioOut, fft_wave::FftWave, AudioBuffer, AudioReader};
use fundsp::hacker::*;

fn main() {
    let mut audio = AudioOut::new();

    // let c = dc(110.0) >> triangle();
    // let c = zero() >> pluck(440.0, 0.8, 0.8);
    //let c = mls();
    //let c = pink();
    //let c = dc(110.0) >> square();
    //let c = (mls() | dc(400.0) | dc(50.0)) >> resonator();
    //let f = 440.0;
    //let m = 2.0;
    //let c = oversample(sine_hz(f) * f * m + f >> sine());
    let c = 0.2 * (organ_hz(midi_hz(57.0))
     + organ_hz(midi_hz(61.0)) + organ_hz(midi_hz(64.0)));
    //let c = 0.2 * (organ_hz(midi_hz(60.0)));
    //let c = oversample(sine_hz(220.0) + 0.4 * sine_hz(440.0));
    //let c = brown();

    //let c = c >> (chorus(0, 0.0, 0.1, 0.2));

    let mut c = c >> declick() >> dcblock();
    /*
        >> (declick() | declick())
        >> (dcblock() | dcblock())
        >> limiter_stereo((1.0, 5.0));
    */
    c.reset(Some(audio.sample_rate() as f64));

    //let mut buffer = AudioReader::read("assets/blip.ogg");
    //let mut buffer = AudioReader::read("assets/sfx_exp_short_hard6.wav");
    //let mut buffer = AudioReader::read("assets/sfx_movement_footsteps1a.wav");
    let mut space = Vec::<f32>::new();
    space.resize(8196, 0.0);
    let space = AudioBuffer::new(space);
    //buffer.extend(space.clone());
    //buffer.extend(AudioReader::read("assets/sfx_movement_footsteps1b.wav"));
    //buffer.extend(space);
    let mut buffer = AudioReader::read("assets/bud.ogg");
    print!("buffer {}\n", buffer.len());
    //audio.open(move || c.get_mono() as f32);
    audio.open(move || buffer.next());


    thread::sleep(time::Duration::from_millis(2000));
    
}
