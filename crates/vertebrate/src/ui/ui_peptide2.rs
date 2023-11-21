use std::{marker::PhantomData, any::{TypeId, type_name}, cell::{Ref, RefCell}};

use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::{paths::{self, Unit}, PathStyle}};
use ui_graphics::{ui_layout::{UiLayout, UiLayoutEvent, BoxId, UiLayoutPlugin}, UiCanvas, UiCanvasPlugin};

use crate::mid_peptides::{MidPeptidesPlugin, MidPeptides, PeptideId, Peptide};

#[derive(Component)]
pub struct UiPeptide2 {
    id: BoxId,
    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<UiPeptide2>,
    peptides: Vec<UiPeptideItem>,

    // peptide_update: UiPeptideUpdate<MidPeptides>,
}

impl UiPeptide2 {
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

    pub fn add(&mut self, label: &str, color: Color) -> UiPeptideId {
        let id = UiPeptideId(self.peptides.len());

        self.peptides.push(UiPeptideItem::new(label, color));

        self.bounds = Bounds::from([self.peptides.len() as f32, 1.]);

        id
    }

    pub fn set_pos(&mut self, set_pos: &Bounds<Canvas>) {
        self.pos = set_pos.clone();
        self.clip = Clip::from(&self.pos);
    }

    pub fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }

    pub fn clip(&self) -> &Clip {
        &self.clip
    }
}

impl Coord for UiPeptide2 {}


struct UiPeptideItem {
    label: String,
    color: Color,
    value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiPeptideId(usize);

impl UiPeptideId {
    fn i(&self) -> usize {
        self.0
    }
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

type UpdateBox<T> = Box<dyn Fn(&T) -> f32 + Sync + Send>;

trait GetValue<T> {
    fn value(&self, data: &T) -> f32;
}

struct UpdateItem<T> {
    id: UiPeptideId,

    update: UpdateBox<T>,

    marker: PhantomData<T>,
}

pub struct UiPeptideUpdate<T> {
    items: Vec<UpdateItem<T>>,
}

impl<T> UiPeptideUpdate<T> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }
}

pub fn ui_peptide_update(
    mut ui: ResMut<UiPeptide2>, 
    updaters: Res<UiPeptideUpdate<MidPeptides>>, 
    peptides: Res<MidPeptides>,
) {
    for item in &updaters.items {
        let value = (item.update)(peptides.get());

        ui.peptides[item.id.i()].value = value;
    }
}

pub fn ui_peptide_resize(
    mut ui_peptide: ResMut<UiPeptide2>, 
    ui_layout: Res<UiLayout>,
    mut read: InEvent<UiLayoutEvent>
) {
    for _ in read.iter() {
        let id = ui_peptide.id;
        ui_peptide.set_pos(ui_layout.get_box(id));
    }
}

pub fn ui_peptide_draw(
    ui_peptide: ResMut<UiPeptide2>, 
    mut ui: ResMut<UiCanvas>
) {
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
    ui.draw_path(&mid, &style);

    let mut text_style = TextStyle::new();
    text_style.valign(VertAlign::Top);

    for (i, item) in ui_peptide.peptides.iter().enumerate() {
        let value = item.value;
        let y = (1. - y_min) * value + y_min;
        let x = i as f32;

        let line = paths::rect::<Unit>(
            [x + x_margin, y_min], 
            [x + 1.0 - x_margin, y]
        ).transform(to_canvas);

        style.color(item.color);

        ui.draw_path(&line, &style);

        ui.draw_text(
            to_canvas.transform_point(Point(x + 0.5, y_min)), 
            &item.label,
            &text_style
        );
    }

    let base = paths::rect::<Unit>(
        [0., y_min], 
        [1. * width as f32, y_min + 0.001]
    ).transform(&ui_peptide.to_canvas());
    
    let mut style = PathStyle::new();
    style.color("black");
    ui.draw_path(&base, &style);
}

pub struct UiPeptide2Plugin {
    bounds: Bounds::<UiLayout>,
    peptides: Vec<UiPeptidePluginItem>,
    colors: Vec<Color>,

    items: Vec<(String, Box<dyn Item>)>,
}

impl UiPeptide2Plugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            peptides: Vec::new(),
            colors: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn peptide(mut self, peptide: impl Peptide, label: &str) -> Self {
        self.peptides.push(UiPeptidePluginItem::new(peptide, label));
        
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

    pub fn colors(mut self, colors: impl Into<Colors>) -> Self {
        self.colors = colors.into().into();

        self
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
        if let Some(update) = fun {
            self.updates.push((id, update));
        } else {
            println!("None {:?}", id);
        }
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
                |updates: Res<PeptideUpdates<T>>, res: Res<T>, mut ui: ResMut<UiPeptide2>| {
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

impl Plugin for UiPeptide2Plugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiCanvasPlugin>() {
            assert!(app.contains_plugin::<MidPeptidesPlugin>());

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

            // let peptides = app.resource_mut::<MidPeptides>();

            let mut ui_peptide = UiPeptide2::new(box_id);

            /*
            for (i, peptide) in self.peptides.iter().enumerate() {
                if let Some(item) = peptides.get_peptide(peptide.peptide.as_ref()) {
                    let color = colors[i % colors.len()];
                    //ui_peptide.peptide(item.id(), &peptide.label, color);
                }
            }
            */

            for (i, (label, item)) in self.items.iter().enumerate() {
                let color = colors[i % colors.len()];

                let id = ui_peptide.add(label, color);

                item.add(id, app);
            }

            app.insert_resource(ui_peptide);

            //app.phase(Update, (DrawWorld, DrawItem, DrawAgent).chain());
            //app.system(Update, draw_world.phase(DrawWorld));
            app.system(Update, ui_peptide_draw);
            app.system(PreUpdate, ui_peptide_resize);

            // app.system(Startup, spawn_ui_world);
        }
    }
}


struct UiPeptidePluginItem {
    peptide: Box<dyn Peptide>,
    label: String,
}

impl UiPeptidePluginItem {
    fn new(peptide: impl Peptide, label: &str) -> Self {
        Self {
            peptide: peptide.box_clone(),
            label: String::from(label),
        }
    }
}
