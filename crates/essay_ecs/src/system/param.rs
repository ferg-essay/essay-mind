use crate::world::prelude::World;

//
// Param
//
 
pub trait Param {
    type Arg<'a>;

    fn get_arg<'w>(world: &'w World) -> Self::Arg<'w>;
}

//
// Param composed of tuples
//

macro_rules! impl_param_tuple {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($param: Param),*> Param for ($($param,)*)
        {
            type Arg<'w> = ($($param::Arg<'w>,)*);

            fn get_arg<'w>(world: &'w World) -> Self::Arg<'w> {
                ($($param::get_arg(world),)*)
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

pub type Arg<'w, P> = <P as Param>::Arg<'w>;
