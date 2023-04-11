use std::{marker::PhantomData};

use crate::{world::prelude::World, store::{prelude::Query}, entity::prelude::IsEntity};

use super::{prelude::Param, system::{System, IntoSystem}, param::Arg};

// IsEach prevents collisions
pub struct IsEach;

pub struct EachSystem<M, F>
where
    F: EachFun<M>
{
    fun: F,
    marker: PhantomData<M>,
}

pub trait EachFun<M> {
    type Entity<'w>:Query<IsEntity>;
    type Params: Param;

    fn run<'a,'w>(&mut self, 
        world: &World<'w>,
        entity: <Self::Entity<'w> as Query<IsEntity>>::Item<'w>, // <'a>, 
        param: Arg<Self::Params>
    );
}

//
// Implementation
//

impl<M, F:'static> EachSystem<M, F>
where
    F: EachFun<M>
{
    fn new<'w>(_world: &mut World<'w>, fun: F) -> Self {

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
    fn run<'w>(&mut self, world: &World<'w>) {
        for entity in world.query::<F::Entity<'w>>() {
            let args = F::Params::get_arg(
                world,
            );

            self.fun.run(world, entity, args);
        }
    }
}    

impl<M, F:'static> IntoSystem<(M,IsEach)> for F
where
    M: 'static,
    F: EachFun<M>
{
    type System = EachSystem<M, F>;

    fn into_system(this: Self, world: &mut World) -> Self::System {
        EachSystem::new(world, this)
    }
}

//
// EachFun: function system matching
//
pub struct IsPlain;
/*
impl<F:'static,T:Query<IsEntity>,P:Param,> EachFun<fn(IsPlain, T, P)> for F
    where for<'w> F:FnMut(T, P,) -> () +
            FnMut(T::Item<'w>, Arg<P>,) -> ()
{
    type Entity<'w> = T;
    type Params = (P,);

    fn run<'b,'w>(&mut self, world: &World<'w>, entity: T::Item<'w>, arg: Arg<(P,)>) {
        let (p1,) = arg;
        self(entity, p1,)
    }
}
*/

macro_rules! impl_each_function {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<F:'static, T:Query<IsEntity>, $($param: Param),*> EachFun<fn(IsPlain, T, $($param,)*)> for F
        where for<'w> F:FnMut(T, $($param),*) -> () +
            FnMut(T::Item<'w>, $(Arg<$param>),*) -> ()
        {
            type Entity<'w> = T;
            type Params = ($($param,)*);

            fn run<'b,'w>(&mut self, _world: &'w World, entity: T::Item<'b>, arg: Arg<($($param,)*)>) {
                let ($($param,)*) = arg;
                self(entity, $($param,)*)
            }
        }
    }
}

impl_each_function!();
impl_each_function!(P1);
impl_each_function!(P1, P2);
impl_each_function!(P1, P2, P3);
impl_each_function!(P1, P2, P3, P4);
impl_each_function!(P1, P2, P3, P4, P5);
impl_each_function!(P1, P2, P3, P4, P5, P6);
impl_each_function!(P1, P2, P3, P4, P5, P6, P7);

#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell, marker::PhantomData, any::type_name};

    use essay_ecs_macros::Component;

    use crate::{app::App, world::prelude::World, system::param::Param};

    #[test]
    fn test_each() {
        let mut app = App::new();

        app.spawn(TestA(1));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));

        //app.add_system(system_each_ref);

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

        app.spawn((TestA(3), TestB(4)));

        app.update();
        assert_eq!(take(&values), "TestA(1), TestA(2), TestA(3)");
    }

    #[test]
    fn test_each_ref() {
        let mut app = App::new();

        app.spawn(TestA(1));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));

        //app.add_system(system_each_ref);

        let ptr = values.clone();
        app.add_system(move |t :&TestA| {
            ptr.borrow_mut().push(format!("{:?}", t));
        });

        app.update();
        assert_eq!(take(&values), "TestA(1)");
    }

    #[test]
    fn test_each_a_b() {
        let mut app = App::new();

        app.spawn(TestA(1));
        app.spawn(TestB(2));
        app.spawn((TestA(3),TestB(4)));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));
        let ptr = values.clone();

        app.add_system(move |t :&mut TestA| {
            ptr.borrow_mut().push(format!("a-{:?}", t));
        });

        let ptr = values.clone();
        app.add_system(move |t :&mut TestB| {
            ptr.borrow_mut().push(format!("b-{:?}", t));
        });

        app.update();
        assert_eq!(take(&values), "a-TestA(1), a-TestA(3), b-TestB(2), b-TestB(4)");
    }

    #[test]
    fn test_each_tuple() {
        let mut app = App::new();

        app.spawn(TestA(1));
        app.spawn(TestB(2));
        app.spawn((TestA(3),TestB(4)));
        app.spawn((TestB(5),TestA(6)));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));
        let ptr = values.clone();

        app.add_system(move |a:(&TestA, &TestB)| {
            ptr.borrow_mut().push(format!("{:?}", a));
        });

        app.update();
        assert_eq!(take(&values), "(TestA(3), TestB(4))");
    }

    #[test]
    fn test_each_tuple_rev() {
        let mut app = App::new();

        app.spawn(TestA(1));
        app.spawn(TestB(2));
        app.spawn((TestA(3),TestB(4)));
        app.spawn((TestB(5),TestA(6)));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));
        let ptr = values.clone();

        app.add_system(move |a:(&TestB, &TestA)| {
            ptr.borrow_mut().push(format!("{:?}", a));
        });

        app.update();
        assert_eq!(take(&values), "(TestA(3), TestB(4))");
    }

    #[test]
    fn test_each_mut() {
        let mut app = App::new();
        /*
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
        */
    }

    #[test]
    fn test_two_each() {
        let mut app = App::new();
        /*
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
        */
    }

    #[test]
    fn test_each_res() {
        /*
        let mut app = App::new();
        app.spawn(TestA(0));
        app.add_resource("hello".to_string());
        */
        /*
        let values = Rc::new(RefCell::new(Vec::<String>::new()));

        let ptr = values.clone();
        app.add_system(move |t :&mut TestA, name: TestArg<String>| {
            ptr.borrow_mut().push(format!("S-A {:?} {:?}", t, name.name));
        });

        app.update();
        assert_eq!(take(&values), "S-A TestA(0) \"alloc::string::String\"");
        */
    }

    #[test]
    fn test_each_in() {
        /*
        let mut app = App::new();
        let ent_ref = app.spawn(TestA(1));
        ent_ref.push(&mut app, TestInFiber(2));


        // app.add_system(system_each_in);

        app.update();

        app.spawn(TestA(3));
        
        app.update();
        // assert_eq!(take(&values), "S-A TestA(0) \"alloc::string::String\"");
        */
    }
    /*
    fn system_each_in(test: &mut TestA, input: In<TestFiber>) {
        println!("system-each-in {:?} {:?}", test, Deref::deref(&input));
    }
    */
    /*
    fn system_each_ref(test: &mut TestA) {
        println!("system-each {:?}", test);
    }
    */

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
}