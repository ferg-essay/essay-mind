use std::{marker::PhantomData, ops::Deref};

use essay_ecs::{app::{App, PreUpdate}, core::ResMut};

use crate::{ticks::Ticks, util::DecayValue};

pub struct Motive<T: MotiveTrait> {
    value: DecayValue,

    delta: f32,
    max: f32,

    marker: PhantomData<T>,
}

impl<T: MotiveTrait> Motive<T> {
    fn new(half_life: usize) -> Self {
        Self {
            value: DecayValue::new(half_life),
            delta: 0.,
            max: 0.,
            marker: Default::default(),
        }
    }

    fn update(&mut self) {
        self.value.update();
        self.value.add(self.delta);
        if self.max > 0. {
            self.value.set_max(self.max);
        }
        self.delta = 0.;
        self.max = 0.;
    }

    pub fn add(&mut self, value: f32) {
        self.delta = value;
    }

    pub fn set_max(&mut self, value: f32) {
        self.max = value;
    }
}

impl<T: MotiveTrait> Default for Motive<T> {
    fn default() -> Self {
        Self {
            value: DecayValue::new(Motives::HALF_LIFE),
            delta: 0.,
            max: 0.,
            marker: Default::default(),
        }
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

pub struct Seek;
impl MotiveTrait for Seek {}

pub struct Surprise;
impl MotiveTrait for Surprise {}


pub struct Motives;

impl Motives {
    const HALF_LIFE : usize = 10;

    pub fn insert<T: MotiveTrait>(app: &mut App, half_life: impl Into<Ticks>) {
        let is_new = ! app.contains_resource::<Motive<T>>();

        let half_life : Ticks = half_life.into();
        let motive = Motive::<T>::new(half_life.ticks());

        app.insert_resource(motive);

        if is_new {
            app.system(PreUpdate, 
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
