use std::{marker::PhantomData, ops::Deref, sync::atomic::{AtomicBool, AtomicU32, Ordering}};

use essay_ecs::{app::{App, PreUpdate}, core::ResMut};

use crate::util::{DecayValue, HalfLife, Ticks};

pub struct Serotonin<T: SerotoninTrait> {
    value: DecayValue,

    excite: AtomicU32,
    inhibit: AtomicU32,
    max: AtomicU32,
    is_clear: AtomicBool,

    marker: PhantomData<T>,
}

impl<T: SerotoninTrait> Serotonin<T> {
    fn new(half_life: impl Into<HalfLife>) -> Self {
        Self {
            value: DecayValue::new(half_life),

            excite: Default::default(),
            inhibit: Default::default(),

            max: Default::default(),
            is_clear: Default::default(),

            marker: Default::default(),
        }
    }

    #[inline]
    pub fn set_threshold(&mut self, threshold: f32) -> &mut Self {
        self.value.set_threshold(threshold);

        self
    }

    #[inline]
    pub fn set_half_life(&mut self, half_life: impl Into<HalfLife>) -> &mut Self {
        self.value.set_half_life(half_life);

        self
    }

    #[inline]
    pub fn value(&self) -> f32 {
        self.value.value()
    }

    #[inline]
    pub fn active_value(&self) -> f32 {
        self.value.active_value()
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.active_value() > 0.
    }

    #[inline]
    pub fn excite(&mut self, value: f32) {
        let value = (value.clamp(0., 1.) * u16::MAX as f32) as u32;

        self.excite.fetch_add(value, Ordering::Relaxed);
    }

    #[inline]
    pub fn inhibit(&mut self, value: f32) {
        let value = (value.clamp(0., 1.) * u16::MAX as f32) as u32;

        self.inhibit.fetch_add(value, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_max(&mut self, value: f32) {
        let value = (value.clamp(0., 1.) * u16::MAX as f32) as u32;

        self.max.fetch_max(value, Ordering::Relaxed);
    }

    fn update(&mut self) {
        self.value.update();

        let excite = self.excite.swap(0, Ordering::SeqCst);
        let inhibit = self.inhibit.swap(0, Ordering::SeqCst);
        let max = self.max.swap(0, Ordering::SeqCst);
        let is_clear = self.is_clear.swap(false, Ordering::SeqCst);

        let excite = (excite as f32 / u16::MAX as f32).min(1.);
        let inhibit = (inhibit as f32 / u16::MAX as f32).min(1.);
        let max = (max as f32 / u16::MAX as f32).min(1.);

        let delta = (excite - inhibit).clamp(0., 1.);

        self.value.add(delta);

        if max > 0. {
            self.value.set_max(max);
        }

        if is_clear {
            self.value.set(0.);
        }
    }
}

impl<T: SerotoninTrait> Default for Serotonin<T> {
    fn default() -> Self {
        Self::new(SerotoninManager::HALF_LIFE)
    }
}

impl<T: SerotoninTrait> Deref for Serotonin<T> {
    type Target = DecayValue;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub trait SerotoninTrait : Sync + Send + 'static {
}

pub struct SerotoninManager;

impl SerotoninManager {
    const HALF_LIFE : HalfLife = HalfLife(1.);

    pub fn insert<T: SerotoninTrait>(app: &mut App, half_life: impl Into<Ticks>) {
        let is_new = ! app.contains_resource::<Serotonin<T>>();

        let half_life : Ticks = half_life.into();
        let motive = Serotonin::<T>::new(half_life);

        app.insert_resource(motive);

        if is_new {
            app.system(PreUpdate, 
                move |mut motive: ResMut<Serotonin<T>>| {
                    motive.update();
            });
        }
    }

    pub fn init<T: SerotoninTrait>(app: &mut App) {
        if ! app.contains_resource::<Serotonin<T>>() {
            let motive = Serotonin::<T>::new(SerotoninManager::HALF_LIFE);

            app.insert_resource(motive);

            app.system(PreUpdate, 
                move |mut motive: ResMut<Serotonin<T>>| {
                    motive.update();
            });
        }
    }
}
