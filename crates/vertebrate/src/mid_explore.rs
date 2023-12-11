use essay_ecs::core::Store;
use essay_ecs::core::store::FromStore;
use essay_plot::prelude::Angle;
use util::random::{random_pareto, random, random_normal};
use crate::{body::{Action, Body}, util::DirVector};

pub struct MidExplore {
    alpha: f32,
    low: f32,
    high: f32,

    turn_mean: f32,
    turn_std: f32,

    avoid_left: f32,
    avoid_right: f32,
    avoid_forward: f32,

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

    pub fn add_avoid(&mut self, avoid_dir: DirVector) {
        if avoid_dir.value() > 0.05 {
            let offset = 2. * avoid_dir.sin(); //  * avoid_dir.value();

            self.avoid_left = offset.clamp(0., 1.);
            self.avoid_right = (- offset).clamp(0., 1.);
            self.avoid_forward = avoid_dir.cos().clamp(0., 1.);
        } else {
            self.avoid_left = 0.;
            self.avoid_right = 0.;
            self.avoid_forward = 0.;
        }
    }

    pub fn avoid_forward(&self) -> f32 {
        self.avoid_forward
    }

    pub fn avoid_left(&self) -> f32 {
        self.avoid_left
    }

    pub fn avoid_right(&self) -> f32 {
        self.avoid_right
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

        self.turn_mean = 0.5 * Self::TURN_MEAN;
        self.turn_std = 0.5 * Self::TURN_STD;
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
        let mut mean = self.turn_mean;
        let mut std = self.turn_std;

        if self.avoid_forward > 0. && random_normal().abs() < self.avoid_forward {
            mean = 2. * Self::TURN_MEAN;
            std = 3. * Self::TURN_STD;
        }

        let angle = mean + (random_normal() * std).clamp(-2. * std, 2. * std);

        // bounded pareto as approximation of Levy walk
        let low = self.low;
        let high = self.high; // 4.;
        let alpha = self.alpha;

        let p_left = (1. - self.avoid_left) / (2. - self.avoid_left - self.avoid_right);

        // semi-brownian
        if self.is_turn {
            self.is_turn = false;

            let len = random_pareto(low, high, alpha);

            let action = Action::new(len, 1., Angle::Unit(0.));

            body.locomotion_mut().action(action);
        } else if random <= p_left {
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

            avoid_left: 0.,
            avoid_right: 0.,
            avoid_forward: 0.,

            is_turn: false,
        }
    }
}
