use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use log::warn;
use mind_ecs::Tick;

use crate::{hind_brain::HindMove, retina::Retina, util::{DecayValue, Seconds, Turn}};

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

    const TURN : f32 = 0.20;
    const U_TURN : f32 = 0.40;

    fn new() -> Self {
        Self {
            threshold: Self::THRESHOLD,
            dim_left: DecayValue::new(Self::DIM_TIME),
            dim_right: DecayValue::new(Self::DIM_TIME),

            light_mid: DecayValue::new(Self::AVG_TIME),
            light_left: DecayValue::new(Self::AVG_TIME),
            light_right: DecayValue::new(Self::AVG_TIME),
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
        self.dim_left.value() > self.threshold
        || self.dim_right.value() > self.threshold
    }
}

fn looming_update(
    mut looming: ResMut<Looming>, 
    retina: Res<Retina>,
    mut hind_move: ResMut<HindMove>
) {
    looming.update();

    looming.dim_left.add((- retina.brighten_left()).max(0.));
    looming.dim_right.add((- retina.brighten_right()).max(0.));

    looming.light_left.add(retina.light_left());
    looming.light_right.add(retina.light_right());
    looming.light_mid.add(0.5 * (retina.light_left() + retina.light_right()));

    if looming.is_looming() {
        let light_mid = looming.light_mid.value();
        let left_dim = -(retina.light_left() - light_mid);
        let right_dim = -(retina.light_right() - light_mid);

        let sum = left_dim.max(0.) + right_dim.max(0.);
        
        if sum * 0.75 < left_dim {
            hind_move.optic().escape(Turn::Unit(Looming::TURN));
            hind_move.set_ss_left(0.75);
        } else if sum * 0.75 < right_dim {
            hind_move.optic().escape(Turn::Unit(-Looming::TURN));
            hind_move.set_ss_right(0.75);
        } else {
            if left_dim < right_dim {
                hind_move.optic().u_turn(Turn::Unit(-Looming::U_TURN));
            } else {
                hind_move.optic().u_turn(Turn::Unit(Looming::U_TURN));
            }
            hind_move.set_ss_forward(0.75);
        }
    }
}

pub struct TectumLoomingPlugin {
}

impl TectumLoomingPlugin {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Plugin for TectumLoomingPlugin {
    fn build(&self, app: &mut App) {
        if ! app.contains_resource::<Retina>() {
            warn!("Looming requires Retina");
            return;
        }

        app.insert_resource(Looming::new());

        app.system(Tick, looming_update);
    }
}
