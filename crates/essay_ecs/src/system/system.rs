use std::{borrow::Cow, any::type_name};

use crate::{world::prelude::World, prelude::Param};

use super::param::Arg;

pub trait System: 'static {
    type Out;

    fn init(&mut self, world: &mut World);

    unsafe fn run_unsafe(&mut self, world: &World) -> Self::Out;

    fn run(&mut self, world: &mut World) -> Self::Out {
        unsafe { self.run_unsafe(world) }
    }

    fn flush(&mut self, world: &mut World);
}

pub struct SystemMeta {
    name: Cow<'static, str>,
}

pub struct SystemState<P:Param + 'static> {
    meta: SystemMeta,
    state: P::State,
}

impl SystemMeta {
    pub(crate) fn new<P>() -> SystemMeta {
        Self {
            name: type_name::<P>().into(),
        }
    }
}

pub trait IntoSystem<Out, Marker>: Sized {
    type System:System<Out=Out> + 'static;

    fn into_system(this: Self) -> Self::System;
}

impl<P:Param> SystemState<P> {
    pub fn new(world: &mut World) -> Self {
        let mut meta = SystemMeta::new::<P>();
        let state = P::init(world, &mut meta);

        Self {
            meta: meta,
            state: state,
        }
    }

    pub fn get<'s, 'w>(&'s mut self, world: &'w World) -> Arg<P>
        where P: Param
    {
        todo!();
    }

}