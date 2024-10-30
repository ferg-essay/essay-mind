use std::cell::RefCell;

use essay_ecs::{core::{Component, ResMut, Res}, app::{event::InEvent, Plugin, App, Update, PreUpdate}};
use essay_plot::api::{renderer::Canvas, Bounds, Clip, Coord, HorizAlign, Point, TextStyle, VertAlign};
use mind_ecs::PostTick;
use ui_graphics::{ui_layout::{BoxId, UiLayout, UiLayoutEvent, UiLayoutPlugin}, UiCanvas, UiCanvasPlugin};

use crate::{body::Body, world::{Wall, World}};


#[derive(Component)]
pub struct UiTable {
    id: BoxId,
    pos: Bounds<Canvas>,
    bounds: Bounds<UiTable>,
    data: Vec<UiTableItem>,
}

impl UiTable {
    pub fn new(id: BoxId) -> Self {
        Self {
            id,
            pos: Bounds::zero(),
            bounds: Bounds::zero(),
            data: Vec::new(),
        }
    }

    fn add(&mut self, label: &str) -> UiPeptideId {
        let id = UiPeptideId(self.data.len());

        self.data.push(UiTableItem::new(label));

        self.bounds = Bounds::from([self.data.len() as f32, 1.]);

        id
    }

    fn set_pos(&mut self, pos: &Bounds<Canvas>) {
        let margin = 10.;

        self.pos = Bounds::new(
            (pos.xmin() + margin, pos.ymin() + margin),
            (pos.xmax() - margin, pos.ymax() - margin),
        );
    }

    pub fn clip(&self) -> &Clip {
        &Clip::None
    }
}

impl Coord for UiTable {}

struct UiTableItem {
    label: String,
    hit: usize,
    total: usize,
}

impl UiTableItem {
    fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
            hit: 0,
            total: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiPeptideId(usize);

type UpdateBox = Box<dyn Fn(&World<Wall>, &Body) -> f32 + Sync + Send>;

pub fn ui_peptide_resize(
    mut ui_peptide: ResMut<UiTable>, 
    ui_layout: Res<UiLayout>,
    mut read: InEvent<UiLayoutEvent>
) {
    for _ in read.iter() {
        let id = ui_peptide.id;
        ui_peptide.set_pos(ui_layout.get_box(id));
    }
}

pub fn ui_peptide_draw(
    ui_table: ResMut<UiTable>, 
    mut ui: ResMut<UiCanvas>
) {
    let pos = &ui_table.pos;

    let size = 10.;
    let dy = 2. * 1.5 * size;
    let mut y = pos.y1();

    let mut text_style = TextStyle::new();
    text_style.size(size);
    text_style.halign(HorizAlign::Left);
    text_style.valign(VertAlign::Top);

    for item in &ui_table.data {
        let value = item.hit as f32 / item.total.max(1) as f32;

        ui.draw_text(
            (pos.x0(), y),
                &format!("{} = {:.3}", item.label, value),
            &text_style
        );

        y -= dy;
    }
}

pub struct UiTablePlugin {
    bounds: Bounds::<UiLayout>,

    items: Vec<(String, Box<dyn Item>)>,
}

impl UiTablePlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            items: Vec::new(),
        }
    }

    pub fn p_item(
        mut self,
        label: &str,
        item: impl IntoItem
    ) -> Self {
        let item = IntoItem::into_item(item);

        self.items.push((String::from(label), item));

        self
    }
}

impl Plugin for UiTablePlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiCanvasPlugin>() {
            if ! app.contains_plugin::<UiLayoutPlugin>() {
                app.plugin(UiLayoutPlugin);
            }

            let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());

            let mut ui_peptide = UiTable::new(box_id);

            for (label, item) in self.items.iter() {
                let id = ui_peptide.add(label);

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

pub struct PeptideUpdates {
    updates: Vec<(UiPeptideId, UpdateBox)>,
}

impl PeptideUpdates {
    fn add(&mut self, id: UiPeptideId, fun: Option<UpdateBox>) {
        self.updates.push((id, fun.unwrap()));
    }
}

struct ItemImpl {
    update: RefCell<Option<UpdateBox>>,
}

impl Item for ItemImpl {
    fn add(&self, id: UiPeptideId, app: &mut App) {
        if ! app.contains_resource::<PeptideUpdates>() {
            let updates: PeptideUpdates = PeptideUpdates {
                updates: Vec::new(),
            };

            app.insert_resource(updates);

            app.system(
                PostTick,
                |updates: Res<PeptideUpdates>, 
                world: Res<World<Wall>>,
                body: Res<Body>,
                mut ui: ResMut<UiTable>| {
                    for (id, fun) in &updates.updates {
                        let value = fun(world.get(), body.get());

                        if value >= 0. {
                            let is_hit = value > 0.5;

                            ui.data[id.0].hit += if is_hit { 1 } else { 0 };
                            ui.data[id.0].total += 1;
                        }
                    }
            });
        }

        app.resource_mut::<PeptideUpdates>().add(id, self.update.take());
    }
} 

pub trait IntoItem {
    fn into_item(this: Self) -> Box<dyn Item>;
}

impl<F> IntoItem for F
where
    F: Fn(&World<Wall>, &Body) -> f32 + Send + Sync + 'static
{
    fn into_item(this: Self) -> Box<dyn Item> {
        Box::new(ItemImpl {
            update: RefCell::new(Some(Box::new(this)))
        })
    }
}
