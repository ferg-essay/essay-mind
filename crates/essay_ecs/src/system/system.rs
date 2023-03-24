use crate::world::prelude::World;

pub trait System: 'static {
    fn run(&mut self, world: &World);
}

pub trait IntoSystem<Marker>: Sized {
    type System:System + 'static;

    fn into_system(this: Self) -> Self::System;
}

impl<Sys: System + 'static> IntoSystem<Sys> for Sys {
    type System = Sys;
    fn into_system(this: Self) -> Sys {
        this
    }
}
