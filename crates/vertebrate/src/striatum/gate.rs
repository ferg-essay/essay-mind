use std::{marker::PhantomData, sync::Mutex};

use essay_ecs::{app::{App, PreUpdate}, core::ResMut};

use crate::util::DecayValue;

pub struct StriatumGate<T: Gate> {
    value: Mutex<DecayValue>,

    marker: PhantomData<T>,
}

impl<T: Gate> StriatumGate<T> {
    pub fn init(app: &mut App) {
        let is_new = ! app.contains_resource::<StriatumGate<T>>();

        if is_new {
            let gate = Self::default();

            app.insert_resource(gate);

            app.system(PreUpdate, 
                move |mut gate: ResMut<StriatumGate<T>>| {
                    gate.update();
            });
        }
    }

    #[inline]
    pub fn open(&self, value: f32) {
        self.value.lock().unwrap().add(value);
    }

    #[inline]
    pub fn close(&self) {
        self.value.lock().unwrap().subtract(1.);
    }

    ///
    /// Activation value [0-1] where 0. is fully closed and 1. is fully open.
    /// 
    #[inline]
    pub fn value(&self) -> f32 {
        self.value.lock().unwrap().value()
    }

    fn update(&self) {
        self.value.lock().unwrap().update();
    }
}

impl<T: Gate> Default for StriatumGate<T> {
    fn default() -> Self {
        Self { 
            marker: Default::default(), 
            value: Mutex::new(DecayValue::new(0.2).set_rest_value(0.1)),
        }
    }
}

pub trait Gate : Sync + Send + 'static {
}