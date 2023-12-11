use essay_plot::api::Angle;

#[derive(Clone, Debug)]
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

    pub fn to_ego(&self, head_dir: Angle) -> DirVector {
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
}
