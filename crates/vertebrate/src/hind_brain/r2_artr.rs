use std::ops;

use crate::util::{DecayValue, Seconds, Turn};

use super::SerotoninTrait;

// Karpenko et al 2020 - ARTR oscillator 20s period
pub struct ArtrR2 {
    side: Side,
    time: DecayValue,
}

impl ArtrR2 {
    pub const TIMEOUT: Seconds = Seconds(20.);
    pub const TURN: Turn = Turn::Unit(0.2 / 3.);

    pub(super) fn new() -> Self {
        Self {
            side: Side::Left,
            time: DecayValue::new(Self::TIMEOUT),
        }
    }

    pub(super) fn update(&mut self) {
        self.time.update();

    }

    pub fn side(&self) -> Option<Side> {
        if self.time.is_active() {
            Some(self.side)
        } else {
            None
        }
    }

    pub(super) fn next_turn(&self) -> Option<Turn> {
        if self.time.is_active() {
            match self.side {
                Side::Left => Some(- Self::TURN),
                Side::Right => Some(Self::TURN),
            }
        } else {
            // self.time.set_max(1.);

            match self.side {
                Side::Left => Some(Self::TURN),
                Side::Right => Some(- Self::TURN),
            }
        }
    }

    pub(super) fn on_turn(&mut self, turn: Turn) {
        if turn.to_unit().abs() < 1e-2 {
        } else if turn.to_unit() < 0. {
            self.side = Side::Left;

            if ! self.time.is_active() {
                self.time.set_max(1.);
            }
        } else {
            self.side = Side::Right;

            if ! self.time.is_active() {
                self.time.set_max(1.);
            }
        }
    }
}

// todo: remove this and dependants
impl SerotoninTrait for ArtrR2 {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Left,
    Right
}

impl ops::Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}
