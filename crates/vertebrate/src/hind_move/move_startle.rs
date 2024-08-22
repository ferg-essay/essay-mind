use crate::{body::Body, util::{Seconds, Turn}};

use super::move_hind::{Action, MoveKind};

pub struct StartleMrs {
    ss_forward: f32,
    ss_left: f32,
    ss_right: f32,
}

impl StartleMrs {
    pub(super) fn new() -> Self {
        Self {
            ss_forward: 0.,
            ss_left: 0.,
            ss_right: 0.,
        }
    }

    pub(super) fn update(&mut self, body: &Body) {
        self.ss_forward = 0.;
        self.ss_left = 0.;
        self.ss_right = 0.;

        if body.is_collide_forward() {
            self.ss_forward = self.ss_forward.max(1.);
        }

        if body.is_collide_left() {
            self.ss_left = self.ss_left.max(1.);
        }

        if body.is_collide_right() {
            self.ss_right = self.ss_right.max(1.);
        }
    }

    pub(super) fn ss_forward(&self) -> f32 {
        self.ss_forward
    }

    pub(super) fn ss_left(&self) -> f32 {
        self.ss_left
    }

    pub(super) fn ss_right(&self) -> f32 {
        self.ss_right
    }
    
    pub(super) fn next_action(&self) -> Option<Action> {
        if self.ss_forward > 0.5 {
            Some(Action::new(MoveKind::Startle, 1., Turn::Unit(0.5), Seconds(2.0)))
        } else if self.ss_left > 0.5 {
            Some(Action::new(MoveKind::Startle, 1., Turn::Unit(0.25), Seconds(1.0)))
        } else if self.ss_right > 0.5 {
            Some(Action::new(MoveKind::Startle, 1., Turn::Unit(-0.25), Seconds(1.0)))
        } else {
            None
        }
    }
}
