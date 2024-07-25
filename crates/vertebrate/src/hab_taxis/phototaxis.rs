///
/// phototaxis
///

use essay_ecs::prelude::*;
use mind_ecs::Tick;

use crate::{
    body::Body, 
    hind_motor::{HindLevyMove, TurnCommand}, 
    util::{DecayValue, DirVector, HalfLife, Heading, Seconds}, 
    world::World
};

use super::{GoalVector, Taxis};

pub struct Phototaxis {
    average: DecayValue,
    value: f32,

    short_average: DecayValue,

    //dir_gradients: Vec<DirGradient>,
    goal_vector: GoalVector,
}

impl Phototaxis {
    pub const N_DIR : usize = 12;
    
    pub fn average(&self) -> f32 {
        self.average.value()
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn gradient(&self) -> f32 {
        self.value() - self.average()
    }

    pub fn short_average(&self) -> f32 {
        self.short_average.value()
    }

    pub fn short_gradient(&self) -> f32 {
        self.value() - self.short_average()
    }

    fn goal_vector(&self) -> DirVector {
        self.goal_vector.to_vector()
    }

    pub fn update(&mut self, value: f32, head_dir: Heading) {
        self.value = value;

        self.average.update();
        self.average.add(value);
        
        self.short_average.update();
        self.short_average.add(value);

        let gradient = self.short_gradient();
        self.goal_vector.avoid(head_dir, gradient);
    }
}

impl Default for Phototaxis {
    fn default() -> Self {
        let half_life = HalfLife(4.);

        Self { 
            // start with 20
            average: DecayValue::new(Seconds(4.)),
            short_average: DecayValue::new(Seconds(0.5)),
            value: 0.,
            goal_vector: GoalVector::new(half_life),
        }
    }
}

fn update_phototaxis(
    body: Res<Body>, 
    world: Res<World>, 
    hind_move: Res<HindLevyMove>,
    mut phototaxis: ResMut<Phototaxis>,
    mut taxis: ResMut<Taxis>,
) {
    let light = world.light(body.head_pos());

    // TODO: negative light is an error value
    if light >= 0.0 {
        // update the light average
        phototaxis.update(light, body.head_dir());
    }

    // the 5HT cells average the past light
    // let average = phototaxis.average.value();

    let diff = phototaxis.gradient();

    let avoid_vector = phototaxis.goal_vector();
    let avoid_ego = avoid_vector.to_ego(body.head_dir());

    let avoid_ego = avoid_ego.scale((-diff).clamp(0., 1.));

    hind_move.turn(TurnCommand::AvoidVector(avoid_ego));

    if diff >= 0.2 {
        // positive gradient, move forward, avoiding the current area
        hind_move.avoid();
    } else if diff <= -0.2 {
        // negative gradient, turn avound
        hind_move.turn(TurnCommand::AvoidUTurn);
    } else if light > 0.5 {
        // hind_move.send_move(MoveCommand::Normal);
    } else {
        hind_move.avoid(); // .send_move(MoveCommand::Avoid);
    }

    let goal_vector = phototaxis.goal_vector();
    taxis.set_avoid_dir(goal_vector);
}

pub struct PhototaxisPlugin;

impl Plugin for PhototaxisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Phototaxis>();
        app.init_resource::<Taxis>();

        app.system(Tick, update_phototaxis);
    }
}
