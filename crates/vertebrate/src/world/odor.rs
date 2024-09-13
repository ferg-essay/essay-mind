use std::fmt;

use crate::util::{Angle, EgoVector, Point};

pub struct Odors {
    odors: Vec<Odor>,
}

impl Odors {
    pub fn new() -> Self {
        Self {
            odors: Vec::new(),
        }
    }

    pub(super) fn add_odor(&mut self, x: usize, y: usize, r: usize, odor: OdorType) {
        self.odors.push(Odor::new_r(x, y, r, odor));
    }

    pub fn odor(&self, pt: Point) -> Option<(OdorType, Angle)> {
        let Point(x, y) = pt;

        let mut best_odor: Option<(OdorType, Angle)> = None;
        let mut best_dist = f32::MAX;

        for food in &self.odors {
            let dx = food.x - x;
            let dy = food.y - y;
            let dist = dx.hypot(dy);

            if dist <= food.r() && dist < best_dist {
                let angle = dy.atan2(dx);

                best_odor = Some((food.odor(), Angle::Rad(angle)));
                best_dist = dist;
            }
        }

        best_odor
    }

    pub fn odors(&self) -> &Vec<Odor> {
        &self.odors
    }

    pub fn odors_by_head(&self, point: Point) -> Vec<(OdorType, EgoVector)> {
        let mut odors = Vec::new();

        for odor in &self.odors {
            let dist = point.dist(&odor.pos());

            if dist < odor.r() {
                let angle = point.heading_to(odor.pos());
                let value = 0.5 / dist.max(0.5);

                odors.push((odor.odor(), EgoVector::new(angle, value)));
            }
        }
        
        odors
    }
}

pub struct Odor {
    x: f32,
    y: f32,
    r: f32,
    odor: OdorType,
}

impl Odor {
    pub const RADIUS: f32 = 3.;

    pub(super) fn new_r(x: usize, y: usize, r:usize, odor: OdorType) -> Self {
        Self {
            x: x as f32 + 0.5,
            y: y as f32 + 0.5,
            r: r as f32,
            odor,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn pos(&self) -> Point {
        Point(self.x, self.y)
    }

    pub fn r(&self) -> f32 {
        self.r
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
        f.debug_tuple("Food").field(&self.x).field(&self.y).finish()
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
