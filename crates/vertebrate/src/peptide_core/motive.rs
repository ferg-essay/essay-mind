use std::{marker::PhantomData, ops::{Deref, DerefMut}};

use essay_ecs::{app::{Plugin, App, PreUpdate}, core::{ResMut, store::FromStore, Store}};
use mind_ecs::PreTick;

use crate::util::DecayValue;

pub struct Motive<T: MotiveTrait> {
    value: DecayValue,
    delta: f32,

    marker: PhantomData<T>,
}

impl<T: MotiveTrait> Motive<T> {
    fn new(half_life: usize) -> Self {
        Self {
            value: DecayValue::new(half_life),
            delta: 0.,
            marker: Default::default(),
        }
    }

    fn update(&mut self) {
        self.value.update();
        self.value.add(self.delta);
        self.delta = 0.;
    }

    pub fn add(&mut self, value: f32) {
        self.delta = value;
    }
}

impl<T: MotiveTrait> Default for Motive<T> {
    fn default() -> Self {
        Self {
            value: DecayValue::new(Motives::HALF_LIFE),
            delta: 0.,
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

pub struct Orexin;
impl MotiveTrait for Orexin {}

pub struct Roam;
impl MotiveTrait for Roam {}

pub struct Dwell;
impl MotiveTrait for Dwell {}

pub struct Seek;
impl MotiveTrait for Seek {}

pub struct Surprise;
impl MotiveTrait for Surprise {}


pub struct Motives;

impl Motives {
    const HALF_LIFE : usize = 10;

    pub fn insert<T: MotiveTrait>(app: &mut App) {
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
