use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}
};
use mind_ecs::Tick;
use crate::{
    body::BodyEat, hind_eat::HindEat, mid_move::{MidMove, MidMovePlugin}, olfactory::{OlfactoryCortex, OlfactoryCortexPlugin}, util::{DecayValue, Seconds}
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
    olfactory: Res<OlfactoryCortex>,
    body_eat: Res<BodyEat>,
    mid_move: Res<MidMove>,
    hind_eat: Res<HindEat>,
    mut motive_eat: ResMut<Motive<Eat>>,
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
    
    // H.l food zone from olfactory
    if olfactory.is_food_zone() {
        foraging.clear();

        // activate eating
        forage.add_eat();

        if ! forage.is_eat_timeout() {
            motive_eat.set_max(1.);
            // H disinhibits R.pb (cite)
            hind_eat.eat();
        }
    } else {
        foraging.set_max(1.);

        // H.sum activation for roaming
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

pub struct MotiveForagePlugin;

impl Plugin for MotiveForagePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidMovePlugin>(), "MotiveForage requires MidMove");
        assert!(app.contains_plugin::<OlfactoryCortexPlugin>(), "MotiveForage requires Olfactory");

        let feeding = Forage::new();
        app.insert_resource(feeding);

        Motives::insert::<Eat>(app, Seconds(1.));
        Motives::insert::<Forage>(app, Seconds(0.1));
        Motives::insert::<Sated>(app, Seconds(5.));

        Motives::insert::<Roam>(app, Seconds(1.));
        Motives::insert::<Dwell>(app, Seconds(4.));

        app.system(Tick, update_forage);
    }
}
