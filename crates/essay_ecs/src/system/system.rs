use crate::world::prelude::World;

pub trait System: 'static {
    type Out;

    fn run(&mut self, world: &World) -> Self::Out;
}

pub trait IntoSystem<Out, Marker>: Sized {
    type System:System<Out=Out> + 'static;

    fn into_system(this: Self, world: &mut World) -> Self::System;
}
/*
impl<'w, Out, Sys: System + 'static> IntoSystem<Out, Sys> for Sys {
    type System = Sys;
    fn into_system(this: Self, _world: &mut World) -> Sys {
        this
    }
}
*/
