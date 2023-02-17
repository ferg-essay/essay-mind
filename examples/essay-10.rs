use std::{thread, time};
use ui_audio::{AudioOut};
use audio::source::{sine,AudioSource, self, sine_phase, spline_peaks};
use fundsp::hacker::*;

fn main() {
    let mut audio = AudioOut::new();

    // let c = dc(110.0) >> triangle();
    // let c = zero() >> pluck(440.0, 0.8, 0.8);
    //let c = mls();
    let c = pink();
    //let c = dc(110.0) >> square();
    //let c = (mls() | dc(400.0) | dc(50.0)) >> resonator();
    //let f = 440.0;
    //let m = 2.0;
    //let c = oversample(sine_hz(f) * f * m + f >> sine());
    //let c = 0.2 * (organ_hz(midi_hz(57.0))
    // + organ_hz(midi_hz(61.0)) + organ_hz(midi_hz(64.0)));
    //let c = 0.2 * (organ_hz(midi_hz(60.0)));
    //let c = oversample(sine_hz(492.0));
    //let c = sine_hz(492.0);
    //let c = brown();
    let c = 0.3 * white();

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
    //let mut space = Vec::<f32>::new();
    //space.resize(8196, 0.0);
    //let space = AudioBuffer::new(space);
    //buffer.extend(space.clone());
    //buffer.extend(AudioReader::read("assets/sfx_movement_footsteps1b.wav"));
    //buffer.extend(space);
    //let mut buffer = AudioReader::read("assets/bud.ogg");
    //print!("buffer {}\n", buffer.len());

    let mut source = 
        0.2 * (sine(220.0) + 
        0.3 * sine_phase(330.0, 0.7) +
        0.1 * sine_phase(440.0, 0.4)
    );

    let mut source =
        0.158 * sine(70.) +
        0.141 * sine(2. * 70.) +
        0.142 * sine(3. * 70.) +
        0.208 * sine(4. * 70.) +
        0.058 * sine(5. * 70.) +
        0.056 * sine(6. * 70.0) +
        0.082 * sine(7. * 70.) +
        0.023 * sine(8. * 70.);

    let freq = 238.;

    let mut source = 0.3 * (
        0.317 * audio::sine(freq)
        + 0.462 * sine( 2. * freq)
        + 0.198 * sine(2. * freq)
    );

    let mut source = audio::white();
    let mut source = 0.5 * (audio::white() >> audio::bandpass_4(6000., 8000.))
    + 4. * (audio::white() >> audio::bandpass_4(3300., 4000.))
    + 0.2 * (audio::white() >> audio::lowpass_2(4000.));

    let mut source = 0.2 * spline_peaks(220., &[
        (0.0, 1.0),
        (0.5, -1.0),
    ]);

    //let mut source = 0.2 * sine(440.);

    //let mut source = source::file("assets/cymbal.wav").unwrap();
    //let mut source = 0.3 * source::file("assets/violin_b3.ogg").unwrap();
    //let mut source = 0.3 * source::file("assets/bead.ogg").unwrap();
    //let mut source = 0.3 * source::file("assets/booed.ogg").unwrap();
    //let mut source = audio::file("assets/shy.ogg").unwrap();
    //let mut source = 0.5 * sine(220.0);
    //let mut source = 0.5 * audio::square(220.0);

    print!("sample rate {}\n", audio.sample_rate());
    source.reset(Some(audio.sample_rate()));

    // audio.open(move || buffer.next());
    //audio.open(move || c.get_mono() as f32);
    audio.open(move || source.next().unwrap());


    thread::sleep(time::Duration::from_millis(2000));
    
}
