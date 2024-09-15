use crate::util::{DecayValue, HalfLife};

pub struct BasalForebrain {
    half_life: HalfLife,

    items: Vec<AttentionItem>,
    max: f32,
    threshold_low: f32,
    threshold_high: f32,
}

impl BasalForebrain {
    pub const HALF_LIFE : HalfLife = HalfLife(2.);
    pub const THRESHOLD_LOW : f32 = 0.1;
    pub const THRESHOLD_HIGH : f32 = 0.98;

    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            half_life: Self::HALF_LIFE,
            max: 0.,
            threshold_low: 0.,
            threshold_high: 0.,
        }
    }

    pub fn push(&mut self) -> AttendId {
        let id = AttendId(self.items.len());
        self.items.push(AttentionItem::new(self.half_life));

        id
    }

    #[inline]
    pub fn is_attend(&self, id: AttendId) -> Attend {
        let value = self.items[id.i()].value();

        if self.threshold_high < value {
            Attend::Attend
        } else if self.threshold_low <= value {
            Attend::Normal
        } else {
            Attend::Ignore
        }
    }

    #[inline]
    pub fn attend(&self, id: AttendId) -> f32 {
        match self.is_attend(id) {
            Attend::Attend => 1.0,
            Attend::Normal => 0.5,
            Attend::Ignore => 0.1,
        }
    }

    pub fn pre_update(&mut self) {
        for item in &mut self.items {
            item.pre_update();
        }
    }

    #[inline]
    pub fn add(&mut self, id: AttendId, value: f32) {
        assert!(0. <= value && value <= 1.);

        self.items[id.i()].add(value);
    }

    pub fn update(&mut self) {
        self.max = 0.;

        for item in &mut self.items {
            self.max = self.max.max(item.value());
        }

        if self.max < 0.1 {
            self.max = 0.;
        }

        self.threshold_low = self.max * Self::THRESHOLD_LOW;
        self.threshold_high = self.max * Self::THRESHOLD_LOW;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Attend {
    Attend,
    Normal,
    Ignore,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AttendId(usize);

impl AttendId {
    #[inline]
    pub fn i(&self) -> usize {
        self.0
    }
}

pub struct AttendValue {
    pub attend: f32, 
    pub value: f32,
}

impl AttendValue {
    pub fn new(value: f32, attend: f32) -> Self {
        Self {
            attend,
            value
        }
    }
}

struct AttentionItem {
    value: DecayValue,
}

impl AttentionItem {
    fn new(half_life: impl Into<HalfLife>) -> Self {
        Self {
            value: DecayValue::new(half_life),
        }
    }

    #[inline]
    fn pre_update(&mut self) {
        self.value.update();
    }

    #[inline]
    fn add(&mut self, value: f32) {
        self.value.add(value);
    }

    #[inline]
    fn value(&self) -> f32 {
        self.value.value()
    }
}
