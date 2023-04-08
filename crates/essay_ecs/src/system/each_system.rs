use std::{marker::PhantomData, ops::{Deref, DerefMut}};

use crate::{world::prelude::World, entity::prelude::Insert};

use super::{prelude::Param, system::{System, IntoSystem}, param::Arg};


pub type EntityArg<'w, P> = <P as EachParam>::Arg<'w>;

//
// EntityParam - parameters specific to an entity
//
 
pub trait EachParam {
    type Entity;
    type Arg<'a>;

    fn get_arg<'w>(world: &'w World, entity: &'w Self::Entity) -> Self::Arg<'w>;
}

//
// EachSystem - a system implemented by a function
// 

pub struct Each<'w, T> {
    world: &'w World<'w>,
    item: &'w mut T,
}
pub struct In<'w, F:Fiber, M=()> {
    world: &'w World<'w>,
    fiber: &'w mut F::In,
    marker: PhantomData<M>,
}

impl<'w, F:Fiber> In<'w,F> {
    fn new(world: &'w World, fiber_in: &'w mut F::In) -> Self {
        Self {
            world: world,
            fiber: fiber_in,
            marker: PhantomData,
        }
    }
}

impl<'w,F:Fiber,M> Deref for In<'w,F,M> {
    type Target = F::In;

    fn deref(&self) -> &Self::Target {
        self.fiber
    }
}

pub struct Out<'w, F:Fiber, M=()> {
    world: &'w World<'w>,
    fiber: &'w mut F::Out,
    marker: PhantomData<M>,
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

    fn create_in<'w>(&self, world: &World<'w>) -> &Self::In;
    fn create_in_mut<'w>(&mut self, world: &World<'w>) -> &mut Self::In;
}

pub trait EachFun<M> {
    type Entity: Insert;
    //type EachParams: EachParam;
    type Params: Param;

    fn run<'w>(&mut self, 
        world: &'w World<'w>,
        each: &'w mut Self::Entity, 
        param: Arg<Self::Params>
    );
}

pub struct EachSystem<M, F>
where
    F: EachFun<M>
{
    fun: F,
    marker: PhantomData<M>,
}

impl<'w, T> Each<'w, T> {
    fn get(&self) -> &T {
        &self.item
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.item
    }
}

impl<M, F:'static> EachSystem<M, F>
where
    F: EachFun<M>
{
    fn new(world: &mut World, fun: F) -> Self {
        let entity_type = world.add_entity_type::<F::Entity>();
        println!("entity-type {:?}", entity_type);
        Self {
            fun: fun,
            marker: PhantomData,
        }
    }
}

impl<M, F:'static> System for EachSystem<M, F>
where
    M: 'static,
    F: EachFun<M>
{
    fn run(&mut self, world: &World) {
        for entity in world.iter_mut::<F::Entity>() {
            let args = F::Params::get_arg(
                world,
            );

            /*
            let each = Each {
                world: world,
                item: each,
            };
             */
    
            self.fun.run(world, entity, args);
        }
    }
}    

// IsFun prevents collision
pub struct IsEach;

impl<M, F:'static> IntoSystem<(M,IsEach)> for F
where
    M: 'static,
    F: EachFun<M>
{
    type System = EachSystem<M, F>;

    fn into_system(this: Self, world: &mut World) -> Self::System {
        EachSystem::new(world, this)
        /* {
            fun: this,
            marker: Default::default()
        }*/
    }
}

//
// Function matching
//
pub struct IsPlain;
pub struct IsIn;
pub struct IsOut;

impl<F:'static,T:Insert,P:Param,> EachFun<fn(IsPlain, T, P)> for F
    where F:FnMut(&mut T, P) -> () +
            FnMut(&mut T, Arg<P>) -> ()
{
    type Entity = T;
    type Params = P;

    fn run(&mut self, world: &World, entity: &mut T, arg: Arg<P>) {
        self(entity, arg)
    }
}
/*
impl<F:'static,T:'static,FB:Fiber,P:Param,> EachFun<fn(IsIn, T, FB, P)> for F
    where F:FnMut(&mut T, In<FB>, P) -> () +
            FnMut(&mut T, In<FB>, Arg<P>) -> ()
{
    type Entity = (T, FB::In);
    type Params = P;

    fn run(&mut self, world: &World, entity: &mut (T, FB::In), arg: Arg<P>) {
        self(&mut entity.0, In::<FB>::new(world, &mut entity.1), arg)
    }
}
*/

    /*
impl<F:'static,T:Component,FB:Fiber,> EachFun<fn(IsIn, T, FB)> for F
    where F:FnMut(&mut T, In<FB>) -> () +
            FnMut(&mut T, In<FB>) -> ()
{
    type Entity = (T, FB::In);
    type Params = ();

    fn run(&mut self, world: &World, entity: &mut (T, FB::In), arg: Arg<()>) {
        self(&mut entity.0, In::<FB>::new(world, &mut entity.1))
    }
}
     */

    /*
impl<F:'static,T:'static,FB:Fiber,P:Param,> EachFun<fn(IsOut, T, FB, P)> for F
    where F:FnMut(&mut T, Out<FB>, P) -> () +
            FnMut(&mut T, Out<FB>, Arg<P>) -> ()
{
    type Entity = (T, FB::Out);
    type Params = P;

    fn run(&mut self, world: &World, entity: &mut (T, FB::Out), arg: Arg<P>) {
        self(&mut entity.0, Out::<FB>::new(world, &mut entity.1), arg)
    }
}
     */

