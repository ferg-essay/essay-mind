use std::cell::RefCell;

use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use ui_graphics::ui_plot::{UiPlot, UiFigurePlugin, UiFigure, PlotKeyId};
use crate::mid_peptides::{BoxPeptide, MidPeptides, PeptideId, Peptide};

#[derive(Component)]
pub struct UiGraph2 {
    plot: UiPlot,

}

impl UiGraph2 {
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

    fn color(&mut self, id: PlotKeyId, color: Color) {
        self.plot.color(id, color);
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
    mut ui_body: ResMut<UiGraph2>,
    peptides: Res<MidPeptides>,
) {
    ui_body.plot.push(peptide.plot, peptides[peptide.peptide]);

    //ui_body.plot.tick();
}

pub fn ui_plot_update(
    mut ui_body: ResMut<UiGraph2>,
) {
    ui_body.plot.tick();
}

pub fn ui_body_spawn_plot(
    mut c: Commands,
    mut plot: ResMut<UiFigure<BodyPlot>>
) {
    c.spawn(UiGraph2::new(plot.get_mut()))
}

pub struct BodyPlot;

pub struct UiGraph2Plugin {
    xy: Point,
    wh: Point,

    lines: Vec<(BoxPeptide, String)>,
    colors: Vec<Color>,

    items: Vec<(String, Box<dyn Item>)>,
}

impl UiGraph2Plugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        Self {
            xy: xy.into(),
            wh: wh.into(),
            lines: Vec::new(),
            colors: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn colors(mut self, colors: impl Into<Colors>) -> Self {
        for color in colors.into().into() {
            self.colors.push(color);
        }

        self
    }

    pub fn line(mut self, peptide: impl Peptide, label: &str) -> Self {
        self.lines.push((peptide.box_clone(), String::from(label)));
        
        self
    }

    pub fn item<T>(
        mut self,
        label: &str,
        item: impl IntoItem<T>
    ) -> Self {

        let item = IntoItem::into_item(item);

        self.items.push((String::from(label), item));

        self
    }
}

impl Plugin for UiGraph2Plugin {
    fn build(&self, app: &mut App) {
        if app.contains_resource::<MidPeptides>() {
            let figure = UiFigurePlugin::<BodyPlot>::new(self.xy, self.wh);
            //app.plugin(figure);
            figure.build(app);

            let mut lines : Vec<(PeptideId, String)> = Vec::new();

            let colors = self.colors.clone();

            for (key, label) in &self.lines {
                if let Some(peptide) = app.resource::<MidPeptides>().get_peptide(key.as_ref()) {
                    lines.push((peptide.id(), label.clone()));
                }
            }


            /*
            app.system(Startup, move |mut c: Commands, mut plot: ResMut<UiFigure<BodyPlot>>| {
                let mut graph = UiGraph2::new(plot.get_mut());

                for (peptide, label) in &lines {
                    let plot_key = graph.line(&label);
                    c.spawn(PeptideLine::new(*peptide, plot_key));

                    let len = colors.len();
                    if len > 0 {
                        graph.color(plot_key, colors[plot_key.i() % len]);
                    }
                }

                c.insert_resource(graph);
            });
            */

            let mut graph = UiGraph2::new(app.resource_mut::<UiFigure<BodyPlot>>());

            for (i, (label, item)) in self.items.iter().enumerate() {
                let color = colors[i % colors.len()];

                let id = graph.line(label);
                
                graph.color(id, color);

                item.add(id, app);
            }

            app.insert_resource(graph);
        
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

pub trait Item {
    fn add(&self, id: PlotKeyId, app: &mut App);
}

type UpdateBox<T> = Box<dyn Fn(&T) -> f32 + Sync + Send>;

pub struct PeptideUpdates<T> {
    updates: Vec<(PlotKeyId, UpdateBox<T>)>,
}

impl<T> PeptideUpdates<T> {
    fn add(&mut self, id: PlotKeyId, fun: Option<UpdateBox<T>>) {
        self.updates.push((id, fun.unwrap()));
    }
}

struct ItemImpl<T> {
    update: RefCell<Option<UpdateBox<T>>>,
}

impl<T: Send + Sync + 'static> Item for ItemImpl<T> {
    fn add(&self, id: PlotKeyId, app: &mut App) {
        assert!(app.contains_resource::<T>());

        if ! app.contains_resource::<PeptideUpdates<T>>() {
            let updates: PeptideUpdates<T> = PeptideUpdates {
                updates: Vec::new(),
            };

            app.insert_resource(updates);

            app.system(
                Update,
                |updates: Res<PeptideUpdates<T>>, res: Res<T>, mut ui: ResMut<UiGraph2>| {
                    for (id, fun) in &updates.updates {
                        ui.plot.push(*id, fun(res.get()));
                    }
            });
        }

        app.resource_mut::<PeptideUpdates<T>>().add(id, self.update.take());
    }
} 

pub trait IntoItem<T> {
    fn into_item(this: Self) -> Box<dyn Item>;
}

impl<T: Send + Sync + 'static, F> IntoItem<T> for F
where
    F: Fn(&T) -> f32 + Send + Sync + 'static
{
    fn into_item(this: Self) -> Box<dyn Item> {
        Box::new(ItemImpl {
            update: RefCell::new(Some(Box::new(this)))
        })
    }
}
