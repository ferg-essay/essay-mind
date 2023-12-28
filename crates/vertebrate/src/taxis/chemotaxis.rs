///
/// phototaxis
///

use essay_ecs::{prelude::{Plugin, App, ResMut, Res}, app::event::{InEvent, OutEvent}};
use mind_ecs::Tick;

use crate::{
    taxis::habenula_seek::HabenulaSeek,
    util::{DecayValue, DirVector, Angle}, olfactory_bulb::{OlfactoryBulb, ObEvent}, body::Body, 
};

use super::{taxis_pons::TaxisEvent, habenula_seek::HabenulaSeekItem};

pub struct Chemotaxis {
    habenula: HabenulaSeekItem,

    value: f32,
}

impl Chemotaxis {
    pub const N_DIR : usize = 12;

    pub fn new() -> Self {
        Self {
            habenula: HabenulaSeekItem::default(),
            value: 0.,
        }
    }

    pub fn pre_update(&mut self) {
        self.habenula.pre_update();
        self.value = 0.;
    }

    pub fn toward(&mut self, value: f32) {
        self.value += value;
        self.habenula.add(value);
    }

    pub fn update(
        &mut self, 
        head_dir: Angle,
        taxis: &mut OutEvent<TaxisEvent>
    ) {
        // update the light average
        self.habenula.update(head_dir);
    
        // the 5HT cells average the past light
        // let average = phototaxis.average.value();
    
        // let diff = self.habenula.gradient();
    
        let approach_vector = self.habenula.goal_vector();
        let approach_ego = approach_vector.to_approach(head_dir);
    
        //let approach_ego = approach_ego.scale(diff.clamp(0., 1.));
    
        if approach_ego.value() > 0.1 {
            taxis.send(TaxisEvent::ApproachVector(approach_ego));
            taxis.send(TaxisEvent::Approach); // small-scale search
        }
    
        /*
        if diff >= 0.2 {
            // positive gradient, move forward, avoiding the current area
            taxis.send(TaxisEvent::Avoid);
        } else if diff <= -0.2 {
            // negative gradient, turn avound
            taxis.send(TaxisEvent::AvoidUTurn);
        }
        */
    
        //let goal_vector = self.habenula.goal_vector();
        // body.set_goal_dir(goal_vector);
     }
    
    fn goal_vector(&self) -> DirVector {
        self.habenula.goal_vector()
    }
}

fn update_chemotaxis(
    mut chemotaxis: ResMut<Chemotaxis>,
    mut ob: InEvent<ObEvent>,
    body: Res<Body>,
    mut taxis: OutEvent<TaxisEvent>,
) {
    let mut value = 0.;

    chemotaxis.pre_update();

    for event in ob.iter() {
        match event {
            ObEvent::Odor(_odor, vector) => {
                // println!("Odor {:?} {:?}", odor, vector);
                chemotaxis.toward(vector.value());
            },
        }
    }

    chemotaxis.update(body.head_dir(), &mut taxis);
}

pub struct ChemotaxisPlugin;

impl Plugin for ChemotaxisPlugin {
    fn build(&self, app: &mut App) {
        // assert!(app.contains_resource::<HabenulaSeek>(), "chemotaxis requires HabenulaSeek");
        assert!(app.contains_resource::<OlfactoryBulb>(), "chemotaxis requires OlfactoryBulb");

        let chemotaxis = Chemotaxis::new();

        app.insert_resource(chemotaxis);

        app.system(Tick, update_chemotaxis);
    }
}
