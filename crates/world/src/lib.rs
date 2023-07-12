mod world;
use essay_ecs::prelude::*;
use ui_graphics::{UiCanvasPlugin, ui_layout::UiLayoutPlugin};
use world::{spawn_world, world_resize};

use self::world::draw_world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        if ! app.contains_plugin::<UiCanvasPlugin>() {
            app.plugin(UiCanvasPlugin);
        }

        if ! app.contains_plugin::<UiLayoutPlugin>() {
            app.plugin(UiLayoutPlugin);
        }

        app.system(Startup, spawn_world);
        app.system(PreUpdate, world_resize);

        app.system(Update, draw_world);
    }
}