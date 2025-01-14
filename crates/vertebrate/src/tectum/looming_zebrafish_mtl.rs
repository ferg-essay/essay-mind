use essay_ecs::{app::App, core::{Res, ResMut}};
use log::warn;
use mind_ecs::Tick;

use crate::{hind_brain::HindMove, retina::Retina, util::{DecayValue, Seconds, Turn}};

use super::looming::LoomingStrategy;

// LoomingZebrafishMtl is an older looming strategy based around the
// Zebrafish M.tl torus longitidunus for maintaining average light

fn looming_update(
    mut looming: ResMut<Looming>, 
    retina: Res<Retina>,
    mut hind_move: ResMut<HindMove>
) {
    looming.update();

    looming.dim_left.add(retina.dim_left());
    looming.dim_right.add(retina.dim_right());

    looming.light_left.add(retina.light_left());
    looming.light_right.add(retina.light_right());
    // Zebrafish M.tl - torus longitidunus
    looming.light_mid.add(0.5 * (retina.light_left() + retina.light_right()));

    if looming.is_looming() {
        let light_mid = looming.light_mid.value();
        let left_dim = light_mid - retina.light_left();
        let right_dim = light_mid - retina.light_right();

        let sum_dim = left_dim.max(0.) + right_dim.max(0.);
        
        if sum_dim * 0.75 < left_dim {
            hind_move.optic().escape(Looming::TURN);
            hind_move.set_ss_left(0.75);
        } else if sum_dim * 0.75 < right_dim {
            hind_move.optic().escape(- Looming::TURN);
            hind_move.set_ss_right(0.75);
        } else {
            if left_dim < right_dim {
                hind_move.optic().u_turn(- Looming::U_TURN);
            } else {
                hind_move.optic().u_turn(Looming::U_TURN);
            }
            
            hind_move.set_ss_forward(0.75);
        }
    }
}


struct Looming {
    threshold: f32,
    dim_left: DecayValue,
    dim_right: DecayValue,

    light_mid: DecayValue,
    light_left: DecayValue,
    light_right: DecayValue,
}

impl Looming {
    const THRESHOLD : f32 = 0.022;
    const DIM_TIME : Seconds = Seconds(0.2);
    const AVG_TIME : Seconds = Seconds(1.);

    const TURN : Turn = Turn::Unit(0.20);
    const U_TURN : Turn = Turn::Unit(0.40);

    fn new() -> Self {
        Self {
            threshold: Self::THRESHOLD,
            dim_left: DecayValue::new(Self::DIM_TIME).fill_decay(),
            dim_right: DecayValue::new(Self::DIM_TIME).fill_decay(),

            // light_mid represents the M.tl average light level
            light_mid: DecayValue::new(Self::AVG_TIME).fill_decay(),

            light_left: DecayValue::new(Self::AVG_TIME).fill_decay(),
            light_right: DecayValue::new(Self::AVG_TIME).fill_decay(),
        }
    }

    fn update(&mut self) {
        self.dim_left.update();
        self.dim_right.update();

        self.light_mid.update();
        self.light_left.update();
        self.light_right.update();
    }

    fn is_looming(&self) -> bool {
        self.threshold < self.dim_left.value()
        || self.threshold < self.dim_right.value()
    }
}

pub struct LoomingZebrafishMtl;

impl LoomingStrategy for LoomingZebrafishMtl {
    fn build(&self, app: &mut App) {
        if ! app.contains_resource::<Retina>() {
            warn!("Looming requires Retina");
            return;
        }

        app.insert_resource(Looming::new());

        app.system(Tick, looming_update);
    }
}
