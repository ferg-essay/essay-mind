use essay_plot::{chart::Chart, artist::{Lines2d, LinesOpt}, api::Color};

pub struct UiPlot {
    chart: Chart,
    lines: Vec<UiLine>,
    tick: usize,
    lim: usize,
    x: Vec<f32>,
}

impl UiPlot {
    pub const LIM: usize = 100;

    pub fn new(chart: Chart) -> Self {
        Self {
            chart,
            lines: Vec::new(),
            tick: 0,
            lim: Self::LIM,
            x: Vec::new(),
        }
    }

    pub fn chart_mut(&mut self) -> &mut Chart {
        &mut self.chart
    }

    pub fn lim(&mut self, lim: usize) {
        self.lim = lim.max(1);
    }

    pub fn line(&mut self, label: &str) -> PlotKeyId {
        let lines = Lines2d::from_xy([0.], [0.]);

        let mut line_opt = self.chart.artist(lines);
        line_opt.label(label);

        let id = PlotKeyId(self.lines.len());

        self.lines.push(UiLine::new(line_opt));

        id
    }

    pub fn push(&mut self, id: PlotKeyId, y: f32) {
        self.lines[id.i()].y.push(y);
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        self.x.push(self.tick as f32);

        for y in &mut self.lines {
            if y.y.len() < self.x.len() {
                y.y.push(0.);
            }
        }

        if self.x.len() > self.lim {
            self.x.remove(0);

            for y in &mut self.lines {
                y.y.remove(0);
            }
        }

        for y in &mut self.lines {
            y.line.set_xy(&self.x, &y.y);
        }
    }

    pub fn x_label(&mut self, label: &str) {
        self.chart.x_label(label);
    }

    pub fn color(&mut self, id: PlotKeyId, color: Color) {
        self.lines[id.i()].color(color);
    }
}

struct UiLine {
    //key: Box<dyn UiKey>,
    line: LinesOpt,
    y: Vec<f32>,
}

impl UiLine {
    fn new(line: LinesOpt) -> Self {
        Self {
            // key: Box::new(key),
            line: line,
            y: Vec::new(),
        }
    }

    fn color(&mut self, color: Color) {
        self.line.color(color);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PlotKeyId(usize);

impl PlotKeyId {
    pub fn i(&self) -> usize {
        self.0
    }
}
