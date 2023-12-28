use super::Angle;

#[derive(Clone, Copy, Debug)]
pub struct DirVector {
    dir: Angle,
    value: f32,
}

impl DirVector {
    pub fn new(dir: Angle, value: f32) -> Self {
        Self {
            dir,
            value
        }
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            dir: Angle::Unit(0.),
            value: 0.,
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.value.abs() < 1.0e-6
    }

    #[inline]
    pub fn value(&self) -> f32 {
        self.value
    }

    #[inline]
    pub fn dir(&self) -> Angle {
        self.dir
    }

    #[inline]
    pub fn to_unit(&self) -> f32 {
        self.dir.to_unit()
    }

    #[inline]
    pub fn to_radians(&self) -> f32 {
        self.dir.to_radians()
    }

    #[inline]
    pub fn cos(&self) -> f32 {
        self.dir.cos()
    }

    #[inline]
    pub fn sin(&self) -> f32 {
        self.dir.sin()
    }

    #[inline]
    pub fn dx(&self) -> f32 {
        self.dir.cos()
    }

    #[inline]
    pub fn dy(&self) -> f32 {
        self.dir.sin()
    }

    pub fn to_ego(&self, head_dir: Angle) -> DirVector {
        Self {
            dir: Angle::Unit(self.dir().to_unit() - head_dir.to_unit()),
            value: self.value
        }
    }

    #[inline]
    pub fn to_approach(&self, head_dir: Angle) -> DirVector {
        Self {
            dir: Angle::Unit(self.dir().to_unit() - head_dir.to_unit()),
            value: self.value
        }
    }

    pub(crate) fn scale(&self, diff: f32) -> DirVector {
        Self {
            dir: self.dir,
            value: self.value * diff,
        }
    }

    #[inline]
    pub fn max(&self, avoid_dir: DirVector) -> DirVector {
        if self.value() < avoid_dir.value() {
            avoid_dir
        } else {
            *self
        }
    }
}
