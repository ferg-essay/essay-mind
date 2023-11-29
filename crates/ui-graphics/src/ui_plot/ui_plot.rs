use essay_plot::{graph::Graph, artist::{Lines2d, LinesOpt}, api::Color};

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

    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    pub fn line(&mut self, label: &str) -> PlotKeyId {
        let lines = Lines2d::from_xy([0.], [0.]);

        let mut line_opt = self.graph.artist(lines);
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
/*
pub type BoxKey = Box<dyn UiKey>;

pub trait UiKey : Send + DynLabel + fmt::Debug {
    fn box_clone(&self) -> Box<dyn UiKey>;
}

impl PartialEq for dyn UiKey {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_dyn_eq())
    }
}

impl Eq for dyn UiKey {}

impl Hash for dyn UiKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PlotKeyId(usize);

impl PlotKeyId {
    pub fn i(&self) -> usize {
        self.0
    }
}
