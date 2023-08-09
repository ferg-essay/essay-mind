use essay_plot::{graph::Graph, artist::{Lines2d, LinesOpt}};
use essay_tensor::prelude::*;

pub struct UiPlot {
    graph: Graph,
    lines: Vec<UiLine>,
    tick: usize,
    x: Vec<f32>,
}

impl UiPlot {
    pub const LIM: usize = 100;

    pub(crate) fn new(graph: Graph) -> Self {
        Self {
            graph,
            lines: Vec::new(),
            tick: 0,
            x: Vec::new(),
        }
    }

    pub fn line(&mut self, key: impl UiKey, label: &str) {
        let lines = Lines2d::from_xy([0.], [0.]);

        let mut line_opt = self.graph.artist(lines);
        line_opt.label(label);

        self.lines.push(UiLine::new(key, line_opt));
    }

    pub fn push(&mut self, key: &dyn UiKey, y: f32) {
        for line in &mut self.lines {
            if line.key.index() == key.index() {
                line.y.push(y);
            }
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        self.x.push(self.tick as f32);

        for y in &mut self.lines {
            if y.y.len() < self.x.len() {
                y.y.push(0.);
            }
        }

        if self.x.len() > Self::LIM {
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
        self.graph.x_label(label);
    }
}

struct UiLine {
    key: Box<dyn UiKey>,
    line: LinesOpt,
    y: Vec<f32>,
}

impl UiLine {
    fn new(key: impl UiKey, line: LinesOpt) -> Self {
        Self {
            key: Box::new(key),
            line: line,
            y: Vec::new(),
        }
    }
}

pub trait UiKey : Send + Sync + 'static {
    fn index(&self) -> usize;
}

impl UiKey for usize {
    fn index(&self) -> usize {
        *self
    }
}
