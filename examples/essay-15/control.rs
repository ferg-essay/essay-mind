use essay_ecs::{prelude::*, core::Local};

use crate::body::Body;

// touch controlling muscles using muscle proprioception to
// resolve conflicts (simultanous left and right touch)
fn touch_muscle_feedback_left(mut body: ResMut<Body>) {
    // inhibition from opposite muscle
    if body.is_sensor_left() && body.muscle_left() <= 0. {
        body.set_muscle_right(1.);
    }
}

fn touch_muscle_feedback_right(mut body: ResMut<Body>) {
    // inhibition from opposite muscle
    if body.is_sensor_right() && body.muscle_right() <= 0. {
        body.set_muscle_left(1.);
    }
}

fn touch_muscle_dopamine(mut body: ResMut<Body>, mut da: Local<DopaminePair>) {
    da.left = (da.left - DopaminePair::DECAY).max(0.);
    da.right = (da.right - DopaminePair::DECAY).max(0.);

    // inhibition from opposite da
    if body.is_sensor_left() && da.left <= 0. {
        body.set_muscle_right(1.);
        da.right = if da.right <= 0. { 1. } else { da.right };
    }

    if body.is_sensor_right() && da.right <= 0. {
        body.set_muscle_left(1.);
        da.left = if da.left <= 0. { 1. } else { da.left };
    }
}

struct DopaminePair {
    left: f32,
    right: f32,
}

impl DopaminePair {
    pub const DECAY: f32 = 0.1;
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

fn satiety_speed_update(mut body: ResMut<Body>) {
    if body.get_satiety() >= 0.75 && ! body.is_sensor_food() {
        body.speed(0.5);
    }
}

pub struct SlugControlPlugin;

impl Plugin for SlugControlPlugin {
    fn build(&self, app: &mut App) {
        //app.system(Update, touch_muscle_update);
        // muscle control with proprioceptive feedback to resolve
        // conflicts
        let is_muscle = false;
        if is_muscle {
            app.system(Update, touch_muscle_feedback_left);
            app.system(Update, touch_muscle_feedback_right);
        } else { 
            // muscle control with dopamine memory to resolve
            // conflicts
            app.system(Update, touch_muscle_dopamine);
        }
        app.system(Update, food_arrest_update);
        app.system(Update, satiety_speed_update);
    }
}