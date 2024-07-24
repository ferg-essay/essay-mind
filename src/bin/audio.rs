use std::{collections::VecDeque, fs};

use audio::AudioReader;
use essay_plot::{api::JoinStyle, artist::{ColorMaps, Norm, Norms}, chart::{Figure, Chart}};
use essay_tensor::{array::stack, init::linspace, signal::rfft_norm, tensor::TensorVec, Tensor};

pub fn main() {
    audio_display();
}

fn audio_work() {
    //let path = "assets/audio/clips/my.ogg";
    let path = "assets/audio/clips/bed.ogg";
    //let path = "assets/audio/clips/sfx_coin_single1.wav";
    //let path = "assets/audio/clips/violin_b4.ogg";
    //let path = "assets/audio/clips/shy.ogg";
    //let path = "assets/audio/clips/zoo.ogg";
    //let path = "assets/audio/clips/wiki-above.ogg";
    //let path = "assets/audio/clips/wiki-boot.ogg";
    //let path = "assets/audio/clips/wiki-sand.ogg";
    //let path = "assets/audio/clips/kite.ogg";
    //let path = "assets/audio/clips/bid.ogg";
    //let path = "/Users/ferg/wsp/assets/audio/animal/bird.ogg";
    //let path = "/Users/ferg/wsp/assets/book-24/237-134500-0042.flac";
    //let path = "/Users/ferg/wsp/assets/audio/human/American-English/Consonants/shy.wav";
    // let path = "assets/audio/American-English/Consonants/sigh.wav";
    //let path = "assets/audio/American-English/Conventions/new.wav";
    //let path = "assets/audio/American-English/Narrative/narrative6.wav";
    //let path = "assets/audio/American-English/Vowels/bode.wav";
    //let path = "assets/audio/American-English/Vowels/booed.wav";
    //let path = "assets/audio/American-English/Vowels/bead.wav";
    //let path = "assets/audio/American-English/Vowels/bayed.wav";
    //let path = "assets/audio/American-English/Vowels/bud.wav";
    //let path = "/Users/ferg/wsp/assets/audio/steps/leaves02.ogg";
    //let path = "/Users/ferg/wsp/assets/audio/animal/cat-meow-14536.ogg";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Fire2.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Frogs3.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Heavy_rain_on_hard_surface.wav";    
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Teletype.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Castanets1.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Horse_trotting_on_cobblestones.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Rain.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Waterfall.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Applause_big_room.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Rhythmic_applause.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Sparrows_large_excited_group.wav";
    let path = "/Users/ferg/wsp/game/assets/audio/sounds/sadeghi-snake.wav";
    let path = "/Users/ferg/wsp/game/assets/audio/sounds/sadeghi-water.wav";
    //let path = "/Users/ferg/wsp/game/assets/audio/sounds/sadeghi-crowd.wav";
    //let path = "/Users/ferg/wsp/game/assets/audio/sounds/sadeghi-bird.wav";
    //let path = "/Users/ferg/wsp/game/assets/audio/sounds/sadeghi-fire.wav";

    let reader = AudioReader::read(path);

    let values = Tensor::from(reader.as_vec());

    let mut figure = Figure::new();

    let mut graph = figure.chart([1., 1.]);

    let is_graph = false;
    let nfft = 512;
    let overlap = 256;

    let nfft3 = 512;
    let overlap3 = 256;

    let mut slice = values.clone();
    //let mut slice = average_n(&slice, 2);
    // slice = values.subslice(8000, 512);
    let n = 4;
    let mut slice2 = average_n(&slice, 1);
    let mut slice3 = average_n(&slice, 2 * n * n);
    let mut slice4 = average_n(&slice, n * n * n);

    // let chunk = nfft;
    let n_avg = n * n * n;
    let fft_vec2 = avg_fft(&slice, nfft, n);
    let fft_vec3 = avg_fft(&slice, nfft, 4 * n * n);
    let fft_vec4 = avg_fft(&slice, nfft, n * n * n);
    /*
    let mut vec_4 = AvgFft::new(n_avg, nfft);
    let mut offset = 0;
    let mut fft_vec4 = Vec::<Tensor>::new();
    while offset < slice.len() {
        vec_4.push(&slice, offset);
        offset += nfft;

        fft_vec4.push(vec_4.fft());
    }
    let fft_vec4 = stack(fft_vec4, 1);
    */

    let rms = rms_n(&slice, 128);
    // let mut slice3 = slice2.clone();

    let subslice = slice.clone();
    //let subslice = slice.subslice(19000, 1024);
    let fft = rfft_norm(&subslice, ());
    let fft2 = rfft_norm(&slice2, ());
    // let fft = fft.log(10.);
    
    let subfft = fft.clone();
    //let subfft = fft.subslice(1, subfft.len() - 1);
    if is_graph {
        graph.plot_y(&subslice).join_style(JoinStyle::Bevel);
        let mut graph2 = figure.chart((0., 1., 1., 2.));
        graph2.plot_y(subfft).join_style(JoinStyle::Bevel);

        let mut graph3 = figure.chart((0., 2., 1., 3.));
        graph3.plot_y(&rms).join_style(JoinStyle::Bevel);
    }
    //graph2.specgram(slice).color_map(ColorMaps::BlueWhite2);
    //graph.ylim(0., 200.).specgram(values).nfft(512).overlap(128);
    //graph.specgram(values).nfft(1024).overlap(256);
    //graph.specgram(slice).nfft(512).overlap(256);
    //graph.specgram(slice).nfft(1024).overlap(3 * 256);
    //graph.ylim(0., 200.).specgram(slice).nfft(1024).overlap(3 * 256);
    if ! is_graph {
        //graph.specgram(&subslice);
        graph.specgram(slice).nfft(nfft).overlap(overlap);
        //graph.specgram(slice);
        //graph.specgram(slice).nfft(1024).overlap(3 * 256);
        //graph.ylim(0., 400.).specgram(&slice).nfft(2048).overlap(3 * 512);
        let mut graph2 = figure.chart((0., 1., 1., 2.));
        let minmax = graph_fft(&mut graph2, &fft_vec2, None);
        //graph2.specgram(slice2).nfft(nfft).overlap(overlap);

        let mut graph3 = figure.chart((0., 2., 1., 3.));
        graph_fft(&mut graph3, &fft_vec3, None);
        //graph3.specgram(slice3).nfft(nfft3).overlap(overlap3);

        //let mut graph4 = figure.new_graph([0., 3., 1., 4.]);
        //graph_fft(&mut graph4, &fft_vec4);

        //let mut graph5 = figure.new_graph([0., 4., 1., 5.]);
        //graph5.plot_y(&rms).join_style(JoinStyle::Bevel);

        //let mut graph6 = figure.new_graph([0., 5., 1., 6.]);
        //graph6.specgram(&rms);
    }
    //graph.ylim(0., 200.).specgram(slice).nfft(2048).overlap(3 * 512);
    //graph.plot_y(values).join_style(JoinStyle::Bevel);

    figure.show();

}

