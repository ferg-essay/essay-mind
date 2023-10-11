use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use essay_tensor::Tensor;
use mind_ecs::Tick;
use test_log::{TestLog, TestLogPlugin};
use crate::body_locomotion::BodyLocomotion;

use crate::world::{OdorType, World, SlugWorldPlugin, WorldPlugin};

#[derive(Component)]
pub struct Body {
    locomotion: BodyLocomotion,

    sensor_food: bool,

    tick_food: usize,
    ticks: usize,
}

impl Body {
    pub fn new(pos: Point) -> Self {
        Self {
            locomotion: BodyLocomotion::new(pos),

            sensor_food: false,

            tick_food: 0,
            ticks: 0,
        }
    }

    pub fn locomotion(&self) -> &BodyLocomotion {
        &self.locomotion
    }

    pub fn locomotion_mut(&mut self) -> &mut BodyLocomotion {
        &mut self.locomotion
    }

    pub fn pos(&self) -> Point {
        self.locomotion.pos()
    }

    pub fn dir(&self) -> Angle {
        self.locomotion.dir()
    }

    pub fn is_touch_left(&self) -> bool {
        self.locomotion.touch_left()
    }

    pub fn is_touch_right(&self) -> bool {
        self.locomotion.touch_right()
    }

    pub fn is_sensor_food(&self) -> bool {
        self.sensor_food
    }

    pub fn p_food(&self) -> f32 {
        self.tick_food as f32 / self.ticks.max(1) as f32
    }

    pub fn odor_turn(&self, world: &World) -> Option<(OdorType, Angle)> {
        if let Some((odor, angle)) = world.odor(self.pos()) {
            let turn = (2. + angle.to_unit() - self.dir().to_unit()) % 1.;

            Some((odor, Angle::Unit(turn)))
        } else {
            None
        }
    }

    pub fn is_food_left(&self, world: &World) -> bool {
        if let Some((_, angle)) = world.odor(self.pos()) {
            let turn = (2. + angle.to_unit() - self.dir().to_unit()) % 1.;

            turn <= 0.5
        } else {
            false
        }
    }

    pub fn is_food_right(&self, world: &World) -> bool {
        if let Some((_, angle)) = world.odor(self.pos()) {
            let turn = (2. + angle.to_unit() - self.dir().to_unit()) % 1.;

            0.5 < turn
        } else {
            false
        }
    }

    pub fn muscle_left(&self) -> f32 {
        self.locomotion.muscle_left()
    }

    pub fn set_muscle_left(&mut self, muscle: f32) {
        self.locomotion.set_muscle_left(muscle);
    }

    pub fn muscle_right(&self) -> f32 {
        self.locomotion.muscle_right()
    }

    pub fn set_muscle_right(&mut self, muscle: f32) {
        self.locomotion.set_muscle_right(muscle);
    }

    pub fn speed(&self) -> f32 {
        self.locomotion.speed()
    }

    pub fn arrest(&self) -> f32 {
        self.locomotion.arrest()
    }

    ///
    /// Stop the muco-cilia beating for a period of time
    /// 
    pub fn set_arrest(&mut self, time: f32) {
        self.locomotion.set_arrest(time);
    }
}

pub fn spawn_body(
    mut commands: Commands,
) {
    commands.insert_resource(Body::new(Point(0.5, 0.5)));
}

///
/// Update the slugs's position based on the cilia movement
/// 
pub fn body_physics(
    mut body: ResMut<Body>,
    world: Res<World>,
) {
    body.locomotion.update(world.get());
    body.sensor_food = world.is_food(body.pos());

    if body.sensor_food {
        body.tick_food += 1;
    }
    body.ticks += 1;
}

/*
pub fn body_habit(
    mut body: ResMut<Body>,
    world: Res<World>,
) {
    let odor = match world.odor(body.pos()) {
        Some((odor, _)) => Some(odor),
        None => None,
    };
}
*/

fn _random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

pub fn body_log(
    body: &Body,
    mut log: ResMut<TestLog>,
) {
    log.log(&format!("body: ({:.1}, {:.1}) dy={:.1} swim={:.1} arrest={:.1}",
        body.pos().x(), body.pos().y(), body.dir().to_unit(), body.speed(), body.arrest()
    ));
}

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<WorldPlugin>(), "BodyPlugin requires WorldPlugin");
        app.system(Startup, spawn_body);

        app.system(Tick, body_physics);
        // app.system(Tick, body_habit);

        if app.contains_plugin::<TestLogPlugin>() {
            app.system(Last, body_log);
        }
    }
}