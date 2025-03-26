use essay_plot::prelude::*;

fn main() {
    let mut figure = Figure::new();
    let mut chart = figure.chart();

    chart.title("Start and Sustain").size(14.);

    let mut vec = Vec::new();

    let v = 1.0;
    vec.push([-3., 0.]);
    vec.push([0., 0.]);
    vec.push([0.1, v]);
    vec.push([2., v]);
    vec.push([2.1, 0.]);
    vec.push([10., 0.]);

    chart.plot_xy(vec).label("Start");

    let mut vec = Vec::new();

    let v = 1.0;
    vec.push([-3., 0.]);
    vec.push([0.9, 0.]);
    for i in 0..40 {
        let value = v * 0.9f32.powi(2 * i);
        vec.push([1. + 0.1 * i as f32, 1. - value]);
    }
    //vec.push([3., v]);
    vec.push([6., v]);
    for i in 0..40 {
        let value = v * 0.95f32.powi(2 * i);
        vec.push([6.1 + 0.1 * i as f32, value]);
    }

    chart.plot_xy(vec).label("Sustain");
    chart.hline(0.5);

    figure.save("../test.png", 200.);
    figure.show();
}
