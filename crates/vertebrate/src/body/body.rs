use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use essay_tensor::Tensor;
use mind_ecs::Tick;
use test_log::{TestLog, TestLogPlugin};
use crate::body::{BodyLocomotion, Action};

use crate::world::{OdorType, World, WorldPlugin};

use super::eat::BodyEat;

// #[derive(Component)]
pub struct Body {
    locomotion: BodyLocomotion,
    eat: BodyEat,

    tick_food: usize,
    ticks: usize,
}

impl Body {
    pub fn new(pos: Point) -> Self {
        let mut locomotion = BodyLocomotion::new(pos);
        locomotion.action_default(Action::forward());

        let mut eat = BodyEat::new();

        Self {
            locomotion,
            eat,

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

    pub fn eat(&self) -> &BodyEat {
        &self.eat
    }

    pub fn eat_mut(&mut self) -> &mut BodyEat {
        &mut self.eat
    }

    pub fn pos(&self) -> Point {
        self.locomotion.pos()
    }

    pub fn pos_head(&self) -> Point {
        self.locomotion.pos_head()
    }

    pub fn dir(&self) -> Angle {
        self.locomotion.dir()
    }

    pub fn is_collide_left(&self) -> bool {
        self.locomotion.is_collide_left()
    }

    pub fn is_collide_right(&self) -> bool {
        self.locomotion.is_collide_right()
    }

    pub fn speed(&self) -> f32 {
        self.locomotion.speed()
    }

    pub fn turn(&self) -> f32 {
        self.locomotion.turn()
    }

    pub fn p_food(&self) -> f32 {
        self.tick_food as f32 / self.ticks.max(1) as f32
    }

    pub fn odor_turn(&self, world: &World) -> Option<(OdorType, Angle)> {
        if let Some((odor, angle)) = world.odor(self.locomotion.pos_head()) {
            let turn = (2. + angle.to_unit() - self.dir().to_unit()) % 1.;

            Some((odor, Angle::Unit(turn)))
        } else {
            None
        }
    }
}

pub fn spawn_body(
    mut commands: Commands,
) {
    commands.insert_resource(Body::new(Point(0.5, 0.5)));
}

///
/// Update the animal's position based on the cilia movement
/// 
pub fn body_physics(
    mut body: ResMut<Body>,
    world: Res<World>,
) {
    body.locomotion.update(world.get());
    let pos_head = body.pos_head();
    body.eat_mut().update(world.get(), pos_head);

    if body.eat().is_sensor_food() {
        body.tick_food += 1;
    }

    body.ticks += 1;
}

fn _random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

pub fn body_log(
    body: Res<Body>,
    mut log: ResMut<TestLog>,
) {
    log.log(&format!("body: ({:.1}, {:.1}) dy={:.1} swim={:.1} turn={:.1}",
        body.pos().x(), body.pos().y(), body.dir().to_unit(), body.speed(), body.turn()
    ));
}

pub struct BodyPlugin {
}

impl BodyPlugin {
    pub fn new() -> Self {
        BodyPlugin {
        }
    }
}

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