use std::{marker::PhantomData, ops::{DerefMut, Deref}, any::type_name};

use crate::{world::prelude::World, prelude::{Param, IntoSystem, System}, entity::prelude::{Query, QueryBuilder, QueryCursor, Insert, InsertBuilder, InsertCursor}};

use super::param::Arg;

pub struct Out<'w, F:Fiber, M=()> {
    world: &'w World<'w>,
    fiber: &'w mut F::Out,
    marker: PhantomData<M>,
}

pub trait Fiber {
    type In:'static;
    type Out:'static;

    fn create_in<'w>(&self, world: &World<'w>) -> &Self::In;
    fn create_in_mut<'w>(&mut self, world: &World<'w>) -> &mut Self::In;
    fn create_out<'w>(&self, world: &World<'w>) -> &Self::Out;
    fn create_out_mut<'w>(&mut self, world: &World<'w>) -> &mut Self::Out;
}

pub trait FiberIn {
    type In:'static;

    //fn create_in<'w>(&self, world: &World<'w>) -> &Self::In;
    fn get(&self) -> &Self::In;
    fn get_mut(&mut self) -> &mut Self::In;
}

type FiberInBox<T> = Box<dyn FiberIn<In=T>>;

pub struct In<'a, F:Fiber> {
    fiber: &'a mut FiberInBox<F::In>,
}

pub struct EachFiberSystem<M, F>
where
    F: EachFiberFun<M>
{
    fun: F,
    marker: PhantomData<M>,
}

pub trait EachFiberFun<M> {
    type Entity<'w>: Query;
    type Fiber: Fiber;
    type Params: Param;

    fn run<'a,'w>(&mut self, 
        world: &World<'w>,
        entity: <Self::Entity<'w> as Query>::Item<'w>, // <'a>, 
        input: In<'a, Self::Fiber>,
        args: Arg<Self::Params>
    );
}

//
// Implementation
//

impl<M, F:'static> EachFiberSystem<M, F>
where
    F: EachFiberFun<M>
{
    fn new<'w>(_world: &mut World<'w>, fun: F) -> Self {

        Self {
            fun: fun,
            marker: PhantomData,
        }
    }
}

impl<M, F:'static> System for EachFiberSystem<M, F>
where
    M: 'static,
    F: EachFiberFun<M>
{
    fn run<'w>(&mut self, world: &World<'w>) {
        for (entity, input) 
        in world.query::<(F::Entity<'w>,FiberInBox<<F::Fiber as Fiber>::In>)>() {
            let args = F::Params::get_arg(
                world,
            );

            self.fun.run(world, entity, In::new(input), args);
        }
    }
}    
struct IsEachFiber;

impl<M, F:'static> IntoSystem<(M,IsEachFiber)> for F
where
    M: 'static,
    F: EachFiberFun<M>
{
    type System = EachFiberSystem<M, F>;

    fn into_system(this: Self, world: &mut World) -> Self::System {
        EachFiberSystem::new(world, this)
    }
}

impl<T:'static> Query for FiberInBox<T> {
    type Item<'t> = &'t mut FiberInBox<T>;

    fn build(builder: &mut QueryBuilder) {
        builder.add_ref::<FiberInBox<T>>();
    }

    unsafe fn query<'a,'t>(cursor: &mut QueryCursor<'a,'t>) -> Self::Item<'t> { //<'a> {
        cursor.deref_mut::<FiberInBox<T>>()
    }
}

impl<T:'static> Insert for FiberInBox<T> {
    fn build(builder: &mut InsertBuilder) {
        builder.add_column::<FiberInBox<T>>();
    }

    unsafe fn insert(cursor: &mut InsertCursor, value: Self) {
        cursor.insert(value);
    }
    /*
    type Item<'t> = &'t mut FiberInBox<T>;

    fn build(builder: &mut QueryBuilder) {
        builder.add_ref::<FiberInBox<T>>();
    }

    unsafe fn query<'a,'t>(cursor: &mut QueryCursor<'a,'t>) -> Self::Item<'t> { //<'a> {
        cursor.deref_mut::<FiberInBox<T>>()
    }
    */
}

//
// Fiber In/Out - external [`system`] api
//

impl<'a, F:Fiber> In<'a, F> {
    fn new(fiber: &'a mut FiberInBox<F::In>) -> Self {
        Self {
            fiber: fiber,
        }
    }
}

impl<'a,F:Fiber> Deref for In<'a, F> {
    type Target = F::In;

    fn deref(&self) -> &Self::Target {
        self.fiber.get()
    }
}

