use ui_audio::AudioReader;
use ui_graphics::*;
use egui::plot;

fn main() {
    let buffer = AudioReader::read("assets/pod.ogg");
    //let buffer = AudioReader::read("assets/blip.ogg");

    let main_loop = main_loop::MainLoop::new();
    main_loop.run(move |ui| {
        let offset = 10000;
        let sin: plot::PlotPoints = (offset..offset + 1024).map(|i| {
            let x = i as f64;
            [x, buffer.get(i) as f64]
        }).collect();

        let line = plot::Line::new(sin);

        plot::Plot::new("my_plot").show(ui, |plot_ui| {
            plot_ui.line(line);
        });
    }).unwrap();
}
