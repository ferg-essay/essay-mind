use essay_ecs::prelude::*;

use crate::Body;

fn touch_muscle_update(
    mut body: ResMut<Body>,
) {
    if body.is_sensor_left() {
        body.set_muscle_right(1.);
    }

    if body.is_sensor_right() {
        body.set_muscle_left(1.);
    }
}

fn food_arrest_update(
    mut body: ResMut<Body>,
) {
    if body.is_sensor_food() {
        body.arrest(1.);
    }
}

pub struct SlugControlPlugin;

impl Plugin for SlugControlPlugin {
    fn build(&self, app: &mut App) {
        app.system(Update, touch_muscle_update);
        app.system(Update, food_arrest_update);
    }
}