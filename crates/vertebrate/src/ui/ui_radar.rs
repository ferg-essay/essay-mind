use std::{cell::RefCell, marker::PhantomData};

use essay_tensor::tensor::Tensor;
use essay_ecs::prelude::*;
use essay_graphics::layout::ViewArc;
use essay_plot::{
    chart::PolarChart, config::ConfigArc, plot::RadarOpt, 
};

use mind_ecs::PostTick;
use ui_graphics::ViewPlugin;

use super::ui_emoji::Emoji;

fn update_radar<M: 'static>(mut radar: ResMut<RadarView<M>>) {
    radar.update();
}

pub struct RadarView<M> {
    radar: RadarOpt,
    items: Vec<f32>,

    marker: PhantomData<fn(M)>,
}

impl<M> RadarView<M> {
    pub fn new(radar: RadarOpt) -> Self {
        Self {
            radar,

            items: Vec::new(),
            marker: PhantomData::default(),
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

impl<M: 'static> Clone for RadarView<M> {
    fn clone(&self) -> Self {
        Self { 
            radar: self.radar.clone(), 
            items: self.items.clone(), 
            marker: self.marker.clone()
         }
    }
}

//
// UiMotivePlugin
//

pub struct UiRadarPlugin<M> {
    items: Vec<Box<dyn PluginItem>>,

    polar: Option<PolarChart>,
    radar: Option<RadarView<M>>,

    marker: PhantomData<fn(M)>,
}

impl<M: 'static> UiRadarPlugin<M> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            polar: None,
            radar: None,

            marker: Default::default(),
        }
    }

    pub fn item<T>(
        mut self, 
        x: f32,
        emoji: Emoji,
        fun: impl Fn(&T) -> f32 + Send + Sync + 'static
    ) -> Self
    where T: Default + Send + Sync + 'static {
        self.items.push(Box::new(Item::<T, M> {
            x,
            emoji,
            fun: RefCell::new(Some(Box::new(fun))),
            marker: Default::default(),
        }));

        self
    }
}

impl<M: 'static> ViewPlugin for UiRadarPlugin<M> {
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

impl<M: 'static> Plugin for UiRadarPlugin<M> {
    fn build(&self, app: &mut App) {
        if let Some(radar) = &self.radar {
            app.insert_resource(radar.clone());

            app.system(PostTick, update_radar::<M>);
        }
    }
}

trait PluginItem {
    fn x(&self) -> f32;
    fn emoji(&self) -> Emoji;
    fn system(&self, id: usize, app: &mut App);
}

struct Item<T: Send + Sync + 'static, M> {
    x: f32,
    emoji: Emoji,
    fun: RefCell<Option<Box<dyn Fn(&T) -> f32 + Send + Sync + 'static>>>,

    marker: PhantomData<fn(M)>,
}

impl<T: Default + Send + Sync + 'static, M: 'static> PluginItem for Item<T, M> {
    fn x(&self) -> f32 {
        self.x
    }

    fn emoji(&self) -> Emoji {
        self.emoji.clone()
    }

    fn system(&self, id: usize, app: &mut App) {
        app.init_resource::<T>();

        let fun = self.fun.take().unwrap();

        app.system(Update, 
            move |mut radar: ResMut<RadarView<M>>, item: Res<T>| {
                let value = fun(item.get());
                radar.set_value(id, value);
            }
        );
    }
} 
