use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{
    body::BodyEat, 
    hind_brain::{HindEat, HindEatPlugin}, 
    motive::{Dwell, Eat, Motives, Sated}, 
    util::{Seconds, TimeoutValue}
};

use super::{Motive, Wake};

//
// MotiveEat includes R.pb, H.pstn, H.pv, S.a, P.bst
// specifically the food-related portions of those nuclei
//
// H.l and H.sum are in Forage, the food seeking module
//
// [Palmiter 2018] R.nts.cck -> R.pb.cgrp
// [Roman et al 2016] R.nts.cck -> R.pb.cgrp ko rescue H.arc.agrp anorexia
// [Roman et al 2017] R.nts.cck -> R.pb, H.pv stop eating. CTA, CPA
// [Torruella-Suárez et al 2024] S.a -> R.pb tonic inhibition (GABA.b), 
// suppressed by chronic pain

#[derive(Default)]
pub struct MotiveEat {
    // food_zone derives from H.l (Forage)
    is_food_zone: TimeoutValue<bool>,
}

impl MotiveEat {
    fn is_food_zone(&self) -> bool {
        self.is_food_zone.value_or(false)
    }

    // food_zone is set by H.l (Forage)
    pub(super) fn set_food_zone(&mut self, value: bool) {
        self.is_food_zone.set(value);
    }

    fn pre_update(&mut self) {
        self.is_food_zone.update();
    }
}

fn update_eat(
    mut eat: ResMut<MotiveEat>,
    body_eat: Res<BodyEat>,
    mut hind_eat: ResMut<HindEat>,
    mut sated: ResMut<Motive<Sated>>,
    wake: Res<Motive<Wake>>,
) {
    eat.pre_update();
    
    if body_eat.glucose() > 0.75 || body_eat.glucose() > 0.25 && sated.is_active() {
        sated.set_max(1.);
    }


    if ! wake.is_active() {
        return;
    } else if ! eat.is_food_zone() {
        // is_food_zone is from Forage (H.l) 
        return;
    }

    if sated.is_active() {
        return;
    }

    // TODO: check current moving
    hind_eat.eat();
}

pub struct MotiveEatPlugin;

impl Plugin for MotiveEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindEatPlugin>(), "MotiveEat requires HindEat");

        Motives::insert::<Eat>(app, Seconds(1.));
        Motives::insert::<Sated>(app, Seconds(5.));

        Motives::insert::<Dwell>(app, Seconds(4.));

        app.insert_resource(MotiveEat::default());

        app.system(Tick, update_eat);
    }
}
