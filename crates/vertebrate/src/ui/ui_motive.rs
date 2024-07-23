use std::cell::RefCell;

use driver::{Drawable, Renderer};
use essay_ecs::prelude::*;
use essay_graphics::layout::{Layout, View};
use essay_plot::{
    prelude::*, 
    artist::{paths::Unit, PathStyle, ColorMaps, ColorMap}
};

use ui_graphics::UiCanvas;
use crate::ui::ui_world_map::UiWorldPlugin;

use super::ui_emoji::{Emoji, SymbolDraw};

struct MotiveView {
    size: f32,
    items: Vec<UiMotiveItem>,

    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<Unit>,

    emoji: Option<FontTypeId>,
}

impl MotiveView {
    pub fn new() -> Self {
        Self {
            size: 16.,
            items: Vec::new(),

            pos: Bounds::zero(),
            clip: Clip::None,
            bounds: Bounds::from([1., 1.]),

            emoji: None,
        }
    }

    fn push(&mut self, item: UiMotiveItem) -> usize {
        let id = self.items.len();

        let pos = item.pos;

        self.items.push(item);

        let width = self.bounds.width();
        let height = self.bounds.height();

        self.bounds = Bounds::from([
            width.max(pos.x() + 0.5),
            height.max(pos.y() + 0.5),
        ]);

        id
    }

    fn set_pos(&mut self, pos: &Bounds<Canvas>) {
        self.pos = Bounds::from([
            pos.xmin() + 0.05 * pos.width(),
            pos.ymin() + 0.05 * pos.height(),
            pos.xmax() - 0.05 * pos.width(),
            pos.ymax() - 0.05 * pos.height()
        ]);

        self.clip = Clip::from(&self.pos);
    }

    fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }
}

impl Drawable for MotiveView {
    fn draw(&mut self, renderer: &mut dyn Renderer, _pos: &Bounds<Canvas>) {
        if self.emoji.is_none() {
            let emoji_path = "/Users/ferg/wsp/essay-mind/assets/font/NotoEmoji-Bold.ttf";

            let mut style = FontStyle::new();
            style.family(emoji_path);
            
            self.emoji = Some(renderer.font(&style).unwrap());
        }

        let color_map: ColorMap = ColorMaps::WhiteBlue.into();

        let mut style = PathStyle::new();

        let mut text_style = TextStyle::new();
        text_style.valign(VertAlign::Center);
        text_style.halign(HorizAlign::Center);
        text_style.size(self.size);
        text_style.font(self.emoji.unwrap());

        let height = self.bounds.height();

        for item in self.items.iter() {
            let pos = Point(item.pos.0, height - item.pos.1);
            let pos = self.to_canvas().transform_point(pos);

            let value = item.value.clamp(0.05, 1.);

            if let Some(color_map) = &item.colormap {
                style.color(color_map.map(value));
            } else {
                style.color(color_map.map(value));
            }

            item.emoji.draw(renderer, pos, &style, &mut text_style);
        }
    }

    fn event(&mut self, _renderer: &mut dyn driver::Renderer, event: &CanvasEvent) {
        if let CanvasEvent::Resize(pos) = event {
            self.set_pos(pos);
        }
    }
}


struct UiMotiveItem {
    pos: Point,

    emoji: Emoji,
    value: f32,
    colormap: Option<ColorMap>,
}

impl UiMotiveItem {
    fn new(item: &Box<dyn PluginItem>) -> Self {
        Self {
            pos: item.pos(),
            emoji: item.emoji(),
            value: 0.,
            colormap: item.colormap(),
        }
    }
}

//
// UiMotivePlugin
//

pub struct UiMotivePlugin {
    bounds: Bounds::<Layout>,
    size: f32,
    items: Vec<Box<dyn PluginItem>>,

    x: usize,
    y: usize,
}

impl UiMotivePlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            size: 12.,
            items: Vec::new(),
            x: 0,
            y: 0,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;

        self
    }

    pub fn row(mut self) -> Self {
        self.y += 1;
        self.x = 0;

        self
    }

    pub fn item<T>(
        mut self, 
        emoji: Emoji,
        fun: impl Fn(&T) -> f32 + Send + Sync + 'static
    ) -> Self
    where T: Default + Send + Sync + 'static {
        self.items.push(Box::new(Item {
            pos: Point(self.x as f32 + 0.5, self.y as f32 + 0.5),
            emoji,
            colormap: None,
            fun: RefCell::new(Some(Box::new(fun)))
        }));

        self.x += 1;

        self
    }

    pub fn colormap(
        mut self,
        colormap: impl Into<ColorMap>,
    ) -> Self {
        assert!(self.items.len() > 0);

        let tail = self.items.len() - 1;
        self.items[tail].set_colormap(colormap.into());

        self

    }
}

trait PluginItem {
    fn pos(&self) -> Point;
    fn emoji(&self) -> Emoji;
    fn set_colormap(&mut self, colormap: ColorMap);
    fn colormap(&self) -> Option<ColorMap>;
    fn system(&self, id: usize, app: &mut App);
}

struct Item<T: Send + Sync + 'static> {
    pos: Point,
    emoji: Emoji,
    colormap: Option<ColorMap>,
    fun: RefCell<Option<Box<dyn Fn(&T) -> f32 + Send + Sync + 'static>>>,
}

impl<T: Send + Sync + 'static> Item<T> {
    
}

impl<T: Default + Send + Sync + 'static> PluginItem for Item<T> {
    fn pos(&self) -> Point {
        self.pos.clone()
    }

    fn emoji(&self) -> Emoji {
        self.emoji.clone()
    }

    fn set_colormap(&mut self, colormap: ColorMap) {
        self.colormap = Some(colormap);
    }

    fn colormap(&self) -> Option<ColorMap> {
        self.colormap.clone()
    }

    fn system(&self, id: usize, app: &mut App) {
        app.init_resource::<T>();

        let fun = self.fun.take().unwrap();

        app.system(PostUpdate, 
            move |mut view: ResMut<View<MotiveView>>, item: Res<T>| {
                let value = fun(item.get());
                view.write(|v| {
                    v.items[id].value = value; 
                });
            }
        );
    }
} 

impl Plugin for UiMotivePlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            let mut motives = MotiveView::new();
            motives.size = self.size;

            for item in &self.items {
                let id = motives.push(UiMotiveItem::new(item));

                item.system(id, app);
            }

            let view = app.resource_mut::<UiCanvas>().view(&self.bounds, motives);

            app.insert_resource(view);
        }
    }
}
