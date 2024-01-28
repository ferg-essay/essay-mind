///
/// phototaxis
///

use essay_ecs::{prelude::{Plugin, App, ResMut, Res}, app::event::{InEvent, OutEvent}};
use mind_ecs::Tick;

use crate::{
    body::Body, 
    hind_motor::HindLocomotorEvent, 
    olfactory_bulb::{OlfactoryBulb, ObEvent}, 
    util::{DirVector, Angle} 
};

use super::habenula_seek::HabenulaSeekItem;

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
    
    fn goal_vector(&self) -> DirVector {
        self.habenula.goal_vector()
    }

    #[inline]
    pub fn gradient(&self) -> f32 {
        self.habenula.gradient()
    }

    #[inline]
    pub fn value(&self) -> f32 {
        self.habenula.value()
    }

    pub fn update(
        &mut self, 
        head_dir: Angle,
        taxis: &mut OutEvent<HindLocomotorEvent>
    ) {
        // update the light average
        self.habenula.update(head_dir);
    
        let approach_vector = self.habenula.goal_vector();
        let approach_ego = approach_vector.to_approach(head_dir);

        if self.habenula.value() > 0.01 || approach_ego.value() > 0.05 {
            taxis.send(HindLocomotorEvent::ApproachVector(approach_ego));
            // taxis.send(TaxisEvent::Roam); // small-scale search
            taxis.send(HindLocomotorEvent::ApproachDisplay(approach_vector));
        }
     }
}

fn update_chemotaxis(
    mut chemotaxis: ResMut<Chemotaxis>,
    mut ob: InEvent<ObEvent>,
    body: Res<Body>,
    mut taxis: OutEvent<HindLocomotorEvent>,
) {
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
