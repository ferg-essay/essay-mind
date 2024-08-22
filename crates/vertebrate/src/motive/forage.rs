use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;
use crate::{
    body::BodyEat, hind_eat::HindEat, mid_move::{MidMove, MidMovePlugin}, util::{DecayValue, Seconds}
};

use super::{
    motive::{Motive, MotiveTrait, Motives}, 
    Wake
};

struct CoreEat {
    timeout: DecayValue,
}

impl CoreEat {
    fn new() -> Self {
        Self {
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
    mut eat: ResMut<CoreEat>,
    body_eat: Res<BodyEat>,
    mut motive_eat: ResMut<Motive<Eat>>,
    wake: Res<Motive<Wake>>,
    mut dwell: ResMut<Motive<Dwell>>,
    mut sated: ResMut<Motive<Sated>>,
    mut food_seek: ResMut<Motive<FoodSearch>>,
    mid_motor: Res<MidMove>,
) {
    if body_eat.glucose() > 0.75 || body_eat.glucose() > 0.25 && sated.is_active() {
        sated.set_max(1.);
    }

    eat.pre_update();

    if ! wake.is_active() || sated.is_active() {
        return;
    }

    // TODO: H.l food zone should be distinct from body_eat.
    if body_eat.is_food_zone() {
        food_seek.clear();

        // activate eating
        eat.add_eat();

        if body_eat.is_eating() {
            // eating sets dwell mode (5HT)
            if ! sated.is_active() {
                dwell.set_max(1.);
            } else {
                dwell.clear();
            }
        }

        if ! eat.is_eat_timeout() {
            motive_eat.set_max(1.);
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

pub struct Roam;
impl MotiveTrait for Roam {}

pub struct Dwell;
impl MotiveTrait for Dwell {}

fn update_roam(
    mut roam: ResMut<Motive<Roam>>,
    mut dwell: ResMut<Motive<Dwell>>,
    hind_eat: Res<HindEat>,
    mid_move: Res<MidMove>,
    wake: Res<Motive<Wake>>,
) {
    if ! wake.is_active() {
        return;
    }

    if hind_eat.is_eat() {
        roam.set_max(wake.value() * 0.2);
        dwell.set_max(wake.value());
    } else {
        roam.set_max(wake.value());
    }

    if dwell.is_active() {
        mid_move.dwell();
    } else if roam.is_active() {
        mid_move.roam();
    }   
}

pub struct MotiveForagePlugin;

impl Plugin for MotiveForagePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidMovePlugin>(), "MotiveForage requires MidMove");

        let feeding = CoreEat::new();
        app.insert_resource(feeding);

        Motives::insert::<Eat>(app, Seconds(1.));
        Motives::insert::<FoodSearch>(app, Seconds(0.1));
        Motives::insert::<Sated>(app, Seconds(5.));

        Motives::insert::<Roam>(app, Seconds(1.));
        Motives::insert::<Dwell>(app, Seconds(4.));

        app.system(Tick, update_eat);
        app.system(Tick, update_roam);
    }
}


pub struct MotiveMovePlugin;

impl Plugin for MotiveMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidMovePlugin>(), "MotiveMove requires MidMove");

    }
}
