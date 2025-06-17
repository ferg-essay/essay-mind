use super::Ticks;

pub struct Timeout {
    timeout: u32,
    last_ticks: u64,

    // config
    threshold: u32,

}

impl Timeout {
    ///
    /// half_life in ticks
    /// 
    pub fn new(timeout: impl Into<Ticks>) -> Self {
        Self {
            threshold: timeout.into().ticks() as u32,
            timeout: 0,
            last_ticks: 0,
        }
    }

    pub fn set_timeout(mut self, timeout: impl Into<Ticks>) -> Self {
        self.threshold = timeout.into().ticks() as u32;

        self
    }

    #[inline]
    pub fn get_timeout(&self) -> u32 {
        self.threshold
    }

    #[inline]
    pub fn value(&self) -> u32 {
        self.timeout
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.timeout > 0
    }

    #[inline]
    pub fn set(&mut self) {
        self.timeout = self.threshold;
    }

    #[inline]
    pub fn set_value(&mut self, value: u32) {
        self.timeout = value;
    }

    #[inline]
    pub fn set_max(&mut self, value: u32) {
        self.timeout = value.max(self.timeout);
    }

    #[inline]
    pub fn update(&mut self) {
        self.last_ticks += 1;
        self.timeout = self.timeout.saturating_sub(1)
    }

    #[inline]
    pub fn update_ticks(&mut self, ticks: u64) {
        let delta_ticks = (ticks - self.last_ticks) as u32;
        self.last_ticks = ticks;

        self.timeout = self.timeout.saturating_sub(delta_ticks);
    }
}

pub struct TimeoutValue<V> {
    timeout: u32,
    value: Option<V>,
    last_ticks: u64,

    // config
    threshold: u32,
}

impl<V> TimeoutValue<V> {
    ///
    /// half_life in ticks
    /// 
    pub fn new(timeout: impl Into<Ticks>) -> Self {
        Self {
            threshold: timeout.into().ticks() as u32,
            timeout: 0,
            value: None,
            last_ticks: 0,
        }
    }

    pub fn set_timeout(mut self, timeout: impl Into<Ticks>) -> Self {
        self.threshold = timeout.into().ticks() as u32;

        self
    }

    #[inline]
    pub fn get_timeout(&self) -> u32 {
        self.threshold
    }

    #[inline]
    pub fn value(&self) -> &Option<V> {
        if self.is_active() {
            &self.value
        } else {
            &None
        }
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.timeout > 0
    }

    #[inline]
    pub fn set(&mut self, value: V) {
        self.timeout = self.threshold;
        self.value = Some(value);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.timeout = 0;
        self.value = None;
    }

    #[inline]
    pub fn update(&mut self) -> &Option<V> {
        self.last_ticks += 1;
        self.timeout = self.timeout.saturating_sub(1);

        if self.timeout == 0 {
            self.value = None;
        }

        &self.value
    }

    #[inline]
    pub fn update_ticks(&mut self, ticks: u64) {
        let delta_ticks = (ticks - self.last_ticks) as u32;
        self.last_ticks = ticks;

        self.timeout = self.timeout.saturating_sub(delta_ticks);

        if self.timeout == 0 {
            self.value = None;
        }
    }
}

impl<V: Clone> TimeoutValue<V> {
    #[inline]
    pub fn value_or(&self, default: V) -> V {
        if self.is_active() {
            match &self.value {
                Some(value) => value.clone(),
                None => default,
            }
        } else {
            default
        }
    }
}

impl<V> Default for TimeoutValue<V> {
    fn default() -> Self {
        Self::new(Ticks(3))
    }
}
