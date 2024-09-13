use std::fmt;

use essay_ecs::core::Component;

use crate::util::Point;

#[derive(Component)]
pub struct Odor {
    pos: Point,
    r: f32,
    odor: OdorType,
}

impl Odor {
    pub const RADIUS: f32 = 3.;

    pub(super) fn new_r(x: usize, y: usize, r:usize, odor: OdorType) -> Self {
        Self {
            pos: Point(x as f32 + 0.5, y as f32 + 0.5),
            r: r as f32,
            odor,
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.pos.x()
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.pos.y()
    }

    #[inline]
    pub fn pos(&self) -> Point {
        self.pos
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.r
    }

    #[inline]
    pub fn contains(&self, pos: Point) -> bool {
        self.pos().dist(pos) <= self.r
    }

    pub fn is_food(&self) -> bool {
        self.odor.is_food()
    }

    pub fn odor(&self) -> OdorType {
        self.odor
    }
}

impl fmt::Debug for Odor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Food").field(&self.x()).field(&self.y()).finish()
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum OdorType {
    FoodA,
    FoodB,
    AvoidA,
    AvoidB,
    OtherA,
    OtherB,
}

impl OdorType {
    pub fn is_food(&self) -> bool {
        match self {
            OdorType::FoodA => true,
            OdorType::FoodB => true,
            _ => false,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            OdorType::FoodA => 0,
            OdorType::FoodB => 1,
            OdorType::AvoidA => 2,
            OdorType::AvoidB => 3,
            OdorType::OtherA => 4,
            OdorType::OtherB => 5,
        }
    }

    pub fn count() -> usize {
        Self::OtherB.index() + 1
    }
}

impl From<usize> for OdorType {
    fn from(value: usize) -> Self {
        match value {
            0 => OdorType::FoodA,
            1 => OdorType::FoodB,
            2 => OdorType::AvoidA,
            3 => OdorType::AvoidB,
            4 => OdorType::OtherA,
            5 => OdorType::OtherB,
            _ => todo!(),
        }
    }
}
