use std::ops::Deref;

use essay_plot::{chart::Chart, artist::{GridColorOpt, GridColor}};
use essay_tensor::Tensor;


pub struct UiChart {
    chart: Chart,
}

impl UiChart {
    pub const LIM: usize = 100;

    pub(crate) fn _new(chart: Chart) -> Self {
        Self {
            chart,
        }
    }

    pub fn chart(&mut self) -> &mut Chart {
        &mut self.chart
    }

    pub fn color_grid(
        &mut self, 
        data: impl Into<Tensor>
    ) -> GridColorOpt {
        self.chart.flip_y(true);
        self.chart.aspect(1.);
        self.chart.x().visible(false);
        self.chart.y().visible(false);
        let colormesh = GridColor::new(data);

        self.chart.artist(colormesh)
    }
}

impl Deref for UiChart {
    type Target = Chart;

    fn deref(&self) -> &Self::Target {
        &self.chart
    }
}