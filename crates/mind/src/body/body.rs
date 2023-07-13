use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use ui_graphics::{UiCanvasPlugin};

use crate::{world::{ApicalWorldPlugin, DrawItem}, body::ui_body::draw_body};

#[derive(Component)]
pub struct Body {
    pos: Point,
}

impl Body {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }
}

pub fn spawn_body(
    mut commands: Commands,
) {
    commands.spawn(Body::new(Point(2.5, 2.5)));
}

pub struct ApicalBodyPlugin;

impl Plugin for ApicalBodyPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<ApicalWorldPlugin>(), "BodyPlugin requires WorldPlugin");
        app.system(Startup, spawn_body);

        if app.contains_plugin::<UiCanvasPlugin>() {
            app.system(Update, draw_body.phase(DrawItem));
        }
    }
}