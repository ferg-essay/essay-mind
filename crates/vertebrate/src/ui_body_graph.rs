use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use ui_graphics::ui_plot::{UiPlot, UiFigurePlugin, UiFigure, PlotKeyId};
use crate::mid_peptides::{BoxPeptide, MidPeptides, PeptideId, Peptide};

#[derive(Component)]
pub struct UiBodyGraph {
    plot: UiPlot,

}

impl UiBodyGraph {
    fn new(figure: &UiFigure<BodyPlot>) -> Self {
        let mut plot = figure.plot_xy((0., 0.), (1., 1.));

        plot.graph_mut().ylim(-0.1, 1.1);

        Self {
            plot,
        }
    }

    fn line(&mut self, label: &str) -> PlotKeyId {
        self.plot.line(label)
    }

    fn push(&mut self, id: PlotKeyId, value: f32) {
        self.plot.push(id, value);
    }
}
/*
pub fn ui_body_plot(
    ui_body: &mut UiBodyGraph,
    body: Res<Body>,
    _world: Res<World>,
    ui_world: Res<UiWorld>,
    mut ui: ResMut<UiCanvas>
) {
    ui_body.plot.push(&Key::PFood, body.p_food());
    ui_body.plot.push(&Key::Turn, (body.turn() + 0.5) % 1.);

    ui_body.plot.tick();
}
*/

pub fn ui_plot_peptide(
    peptide: &PeptideLine,
    mut ui_body: ResMut<UiBodyGraph>,
    peptides: Res<MidPeptides>,
) {
    ui_body.plot.push(peptide.plot, peptides[peptide.peptide]);

    //ui_body.plot.tick();
}

pub fn ui_plot_update(
    mut ui_body: ResMut<UiBodyGraph>,
) {
    ui_body.plot.tick();
}

pub fn ui_body_spawn_plot(
    mut c: Commands,
    mut plot: ResMut<UiFigure<BodyPlot>>
) {
    c.spawn(UiBodyGraph::new(plot.get_mut()))
}

pub struct BodyPlot;

pub struct UiGraphPlugin {
    xy: Point,
    wh: Point,

    lines: Vec<(BoxPeptide, String)>,
}

impl UiGraphPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        Self {
            xy: xy.into(),
            wh: wh.into(),
            lines: Vec::new(),
        }
    }

    pub fn line(mut self, peptide: impl Peptide, label: &str) -> Self {
        self.lines.push((peptide.box_clone(), String::from(label)));
        
        self
    }
}

impl Plugin for UiGraphPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_resource::<MidPeptides>() {
            let figure = UiFigurePlugin::<BodyPlot>::new(self.xy, self.wh);
            app.plugin(figure);

            let mut lines : Vec<(PeptideId, String)> = Vec::new();

            for (key, label) in &self.lines {
                if let Some(peptide) = app.resource::<MidPeptides>().get_peptide(key.as_ref()) {
                    lines.push((peptide.id(), label.clone()));
                }
            }

            app.system(Startup, move |mut c: Commands, mut plot: ResMut<UiFigure<BodyPlot>>| {
                let mut graph = UiBodyGraph::new(plot.get_mut());

                for (peptide, label) in &lines {
                    let plot_key = graph.line(&label);
                    c.spawn(PeptideLine::new(*peptide, plot_key))
                }

                c.insert_resource(graph);
            });
            
            app.system(PostUpdate, ui_plot_peptide);
            app.system(PreUpdate, ui_plot_update);
        }
    }
}

#[derive(Component)]
pub struct PeptideLine {
    peptide: PeptideId,
    plot: PlotKeyId,
}

impl PeptideLine {
    fn new(peptide: PeptideId, plot: PlotKeyId) -> Self {
        Self {
            peptide,
            plot
        }
    }
}
