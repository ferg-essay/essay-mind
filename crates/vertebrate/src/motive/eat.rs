use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;
use crate::{body::BodyEat, mid_motor::{MidMotor, MidMotorPlugin}, util::{DecayValue, Seconds}};

use super::{motive::{Motive, MotiveTrait, Motives}, timeout::Timeout, Dwell, Wake};

struct CoreEat {
    _persist: Timeout,
    timeout: DecayValue,
}

impl CoreEat {
    fn new() -> Self {
        Self {
            _persist: Timeout::new(Seconds(4.)),
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
    wake: Res<Motive<Wake>>,
    mut dwell: ResMut<Motive<Dwell>>,
    mut sated: ResMut<Motive<Sated>>,
    mut food_seek: ResMut<Motive<FoodSearch>>,
    mid_motor: Res<MidMotor>,
) {
    if body_eat.glucose() > 0.75 || body_eat.glucose() > 0.25 && sated.is_active() {
        sated.set_max(1.);
    }

    core_eat.pre_update();

    if ! wake.is_active() || sated.is_active() {
        return;
    }

    // TODO: H.l food zone should be distinct from body_eat.
    if body_eat.is_food_zone() {
        food_seek.clear();

        // activate eating
        core_eat.add_eat();

        if body_eat.is_eating() {
            // eating sets dwell mode (5HT)
            if ! sated.is_active() {
                dwell.set_max(1.);
            } else {
                dwell.clear();
            }
        }

        if ! core_eat.is_eat_timeout() {
            eat_motive.set_max(1.);
            mid_motor.eat();
        }
    } else {
        food_seek.set_max(1.);
    }
}
pub struct Eat;
impl MotiveTrait for Eat {}

pub struct Sated;
impl MotiveTrait for Sated {}

pub struct FoodSearch;
impl MotiveTrait for FoodSearch {}

pub struct CoreEatingPlugin;

impl Plugin for CoreEatingPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidMotorPlugin>(), "CoreEating requires MidMotor");

        let feeding = CoreEat::new();
        app.insert_resource(feeding);

        Motives::insert::<Eat>(app, Seconds(1.));
        Motives::insert::<FoodSearch>(app, Seconds(0.1));
        Motives::insert::<Sated>(app, Seconds(5.));

        app.system(Tick, update_eat);

        // if app.contains_resource::<OlfactoryBulb>() {
        //    app.system(Tick, update_feeding_olfactory);
        // }
    }
}
