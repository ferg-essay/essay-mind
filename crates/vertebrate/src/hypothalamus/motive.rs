use std::{marker::PhantomData, ops::Deref};

use essay_ecs::{app::{App, PreUpdate}, core::ResMut};
use mind_ecs::PreTick;

use crate::util::{DecayValue, HalfLife, Ticks};

pub struct Motive<T: MotiveTrait> {
    value: DecayValue,

    delta: f32,
    max: f32,
    is_clear: bool,
    threshold: f32,

    marker: PhantomData<T>,
}

impl<T: MotiveTrait> Motive<T> {
    fn new(half_life: impl Into<HalfLife>) -> Self {
        Self {
            value: DecayValue::new(half_life),
            delta: 0.,
            max: 0.,
            is_clear: false,
            threshold: 0.125,
            marker: Default::default(),
        }
    }

    fn update(&mut self) {
        self.value.update();
        self.value.add(self.delta);

        if self.max > 0. {
            self.value.set_max(self.max);
        }

        if self.is_clear {
            self.value.set(0.);
        }

        self.delta = 0.;
        self.max = 0.;
        self.is_clear = false;
    }

    #[inline]
    pub fn add(&mut self, value: f32) {
        self.delta = value;
    }

    #[inline]
    pub fn set_max(&mut self, value: f32) {
        self.max = value;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.is_clear = true;
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.value() > self.threshold
    }
}

impl<T: MotiveTrait> Default for Motive<T> {
    fn default() -> Self {
        Self::new(Motives::HALF_LIFE)
    }
}


impl<T: MotiveTrait> Deref for Motive<T> {
    type Target = DecayValue;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub trait MotiveTrait : Sync + Send + 'static {
}

pub struct Surprise;
impl MotiveTrait for Surprise {}


pub struct Motives;

impl Motives {
    const HALF_LIFE : HalfLife = HalfLife(1.);

    pub fn insert<T: MotiveTrait>(app: &mut App, half_life: impl Into<Ticks>) {
        let is_new = ! app.contains_resource::<Motive<T>>();

        let half_life : Ticks = half_life.into();
        let motive = Motive::<T>::new(half_life);

        app.insert_resource(motive);

        if is_new {
            app.system(PreTick, 
                move |mut motive: ResMut<Motive<T>>| {
                    motive.update();
            });
        }
    }

    pub fn init<T: MotiveTrait>(app: &mut App) {
        if ! app.contains_resource::<Motive<T>>() {
            let motive = Motive::<T>::new(Motives::HALF_LIFE);

            app.insert_resource(motive);

            app.system(PreUpdate, 
                move |mut motive: ResMut<Motive<T>>| {
                    motive.update();
            });
        }
    }
}
