use crate::{body::Body, util::{DecayValue, Seconds, Turn}};

use super::hind_locomotor::{Action, MoveKind};

pub struct StartleR4 {
    ss_forward: f32,
    ss_left: DecayValue,
    ss_right: DecayValue,

    next_action: Option<Action>,
}

impl StartleR4 {
    pub(super) fn new() -> Self {
        Self {
            ss_forward: 0.,
            ss_left: DecayValue::default(),
            ss_right: DecayValue::default(),
            next_action: None,
        }
    }

    pub fn escape_left(&mut self, value: f32) {
        self.ss_left.set_max(value);
    }

    pub fn escape_right(&mut self, value: f32) {
        self.ss_right.set_max(value);
    }

    pub(super) fn update(&mut self, _body: &Body) {
        let left = self.ss_left.value();
        let right = self.ss_right.value();

        let forward = left.min(right);

        self.ss_left.update();
        self.ss_right.update();

        self.next_action = if forward > 0.5 {
            Some(Action::new(MoveKind::Startle, 1., Turn::Unit(0.5), Seconds(2.0)))
        } else if left > 0.5 {
            Some(Action::new(MoveKind::Startle, 1., Turn::Unit(0.25), Seconds(1.0)))
        } else if right > 0.5 {
            Some(Action::new(MoveKind::Startle, 1., Turn::Unit(-0.25), Seconds(1.0)))
        } else {
            None
        }

        /*
        if body.is_collide_forward() {
            self.ss_forward = self.ss_forward.max(1.);
        }

        if body.is_collide_left() {
            self.ss_left = self.ss_left.max(1.);
        }

        if body.is_collide_right() {
            self.ss_right = self.ss_right.max(1.);
        }
        */
    }

    pub(super) fn ss_forward(&self) -> f32 {
        self.ss_forward
    }

    pub(super) fn ss_left(&self) -> f32 {
        self.ss_left.value()
    }

    pub(super) fn ss_right(&self) -> f32 {
        self.ss_right.value()
    }
    
    pub(super) fn next_action(&self) -> Option<Action> {
        self.next_action.clone()
    }
}
