use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::{paths::{self, Unit}, PathStyle}};
use ui_graphics::{ui_layout::{UiLayout, UiLayoutEvent, BoxId, UiLayoutPlugin}, UiCanvas, UiCanvasPlugin};

use crate::mid_peptide_canal::{MidPeptideCanalPlugin, PeptideCanal, PeptideId, Peptide};

#[derive(Component)]
pub struct UiPeptide {
    id: BoxId,
    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<UiPeptide>,
    peptides: Vec<UiPeptideItem>,
}

impl UiPeptide {
    pub fn new(id: BoxId) -> Self {
        Self {
            id,
            pos: Bounds::zero(),
            clip: Clip::None,
            peptides: Vec::new(),
            bounds: Bounds::from([1., 1.]),
        }
    }

    pub fn peptide(&mut self, id: PeptideId, label: &str, color: Color) -> &mut Self {
        self.peptides.push(UiPeptideItem::new(id, label, color));

        self.bounds = Bounds::from([self.peptides.len() as f32, 1.]);

        self
    }

    pub fn set_pos(&mut self, set_pos: &Bounds<Canvas>) {
        /*
        let aspect = self.bounds.width() / self.bounds.height();

        let (c_width, c_height) = if aspect * set_pos.height() <= set_pos.width() {
            (aspect * set_pos.height(), set_pos.height())
        } else {
            (set_pos.width(), set_pos.width() / aspect)
        };

        let pos = Bounds::<Canvas>::new(
            Point(set_pos.xmin(), set_pos.ymin()),
            Point(set_pos.xmin() + c_width, set_pos.ymin() + c_height),
        );
        */

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

impl Coord for UiPeptide {}


struct UiPeptideItem {
    id: PeptideId,
    label: String,
    color: Color,
}

impl UiPeptideItem {
    fn new(id: PeptideId, label: &str, color: Color) -> Self {
        Self {
            id,
            label: String::from(label),
            color,
        }
    }
}


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
    peptides: Res<PeptideCanal>, 
    ui_peptide: ResMut<UiPeptide>, 
    mut ui: ResMut<UiCanvas>
) {
    let to_canvas = &ui_peptide.to_canvas();
    let y_min = 0.1;
    let x_margin = 0.2;
    let width = ui_peptide.peptides.len().max(1);

    let mut style = PathStyle::new();

    let mut text_style = TextStyle::new();
    text_style.valign(VertAlign::Top);

    for (i, item) in ui_peptide.peptides.iter().enumerate() {
        let value = peptides[item.id];
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

pub struct UiPeptidePlugin {
    bounds: Bounds::<UiLayout>,
    peptides: Vec<UiPeptidePluginItem>,
}

impl UiPeptidePlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            peptides: Vec::new(),
        }
    }

    pub fn peptide(mut self, peptide: impl Peptide, label: &str) -> Self {
        self.peptides.push(UiPeptidePluginItem::new(peptide, label));
        
        self
    }
}

impl Plugin for UiPeptidePlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiCanvasPlugin>() {
            assert!(app.contains_plugin::<MidPeptideCanalPlugin>());

            if ! app.contains_plugin::<UiLayoutPlugin>() {
                app.plugin(UiLayoutPlugin);
            }

            let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());

            let colors = [
                Color::from("beige"),
                Color::from("sky"),
                Color::from("green"),
            ];

            let peptides = app.resource_mut::<PeptideCanal>();

            let mut ui_peptide = UiPeptide::new(box_id);

            for (i, peptide) in self.peptides.iter().enumerate() {
                if let Some(item) = peptides.get_peptide(peptide.peptide.as_ref()) {
                    let color = colors[i % colors.len()];
                    ui_peptide.peptide(item.id(), &peptide.label, color);
                }
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