fn audio_display() {
    //let path = "assets/audio/clips/my.ogg";
    // let path = "assets/audio/clips/bed.ogg";
    //let path = "assets/audio/clips/sfx_coin_single1.wav";
    //let path = "assets/audio/clips/violin_b4.ogg";
    //let path = "assets/audio/clips/shy.ogg";
    //let path = "assets/audio/clips/zoo.ogg";
    //let path = "assets/audio/clips/wiki-above.ogg";
    //let path = "assets/audio/clips/wiki-boot.ogg";
    //let path = "assets/audio/clips/wiki-sand.ogg";
    //let path = "assets/audio/clips/kite.ogg";
    //let path = "assets/audio/clips/bid.ogg";
    //let path = "/Users/ferg/wsp/assets/audio/animal/bird.ogg";
    //let path = "/Users/ferg/wsp/assets/book-24/237-134500-0042.flac";
    //let path = "/Users/ferg/wsp/assets/audio/human/American-English/Consonants/shy.wav";
    // let path = "assets/audio/American-English/Consonants/sigh.wav";
    //let path = "assets/audio/American-English/Conventions/new.wav";
    let path1 = "/Users/ferg/wsp/assets/audio/human/American-English/Narrative/narrative6.wav";
    //let path = "assets/audio/American-English/Vowels/bode.wav";
    //let path1 = "/Users/ferg/wsp/assets/audio/human/American-English/Vowels/booed.wav";
    //let path = "assets/audio/American-English/Vowels/bead.wav";
    //let path = "assets/audio/American-English/Vowels/bayed.wav";
    let path1 = "/Users/ferg/wsp/assets/audio/human/American-English/Vowels/bud.wav";
    let path3 = "/Users/ferg/wsp/assets/audio/steps/leaves02.ogg";
    //let path1 = "/Users/ferg/wsp/assets/audio/animal/cat-meow-14536.ogg";
    let path2 = "/Users/ferg/wsp/assets/audio/natural/2500_ex1_1_Fire2.wav";
    let path1 = "/Users/ferg/wsp/assets/audio/natural/2500_ex1_1_Frogs3.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Heavy_rain_on_hard_surface.wav";    
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Teletype.wav";
    //let path4 = "/Users/ferg/wsp/assets/audio/natural/2500_ex1_1_Castanets1.wav";
    //let path4 = "/Users/ferg/wsp/assets/audio/natural/2500_ex1_1_Horse_trotting_on_cobblestones.wav";
    let path3 = "/Users/ferg/wsp/assets/audio/natural/2500_ex1_1_Rain.wav";
    //let path4 = "/Users/ferg/wsp/assets/audio/natural/2500_ex1_1_Horse_and_buggy.wav";
    let path4 = "/Users/ferg/wsp/assets/audio/natural/2500_ex1_1_Rhythmic_applause.wav";
    //let path3 = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Waterfall.wav";
    //let path4 = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Applause_big_room.wav";
    //let path4 = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Rhythmic_applause.wav";
    //let path = "/Users/ferg/wsp/assets/audio/natural/1093_ex1_1_Sparrows_large_excited_group.wav";
    //let path3 = "/Users/ferg/wsp/game/assets/audio/sounds/sadeghi-snake.wav";
    //let path4 = "/Users/ferg/wsp/game/assets/audio/sounds/sadeghi-water.wav";
    //let path = "/Users/ferg/wsp/game/assets/audio/sounds/sadeghi-crowd.wav";
    //let path = "/Users/ferg/wsp/game/assets/audio/sounds/sadeghi-bird.wav";
    //let path4 = "/Users/ferg/wsp/game/assets/audio/sounds/sadeghi-fire.wav";

    let mut figure = Figure::new();

    let nfft = 256;
    let overlap = 256;
    let n = 1;

    {
        let mut graph = figure.chart((0., 0., 1., 1.));
        let reader = AudioReader::read(path1);
        let value = Tensor::from(reader.as_vec());
    
        let fft_vec = avg_fft(&value, nfft, n);

        graph.x().visible(false);
        graph.y_label("Freq (kHz)");

        graph_fft(&mut graph, &fft_vec, None);
    }

    {
        let mut graph = figure.chart((1., 0., 2., 1.));
        let reader = AudioReader::read(path2);
        let value = Tensor::from(reader.as_vec());
    
        let fft_vec = avg_fft(&value, nfft, n);

        graph.x().visible(false);
        graph.y().visible(false);

        graph_fft(&mut graph, &fft_vec, None);
    }

    {
        let mut graph = figure.chart((0., 1., 1., 2.));
        let reader = AudioReader::read(path3);
        let value = Tensor::from(reader.as_vec());
    
        let fft_vec = avg_fft(&value, nfft, n);

        graph.x_label("Time (s)");
        graph.y_label("Freq (kHz)");

        graph_fft(&mut graph, &fft_vec, None);
    }

    {
        let mut graph = figure.chart((1., 1., 2., 2.));
        let reader = AudioReader::read(path4);
        let value = Tensor::from(reader.as_vec());
    
        let fft_vec = avg_fft(&value, nfft, n);

        graph.x_label("Time (s)");
        graph.y().visible(false);

        graph_fft(&mut graph, &fft_vec, None);
    }

    figure.show();

}

