use essay_ecs::{app::{App, Plugin}, core::ResMut};
use mind_ecs::Tick;

use crate::{
    hind_move::{HindMove, HindMovePlugin}, 
    motive::{Motive, Motives}, 
    taxis::chemotaxis::Avoid, 
    util::{Seconds, Timeout}
};

pub struct TaxisAvoid {
    is_avoid: Timeout,
}

impl TaxisAvoid {
    fn new() -> Self {
        Self {
            is_avoid: Timeout::new(Seconds(25.)),
        }
    }

    fn is_avoid(&self) -> bool {
        self.is_avoid.is_active()
    }

    pub fn avoid(&mut self) {
        self.is_avoid.set();
    }

    fn update(&mut self) {
        self.is_avoid.update();
    }
}

pub struct TaxisAvoidPlugin {
}

impl TaxisAvoidPlugin {
    pub fn new() -> Self {
        Self {
        }
    }
}

fn update_avoid(
    mut avoid: ResMut<TaxisAvoid>,
    mut hind_move: ResMut<HindMove>,
    mut motive_avoid: ResMut<Motive<Avoid>>,
) {
    avoid.update();

    if avoid.is_avoid() {
        // println!("Avoid");
        motive_avoid.set_max(1.);
        hind_move.avoid();
    }
}

impl Plugin for TaxisAvoidPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "TaxisAvoidPlugin requires HindMovePlugin");

        let avoid = TaxisAvoid::new();
        
        app.insert_resource(avoid);

        app.system(Tick, update_avoid);

        Motives::insert::<Avoid>(app, Seconds(0.2));
    }
}

