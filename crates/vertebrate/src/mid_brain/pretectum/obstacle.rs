use essay_ecs::{app::{App, Plugin}, core::ResMut};
use mind_ecs::Tick;

use crate::{
    hind_brain::HindMove, 
    util::{DecayValue, Ticks, Turn}
};

fn obstacle_update(
    mut obstacle: ResMut<ObstaclePretectum>,
    mut hind_move: ResMut<HindMove>
) {
    obstacle.update(hind_move.get_mut());
}

pub struct ObstaclePretectum {
    threshold: f32, 
    startle: f32,

    obstacle_left: DecayValue,
    obstacle_right: DecayValue,

    is_enable: bool,
}

impl ObstaclePretectum {
    const THRESHOLD : f32 = 0.5;
    const STARTLE : f32 = 0.9;

    const TURN : Turn = Turn::Unit(0.10);
    const _U_TURN : Turn = Turn::Unit(0.40);

    fn new() -> Self {
        Self {
            threshold: Self::THRESHOLD,
            startle: Self::STARTLE,

            obstacle_left: DecayValue::new(Ticks(2)),
            obstacle_right: DecayValue::new(Ticks(2)),

            is_enable: false,
        }
    }

    pub fn left(&self) -> f32 {
        if self.is_enable { self.obstacle_left.value() } else { 0. }
    }

    pub fn right(&self) -> f32 {
        if self.is_enable { self.obstacle_right.value() } else { 0. }
    }

    pub fn forward(&self) -> f32 {
        if self.is_enable { 
            self.obstacle_left.value().min(self.obstacle_right.value())
        } else { 
            0. 
        }
    }

    pub fn set_max_left(&mut self, value: f32) {
        self.obstacle_left.set_max(value);
    }

    pub fn set_max_right(&mut self, value: f32) {
        self.obstacle_right.set_max(value);
    }

    fn update(&mut self, hind_move: &mut HindMove) {
        let left = self.left();
        let right = self.right();

        if self.startle <= left {
            hind_move.startle().escape_left(left);
        }

        if self.startle <= right {
            hind_move.startle().escape_right(right);
        }

        if self.threshold < left || self.threshold < right {
            if right < left {
                hind_move.optic().escape(Self::TURN);
                hind_move.set_ss_left(0.75);
            } else {
                hind_move.optic().escape(- Self::TURN);
                hind_move.set_ss_right(0.75);
            }
        }

        self.obstacle_left.update();
        self.obstacle_right.update();
    }
}

pub struct ObstaclePretectumPlugin {
    is_enable: bool,
}

impl ObstaclePretectumPlugin {
    pub fn new() -> Self {
        Self {
            is_enable: true,
        }
    }

    pub fn enable(&mut self, is_enable: bool) -> &mut Self {
        self.is_enable = is_enable;

        self
    }
}

impl Plugin for ObstaclePretectumPlugin {
    fn build(&self, app: &mut App) {
        let mut obstacle = ObstaclePretectum::new();

        obstacle.is_enable = self.is_enable;
        app.insert_resource(obstacle);

        if self.is_enable {
            app.system(Tick, obstacle_update);
        }
    }
}
