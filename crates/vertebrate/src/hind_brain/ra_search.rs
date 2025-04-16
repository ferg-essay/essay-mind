use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;
use util::random::{random_pareto, Rand32};

use crate::{body::BodyEatPlugin, hind_brain::SerotoninManager, util::{Seconds, Ticks, Turn}};

use super::{HindMove, Serotonin, SerotoninTrait};

// Karpenko et al 2020 - ARTR oscillator 20s period

fn update_hind_search(
    mut hind_search: ResMut<HindSearch>,
    mut hind_move: ResMut<HindMove>,
    serotonin_search: Res<Serotonin<HindSearch>>,
) {
    if ! serotonin_search.is_active() {
        return;
    }

    hind_move.roam();

    if let Some(turn) = hind_search.next_turn() {
        // hind search (ARTR) is low priority
        hind_move.turn_if_new(turn);
    }
}

pub struct HindSearch {
}

impl HindSearch {
    const ROAM_LOW : f32 = 1.;
    const ROAM_HIGH : f32 = 5.;

    const DWELL_LOW : f32 = 1.;
    const DWELL_HIGH : f32 = 1.;
    
    const ALPHA : f32 = 2.;

    pub(super) fn new() -> Self {
        Self {
        }
    }

    pub(super) fn next_turn(&mut self) -> Option<Turn> {
        let mut rand = Rand32::new();
        // semi-brownian
        if rand.next_uniform() <= 0.5 {
            Some(Turn::Deg(0.))
        } else if rand.next_uniform() <= 0.5 {
            Some(Turn::Deg(-30.))
        } else {
            Some(Turn::Deg(30.))
        }
    }

    #[allow(unused)]
    fn levy_run_len(&self, is_dwell: bool) -> f32 {
        if is_dwell {
            random_pareto(Self::DWELL_LOW, Self::DWELL_HIGH, Self::ALPHA)
        } else {
            random_pareto(Self::ROAM_LOW, Self::ROAM_HIGH, Self::ALPHA)
        }
    }
}

impl SerotoninTrait for HindSearch {}

// Karpenko et al 2020 - ARTR oscillator 20s period
pub struct OscillatorArs {
}

impl OscillatorArs {
    const ROAM_LOW : f32 = 1.;
    const ROAM_HIGH : f32 = 5.;

    const DWELL_LOW : f32 = 1.;
    const DWELL_HIGH : f32 = 1.;
    
    const ALPHA : f32 = 2.;

    pub(super) fn new() -> Self {
        Self {
        }
    }

    pub(super) fn _next_turn(&mut self) -> Option<Turn> {
        let mut rand = Rand32::new();
        // semi-brownian
        if rand.next_uniform() <= 0.5 {
            Some(Turn::Deg(0.))
        } else if rand.next_uniform() <= 0.5 {
            Some(Turn::Deg(-30.))
        } else {
            Some(Turn::Deg(30.))
        }
    }

    #[allow(unused)]
    fn levy_run_len(&self, is_dwell: bool) -> f32 {
        if is_dwell {
            random_pareto(Self::DWELL_LOW, Self::DWELL_HIGH, Self::ALPHA)
        } else {
            random_pareto(Self::ROAM_LOW, Self::ROAM_HIGH, Self::ALPHA)
        }
    }
}

pub struct HindSearchPlugin {
    search_time: Ticks,
}

impl HindSearchPlugin {
    pub fn new() -> Self {
        Self {
            search_time: Seconds(2.).into(),
        }
    }
}

impl Plugin for HindSearchPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyEatPlugin>(), "HindEatPlugin requires BodyEatPlugin");

        SerotoninManager::insert::<HindSearch>(app, self.search_time);

        let hind_search = HindSearch::new();

        // hind_search.is_eating = TimeoutValue::new(self.search_time);

        app.insert_resource(hind_search);

        app.system(Tick, update_hind_search);
    }
}
