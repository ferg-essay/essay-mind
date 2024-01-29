use std::cell::RefCell;

use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::{paths::{self, Unit}, PathStyle, ColorMap, ColorMaps}};
use ui_graphics::{ui_layout::{UiLayout, UiLayoutEvent, BoxId, UiLayoutPlugin}, UiCanvas, UiCanvasPlugin};

use crate::{core_motive::mid_peptides::CorePeptidesPlugin, pallidum::basal_forebrain::AttendValue};

#[derive(Component)]
pub struct UiAttention {
    id: BoxId,
    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<UiAttention>,

    attention: Vec<UiAttentionItem>,
    boxes: Vec<Path<Canvas>>,

    colors: ColorMap,
    // peptide_update: UiPeptideUpdate<MidPeptides>,
}

impl UiAttention {
    pub fn new(id: BoxId) -> Self {
        Self {
            id,
            pos: Bounds::zero(),
            clip: Clip::None,
            attention: Vec::new(),
            bounds: Bounds::from([1., 1.]),
            boxes: Vec::new(),

            // colors: ColorMap::from(ColorMaps::WhiteBlack),
            colors: ColorMap::from(ColorMaps::BlueOrange),
            // peptide_update: UiPeptideUpdate::<MidPeptides>::new(),
        }
    }

    fn add(&mut self, color: Color) -> UiAttentionId {
        let id = UiAttentionId(self.attention.len());

        self.attention.push(UiAttentionItem::new(color));

        //self.bounds = Bounds::from([self.attention.len() as f32, 1.]);
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

    pub fn clip(&self) -> &Clip {
        &self.clip
    }
}

impl Coord for UiAttention {}


struct UiAttentionItem {
    color: Color,
    value: f32,
    attend: f32,
}

impl UiAttentionItem {
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

type UpdateBox<T> = Box<dyn Fn(&T) -> AttendValue + Sync + Send>;

fn ui_attention_resize(
    mut ui_attention: ResMut<UiAttention>, 
    ui_layout: Res<UiLayout>,
    mut read: InEvent<UiLayoutEvent>
) {
    for _ in read.iter() {
        let id = ui_attention.id;
        ui_attention.set_pos(ui_layout.get_box(id));
    }
}

fn ui_attention_draw(
    ui_attention: ResMut<UiAttention>, 
    mut ui: ResMut<UiCanvas>
) {
    if let Some(mut ui_render) = ui.renderer(Clip::None) {
        let mut style = PathStyle::new();
        style.line_width(1.);

        for (item, path) in ui_attention.attention.iter().zip(&ui_attention.boxes) {
            style.face_color(item.color.set_alpha(item.value));
            style.edge_color(ui_attention.colors.map(item.attend));

            ui_render.draw_path(&path, &style);
        }
    }
}

pub struct UiAttentionPlugin {
    bounds: Bounds::<UiLayout>,
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
            assert!(app.contains_plugin::<CorePeptidesPlugin>());

            if ! app.contains_plugin::<UiLayoutPlugin>() {
                app.plugin(UiLayoutPlugin);
            }

            let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());

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

            let mut ui_peptide = UiAttention::new(box_id);

            for (i, item) in self.items.iter().enumerate() {
                let color = colors[i % colors.len()];

                let id = ui_peptide.add(color);

                item.add(id, app);
            }

            app.insert_resource(ui_peptide);

            app.system(Update, ui_attention_draw);
            app.system(PreUpdate, ui_attention_resize);
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
                Update,
                |updates: Res<AttentionUpdates<T>>, res: Res<T>, mut ui: ResMut<UiAttention>| {
                    for (id, fun) in &updates.updates {
                        let value = fun(res.get());

                        ui.attention[id.i()].value = value.value * value.attend;
                        ui.attention[id.i()].attend = value.attend;
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
