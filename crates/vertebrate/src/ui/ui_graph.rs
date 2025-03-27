use std::{cell::RefCell, any::type_name};

use essay_ecs::prelude::*;
use essay_graphics::layout::ViewArc;
use essay_plot::{artist::LinesOpt, chart::Chart, prelude::*};

use mind_ecs::{PostTick, PreTick};
use ui_graphics::ViewPlugin;

pub fn ui_plot_update(
    mut graph: ResMut<UiGraph>,
) {
    graph.tick();
}

pub struct UiGraph {
    tick: usize,
    x: LineData,
}

impl UiGraph {
    const LIM : usize = 256;

    fn new() -> Self {
        Self {
            tick: 0,
            x: LineData::new(),
        }
    }

    fn tick(&mut self) {
        self.x.push(self.tick as f32);
    
        self.tick += 1;
    
    }

    fn update(&self, line: &mut LinesOpt, y: &mut LineData, value: f32) {
        y.push(value);
        y.trim(&self.x);

        line.set_xy(&self.x.data, &y.data);
    }
}

struct LineData {
    lim: usize,
    data: Vec<f32>,
}

impl LineData {
    fn new() -> Self {
        Self {
            lim: UiGraph::LIM,
            data: Vec::new(),
        }
    }

    fn push(&mut self, value: f32) {
        self.data.push(value);

        while self.lim < self.data.len() {
            self.data.remove(0);
        }
    }

    fn trim(&mut self, x: &LineData) {
        while self.data.len() < x.data.len() {
            self.data.push(0.);
        }

        while x.data.len() < self.data.len() {
            self.data.remove(0);
        }
    }
}

pub struct UiGraphPlugin {
    colors: Vec<Color>,

    items: Vec<(String, Box<dyn Item>)>,

    view: Option<Chart>,
}

impl UiGraphPlugin {
    pub fn new() -> Self {
        Self {
            colors: Vec::new(),
            items: Vec::new(),
            view: None,
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

impl ViewPlugin for UiGraphPlugin {
    fn view(&mut self, _app: &mut App) -> Option<&ViewArc> {
        let chart = Chart::default();

        self.view = Some(chart);
        
        self.view.as_ref().map(|v| v.view().arc())
    }
}

impl Plugin for UiGraphPlugin {
    fn build(&self, app: &mut App) {
        if let Some(chart) = &self.view {
            let mut chart = chart.clone();

            let colors = self.colors.clone();

            chart.ylim(-0.1, 1.1);
        
            let graph = UiGraph::new();

            app.insert_resource(graph);
            app.system(PreTick, ui_plot_update);
        
            for (i, (label, item)) in self.items.iter().enumerate() {

                let mut line = chart.plot([0.], [0.]);

                if colors.len() > 0 {
                    let color = colors[i % colors.len()];
                    line.color(color);
                }
                line.label(label);

                item.add(line, app);
            }
        }
    }
}

pub trait Item {
    fn add(&self, id: LinesOpt, app: &mut App);
}

type UpdateBox<T> = Box<dyn Fn(&T) -> f32 + Sync + Send>;

struct ItemImpl<T> {
    update: RefCell<Option<UpdateBox<T>>>,
}

impl<T: Send + Sync + 'static> Item for ItemImpl<T> {
    fn add(&self, line: LinesOpt, app: &mut App) {
        assert!(app.contains_resource::<T>(),
            "{:?} is an unregistered resource", type_name::<T>());

        if let Some(fun) = self.update.take() {
            let mut y = LineData::new();
            let mut line = line;

            app.system(
                PostTick,
                move |res: Res<T>, graph: Res<UiGraph>| {
                    graph.update(&mut line, &mut y, fun(res.get()));
                }
            );
        }
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
