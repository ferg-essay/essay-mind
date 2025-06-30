use std::marker::PhantomData;

use mind_ecs::AppTick;

use crate::{hippocampus::Engram64, util::{lru_cache::LruCache, Seconds, Ticks}};

use super::MosaicType;

pub struct Striatum<T> {
    left: StriatumSide<T>,
    right: StriatumSide<T>,
}

impl<T> Striatum<T> {
    pub fn new() -> Self {
        Self {
            left: StriatumSide::new(),
            right: StriatumSide::new(),
        }
    }

    pub fn timeout(&mut self, time: impl Into<Ticks>) -> &mut Self {
        let time = time.into();

        self.left.timeout(time);
        self.right.timeout(time);

        self
    }

    pub fn recover(&mut self, time: impl Into<Ticks>) -> &mut Self {
        let time = time.into();

        self.left.recover(time);
        self.right.recover(time);

        self
    }

    pub fn left_mut(&mut self) -> &mut StriatumSide<T> {
        &mut self.left
    }

    pub fn right_mut(&mut self) -> &mut StriatumSide<T> {
        &mut self.right
    }
}

pub struct StriatumSide<T> {
    timeout: f32,
    recover: f32,

    init_threshold: f32,

    active_gap: Ticks,

    cache: LruCache<Engram64, Item>,

    engram: Option<Engram64>,
    _next_engram: Option<Engram64>,

    marker: PhantomData<fn(T)>,
}

//unsafe impl<T: MosaicType> Sync for Striatum<T> {}

impl<T> StriatumSide<T> {
    const BUILDUP : f32 = 25.;
    const DECAY : f32 = 1.5 * Self::BUILDUP;

    pub fn new() -> Self {
        let ltd : Ticks = Seconds(Self::BUILDUP).into();
        let decay : Ticks = Seconds(Self::DECAY).into();

        Self {
            timeout: 1. / ltd.ticks().max(1) as f32,
            recover: 1. / decay.ticks().max(1) as f32,

            active_gap: Ticks(3),
            init_threshold: 0.,

            cache: LruCache::new(16),

            engram: None,
            _next_engram: None,

            marker: Default::default(),
        }
    }

    pub fn timeout(&mut self, time: impl Into<Ticks>) -> &mut Self {
        let time: Ticks = time.into();
        self.timeout = 1. / time.ticks().max(1) as f32;

        self
    }

    pub fn recover(&mut self, time: impl Into<Ticks>) -> &mut Self {
        let time: Ticks = time.into();
        self.recover = 1. / time.ticks().max(1) as f32;

        self
    }

    pub fn init_threshold(&mut self, init_threshold: f32) -> &mut Self {
        assert!(0. <= init_threshold && init_threshold <= 1.);

        self.init_threshold = init_threshold;

        self
    }

    pub fn active(&mut self, tick: &AppTick) -> StriatumValue2 {
        let now = tick.ticks();

        let engram = self.get_engram();

        let mut entry = self.cache.get_or_insert(engram, || Item::new());

        entry.write(|v| {
            let last_time = v.last_time;
            v.last_time = now;
        
            // continuation of active
            if (now - v.last_active) < self.active_gap.ticks() as u64 {
                v.timeout = (v.timeout + self.timeout * (now - last_time) as f32).min(1.);

                if v.timeout < 1. {
                    v.last_active = now;
                    StriatumValue2::Active
                } else {
                    StriatumValue2::Timeout
                }
            } else {
                // decay timeout since last time
                v.timeout = (v.timeout - (now - last_time) as f32 * self.recover).max(0.);

                if v.timeout <= self.init_threshold {
                    v.last_active = now;
                    v.timeout = (v.timeout + self.timeout).min(1.);

                    StriatumValue2::Active
                } else {
                    StriatumValue2::Timeout
                }
            }
        })
    }

    pub fn state(&mut self, tick: &AppTick) -> StriatumValue2 {
        let now = tick.ticks();

        let engram = self.get_engram();

        if let Some(entry) = self.cache.get(engram) {
            entry.read(|v| {
                // continuation of active
                if (now - v.last_active) < self.active_gap.ticks() as u64 {
                    let timeout = (v.timeout + self.timeout).min(1.);

                    if timeout < 1. {
                        StriatumValue2::Active
                    } else {
                        StriatumValue2::Timeout
                    }
                } else {
                    // decay timeout since last time
                    let timeout = (v.timeout - (now - v.last_time) as f32 * self.recover).max(0.);

                    if timeout <= self.init_threshold {
                        StriatumValue2::None
                    } else {
                        StriatumValue2::Timeout
                    }
                }
            })
        } else {
            StriatumValue2::None
        }
    }

    fn get_engram(&self) -> Engram64 {
        self.engram.map_or(Engram64::default(), |v| v.clone())
    }

    pub fn is_active(&mut self, tick: &AppTick) -> bool {
        self.active(tick) == StriatumValue2::Active
    } 
}

struct Item {
    timeout: f32,

    last_active: u64,
    last_time: u64,
}

impl Item {
    fn new() -> Self {
        Self {
            timeout: 0.,
            last_active: 0,
            last_time: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StriatumValue2 {
    None,
    Active,
    Timeout,
}
