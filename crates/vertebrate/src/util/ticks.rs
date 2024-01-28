#[derive(Clone, Copy, Debug)]
pub struct Ticks(pub usize);

impl Ticks {
    pub const TICKS_PER_SECOND : usize = 10;

    #[inline]
    pub fn ticks(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Seconds(pub f32);

impl Into<Ticks> for Seconds {
    #[inline]
    fn into(self) -> Ticks {
        let seconds = self.0;

        if seconds > 0. {
            Ticks((self.0 * Ticks::TICKS_PER_SECOND as f32).max(1.) as usize)
        } else {
            Ticks(0)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HalfLife(pub f32);

impl HalfLife {
    #[inline]
    pub fn ticks(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn decay(&self) -> f32 {
        if self.0 > 0. {
            (- 2.0f32.ln() / self.0).exp()
        } else {
            0.
        }
    }
}

impl Into<HalfLife> for Seconds {
    #[inline]
    fn into(self) -> HalfLife {
        let seconds = self.0;

        HalfLife(self.0 * Ticks::TICKS_PER_SECOND as f32)
    }
}

impl Into<HalfLife> for Ticks {
    #[inline]
    fn into(self) -> HalfLife {
        HalfLife(self.0 as f32)
    }
}

impl Into<HalfLife> for f32 {
    #[inline]
    fn into(self) -> HalfLife {
        let seconds = self;

        HalfLife(seconds * Ticks::TICKS_PER_SECOND as f32)
    }
}
