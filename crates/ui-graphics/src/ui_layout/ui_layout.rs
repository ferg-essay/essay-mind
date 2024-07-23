use driver::Drawable;
use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use crate::ui_canvas::UiWindowEvent;


pub struct UiLayout {
    boxes: Vec<UiBox>,
}

impl UiLayout {
    fn new() -> Self {
        Self {
            boxes: Vec::new(),
        }
    }

    pub fn add_box(&mut self, pos: impl Into<Bounds<UiLayout>>) -> BoxId {
        let id = BoxId(self.boxes.len());

        self.boxes.push(UiBox::new(id, pos.into()));

        id
    }

    pub fn get_box(&self, id: BoxId) -> &Bounds<Canvas> {
        for ui_box in &self.boxes {
            if ui_box.id == id {
                return &ui_box.pos_canvas;
            }
        }

        todo!()
    }

    fn window_bounds(&mut self, width: u32, height: u32) {
        let grid_bounds = self.grid_bounds();
        let x_grid = width as f32 / grid_bounds.xmax();
        let y_grid = height as f32 / grid_bounds.ymax();

        for ui_box in &mut self.boxes {
            ui_box.update(x_grid, y_grid);
        }
    }

    fn grid_bounds(&self) -> Bounds<UiLayout> {
        let mut x_max: f32 = 1.;
        let mut y_max: f32 = 1.;

        for ui_box in &self.boxes {
            x_max = x_max.max(ui_box.pos_grid.xmax());
            y_max = y_max.max(ui_box.pos_grid.ymax());
        }

        Bounds::new(Point(0., 0.), Point(x_max, y_max))
    }
}

pub struct UiBox {
    id: BoxId,

    pos_grid: Bounds<UiLayout>,
    pos_canvas: Bounds<Canvas>,
}

impl UiBox {
    fn new(id: BoxId, pos: Bounds<UiLayout>) -> Self {
        Self {
            id,
            pos_grid: pos,
            pos_canvas: Bounds::unit()
        }
    }

    fn update(&mut self, x_grid: f32, y_grid: f32) {
        self.pos_canvas = Bounds::new(
            Point(
                self.pos_grid.xmin() * x_grid, 
                self.pos_grid.ymin() * y_grid, 
            ), Point (
                self.pos_grid.xmax() * x_grid, 
                self.pos_grid.ymax() * y_grid, 
            )
        );
    }
}

impl Coord for UiLayout {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoxId(usize);

impl BoxId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

fn layout_event(
    mut layout: ResMut<UiLayout>, 
    mut reader: InEvent<UiWindowEvent>,
    mut writer: OutEvent<UiLayoutEvent>
) {
    for event in reader.iter() {
        match event {
            UiWindowEvent::Resized(width, height) => {
                layout.window_bounds(*width, *height);
                writer.send(UiLayoutEvent);
            }
        }
    }
}

pub struct UiLayoutPlugin;

impl Plugin for UiLayoutPlugin {
    fn build(&self, app: &mut essay_ecs::prelude::App) {
        app.insert_resource(UiLayout::new());

        app.event::<UiLayoutEvent>();

        app.system(First, layout_event);
    }
}

#[derive(Event)]
pub struct UiLayoutEvent;