fn rms_n(value: &Tensor, n: usize) -> Tensor {
    let len = value.len();

    let mut vec : Vec<f32> = Vec::new();
    let f = 1. / n as f32;

    let mut j = 0;

    while j + n <= len {
        let mut avg = 0.;
        let mut v = 0.;

        for i in 0..n {
            let v0 = value[i + j];
            avg += v0;
            v += v0 * v0;
        }

        //vec.push((f * (v - avg * avg)).max(0.).sqrt());
        vec.push((f * v).max(0.).sqrt());

        j += n;
    }

    Tensor::from(vec)
}

fn average_n(value: &Tensor, n: usize) -> Tensor {
    let len = value.len();

    let mut vec : Vec<f32> = Vec::new();
    let f = 1. / n as f32;

    let mut j = 0;
    while j + n <= len {
        let mut v = 0.;

        for i in 0..n {
            v += value[j + i];
        }

        vec.push(f * v);

        j += n;
    }

    Tensor::from(vec)
}

fn avg_fft(tensor: &Tensor, nfft: usize, n: usize) -> Tensor {
    let mut avg_fft = AvgFft::new(n, nfft);
    let mut offset = 0;
    let mut fft_vec4 = Vec::<Tensor>::new();
    while offset < tensor.len() {
        avg_fft.push(&tensor, offset);
        offset += nfft;

        fft_vec4.push(avg_fft.fft());
    }

    stack(fft_vec4, 1)
}

