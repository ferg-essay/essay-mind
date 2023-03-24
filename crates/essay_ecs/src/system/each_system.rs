use std::marker::PhantomData;

use crate::world::prelude::World;

use super::{prelude::Param, system::{System, IntoSystem}, param::Arg};


//
// EachSystem - a system implemented by a function
// 

pub trait EachFun<M> {
    type Item;
    type Params: Param;

    fn run(&mut self, each: &mut Self::Item, param: Arg<Self::Params>);
}

pub struct Each<'w, T> {
    world: &'w World<'w>,
    item: &'w mut T,
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

impl<M, F:'static> System for EachSystem<M, F>
where
    M: 'static,
    F: EachFun<M>
{
    fn run(&mut self, world: &World) {
        for each in world.iter_mut::<F::Item>() {
            let args = F::Params::get_arg(
                world,
            );

            /*
            let each = Each {
                world: world,
                item: each,
            };
             */
    
            self.fun.run(each, args);
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

    fn into_system(this: Self) -> Self::System {
        EachSystem {
            fun: this,
            marker: Default::default()
        }
    }
}

//
// Function matching
//

impl<F:'static,T:'static,P:Param,> EachFun<fn(T, P)> for F
    where F:FnMut(&mut T, P) -> () +
            FnMut(&mut T, Arg<P>) -> ()
{
    type Item = T;
    type Params = P;

    fn run(&mut self, each: &mut T, arg: Arg<P>) {
        self(each, arg)
    }
}

macro_rules! impl_system_function {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<F: 'static, T: 'static, $($param: Param),*> EachFun<fn(T, $($param,)*)> for F
        where F:FnMut(&mut T, $($param),*) -> () +
            FnMut(&mut T, $(Arg<$param>),*) -> ()
        {
            type Item = T;
            type Params = ($($param),*);

            fn run(&mut self, each: &mut T, arg: Arg<($($param,)*)>) {
                let ($($param,)*) = arg;
                self(each, $($param,)*)
            }
        }
    }
}

impl_system_function!();
//impl_system_function!(P1);
impl_system_function!(P1, P2);
impl_system_function!(P1, P2, P3);
impl_system_function!(P1, P2, P3, P4);
impl_system_function!(P1, P2, P3, P4, P5);
impl_system_function!(P1, P2, P3, P4, P5, P6);
impl_system_function!(P1, P2, P3, P4, P5, P6, P7);

#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell};

    use crate::app::App;

    use super::Each;

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

    fn take(values: &Rc<RefCell<Vec<String>>>) -> String {
        let v : Vec<String> = values.borrow_mut().drain(..).collect();

        v.join(", ")
    }

    fn ticker(ticker :Each<TestA>) {
        println!("ticker");
    }

    #[derive(PartialEq, Debug)]
    struct TestA(u32);
}