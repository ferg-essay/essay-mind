use std::fmt;

use essay_ecs::{app::{App, Plugin, Startup}, core::{Commands, Component}};

use crate::util::Point;

#[derive(Component)]
pub struct Odor<T: OdorType> {
    pos: Point,
    r: f32,
    odor: T,
}

impl<T: OdorType> Odor<T> {
    pub const RADIUS: f32 = 3.;

    pub(super) fn new_r(x: usize, y: usize, r:usize, odor: T) -> Self {
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
        self.odor.innate().is_food()
    }

    pub fn odor(&self) -> &T {
        &self.odor
    }
}

impl<T: OdorType> fmt::Debug for Odor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Food")
        .field(&self.x())
        .field(&self.y())
        .field(&self.odor())
        .finish()
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum OdorInnate {
    None,
    Food,
    Avoid,
}

impl OdorInnate {
    #[inline]
    pub fn is_food(&self) -> bool {
        match self {
            OdorInnate::Food => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_avoid(&self) -> bool {
        match self {
            OdorInnate::Avoid => true,
            _ => false,
        }
    }
}

// #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub trait OdorType : Clone + fmt::Debug + Send + Sync + 'static {
    fn innate(&self) -> OdorInnate;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OdorKind {
    None,
    FoodA,
    FoodB,
    AvoidA,
    AvoidB,
    OtherA,
    Bogus,
}

impl Default for OdorKind {
    fn default() -> Self {
        OdorKind::None
    }
}

impl OdorType for OdorKind {
    fn innate(&self) -> OdorInnate {
        match self {
            OdorKind::None => OdorInnate::None,
            OdorKind::FoodA => OdorInnate::Food,
            OdorKind::FoodB => OdorInnate::Food,
            OdorKind::AvoidA => OdorInnate::Avoid,
            OdorKind::AvoidB => OdorInnate::Avoid,
            _ => OdorInnate::None,
        }
    }
}

pub struct OdorPlugin {
    odors: Vec<OdorItem<OdorKind>>,
}

impl OdorPlugin {
    pub fn new() -> Self {
        Self {
            odors: Vec::new(),
        }
    }

    pub fn odor(&mut self, x: usize, y: usize, odor: OdorKind) -> &mut Self {
        // assert!(x < self.width);
        // assert!(y < self.height);

        let r = Odor::<OdorKind>::RADIUS as usize;

        self.odors.push(OdorItem::new(x, y, r, odor));

        self
    }

    pub fn odor_r(&mut self, x: usize, y: usize, r: usize, odor: OdorKind) -> &mut Self {
        // assert!(x < self.width);
        // assert!(y < self.height);

        self.odors.push(OdorItem::new(x, y, r, odor));

        self
    }
}

impl Plugin for OdorPlugin {
    fn build(&self, app: &mut App) {
        let mut odors : Vec<Odor<OdorKind>> = self.odors.iter().map(|odor| {
            Odor::new_r(odor.pos.0, odor.pos.1, odor.r, odor.odor)
        }).collect();
    
        app.system(Startup, move |mut cmd: Commands| {
            for odor in odors.drain(..) {
                cmd.spawn(odor);
            }
        });
    }
}

struct OdorItem<T: OdorType> {
    pos: (usize, usize),
    r: usize,
    odor: T,
}

impl<T: OdorType> OdorItem<T> {
    fn new(x: usize, y: usize, r: usize, odor: T) -> Self {
        Self { 
            pos: (x, y), 
            r,
            odor 
        }
    }
}
