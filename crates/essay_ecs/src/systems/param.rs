use std::ops::{Deref, DerefMut};

use crate::{world::{World}, schedule::SystemMeta};

//
// Param
//
 
pub trait Param {
    type Arg<'w, 's>;
    type State;

    fn init(world: &mut World, meta: &mut SystemMeta) -> Self::State;

    fn arg<'w,'s>(
        world: &'w World,
        state: &'s mut Self::State, 
    ) -> Self::Arg<'w, 's>;

    fn flush(_world: &mut World, _state: &mut Self::State) {
    }
}

pub type Arg<'w, 's, P> = <P as Param>::Arg<'w, 's>;

//
// Local param
//

pub struct Local<'s, T:Default + 'static>(pub(crate) &'s mut T);

impl<'s, T:Default+'static> Deref for Local<'s, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'s, T:Default+'static> DerefMut for Local<'s, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a, T:Default + 'static> Param for Local<'a, T> {
    type State = T;
    type Arg<'w, 's> = Local<'s, T>;

    fn init(_world: &mut World, _meta: &mut SystemMeta) -> Self::State {
        // let exl = std::sync::Exclusive::new(T::default());
        T::default()
    }

    fn arg<'w, 's>(
        _world: &'w World, 
        state: &'s mut Self::State, 
    ) -> Self::Arg<'w, 's> {
        Local(state)
    }

    fn flush(_world: &mut World, _state: &mut Self::State) {
    }
}

//
// Param composed of tuples
//

macro_rules! impl_param_tuple {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($param: Param,)*> Param for ($($param,)*)
        {
            type Arg<'w, 's> = ($($param::Arg<'w, 's>,)*);
            type State = ($(<$param as Param>::State,)*);

            fn init(
                world: &mut World, 
                meta: &mut SystemMeta
            ) -> Self::State {
                ($($param::init(world, meta),)*)
            }

            fn arg<'w, 's>(
                world: &'w World,
                state: &'s mut Self::State,
            ) -> Self::Arg<'w, 's> {
                let ($($param,)*) = state;

                ($($param::arg(world, $param),)*)
            }

            fn flush(
                world: &mut World, 
                state: &mut Self::State
            ) {
                let ($($param,)*) = state;

                $(
                    $param::flush(world, $param);
                )*
            }
        }
    }
}

impl_param_tuple!();
impl_param_tuple!(P1);
impl_param_tuple!(P1, P2);
impl_param_tuple!(P1, P2, P3);
impl_param_tuple!(P1, P2, P3, P4);
impl_param_tuple!(P1, P2, P3, P4, P5);
impl_param_tuple!(P1, P2, P3, P4, P5, P6);
impl_param_tuple!(P1, P2, P3, P4, P5, P6, P7);

#[cfg(test)]
mod tests {
    use crate::{world::{World, ResMut}, prelude::App, schedule::Schedule};

    use super::Local;


    #[test]
    fn test_local() {
        let mut world = World::new();
        world.insert_resource::<String>("none".to_string());

        let mut schedule = Schedule::new();
        schedule.add_system(local_system);

        schedule.run(&mut world);
        assert_eq!(world.resource::<String>(), "local(1)");

        schedule.run(&mut world);
        assert_eq!(world.resource::<String>(), "local(2)");
    }

    fn local_system(mut local: Local<usize>, mut value: ResMut<String>) {
        *local = *(local.0) + 1;
        *value = format!("local({})", local.0);
    }

}