///
/// chemotaxis
///

use essay_ecs::{prelude::{Plugin, App, ResMut, Res}, app::event::InEvent};
use mind_ecs::Tick;

use crate::{
    body::Body, 
    hind_brain::{HindMove, HindMovePlugin}, 
    hypothalamus::{Motive, MotiveTrait, Motives, Sated, Wake}, 
    olfactory::olfactory_bulb::{ObEvent, OlfactoryBulb}, 
    util::{Heading, Seconds} 
};

pub struct Seek;
impl MotiveTrait for Seek {}

pub struct Avoid;
impl MotiveTrait for Avoid {}


use super::{habenula_seek::HabenulaSeekItem, Taxis};

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
        head_dir: Heading,
        _hind_move: &HindMove,
        taxis: &mut ResMut<Taxis>,
        seek_motive: &mut Motive<Seek>,
    ) {
        // update the light average
        self.habenula.update(head_dir);
    
        let approach_vector = self.habenula.goal_vector();
        let approach_ego = approach_vector.to_approach(head_dir);

        if self.habenula.value() > 0.01 || approach_ego.value() > 0.05 {
            seek_motive.set_max(1.);
            // hind_move.turn(approach_ego.turn()); // TurnCommand::ApproachVector(approach_ego));

            taxis.set_approach_dir(approach_vector);
            todo!();
        }
     }
}

fn update_chemotaxis(
    mut chemotaxis: ResMut<Chemotaxis>,
    mut ob: InEvent<ObEvent>,
    body: Res<Body>,
    hind_move: Res<HindMove>,
    sated: Res<Motive<Sated>>,
    wake: Res<Motive<Wake>>,
    mut taxis: ResMut<Taxis>,
    mut seek_motive: ResMut<Motive<Seek>>,
) {
    chemotaxis.pre_update();

    if ! wake.is_active() {
        return;
    }

    if sated.is_active() {
        return;
    }

    for event in ob.iter() {
        match event {
            ObEvent::Odor(_odor, vector) => {
                chemotaxis.toward(vector.value());
            },
        }
    }

    chemotaxis.update(body.head_dir(), hind_move.get(), &mut taxis, seek_motive.get_mut());
}

pub struct ChemotaxisPlugin;

impl Plugin for ChemotaxisPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "chemotaxis requires HindMovePlugin");
        
        assert!(app.contains_resource::<OlfactoryBulb>(), "chemotaxis requires OlfactoryBulb");

        app.init_resource::<Taxis>();

        Motives::insert::<Seek>(app, Seconds(0.5));
        Motives::init::<Sated>(app);

        let chemotaxis = Chemotaxis::new();

        app.insert_resource(chemotaxis);

        app.system(Tick, update_chemotaxis);
    }
}
