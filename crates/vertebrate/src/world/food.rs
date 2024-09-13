use essay_ecs::core::Component;

use crate::util::Point;

#[derive(Component, Debug)]
pub struct Food {
    pos: Point,
}

impl Food {
    pub(super) fn new(pos: impl Into<Point>) -> Self {
        Self {
            pos: pos.into(),
        }
    }

    #[inline]
    pub fn pos(&self) -> Point {
        self.pos
    }

    #[inline]
    pub fn is_pos(&self, pos: impl Into<Point>) -> bool {
        self.pos.dist(pos) < 0.3
    }
}

