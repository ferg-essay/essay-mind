use std::cell::RefCell;

use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::{paths::{self, Unit}, PathStyle}};
use renderer::Canvas;
use ui_graphics::{
    ui_layout::{UiLayout, UiLayoutEvent, BoxId, UiLayoutPlugin}, 
    UiCanvas, UiCanvasPlugin
};

#[derive(Component)]
pub struct UiPeptide {
    id: BoxId,
    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<UiPeptide>,
    peptides: Vec<UiPeptideItem>,

    // peptide_update: UiPeptideUpdate<MidPeptides>,
}

impl UiPeptide {
    pub fn new(id: BoxId) -> Self {
        Self {
            id,
            pos: Bounds::zero(),
            clip: Clip::None,
            peptides: Vec::new(),
            bounds: Bounds::from([1., 1.]),

            // peptide_update: UiPeptideUpdate::<MidPeptides>::new(),
        }
    }

    fn add(&mut self, label: &str, color: Color) -> UiPeptideId {
        let id = UiPeptideId(self.peptides.len());

        self.peptides.push(UiPeptideItem::new(label, color));

        self.bounds = Bounds::from([self.peptides.len() as f32, 1.]);

        id
    }

    fn set_pos(&mut self, set_pos: &Bounds<Canvas>) {
        self.pos = Bounds::new(
            (set_pos.xmin(), set_pos.ymin()), 
            (set_pos.xmin() + set_pos.width() * 0.95,  
            set_pos.ymin() + set_pos.height() * 0.95),  
        );
        self.clip = Clip::from(&self.pos);
    }

    fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }

    pub fn clip(&self) -> &Clip {
        &self.clip
    }
}

impl Coord for UiPeptide {}


struct UiPeptideItem {
    label: String,
    color: Color,
    value: f32,
}

impl UiPeptideItem {
    fn new(label: &str, color: Color) -> Self {
        Self {
            label: String::from(label),
            color,
            value: 0.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiPeptideId(usize);

impl UiPeptideId {
    fn i(&self) -> usize {
        self.0
    }
}

type UpdateBox<T> = Box<dyn Fn(&T) -> f32 + Sync + Send>;

pub fn ui_peptide_resize(
    mut ui_peptide: ResMut<UiPeptide>, 
    ui_layout: Res<UiLayout>,
    mut read: InEvent<UiLayoutEvent>
) {
    for _ in read.iter() {
        let id = ui_peptide.id;
        ui_peptide.set_pos(ui_layout.get_box(id));
    }
}

pub fn ui_peptide_draw(
    ui_peptide: ResMut<UiPeptide>, 
    mut ui: ResMut<UiCanvas>
) {
    if let Some(mut ui_render) = ui.renderer() {
        let to_canvas = &ui_peptide.to_canvas();
        let y_min = 0.1;
        let x_margin = 0.2;
        let width = ui_peptide.peptides.len().max(1);

        let y_mid = y_min + 0.5 * (1. - y_min);

        let mid = paths::rect::<Unit>(
            [0., y_mid], 
            [1. * width as f32, y_mid + 0.001]
        ).transform(&ui_peptide.to_canvas());
    
        let mut style = PathStyle::new();
        style.color(0xe0e0e0);
        ui_render.draw_path(&mid, &style);

        let mut text_style = TextStyle::new();
        text_style.valign(VertAlign::Top);
        let text_path_style = PathStyle::new();

        for (i, item) in ui_peptide.peptides.iter().enumerate() {
            let value = item.value;
            let y = (1. - y_min) * value + y_min;
            let x = i as f32;

            let line = paths::rect::<Unit>(
                [x + x_margin, y_min], 
                [x + 1.0 - x_margin, y]
            ).transform(to_canvas);

            style.color(item.color);

            ui_render.draw_path(&line, &style);

            ui_render.draw_text(
                to_canvas.transform_point(Point(x + 0.5, y_min)), 
                &item.label,
                &text_path_style,
                &text_style
            );
        }

        let base = paths::rect::<Unit>(
            [0., y_min], 
            [1. * width as f32, y_min + 0.001]
        ).transform(&ui_peptide.to_canvas());
    
        let mut style = PathStyle::new();
        style.color("black");
        ui_render.draw_path(&base, &style);
    }
}

pub struct UiPeptidePlugin {
    bounds: Bounds::<UiLayout>,
    colors: Vec<Color>,

    items: Vec<(String, Box<dyn Item>)>,
}

impl UiPeptidePlugin {
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
        label: &str,
        item: impl IntoItem<T>
    ) -> Self {

        let item = IntoItem::into_item(item);

        self.items.push((String::from(label), item));

        self
    }

    pub fn colors(mut self, colors: impl Into<Colors>) -> Self {
        self.colors = colors.into().into();

        self
    }
}

impl Plugin for UiPeptidePlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiCanvasPlugin>() {
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

            let mut ui_peptide = UiPeptide::new(box_id);

            for (i, (label, item)) in self.items.iter().enumerate() {
                let color = colors[i % colors.len()];

                let id = ui_peptide.add(label, color);

                item.add(id, app);
            }

            app.insert_resource(ui_peptide);

            app.system(Update, ui_peptide_draw);
            app.system(PreUpdate, ui_peptide_resize);
        }
    }
}

pub trait Item {
    fn add(&self, id: UiPeptideId, app: &mut App);
}

pub struct PeptideUpdates<T> {
    updates: Vec<(UiPeptideId, UpdateBox<T>)>,
}

impl<T> PeptideUpdates<T> {
    fn add(&mut self, id: UiPeptideId, fun: Option<UpdateBox<T>>) {
        self.updates.push((id, fun.unwrap()));
    }
}

struct ItemImpl<T> {
    update: RefCell<Option<UpdateBox<T>>>,
}

impl<T: Send + Sync + 'static> Item for ItemImpl<T> {
    fn add(&self, id: UiPeptideId, app: &mut App) {
        assert!(app.contains_resource::<T>());

        if ! app.contains_resource::<PeptideUpdates<T>>() {
            let updates: PeptideUpdates<T> = PeptideUpdates {
                updates: Vec::new(),
            };

            app.insert_resource(updates);

            app.system(
                Update,
                |updates: Res<PeptideUpdates<T>>, res: Res<T>, mut ui: ResMut<UiPeptide>| {
                    for (id, fun) in &updates.updates {
                        ui.peptides[id.i()].value = fun(res.get());
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
