use util::random::{random_pareto, Rand32};

use crate::util::Turn;

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