impl<'a,F:Fiber> DerefMut for In<'a, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.fiber.get_mut()
    }
}

impl<'w, F:Fiber> Out<'w,F> {
    fn new(world: &'w World, fiber_out: &'w mut F::Out) -> Self {
        Self {
            world: world,
            fiber: fiber_out,
            marker: PhantomData,
        }
    }
}

//
// EachFun: function system matching
//
pub struct IsPlain;

macro_rules! impl_each_in_function {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<F:'static, Fb:Fiber, T:Query, $($param: Param),*> 
        EachFiberFun<fn(IsPlain, T, Fb, $($param,)*)> for F
        where for<'w> F:FnMut(T, In<Fb>, $($param),*) -> () +
            FnMut(T::Item<'w>, In<Fb>, $(Arg<$param>),*) -> ()
        {
            type Entity<'w> = T;
            type Fiber = Fb;
            type Params = ($($param,)*);

            fn run<'b,'w>(
                &mut self, 
                _world: &World<'w>, 
                entity: T::Item<'w>, 
                input: In<'b,Self::Fiber>,
                arg: Arg<($($param,)*)>
            ) {
                let ($($param,)*) = arg;
                self(entity, input, $($param,)*)
            }
        }
    }
}

impl_each_in_function!();
impl_each_in_function!(P1);
impl_each_in_function!(P1, P2);
impl_each_in_function!(P1, P2, P3);
impl_each_in_function!(P1, P2, P3, P4);
impl_each_in_function!(P1, P2, P3, P4, P5);
impl_each_in_function!(P1, P2, P3, P4, P5, P6);
impl_each_in_function!(P1, P2, P3, P4, P5, P6, P7);

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::{prelude::App, world::prelude::World};

    use super::{In, Fiber, FiberIn, FiberInBox};

    use std::{rc::Rc, cell::RefCell, any::type_name};

    use essay_ecs_macros::Component;

    use crate::{system::param::Param};

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

    #[test]
    fn test_each_in() {
        let mut app = App::new();

        app.spawn((TestA(1), InFiberA::new_box()));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));

        //app.add_system(system_each_ref);

        let ptr = values.clone();
        app.add_system(move |t :&mut TestA, input: In<FiberA>| {
            ptr.borrow_mut().push(format!("{:?}", t));
            ptr.borrow_mut().push(format!("{:?}", input.read()));
        });

        app.update();
        assert_eq!(take(&values), "TestA(1), \"read-value\"");
    }

    fn take(values: &Rc<RefCell<Vec<String>>>) -> String {
        let v : Vec<String> = values.borrow_mut().drain(..).collect();

        v.join(", ")
    }

    #[derive(Component,PartialEq, Debug)]
    struct TestA(u32);

    #[derive(Component,PartialEq, Debug)]
    struct TestB(u32);
    
    #[derive(Debug)]
    struct TestArg<V> {
        name: String,
        marker: PhantomData<V>,
    }

    impl<V> Param for TestArg<V> {
        type Arg<'w> = TestArg<V>;

        fn get_arg<'w>(_world: &'w World) -> Self::Arg<'w> {
            Self {
                name: type_name::<V>().to_string(),
                marker: PhantomData,
            }
        }
    }
    #[derive(Debug)]
    struct TestInFiber(usize);

    struct TypeOutFiber(usize);


    struct TickerA;
    struct TickerB;
    struct FiberA;
    struct InFiberA;
    struct OutFiberA;

    struct Tag<const T:char> {
        //marker: PhantomData<T>,
    }

    impl InFiberA {
        fn new() -> Self {
            Self {
            }
        }

        fn new_box() -> FiberInBox<InFiberA> {
            Box::new(Self {
            })
        }

        fn read(&self) -> String {
            "read-value".to_string()
        }
    }

    impl FiberIn for InFiberA {
        type In = InFiberA;

        fn get(&self) -> &Self::In {
            &self
        }

        fn get_mut(&mut self) -> &mut Self::In {
            todo!()
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
    /*
    struct TestFiber {

    }
    
    impl Fiber for TestFiber {
        type In = TestInFiber;
        type Out = TypeOutFiber;

        fn create_in<'w>(&self, world: &World<'w>) -> &Self::In {
            todo!()
        }

        fn create_in_mut<'w>(&mut self, world: &World<'w>) -> &mut Self::In {
            todo!()
        }

        fn create_out<'w>(&self, world: &World<'w>) -> &Self::Out {
            todo!()
        }

        fn create_out_mut<'w>(&mut self, world: &World<'w>) -> &mut Self::Out {
            todo!()
        }   
    }
    */
}