use std::ops::Deref;

use essay_plot::{graph::Graph, artist::{GridColorOpt, GridColor}};
use essay_tensor::Tensor;


pub struct UiGraph {
    graph: Graph,
}

impl UiGraph {
    pub const LIM: usize = 100;

    pub(crate) fn new(graph: Graph) -> Self {
        Self {
            graph,
        }
    }

    pub fn graph(&mut self) -> &mut Graph {
        &mut self.graph
    }

    pub fn color_grid(
        &mut self, 
        data: impl Into<Tensor>
    ) -> GridColorOpt {
        self.graph.flip_y(true);
        self.graph.aspect(1.);
        self.graph.x().visible(false);
        self.graph.y().visible(false);
        let colormesh = GridColor::new(data);

        self.graph.artist(colormesh)
    }
}

impl Deref for UiGraph {
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}