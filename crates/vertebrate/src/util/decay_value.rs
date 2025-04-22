use super::{ticks::HalfLife, Ticks};

pub struct DecayValue {
    decay: f32,
    fill: f32,
    threshold: f32,
    rest_value: f32,

    value: f32,
    last_ticks: u64,
}

impl DecayValue {
    ///
    /// half_life in ticks
    /// 
    pub fn new(half_life: impl Into<HalfLife>) -> Self {
        let decay = half_life.into().decay();

        Self {
            decay,
            fill: 1., // default fill immediately //  - decay,
            threshold: 0.5,
            rest_value: 0.,
            value: 0.,
            last_ticks: 0,
        }
    }

    pub fn fill_time(mut self, ticks: impl Into<Ticks>) -> Self {
        let ticks: Ticks = ticks.into();

        self.fill = 1. / ticks.ticks().max(1) as f32;

        self
    }

    pub fn fill_decay(mut self) -> Self {
        self.fill = 1. - self.decay;

        self
    }

    #[inline]
    pub fn decay(&self) -> f32 {
        self.decay
    }

    #[inline]
    pub fn set_half_life(&mut self, half_life: impl Into<HalfLife>) {
        self.decay = half_life.into().decay();
    }

    #[inline]
    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    #[inline]
    pub fn set_threshold(&mut self, threshold: f32) -> &mut Self {
        self.threshold = threshold;

        self
    }

    #[inline]
    pub fn rest_value(&self) -> f32 {
        self.rest_value
    }

    #[inline]
    pub fn set_rest_value(mut self, value: f32) -> Self {
        self.rest_value = value;

        self
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.value() >= self.threshold
    }

    #[inline]
    pub fn value(&self) -> f32 {
        self.value
    }

    ///
    /// Active value above the threshold, scaled to [0., 1.]
    /// 
    #[inline]
    pub fn active_value(&self) -> f32 {
        (self.value - self.threshold).max(0.) / (1. - self.threshold).max(1.0e-6)
    }

    #[inline]
    pub fn add(&mut self, value: f32) {
        self.value = (self.value + self.fill * value.clamp(0., 1.))
            .clamp(0., 1.);
    }

    #[inline]
    pub fn subtract(&mut self, value: f32) {
        self.value = (self.value + self.fill * value.clamp(0., 1.))
            .clamp(0., 1.);
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
    pub fn set_max_threshold(&mut self) {
        self.value = self.value.max(self.threshold);
    }

    #[inline]
    pub fn update(&mut self) {
        self.value = (self.value - self.rest_value) * self.decay + self.rest_value;
        self.last_ticks += 1;
    }

    #[inline]
    pub fn update_ticks(&mut self, ticks: u64) {
        let delta_ticks = ticks - self.last_ticks;
        self.last_ticks = ticks;

        let decay = self.decay.powi(delta_ticks as i32);

        self.value = (self.value - self.rest_value) * decay + self.rest_value;
    }
}

impl Default for DecayValue {
    fn default() -> Self {
        Self::new(Ticks(3))
    }
}

#[cfg(test)]
mod test {
    use crate::util::{Seconds, Ticks};

    use super::DecayValue;

    #[test]
    fn half_life() {
        let hl_1 = DecayValue::new(Ticks(1));
        assert_eq!(hl_1.decay(), 0.5);

        let hl_2 = DecayValue::new(Ticks(2));
        assert_eq!(hl_2.decay(), 0.70710677);
        assert_eq!(hl_2.decay().powi(2), 0.49999997);

        let hl_10 = DecayValue::new(Ticks(10));
        assert_eq!(hl_10.decay().powi(10), 0.5);

        let hl_1s = DecayValue::new(Seconds(1.));
        assert_eq!(hl_1s.decay().powi(10), 0.5);

        let hl_100ms = DecayValue::new(Seconds(0.1));
        assert_eq!(hl_100ms.decay(), 0.5);

        let hl_0_1 = DecayValue::new(0.1);
        assert_eq!(hl_0_1.decay(), 0.5);
    }
}