macro_rules! impl_each_function {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<F: 'static, T: Insert, $($param: Param),*> EachFun<fn(IsPlain, T, $($param,)*)> for F
        where F:FnMut(&mut T, $($param),*) -> () +
            FnMut(&mut T, $(Arg<$param>),*) -> ()
        {
            type Entity = T;
            type Params = ($($param),*);

            fn run(&mut self, world: &World, each: &mut T, arg: Arg<($($param,)*)>) {
                let ($($param,)*) = arg;
                self(each, $($param,)*)
            }
        }
    }
}

impl_each_function!();
//impl_each_function!(P1);
impl_each_function!(P1, P2);
impl_each_function!(P1, P2, P3);
impl_each_function!(P1, P2, P3, P4);
impl_each_function!(P1, P2, P3, P4, P5);
impl_each_function!(P1, P2, P3, P4, P5, P6);
impl_each_function!(P1, P2, P3, P4, P5, P6, P7);

#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell, marker::PhantomData, any::type_name, ops::Deref};

    use essay_ecs_macros::Component;

    use crate::{app::App, world::prelude::World, system::param::Param};

    use super::{Each, In, Fiber};

    #[test]
    fn test_each() {
        let mut app = App::new();
        app.spawn(TestA(1));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));
        let ptr = values.clone();

        app.add_system(move |t :&mut TestA| {
            ptr.borrow_mut().push(format!("{:?}", t));
        });

        app.update();
        assert_eq!(take(&values), "TestA(1)");

        app.update();
        assert_eq!(take(&values), "TestA(1)");

        app.spawn(TestA(2));

        app.update();
        assert_eq!(take(&values), "TestA(1), TestA(2)");

        app.update();
        assert_eq!(take(&values), "TestA(1), TestA(2)");
    }

    #[test]
    fn test_each_mut() {
        let mut app = App::new();
        app.spawn(TestA(0));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));
        let ptr = values.clone();

        app.add_system(move |t :&mut TestA| {
            t.0 += 1;
            ptr.borrow_mut().push(format!("{:?}", t));
        });

        app.update();
        assert_eq!(take(&values), "TestA(1)");

        app.update();
        assert_eq!(take(&values), "TestA(2)");

        app.spawn(TestA(0));

        app.update();
        assert_eq!(take(&values), "TestA(3), TestA(1)");

        app.update();
        assert_eq!(take(&values), "TestA(4), TestA(2)");
    }

    #[test]
    fn test_two_each() {
        let mut app = App::new();
        app.spawn(TestA(0));
        app.spawn(TestB(0));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));

        let ptr = values.clone();
        app.add_system(move |t :&mut TestA| {
            ptr.borrow_mut().push(format!("S-A {:?}", t));
        });

        let ptr = values.clone();
        app.add_system(move |t :&mut TestB| {
            ptr.borrow_mut().push(format!("S-B {:?}", t));
        });

        app.update();
        assert_eq!(take(&values), "S-A TestA(0), S-B TestB(0)");

        app.update();
        assert_eq!(take(&values), "S-A TestA(0), S-B TestB(0)");

        app.spawn(TestA(0));

        app.update();
        assert_eq!(take(&values), "S-A TestA(0), S-A TestA(0), S-B TestB(0)");

        app.update();
        assert_eq!(take(&values), "S-A TestA(0), S-A TestA(0), S-B TestB(0)");
    }

    #[test]
    fn test_each_res() {
        let mut app = App::new();
        app.spawn(TestA(0));
        app.add_resource("hello".to_string());

        let values = Rc::new(RefCell::new(Vec::<String>::new()));

        let ptr = values.clone();
        app.add_system(move |t :&mut TestA, name: TestArg<String>| {
            ptr.borrow_mut().push(format!("S-A {:?} {:?}", t, name.name));
        });

        app.update();
        assert_eq!(take(&values), "S-A TestA(0) \"alloc::string::String\"");
    }

    #[test]
    fn test_each_in() {
        let mut app = App::new();
        let ent_ref = app.spawn(TestA(1));
        ent_ref.push(&mut app, TestInFiber(2));

        // app.add_system(system_each_in);

        app.update();

        app.spawn(TestA(3));
        
        app.update();
        // assert_eq!(take(&values), "S-A TestA(0) \"alloc::string::String\"");
    }

    fn system_each_in(test: &mut TestA, input: In<TestFiber>) {
        println!("system-each-in {:?} {:?}", test, Deref::deref(&input));
    }

    fn take(values: &Rc<RefCell<Vec<String>>>) -> String {
        let v : Vec<String> = values.borrow_mut().drain(..).collect();

        v.join(", ")
    }

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

    #[derive(Debug)]
    struct TestInFiber(usize);

    struct TypeOutFiber(usize);

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

        fn get_arg<'w>(world: &'w World) -> Self::Arg<'w> {
            Self {
                name: type_name::<V>().to_string(),
                marker: PhantomData,
            }
        }
    }
}