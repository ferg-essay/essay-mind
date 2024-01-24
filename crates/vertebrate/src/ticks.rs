pub struct Ticks(pub usize);

impl Ticks {
    pub const TICKS_PER_SECOND : usize = 10;

    #[inline]
    pub fn ticks(&self) -> usize {
        self.0
    }
}

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