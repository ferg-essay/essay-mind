use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}
};
use mind_ecs::Tick;
use crate::{
    body::BodyEat, 
    hind_eat::HindEat, 
    mid_move::{MidMove, MidMovePlugin}, 
    util::{DecayValue, Seconds}
};

use super::{
    Motive, MotiveTrait, Motives, Wake
};

pub struct Forage {
    timeout: DecayValue,
}

impl Forage {
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

fn update_forage(
    mut forage: ResMut<Forage>,
    body_eat: Res<BodyEat>,
    mid_move: Res<MidMove>,
    hind_eat: Res<HindEat>,
    mut motive_eat: ResMut<Motive<Eat>>,
    mut dwell: ResMut<Motive<Dwell>>,
    mut foraging: ResMut<Motive<Forage>>,
    mut sated: ResMut<Motive<Sated>>,
    wake: Res<Motive<Wake>>,
) {
    if body_eat.glucose() > 0.75 || body_eat.glucose() > 0.25 && sated.is_active() {
        sated.set_max(1.);
    }

    forage.pre_update();

    if ! wake.is_active() {
        return;
    } else if sated.is_active() {
        // TODO: roam not strictly justified, but w/o this the animal remains 
        // paused at the food
        mid_move.roam();
        return;
    }

    // TODO: H.l food zone should be distinct from body_eat.
    if body_eat.is_food_zone() {
        foraging.clear();

        // activate eating
        forage.add_eat();

        if body_eat.is_eating() {
            // eating sets dwell mode (5HT)
            if ! sated.is_active() {
                dwell.set_max(1.);
            } else {
                dwell.clear();
            }
        }

        if ! forage.is_eat_timeout() {
            motive_eat.set_max(1.);
            hind_eat.eat();
        }
    } else {
        foraging.set_max(1.);

        //roam.set_max(wake.value());
        //if dwell.is_active() {
        //    mid_move.dwell();
        //} else if roam.is_active() {
            mid_move.roam();
        }
}
pub struct Eat;
impl MotiveTrait for Eat {}

pub struct Sated;
impl MotiveTrait for Sated {}

// pub struct Forage;
impl MotiveTrait for Forage {}

pub struct Roam;
impl MotiveTrait for Roam {}

pub struct Dwell;
impl MotiveTrait for Dwell {}

fn _update_roam(
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
        if dwell.is_active() {
            mid_move.dwell();
        } else if roam.is_active() {
            mid_move.roam();
        }   
    }
}

pub struct MotiveForagePlugin;

impl Plugin for MotiveForagePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidMovePlugin>(), "MotiveForage requires MidMove");

        let feeding = Forage::new();
        app.insert_resource(feeding);

        Motives::insert::<Eat>(app, Seconds(1.));
        Motives::insert::<Forage>(app, Seconds(0.1));
        Motives::insert::<Sated>(app, Seconds(5.));

        Motives::insert::<Roam>(app, Seconds(1.));
        Motives::insert::<Dwell>(app, Seconds(4.));

        app.system(Tick, update_forage);
        // app.system(Tick, update_roam);
    }
}


pub struct MotiveMovePlugin;

impl Plugin for MotiveMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidMovePlugin>(), "MotiveMove requires MidMove");

    }
}
