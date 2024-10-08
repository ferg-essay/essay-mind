use std::cell::RefCell;

use renderer::{Canvas, Drawable, Event, Renderer};
use essay_ecs::prelude::*;
use essay_graphics::layout::{Layout, View};
use essay_plot::{artist::{paths::{self, Unit}, ColorMap, ColorMaps, PathStyle}, chart::Data, prelude::*};
use ui_graphics::{ui_layout::UiLayoutPlugin, UiCanvas, UiCanvasPlugin};

use crate::subpallium::AttendValue;

#[derive(Component)]
pub struct UiAttention {
    view: View<AttentionDraw>,
}

impl UiAttention {
    fn new(view: View<AttentionDraw>) -> Self {
        Self {
            view,
        }
    }

    #[inline]
    fn set_value(&mut self, id: UiAttentionId, value: AttendValue) {
        self.view.write(|ui| {
            ui.attention[id.i()].value = value.value * value.attend;
            ui.attention[id.i()].attend = value.attend;
        });
    }
}

impl Coord for UiAttention {}


type UpdateBox<T> = Box<dyn Fn(&T) -> AttendValue + Sync + Send>;

struct AttentionDraw {
    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<Data>,

    attention: Vec<AttentionItem>,
    boxes: Vec<Path<Canvas>>,

    colors: ColorMap,
}

impl AttentionDraw {
    pub fn new() -> Self {
        Self {
            pos: Bounds::zero(),
            clip: Clip::None,
            attention: Vec::new(),
            bounds: Bounds::from([1., 1.]),
            boxes: Vec::new(),

            colors: ColorMap::from(ColorMaps::BlueOrange),
        }
    }

    fn add(&mut self, color: Color) -> UiAttentionId {
        let id = UiAttentionId(self.attention.len());

        self.attention.push(AttentionItem::new(color));

        self.bounds = Bounds::from([1., 1.]);

        id
    }


    fn set_pos(&mut self, set_pos: &Bounds<Canvas>) {
        let aspect = 1.;
        let size = (aspect * set_pos.width()).min(set_pos.height()) * 0.95;

        let dw = 0.5 * (set_pos.width() - size);
        let dh = 0.5 * (set_pos.height() - size);

        self.pos = Bounds::new(
            (set_pos.xmin() + dw, set_pos.ymin() + dh), 
            (set_pos.xmax() - dw, set_pos.ymax() - dh),
        );

        let mut boxes = Vec::new();
        let dx = 1. / (self.attention.len() as f32).max(1.);

        let len = self.attention.len();

        for i in 0..len {
            let x = i as f32 * dx;
            let y = 0.;

            let path: Path<Canvas> = paths::rect::<Unit>(
                (x, y), 
                (x + dx * 0.9, y + dx * 0.9)
            ).transform(&self.to_canvas());

            boxes.push(path);
        }

        self.boxes = boxes;
        self.clip = Clip::from(&self.pos);
    }

    fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }
}

impl Drawable for AttentionDraw {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        let mut style = PathStyle::new();
        style.line_width(1.);

        for (item, path) in self.attention.iter().zip(&self.boxes) {
            style.face_color(item.color.set_alpha(item.value));
            style.edge_color(self.colors.map(item.attend));

            renderer.draw_path(&path, &style)?;
        }

        Ok(())
    }

    fn event(&mut self, _renderer: &mut dyn Renderer, event: &Event) {
        if let Event::Resize(pos) = event {
            self.set_pos(pos);
        }
    }
}

struct AttentionItem {
    color: Color,
    value: f32,
    attend: f32,
}

impl AttentionItem {
    fn new(color: Color) -> Self {
        Self {
            color,
            value: 0.,
            attend: 0.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiAttentionId(usize);

impl UiAttentionId {
    fn i(&self) -> usize {
        self.0
    }
}

//
// Plugin configuration
//

pub struct UiAttentionPlugin {
    bounds: Bounds::<Layout>,
    colors: Vec<Color>,

    items: Vec<Box<dyn Item>>,
}

impl UiAttentionPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            colors: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn item<T>(
        mut self,
        item: impl IntoItem<T>
    ) -> Self {

        let item = IntoItem::into_item(item);

        self.items.push(item);

        self
    }

    pub fn colors(mut self, colors: impl Into<Colors>) -> Self {
        self.colors = colors.into().into();

        self
    }
}

impl Plugin for UiAttentionPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiCanvasPlugin>() {
            if ! app.contains_plugin::<UiLayoutPlugin>() {
                app.plugin(UiLayoutPlugin);
            }

            let mut ui_view = AttentionDraw::new();

            let colors = if self.colors.len() > 0 {
                self.colors.clone()
            } else {
                vec!(
                    Color::from("sky"),
                    Color::from("red"),
                    Color::from("beige"),
                    Color::from("purple"),
                    Color::from("olive"),
                )
            };

            for (i, item) in self.items.iter().enumerate() {
                let color = colors[i % colors.len()];

                let id = ui_view.add(color);

                item.add(id, app);
            }

            let view = app.resource_mut::<UiCanvas>().view(&self.bounds, ui_view);

            let ui_attention = UiAttention::new(view);

            app.insert_resource(ui_attention);
        }
    }
}

pub trait Item {
    fn add(&self, id: UiAttentionId, app: &mut App);
}

pub struct AttentionUpdates<T> {
    updates: Vec<(UiAttentionId, UpdateBox<T>)>,
}

impl<T> AttentionUpdates<T> {
    fn add(&mut self, id: UiAttentionId, fun: Option<UpdateBox<T>>) {
        self.updates.push((id, fun.unwrap()));
    }
}

struct ItemImpl<T> {
    update: RefCell<Option<UpdateBox<T>>>,
}

impl<T: Send + Sync + 'static> Item for ItemImpl<T> {
    fn add(&self, id: UiAttentionId, app: &mut App) {
        assert!(app.contains_resource::<T>());

        if ! app.contains_resource::<AttentionUpdates<T>>() {
            let updates: AttentionUpdates<T> = AttentionUpdates {
                updates: Vec::new(),
            };

            app.insert_resource(updates);

            app.system(
                PreUpdate, // TODO: PostTick?
                |updates: Res<AttentionUpdates<T>>, res: Res<T>, mut ui: ResMut<UiAttention>| {
                    for (id, fun) in &updates.updates {
                        let value = fun(res.get());

                        ui.set_value(*id, value);
                    }
            });
        }

        app.resource_mut::<AttentionUpdates<T>>().add(id, self.update.take());
    }
} 

pub trait IntoItem<T> {
    fn into_item(this: Self) -> Box<dyn Item>;
}

impl<T: Send + Sync + 'static, F> IntoItem<T> for F
where
    F: Fn(&T) -> AttendValue + Send + Sync + 'static
{
    fn into_item(this: Self) -> Box<dyn Item> {
        Box::new(ItemImpl {
            update: RefCell::new(Some(Box::new(this)))
        })
    }
}
