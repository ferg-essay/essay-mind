#[derive(Clone, Copy, Debug)]
pub struct Ticks(pub usize);

impl Ticks {
    pub const TICKS_PER_SECOND : usize = 10;

    #[inline]
    pub fn ticks(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn to_seconds(&self) -> f32 {
        self.0 as f32 / Self::TICKS_PER_SECOND as f32
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Seconds(pub f32);

impl Seconds {
    #[inline]
    pub fn v(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn max(&self, y: impl Into<Seconds>) -> Self {
        Self(self.0.max(y.into().0))
    }
}

impl Into<Seconds> for f32 {
    #[inline]
    fn into(self) -> Seconds {
        Seconds(self)
    }
}

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

impl Into<Seconds> for Ticks {
    #[inline]
    fn into(self) -> Seconds {
        let ticks = self.0;

        if ticks > 0 {
            Seconds(self.0 as f32 / Ticks::TICKS_PER_SECOND as f32)
        } else {
            Seconds(0.)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HalfLife(pub f32);

impl HalfLife {
    #[inline]
    pub fn ticks(&self) -> f32 {
        self.0 * Ticks::TICKS_PER_SECOND as f32
    }

    #[inline]
    pub fn seconds(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn decay(&self) -> f32 {
        if self.0 > 0. {
            let ticks = self.ticks();

            (- 2.0f32.ln() / ticks).exp()
        } else {
            0.
        }
    }
}

impl Into<HalfLife> for Seconds {
    #[inline]
    fn into(self) -> HalfLife {
        HalfLife(self.0)
    }
}

impl Into<HalfLife> for Ticks {
    #[inline]
    fn into(self) -> HalfLife {
        HalfLife(self.0 as f32 / Ticks::TICKS_PER_SECOND as f32)
    }
}

impl Into<HalfLife> for f32 {
    #[inline]
    fn into(self) -> HalfLife {
        HalfLife(self)
    }
}


pub struct TickDelta {
    last_ticks: u64,
}

impl TickDelta {
    pub fn new() -> Self {
        Self {
            last_ticks: 0
        }
    }

    pub fn update(&mut self, ticks: u64) -> u64 {
        let delta = ticks - self.last_ticks;
        self.last_ticks = ticks;

        delta
    }
}

impl Default for TickDelta {
    fn default() -> Self {
        Self {
            last_ticks: 0,
        }
    }
}
