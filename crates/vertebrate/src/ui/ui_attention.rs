use std::cell::RefCell;

use renderer::{Canvas, Drawable, Renderer};
use essay_ecs::prelude::*;
use essay_graphics::layout::{View, ViewArc};
use essay_plot::{artist::{paths::{self, Unit}, ColorMap, ColorMaps, PathStyle}, chart::Data, prelude::*};
use ui_graphics::ui_canvas::ViewPlugin;

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

pub struct AttentionDraw {
    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<Data>,

    attention: Vec<AttentionItem>,
    boxes: Vec<Path<Canvas>>,

    colors: ColorMap,

    canvas_pos: Bounds<Canvas>,
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

            canvas_pos: Bounds::none(),
        }
    }

    fn add(&mut self, color: Color) -> UiAttentionId {
        let id = UiAttentionId(self.attention.len());

        self.attention.push(AttentionItem::new(color));

        self.bounds = Bounds::from([1., 1.]);

        id
    }


    fn set_pos(&mut self, set_pos: &Bounds<Canvas>) {
        if &self.canvas_pos == set_pos {
            return;
        }
        self.canvas_pos = set_pos.clone();

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
        self.set_pos(renderer.pos());

        let mut style = PathStyle::new();
        style.line_width(1.);

        for (item, path) in self.attention.iter().zip(&self.boxes) {
            style.face_color(item.color.set_alpha(item.value));
            style.edge_color(self.colors.map(item.attend));

            renderer.draw_path(&path, &style)?;
        }

        Ok(())
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
    colors: Vec<Color>,

    items: Vec<Box<dyn Item>>,
    view: Option<View<AttentionDraw>>,
}

impl UiAttentionPlugin {
    pub fn new() -> Self {
        Self {
            colors: Vec::new(),
            items: Vec::new(),
            view: None,
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

impl ViewPlugin for UiAttentionPlugin {
    fn view(&mut self, app: &mut App) -> Option<&ViewArc> {
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

        self.view = Some(View::from(ui_view));

        self.view.as_ref().map(|v| v.arc())
    }
}

impl Plugin for UiAttentionPlugin {
    fn build(&self, app: &mut App) {
        if let Some(view) = &self.view {
            let ui_attention = UiAttention::new(view.clone());

            app.insert_resource(ui_attention);
        }
    }
}

pub trait Item {
    fn add(&self, id: UiAttentionId, app: &mut App);
}

struct ItemImpl<T> {
    update: RefCell<Option<UpdateBox<T>>>,
}

impl<T: Send + Sync + 'static> Item for ItemImpl<T> {
    fn add(&self, id: UiAttentionId, app: &mut App) {
        assert!(app.contains_resource::<T>());

        if let Some(fun) = self.update.take() {
            app.system(
                PreUpdate, // TODO: PostTick?
                move |res: Res<T>, mut ui: ResMut<UiAttention>| {
                    let value = fun(res.get());

                    ui.set_value(id, value);
            });
        }
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
