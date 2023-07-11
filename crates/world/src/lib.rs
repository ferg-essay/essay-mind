mod world;
use essay_ecs::prelude::*;
use ui_graphics::UiCanvasPlugin;
use world::spawn_world;

use self::world::draw_world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        if ! app.contains_plugin::<UiCanvasPlugin>() {
            app.plugin(UiCanvasPlugin);
        }

        app.system(Startup, spawn_world);

        app.system(Update, draw_world);
    }
}