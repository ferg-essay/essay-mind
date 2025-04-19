use std::cell::RefCell;

use essay_tensor::tensor::Tensor;
use essay_ecs::prelude::*;
use essay_graphics::layout::ViewArc;
use essay_plot::{
    chart::PolarChart, config::ConfigArc, plot::{radar, RadarOpt}, 
};

use ui_graphics::ViewPlugin;

use super::ui_emoji::Emoji;

fn update_radar(mut radar: ResMut<RadarView>) {
    radar.update();
}

#[derive(Clone)]
pub struct RadarView {
    radar: RadarOpt,
    items: Vec<f32>,
}

impl RadarView {
    pub fn new(radar: RadarOpt) -> Self {
        Self {
            radar,

            items: Vec::new(),
        }
    }

    fn push(&mut self) -> usize {
        let id = self.items.len();

        self.items.push(0.);

        id
    }

    pub fn set_value(&mut self, i: usize, value: f32) {
        self.items[i] = value;
    }

    fn update(&mut self) {
        self.radar.set_y(&self.items);
    }
}

//
// UiMotivePlugin
//

pub struct UiRadarPlugin {
    items: Vec<Box<dyn PluginItem>>,

    polar: Option<PolarChart>,
    radar: Option<RadarView>,
}

impl UiRadarPlugin {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            polar: None,
            radar: None,
        }
    }

    pub fn item<T>(
        mut self, 
        x: f32,
        emoji: Emoji,
        fun: impl Fn(&T) -> f32 + Send + Sync + 'static
    ) -> Self
    where T: Default + Send + Sync + 'static {
        self.items.push(Box::new(Item {
            x,
            emoji,
            fun: RefCell::new(Some(Box::new(fun)))
        }));

        self
    }
}

impl ViewPlugin for UiRadarPlugin {
    fn view(&mut self, app: &mut App) -> Option<&ViewArc> {
        let mut polar = PolarChart::new(&ConfigArc::default());

        polar.y().visible(false);
        polar.xlim(0., 360.);
        let x: Tensor = self.items.iter().map(|item| item.x()).collect();
        let y = Tensor::fill([self.items.len()], 0.);
        
        let radar = polar.radar_xy(x, y);

        let mut radar = RadarView::new(radar);
        
        let mut ticks: Vec<(f32, String)> = Vec::new();

        for item in self.items.iter() {
            ticks.push((item.x(), String::from(item.emoji().code())));
            // ticks.push((i as f32, String::from(item.emoji().code())));

            let id = radar.push();

            item.system(id, app);
        }

        polar.x().tick_labels(&ticks);

        self.radar = Some(radar);

        self.polar = Some(polar);

        self.polar.as_ref().map(|p| p.view().arc())
    }
}

impl Plugin for UiRadarPlugin {
    fn build(&self, app: &mut App) {
        if let Some(radar) = &self.radar {
            app.insert_resource(radar.clone());

            app.system(Update, update_radar);
        }
    }
}

trait PluginItem {
    fn x(&self) -> f32;
    fn emoji(&self) -> Emoji;
    fn system(&self, id: usize, app: &mut App);
}

struct Item<T: Send + Sync + 'static> {
    x: f32,
    emoji: Emoji,
    fun: RefCell<Option<Box<dyn Fn(&T) -> f32 + Send + Sync + 'static>>>,
}

impl<T: Default + Send + Sync + 'static> PluginItem for Item<T> {
    fn x(&self) -> f32 {
        self.x
    }

    fn emoji(&self) -> Emoji {
        self.emoji.clone()
    }

    fn system(&self, id: usize, app: &mut App) {
        app.init_resource::<T>();

        let fun = self.fun.take().unwrap();

        app.system(PostUpdate, 
            move |mut radar: ResMut<RadarView>, item: Res<T>| {
                let value = fun(item.get());
                radar.set_value(id, value);
            }
        );
    }
} 
