pub struct DecayValue {
    decay: f32,
    fill: f32,

    value: f32,
}

impl DecayValue {
    ///
    /// half_life in ticks
    /// 
    pub fn new(half_life: usize) -> Self {
        let decay = if half_life > 0 {
            (- 2.0f32.ln() / half_life as f32).exp()
        } else {
            0.
        };

        Self {
            decay,
            fill: 1. - decay,
            value: 0.,
        }
    }

    pub fn fill_time(mut self, half_life: usize) -> Self {
        let decay = if half_life > 0 {
            (- 2.0f32.ln() / half_life as f32).exp()
        } else {
            0.
        };

        self.fill = 1. - decay;

        self
    }

    #[inline]
    pub fn decay(&self) -> f32 {
        self.decay
    }

    #[inline]
    pub fn value(&self) -> f32 {
        self.value
    }

    #[inline]
    pub fn add(&mut self, value: f32) {
        self.value += self.fill * value.clamp(0., 1.);
        self.value = self.value.clamp(0., 1.);
    }

    #[inline]
    pub fn subtract(&mut self, value: f32) {
        self.value -= self.fill * value.clamp(0., 1.);
        self.value = self.value.clamp(0., 1.);
    }

    #[inline]
    pub fn set(&mut self, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.value = value;
    }

    #[inline]
    pub fn set_max(&mut self, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.value = self.value.max(value);
    }

    #[inline]
    pub fn update(&mut self) {
        self.value = self.value * self.decay;
    }
}

#[cfg(test)]
mod test {
    use super::DecayValue;

    #[test]
    fn half_life() {
        let hl_1 = DecayValue::new(1);
        assert_eq!(hl_1.decay(), 0.5);

        let hl_2 = DecayValue::new(2);
        assert_eq!(hl_2.decay(), 0.70710677);
        assert_eq!(hl_2.decay().powi(2), 0.49999997);

        let hl_10 = DecayValue::new(10);
        assert_eq!(hl_10.decay().powi(10), 0.5);
    }
}