use essay_ecs::core::Store;
use essay_ecs::core::store::FromStore;
use essay_plot::prelude::Angle;
use util::random::{random_pareto, random, random_normal};
use crate::body::{Action, Body};

pub struct MidExplore {
    alpha: f32,
    low: f32,
    high: f32,

    turn_mean: f32,
    turn_std: f32,

    is_turn: bool,
}

impl MidExplore {
    const LOW : f32 = 1.;
    const HIGH : f32 = 5.;
    const ALPHA : f32 = 2.;

    const TURN_MEAN : f32 = 60.;
    const TURN_STD : f32 = 15.;

    const _CPG_TIME : f32 = 1.;

    pub fn normal(&mut self) {
        self.low = Self::LOW;
        self.high = Self::HIGH;
        self.alpha = Self::ALPHA;

        self.turn_mean = Self::TURN_MEAN;
        self.turn_std = Self::TURN_STD;
    }

    pub fn avoid(&mut self) {
        // prefer
        //self.low = 2. * Self::HIGH;
        //self.high = 2. * Self::HIGH;
        //self.alpha = 1.;

        //self.turn_mean = 2. * Self::TURN_MEAN;
        //self.turn_std = Self::TURN_STD;

        // self.low = Self::LOW;
        // self.high = 2. * Self::HIGH;
        // self.alpha = Self::ALPHA;

        // self.turn_mean = 2. * Self::TURN_MEAN;
        // self.turn_std = Self::TURN_STD;

        // self.low = Self::LOW;
        // self.high = 2. * Self::HIGH;
        // self.alpha = 3.;

        // self.turn_mean = 2. * Self::TURN_MEAN;
        // self.turn_std = Self::TURN_STD;

        self.low = Self::HIGH;
        self.high = 2. * Self::HIGH;
        self.alpha = Self::ALPHA;

        self.turn_mean = Self::TURN_MEAN;
        self.turn_std = Self::TURN_STD;
    }

    pub fn avoid_turn(&mut self) {
        self.low = Self::LOW;
        self.high = 0.5 * Self::HIGH;
        self.alpha = 1.;

        self.turn_mean = 2. * Self::TURN_MEAN;
        self.turn_std = 3. * Self::TURN_STD;
    }

    pub fn prefer(&mut self) {
        self.low = Self::LOW;
        self.high = Self::HIGH;
        self.alpha = Self::ALPHA;
    }

    pub fn is_turn(&self) -> bool {
        self.is_turn
    }

    pub fn update(
        &mut self,
        body: &mut Body
    ) {
        if ! body.locomotion().is_idle() {
            return;
        }

        let random = random();
        let mean = self.turn_mean;
        let std = self.turn_std;

        let angle = mean + (random_normal() * std).clamp(-2. * std, 2. * std);

        // bounded pareto as approximation of Levy walk
        let low = self.low;
        let high = self.high; // 4.;
        let alpha = self.alpha;

        // semi-brownian
        if self.is_turn {
            self.is_turn = false;

            let len = random_pareto(low, high, alpha);

            let action = Action::new(len, 1., Angle::Unit(0.));

            body.locomotion_mut().action(action);
        } else if random <= 0.5 {
            self.is_turn = true;

            let action = Action::new(1., 1., Angle::Deg(angle));
            body.locomotion_mut().action(action);
            // tectum.toward().action_copy(Turn::Left)
        } else {
            self.is_turn = true;

            let action = Action::new(1., 1., Angle::Deg(-angle));
            body.locomotion_mut().action(action);
            // tectum.toward().action_copy(Turn::Right)
        }
    }
}

impl FromStore for MidExplore {
    fn init(_store: &mut Store) -> Self {
        MidExplore {
            low: Self::LOW,
            high: Self::HIGH,
            alpha: Self::ALPHA,

            turn_mean: Self::TURN_MEAN,
            turn_std: Self::TURN_STD,

            is_turn: false,
        }
    }
}
