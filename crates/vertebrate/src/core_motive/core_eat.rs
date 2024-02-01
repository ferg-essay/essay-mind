use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;
use crate::{body::BodyEat, mid_motor::MidMotor, util::{DecayValue, Seconds}};

use super::{give_up::GiveUp, mid_peptides::MidPeptides, motive::{Motive, MotiveTrait, Motives}, Dwell};

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

fn _update_feeding_old(
    mut feeding: ResMut<CoreEat>,
    mut peptides2: ResMut<MidPeptides>
) {
    // orexin - base exploratory drive
    let explore_v = 0.5;
    //peptides.add(feeding.explore_id, explore_v);
    peptides2.explore_food_mut().add(explore_v);

    // habenula - give-up timer
    feeding._persist.update();

    // H.l stimulates habenula, here based on DA feedback
    if peptides2.seek_food() > 0.25 {
        feeding._persist.excite(1.);
    }

    // serotonin - high serotonin increases persistence
    let patience_5ht = (peptides2.urgency() - 0.7).clamp(0., 0.25);
    feeding._persist.inhibit(patience_5ht);

    peptides2.give_up_seek_food_mut().add(feeding._persist.value());

    // serotonin - urgency
    let urgency_v = (
        peptides2.explore_food()
        - (peptides2.give_up_seek_food() - 0.5)
    ).clamp(0., 1.);
    peptides2.urgency_mut().add(urgency_v);

    // dopamine - trigger for seeking a food cue
    let mut seek = 0.;

    // H.l senses glucose
    if peptides2.glucose() < 0.3 {
        // baseline DA from orexin
        seek += peptides2.explore_food() * 0.4;
        // ghrelin - food cue (ghrelin) prompts
        seek += peptides2.cue_seek_food();
        // nts - neurotensin suppresses food seeking
        seek -= peptides2.cue_avoid_food();
    }

    // orexin - high orexin avoids
    seek -= (peptides2.explore_food() - 0.5).max(0.);

    // habenula - give-up circuit suppresses
    seek -= 2. * (peptides2.give_up_seek_food() - 0.5).max(0.);

    peptides2.seek_food_mut().add(seek.clamp(0., 1.));
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
        dwell.set_max(1.);

        if ! core_eat.is_eat_timeout() {
            // locomotor_event.send(HindLocomotorEvent::Stop);
            eat_motive.set_max(1.);
            mid_motor.eat();
        }
    }
    
        /*
    if peptides.near_food() > 0.5 {
        println!("Near eat");
        if body.eat().glucose() < 0.8 && body.eat().is_eating()
            || body.eat().glucose() < 0.3 {
            body.locomotion_mut().arrest();
            body.eat_mut().eat();
        }
    }
        */
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
