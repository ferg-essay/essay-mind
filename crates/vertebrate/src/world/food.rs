use essay_ecs::core::Component;

use crate::util::Point;

#[derive(Component, Debug, Clone)]
pub struct Food {
    pos: Point,
    kind: FoodKind,
}

impl Food {
    pub(super) fn new(pos: impl Into<Point>) -> Self {
        Self {
            pos: pos.into(),
            kind: FoodKind::Plain,
        }
    }

    pub(super) fn set_kind(mut self, kind: FoodKind) -> Self {
        self.kind = kind;

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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FoodKind {
    Plain,
    Sweet,
    Bitter,
    Sick,
}

