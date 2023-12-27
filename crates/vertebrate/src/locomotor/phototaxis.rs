///
/// phototaxis
///

use essay_ecs::{prelude::{Plugin, App, ResMut, Res}, app::event::OutEvent};
use mind_ecs::Tick;

use crate::{
    body::Body, 
    world::World, 
    locomotor::mid_locomotor::MidLocomotorPlugin, 
    util::{DecayValue, DirVector, Angle},
};

use super::taxis_pons::TaxisEvent;

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

    pub fn update(&mut self, value: f32, head_dir: Angle) {
        self.value = value;

        self.average.update();
        self.average.add(value);
        
        self.short_average.update();
        self.short_average.add(value);

        //for dir_gradients in &mut self.dir_gradients {
        //    dir_gradients.update();
        //}

        let gradient = self.short_gradient();
        self.goal_vector.avoid(head_dir, gradient);
    }
}

impl Default for Phototaxis {
    fn default() -> Self {
        let half_life = 40;

        Self { 
            // start with 20
            average: DecayValue::new(40),
            short_average: DecayValue::new(5),
            value: 0.,
            goal_vector: GoalVector::new(half_life),
        }
    }
}

fn update_phototaxis(
    mut body: ResMut<Body>, 
    world: Res<World>, 
    //mut explore: ResMut<MidExplore>,
    mut explore: OutEvent<TaxisEvent>,
    mut phototaxis: ResMut<Phototaxis>,
) {
    let light = world.light(body.pos_head());

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

    explore.send(TaxisEvent::AvoidVector(avoid_ego));

    if diff >= 0.2 {
        // positive gradient, move forward, avoiding the current area
        explore.send(TaxisEvent::Avoid);
    } else if diff <= -0.2 {
        // negative gradient, turn avound
        explore.send(TaxisEvent::AvoidUTurn);
    } else if light > 0.5 {
        explore.send(TaxisEvent::Normal);
    } else {
        explore.send(TaxisEvent::Avoid);
    }

    /*
    explore.add_avoid(avoid_ego.scale((-diff).clamp(0., 1.)));

    if diff >= 0.2 {
        // positive gradient, move forward, avoiding the current area
        explore.avoid(); 
    } else if diff <= -0.2 {
        // negative gradient, turn avound
        explore.avoid_turn();
    } else if light > 0.5 {
        explore.normal();
    } else {
        explore.avoid();
    }
    */

    let goal_vector = phototaxis.goal_vector();
    body.set_goal_dir(goal_vector);
}

pub struct GoalVector {
    dir: Angle,
    value: DecayValue,
}

impl GoalVector {
    fn new(half_life: usize) -> Self {
        Self {
            dir: Angle::Unit(0.),
            value: DecayValue::new(half_life),
        }
    }

    fn avoid(&mut self, dir: Angle, gradient: f32) {
        self.value.update();

        if gradient > 0. {
            let reverse_dir = Angle::Unit(dir.to_unit() + 0.5);

            self.add_vector(reverse_dir, gradient);
            //self.dir_gradient(reverse_dir).set_max(gradient);
            // self.goal_vector.update(reverse_dir, gradient);
        } else if gradient < 0. {
            //self.dir_gradient(head_dir).set_max(- gradient);
            // self.goal_vector.update(head_dir, - gradient);
            self.add_vector(dir, - gradient);
        }
    }

    fn add_vector(&mut self, dir: Angle, value: f32) {
        if self.value.value() < value {
            self.dir = dir;
            self.value.set(value);
        }
    }

    fn to_vector(&self) -> DirVector {
        DirVector::new(self.dir, self.value.value())
    }
}

pub struct PhototaxisPlugin;

impl Plugin for PhototaxisPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidLocomotorPlugin>());

        app.init_resource::<Phototaxis>();

        app.system(Tick, update_phototaxis);
    }
}
