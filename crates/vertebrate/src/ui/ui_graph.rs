use std::{cell::RefCell, any::type_name};

use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use mind_ecs::{PostTick, PreTick};
use ui_graphics::ui_plot::{UiPlot, UiFigurePlugin, UiFigure, PlotKeyId};

#[derive(Component)]
pub struct UiGraph {
    plot: UiPlot,

}

impl UiGraph {
    fn new(figure: &UiFigure<BodyPlot>) -> Self {
        let mut plot = figure.plot_xy((0., 0.), (1., 1.));
        plot.lim(256);

        plot.graph_mut().ylim(-0.1, 1.1);

        Self {
            plot,
        }
    }

    fn line(&mut self, label: &str) -> PlotKeyId {
        self.plot.line(label)
    }

    fn _push(&mut self, id: PlotKeyId, value: f32) {
        self.plot.push(id, value);
    }

    fn color(&mut self, id: PlotKeyId, color: Color) {
        self.plot.color(id, color);
    }
}

pub fn ui_plot_update(
    mut ui_body: ResMut<UiGraph>,
) {
    ui_body.plot.tick();
}

pub fn ui_body_spawn_plot(
    mut c: Commands,
    mut plot: ResMut<UiFigure<BodyPlot>>
) {
    c.spawn(UiGraph::new(plot.get_mut()))
}

pub struct BodyPlot;

pub struct UiGraphPlugin {
    xy: Point,
    wh: Point,

    colors: Vec<Color>,

    items: Vec<(String, Box<dyn Item>)>,
}

impl UiGraphPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        Self {
            xy: xy.into(),
            wh: wh.into(),
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

impl Plugin for UiGraphPlugin {
    fn build(&self, app: &mut App) {
        let figure = UiFigurePlugin::<BodyPlot>::new(self.xy, self.wh);
        figure.build(app);

        let colors = self.colors.clone();
        
        let mut graph = UiGraph::new(app.resource_mut::<UiFigure<BodyPlot>>());

        for (i, (label, item)) in self.items.iter().enumerate() {
            let color = colors[i % colors.len()];

            let id = graph.line(label);
                
            graph.color(id, color);

            item.add(id, app);
        }

        app.insert_resource(graph);
        
        //app.system(PostUpdate, ui_plot_peptide);
        app.system(PreTick, ui_plot_update);
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
        assert!(app.contains_resource::<T>(),
            "{:?} is an unregistered resource", type_name::<T>());

        if ! app.contains_resource::<PeptideUpdates<T>>() {
            let updates: PeptideUpdates<T> = PeptideUpdates {
                updates: Vec::new(),
            };

            app.insert_resource(updates);

            app.system(
                PostTick,
                |updates: Res<PeptideUpdates<T>>, res: Res<T>, mut ui: ResMut<UiGraph>| {
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
