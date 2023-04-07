use std::{marker::PhantomData, ops::{DerefMut, Deref}};

use crate::{world::prelude::World, prelude::Param};

//
// Fiber In/Out - external [`system`] api
//

pub trait In {
    type Arg<'a>;

    fn get_arg<'w>(world: &'w World) -> Self::Arg<'w>;
}
/*
pub struct In<'w, F:Fiber, M=()> {
    world: &'w World<'w>,
    fiber: F,
    marker: PhantomData<M>,
}

pub struct Out<'w, F:Fiber, M=()> {
    world: &'w World<'w>,
    fiber: F,
    marker: PhantomData<M>,
}
 */

//
// Fiber internal driver api
//

pub trait Fiber {
    type In;
    type Out;

    fn create_in<'w>(&self, world: &World<'w>) -> &Self::In;
    fn create_in_mut<'w>(&mut self, world: &World<'w>) -> &mut Self::In;
    fn create_out<'w>(&self, world: &World<'w>) -> &Self::Out;
    fn create_out_mut<'w>(&mut self, world: &World<'w>) -> &mut Self::Out;
}

//
// In implementation
//
/*
impl<'w, F:Fiber, M> Deref for In<'w, F, M> {
    type Target = F::In;

    fn deref(&self) -> &Self::Target {
        self.fiber.create_in(self.world)
    }
}

impl<'w, F:Fiber, M> DerefMut for In<'w, F, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.fiber.create_in_mut(self.world)
    }
}

impl<'a, F:Fiber, M> Param for In<'a, F, M> {
    type Arg<'w> = F::In; // ResMut<'w, T>;

    fn get_arg<'w>(world: &'w World) -> F::In { // ResMut<'w, T> {
        todo!()
    }
}
 */

//
// Out implementation
//
/*
impl<'w, F:Fiber, M> Deref for Out<'w, F, M> {
    type Target = F::Out;

    fn deref(&self) -> &Self::Target {
        self.fiber.create_out(self.world)
    }
}

impl<'w, F:Fiber, M> DerefMut for Out<'w, F, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.fiber.create_out_mut(self.world)
    }
}
 */
/*
impl<'a, F:Fiber, M=()> Param for Out<'a, F, M> {
    type Arg<'w> = F::Out; // ResMut<'w, T>;

    fn get_arg<'w>(world: &'w World) -> F::Out { // ResMut<'w, T> {
        ResMut {
            world: world,
            marker: PhantomData,
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::{prelude::App, world::prelude::World};

    use super::{In, Fiber};

    #[test]
    fn test() {
        let mut app = App::new();
        //app.add_system(test_a);
        //app.add_system(test_b);
    }
    /*
    fn test_a(ticker: &mut TickerA, out: Out<FiberA>) {
        println!("test_a");
        //out.write("test_a".to_string());
    }

    fn test_b(ticker: &mut TickerB, mut input: In<FiberA>) {
        println!("test_b {:?}", input.read());
    }
     */

    struct TickerA;
    struct TickerB;
    struct FiberA;
    struct InFiberA;
    struct OutFiberA;

    struct Tag<const T:char> {
        //marker: PhantomData<T>,
    }

    impl InFiberA {
        fn read(&mut self) -> String {
            "value".to_string()
        }
    }

    impl OutFiberA {
        fn write(&mut self, value: String) {
            println!("write {:?}", value);
        }
    }

    impl Fiber for FiberA {
        type In = InFiberA;
        type Out = OutFiberA;

        fn create_in<'w>(&self, world: &World<'w>) -> &Self::In {
            todo!()
        }

        fn create_out<'w>(&self, world: &World<'w>) -> &Self::Out {
            todo!()
        }

        fn create_in_mut<'w>(&mut self, world: &World<'w>) -> &mut Self::In {
            todo!()
        }

        fn create_out_mut<'w>(&mut self, world: &World<'w>) -> &mut Self::Out {
            todo!()
        }
    }
}