use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;
use crate::{body::BodyEat, mid_motor::MidMotor, util::{DecayValue, Seconds}};

use super::{give_up::GiveUp, motive::{Motive, MotiveTrait, Motives}, Dwell};

struct CoreEat {
    _persist: GiveUp,
    timeout: DecayValue,
}

impl CoreEat {
    fn new() -> Self {
        Self {
            _persist: GiveUp::new(Seconds(4.)),
            timeout: DecayValue::new(2.),
        }
    }

    fn pre_update(&mut self) {
        self.timeout.update();
    }

    fn add_eat(&mut self) {
        self.timeout.add(1.);
    }

    fn is_eat_timeout(&mut self) -> bool {
        self.timeout.value() > 0.5
    }
}

fn update_eat(
    mut core_eat: ResMut<CoreEat>,
    body_eat: Res<BodyEat>,
    mut eat_motive: ResMut<Motive<Eat>>,
    mut dwell: ResMut<Motive<Dwell>>,
    mut sated: ResMut<Motive<Sated>>,
    mid_motor: Res<MidMotor>,
) {
    if body_eat.glucose() > 0.75 || body_eat.glucose() > 0.25 && sated.is_active() {
        sated.set_max(1.);
    }

    core_eat.pre_update();

    if ! sated.is_active() && body_eat.is_food_zone() {
        core_eat.add_eat();
        if body_eat.is_eating() {
            dwell.set_max(1.);
        //} else if dwell.is_active() {
        //    dwell.set_max(1.);
        }

        if ! core_eat.is_eat_timeout() {
            // locomotor_event.send(HindLocomotorEvent::Stop);
            eat_motive.set_max(1.);
            mid_motor.eat();
        }
    }
}
pub struct Eat;
impl MotiveTrait for Eat {}

pub struct Sated;
impl MotiveTrait for Sated {}

pub struct CoreEatingPlugin;

impl Plugin for CoreEatingPlugin {
    fn build(&self, app: &mut App) {
        let feeding = CoreEat::new();
        app.insert_resource(feeding);

        Motives::insert::<Eat>(app, Seconds(1.));
        Motives::insert::<Sated>(app, Seconds(5.));

        app.system(Tick, update_eat);

        // if app.contains_resource::<OlfactoryBulb>() {
        //    app.system(Tick, update_feeding_olfactory);
        // }
    }
}