fn graph_fft(chart: &mut Chart, tensor: &Tensor, minmax: Option<(f32, f32)>) -> (f32, f32) {
    let mut norm = Norm::from(Norms::Ln);
    norm.set_bounds(&tensor);
    let (min, max) = (norm.min(), norm.max());
    norm.set_vmin(norm.min().max(norm.max() - 4.));

    if let Some((min, max)) = minmax {
        norm.set_vmin(min);
        norm.set_vmax(max);
    }

    //let x = linspace(0., 10. * tensor.cols() as f32, tensor.cols());
    //let y = linspace(0., 10. * tensor.rows() as f32, tensor.rows());
    let ms = 20000 as f32 / 256 as f32;

    let xmax = tensor.cols() as f32 / ms;
    let ymax = ms * 1.0e-3 * tensor.rows() as f32;

    //graph.grid_color(tensor).color_map(ColorMaps::BlueOrange).norm(norm);
    chart.image(tensor).color_map(ColorMaps::BlueOrange).norm(norm).extent([xmax, ymax]);
    //graph.image(tensor);

    (min, max)
}

struct AvgFft {
    n: usize,
    chunk: usize,
    vec: VecDeque<Vec<f32>>,
}

impl AvgFft {
    fn new(n: usize, nfft: usize) -> Self {
        assert!(nfft % n == 0);

        let chunk = nfft / n;

        let mut deque = VecDeque::<Vec<f32>>::new();

        for _ in 0..n {
            let mut vec = Vec::<f32>::new();
            vec.resize(chunk, 0.);

            deque.push_back(vec);
        }

        Self {
            n,
            chunk,
            vec: deque,
        }
    }

    fn push(&mut self, value: &Tensor, offset: usize) {
        let len = value.len();
        let mut offset = offset;

        let sublen = ((len - offset) / self.n).min(self.chunk);

        self.vec.pop_front();


        let mut vec = Vec::<f32>::new();
        vec.resize(self.chunk, 0.);

        let f = 1. / self.n as f32;

        for j in 0..sublen {
            let mut v = 0.;

            for _ in 0..self.n {
                v += value[offset];

                offset += 1;
            }

            vec[j] = f * v;
        }

        self.vec.push_back(vec);
    }

    fn fft(&self) -> Tensor {
        let mut vec: TensorVec<f32> = TensorVec::<f32>::new();

        for item in &self.vec {
            for v in item {
                vec.push(*v);
            }
        }

        let value = vec.into_tensor();

        let result = rfft_norm(value, ());

        result.subslice(1, result.len() - 1)
    }
}

