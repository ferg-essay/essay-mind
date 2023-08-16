use essay_ecs::{prelude::*, core::Local};
use essay_tensor::Tensor;

use crate::{body::Body, world::World};

fn command_muscle_dopamine(
    mut body: ResMut<Body>, 
    world: Res<World>, 
    mut da: Local<DopaminePair>
) {
    let left_touch = body.is_touch_left();
    let right_touch = body.is_touch_right();

    let mut left_food = body.is_food_left(world.get());
    let mut right_food = body.is_food_right(world.get());

    // update habituation
    //left_food = habituate.update_left(left_food);
    //right_food = habituate.update_right(right_food);

    // touch priority over food
    if left_touch || right_touch {
        left_food = false;
        right_food = false;
    }

    // touch crosses, food is straight
    let left = right_touch || left_food;
    let right = left_touch || right_food;

    // DA as short-term memory of previous direction
    da.left = (da.left - DopaminePair::DECAY).max(0.);
    da.right = (da.right - DopaminePair::DECAY).max(0.);

    if left && right && da.left <= 0. && da.right <= 0. {
        if Tensor::random_uniform([1], ())[0] < 0.5 {
            body.set_muscle_left(1.);
            da.left = if da.left <= 0. { 1. } else { da.left };
        } else {
            body.set_muscle_right(1.);
            da.right = if da.right <= 0. { 1. } else { da.right };
        }
    }

    // inhibition from opposite da
    if left && da.right <= 0. {
        body.set_muscle_left(1.);
        da.left = if da.left <= 0. { 1. } else { da.left };
    }

    // inhibition from opposite da
    if right && da.left <= 0. {
        body.set_muscle_right(1.);
        da.right = if da.right <= 0. { 1. } else { da.right };
    }
}

struct DopaminePair {
    left: f32,
    right: f32,
}

impl DopaminePair {
    pub const DECAY: f32 = 0.025;
}

impl Default for DopaminePair {
    fn default() -> Self {
        Self { left: Default::default(), right: Default::default() }
    }
}

fn food_arrest_update(mut body: ResMut<Body>) {
    if body.is_sensor_food() {
        body.arrest(1.);
    }
}

pub struct SlugControlPlugin;

impl Plugin for SlugControlPlugin {
    fn build(&self, app: &mut App) {
        //app.event::<DirCommand>();

        //app.system(Update, touch_sense);
        // muscle control with dopamine memory to resolve
        // conflicts
        app.system(Update, command_muscle_dopamine);

        app.system(Update, food_arrest_update);
    }
}