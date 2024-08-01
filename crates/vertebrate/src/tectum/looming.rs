use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{hind_motor::HindMove, retina::{Retina, RetinaPlugin}, util::{DecayValue, Seconds, Turn}};

struct Looming {
    threshold: f32,
    dim_left: DecayValue,
    dim_right: DecayValue,

    light_mid: DecayValue,
    light_left: DecayValue,
    light_right: DecayValue,
}

impl Looming {
    const THRESHOLD : f32 = 0.02;
    const DIM_TIME : Seconds = Seconds(0.2);
    const AVG_TIME : Seconds = Seconds(1.);

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
        /*
        println!("Loom ({:.3}, {:.3}) {:.2} {:.2}({:.2}), {:.2}({:.2}))", 
            looming.dim_left.value(), 
            looming.dim_right.value(), 
            looming.light_mid.value(),
            retina.light_left() - looming.light_left.value(),
            retina.light_left() - looming.light_mid.value(),
            retina.light_right() - looming.light_right.value(),
            retina.light_right() - looming.light_mid.value(),
        );
        */

        let light_mid = looming.light_mid.value();
        let left_dim = retina.light_left() - light_mid;
        let right_dim = retina.light_right() - light_mid;

        if light_mid * 0.1 < left_dim - right_dim {
            hind_move.optic().escape(Turn::Unit(-0.12));
        } else if light_mid * 0.1 < right_dim - left_dim {
            hind_move.optic().escape(Turn::Unit(0.12));
        } else {
            hind_move.optic().u_turn();
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
        assert!(app.contains_plugin::<RetinaPlugin>(), "Looming requires Retina");

        app.insert_resource(Looming::new());

        app.system(Tick, looming_update);
    }
}
