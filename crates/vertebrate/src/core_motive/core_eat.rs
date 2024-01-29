use essay_ecs::{app::{event::OutEvent, App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;
use crate::{body::{Body, BodyEat}, hind_motor::{HindEat, HindLocomotorEvent}, olfactory_bulb::OlfactoryBulb, util::Seconds};

use super::{give_up::HabenulaGiveUp, mid_peptides::MidPeptides, motive::Motive, Dwell};

struct Eating {
    give_up_hb: HabenulaGiveUp,
}

impl Eating {
    fn new() -> Self {
        Self {
            give_up_hb: HabenulaGiveUp::new(Seconds(4.)),
        }
    }
}

fn update_feeding_old(
    mut feeding: ResMut<Eating>,
    mut peptides2: ResMut<MidPeptides>
) {
    // orexin - base exploratory drive
    let explore_v = 0.5;
    //peptides.add(feeding.explore_id, explore_v);
    peptides2.explore_food_mut().add(explore_v);

    // habenula - give-up timer
    feeding.give_up_hb.update();

    // H.l stimulates habenula, here based on DA feedback
    if peptides2.seek_food() > 0.25 {
        feeding.give_up_hb.excite(1.);
    }

    // serotonin - high serotonin increases persistence
    let patience_5ht = (peptides2.urgency() - 0.7).clamp(0., 0.25);
    feeding.give_up_hb.inhibit(patience_5ht);

    peptides2.give_up_seek_food_mut().add(feeding.give_up_hb.value());

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

fn update_feeding(
    mut eating: ResMut<Eating>,
    body_eat: Res<BodyEat>
) {
    if body_eat.is_food_zone() {
        println!("FoodZone2");
    }
}

fn update_feeding_olfactory(
    olfactory: Res<OlfactoryBulb>,
    mut peptides: ResMut<MidPeptides>
) {
    if olfactory.food_dir().is_some() {
        peptides.cue_seek_food_mut().add(0.8);
    }
}

fn update_near_food(
    body: Res<Body>, 
    mut peptides: ResMut<MidPeptides>
) {
    println!("Near food");
    /*
    if body.eat().is_sensor_food() {
        peptides.near_food_mut().add(1.0);
    }
    */
}

fn update_eat(
    core_eat: ResMut<Eating>,
    body_eat: Res<BodyEat>,
    mut dwell: ResMut<Motive<Dwell>>,
    mut locomotor_event: OutEvent<HindLocomotorEvent>,
) {
    if body_eat.is_food_zone() {
        dwell.set_max(1.);
        locomotor_event.send(HindLocomotorEvent::Stop);
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


pub struct CoreEatingPlugin;

impl Plugin for CoreEatingPlugin {
    fn build(&self, app: &mut App) {
        let feeding = Eating::new();

        app.insert_resource(feeding);
        // app.system(Tick, update_body_glucose);
        // app.system(Tick, update_feeding);
        // app.system(Tick, update_near_food);
        app.system(Tick, update_eat);

        // if app.contains_resource::<OlfactoryBulb>() {
        //    app.system(Tick, update_feeding_olfactory);
        // }
    }
}
