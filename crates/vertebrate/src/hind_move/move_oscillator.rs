use util::random::Rand32;

use crate::util::Turn;

// Karpenko et al 2020 - ARTR oscillator 20s period
pub struct OscillatorArs {
}

impl OscillatorArs {
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
}
