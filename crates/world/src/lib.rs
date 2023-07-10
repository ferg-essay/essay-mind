mod world;
use essay_ecs::prelude::*;
use ui_graphics::UiCanvasPlugin;
use world::spawn_world;

use self::world::draw_world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        if ! app.is_plugin_added::<UiCanvasPlugin>() {
            app.add_plugin(UiCanvasPlugin);
        }

        app.add_system(Startup, spawn_world);

        app.add_system(PreUpdate, draw_world);
    }
}