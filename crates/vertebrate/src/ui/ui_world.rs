use essay_ecs::prelude::*;
use essay_plot::{prelude::*, artist::paths};
use essay_tensor::Tensor;
use ui_graphics::{ui_layout::{UiLayout, UiLayoutEvent, BoxId, UiLayoutPlugin}, UiCanvas, UiCanvasPlugin};

use crate::world::{World, OdorType, WorldPlugin};

use crate::world::WorldCell;

#[derive(Component)]
pub struct UiWorld {
    id: BoxId,
    pos: Bounds<Canvas>,
    clip: Clip,
    to_canvas: Affine2d,
    bounds: Bounds<UiWorld>,
    width: usize,
    height: usize,
    image: Option<ImageId>,
}

impl UiWorld {
    pub fn new(id: BoxId, width: usize, height: usize) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || WorldCell::Empty);

        Self {
            id,
            pos: Bounds::zero(),
            clip: Clip::None,
            to_canvas: Affine2d::eye(),
            width,
            height,
            bounds: Bounds::from([width as f32, height as f32]),
            image: None,
        }
    }

    pub fn _extent(&self) -> (f32, f32) {
        (self.bounds.xmax(), self.bounds.ymax())
    }

    pub fn set_pos(&mut self, pos: &Bounds<Canvas>) {
        let aspect = self.bounds.width() / self.bounds.height();

        // force bounds to match aspect ratio
        let (c_width, c_height) = if aspect * pos.height() <= pos.width() {
            (aspect * pos.height(), pos.height())
        } else {
            (pos.width(), pos.width() / aspect)
        };

        // center the box
        let xmin = pos.xmin() + 0.5 * (pos.width() - c_width);
        let ymin = pos.ymin() + 0.5 * (pos.height() - c_height);

        let xmin = xmin.max(10.);
        let ymin = ymin.max(10.);

        //let xmin = pos.xmin();
        //let ymin = pos.ymin();

        let c_width = c_width - xmin - pos.xmin();
        let c_height = c_height - xmin - pos.xmin();

        let pos = Bounds::<Canvas>::new(
            Point(xmin, ymin),
            Point(xmin + c_width, ymin + c_height),
        );

        self.pos = pos;
        self.clip = Clip::from(&self.pos);
        self.to_canvas = self.bounds.affine_to(&self.pos);
    }

    pub fn to_canvas(&self) -> Affine2d {
        self.to_canvas.clone()
    }

    pub fn to_canvas_scale(&self) -> Affine2d {
        Affine2d::eye().scale(
            self.pos.width() / self.bounds.width(), 
            self.pos.height() / self.bounds.height(),
        )
    }

    pub fn clip(&self) -> &Clip {
        &self.clip
    }
}

impl Coord for UiWorld {}


pub fn world_resize(
    mut ui_world: ResMut<UiWorld>, 
    ui_layout: Res<UiLayout>,
    mut read: InEvent<UiLayoutEvent>
) {
    for _ in read.iter() {
        let id = ui_world.id;
        ui_world.set_pos(ui_layout.get_box(id));
    }
}

pub fn draw_world(
    world: Res<World>, 
    mut ui_world: ResMut<UiWorld>, 
    mut ui_canvas: ResMut<UiCanvas>
) {
    if ui_world.image.is_none() {
        let mut vec = Vec::<[u8; 4]>::new();

        for j in 0..ui_world.height {
            for i in 0..ui_world.width {
                vec.push(Color::from(&world[(i, j)]).to_rgba_vec());
            }
        }

        let colors = Tensor::from(&vec);
        let colors = colors.reshape([ui_world.height as usize, ui_world.width as usize, 4]);

        ui_world.image = ui_canvas.create_image(colors);
    }

    // TODO: cache texture when unmodified

    if let Some(mut ui) = ui_canvas.renderer(ui_world.clip().clone()) {
        let circle = paths::circle().transform(&ui_world.to_canvas_scale());
        let mut xy : Vec<[f32; 2]> = Vec::new();
        let mut sizes : Vec<[f32; 2]> = Vec::new();
        let mut colors : Vec<Color> = Vec::new();

        for odor in world.odors() {
            xy.push([odor.x(), odor.y()]);
            sizes.push([odor.r(), odor.r()]);

            colors.push(Color::from(odor.odor()).set_alpha(0.2));
        }

        let xy = ui_world.to_canvas().transform(&Tensor::from(xy));

        if xy.len() > 0 {
            ui.draw_markers(&circle, xy, sizes, &colors); // , &ui_world.clip());
        }

        if let Some(image) = &ui_world.image {
            ui.draw_image(&ui_world.pos, image.clone());
        }
    }
}

impl From<&WorldCell> for Color {
    fn from(value: &WorldCell) -> Self {
        match value {
            WorldCell::Empty => Color::from_hsv(0.25, 0.0, 0.98),
            WorldCell::Food => Color::from("green"),
            WorldCell::Wall => Color::from("dark beige"),

            WorldCell::FloorLight => Color::from(0xf8f8f8),
            WorldCell::FloorDark => Color::from(0x606060),
        }
    }
}

impl From<OdorType> for Color {
    fn from(value: OdorType) -> Self {
        match value {
            OdorType::FoodA => Color::from("green"),
            OdorType::FoodB => Color::from("azure"),
            OdorType::AvoidA => Color::from("red"),
            OdorType::AvoidB => Color::from("tomato"),
            OdorType::OtherA => Color::from("purple"),
            OdorType::OtherB => Color::from("blue"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawWorld;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawItem;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawAgent;

pub struct UiWorldPlugin {
    bounds: Bounds::<UiLayout>,
}

impl UiWorldPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
        }
    }
}

impl Plugin for UiWorldPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiCanvasPlugin>() {
            assert!(app.contains_plugin::<WorldPlugin>());

            if ! app.contains_plugin::<UiLayoutPlugin>() {
                app.plugin(UiLayoutPlugin);
            }

            let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());
            let world = app.get_plugin::<WorldPlugin>().unwrap();
            let ui_world = UiWorld::new(box_id, world.width(), world.height());
            app.insert_resource(ui_world);

            app.phase(Update, (DrawWorld, DrawItem, DrawAgent).chain());
            app.system(Update, draw_world.phase(DrawWorld));
            app.system(PreUpdate, world_resize);

            // app.system(Startup, spawn_ui_world);
        }
    }
}