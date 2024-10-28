use essay_ecs::core::Component;
use util::random::Rand32;

use crate::util::Point;

#[derive(Component, Debug, Clone)]
pub struct Food {
    pos: Point,
    kind: FoodKind,
    value: f32,
    probability: f32,
}

impl Food {
    pub(super) fn new(pos: impl Into<Point>) -> Self {
        Self {
            pos: pos.into(),
            kind: FoodKind::Plain,
            value: f32::MAX,
            probability: 1.,
        }
    }

    pub(super) fn set_kind(mut self, kind: FoodKind) -> Self {
        self.kind = kind;

        self
    }

    pub(super) fn _set_probability(mut self, p: f32) -> Self {
        self.probability = p;

        self
    }

    #[inline]
    pub fn pos(&self) -> Point {
        self.pos
    }

    #[inline]
    pub fn kind(&self) -> FoodKind {
        self.kind
    }

    #[inline]
    pub fn is_pos(&self, pos: impl Into<Point>) -> bool {
        self.pos.dist(pos) < 0.4
    }

    ///
    /// Stochastic food eating
    /// 
    #[inline]
    pub fn eat_probability(&mut self) -> bool {
        if self.value >= 1. && Rand32::new().next_uniform() <= self.probability {
            self.value -= 1.;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FoodKind {
    None,
    Plain,
    Sweet,
    Bitter,
    Sick,
}

impl Default for FoodKind {
    fn default() -> Self {
        FoodKind::None
    }
}

