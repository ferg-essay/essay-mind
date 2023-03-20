use std::marker::PhantomData;

pub trait System {
    fn run(&mut self);
}


pub struct Res<V> {
    value: PhantomData<V>,
}

/*
impl<F> System for F
    where F:FnMut() -> ()
{
    fn run(&mut self) {
        (self)()
    }
}
*/

pub trait SystemParam {
    type Item;

    fn get_param() -> Self::Item;
}

impl<V> SystemParam for Res<V> {
    type Item = Res<V>;

    fn get_param() -> Self::Item {
        todo!()
    }
}

//
// tuples
//

impl SystemParam for () {
    type Item = ();

    fn get_param() -> Self::Item {
        ()
    }
}

impl<P1:SystemParam, P2:SystemParam> SystemParam for (P1, P2) {
    type Item = (P1::Item, P2::Item);

    fn get_param() -> Self::Item {
        (P1::get_param(), P2::get_param())
    }
}

pub trait IntoSystem<Marker>: Sized {
    type System:System + 'static;

    fn into_system(this: Self) -> Self::System;
}
//struct Dummy;

impl<Sys: System + 'static> IntoSystem<Sys> for Sys {
    type System = Sys;
    fn into_system(this: Self) -> Sys {
        this
    }
}

pub struct FunctionSystem<Marker, F>
where
    F: SystemFunction<Marker>
{
    fun: F,
    marker: PhantomData<Marker>,
}

impl<Marker, F> System for FunctionSystem<Marker, F>
where
    Marker: 'static,
    F: SystemFunction<Marker>
{
    fn run(&mut self) {
        let params = F::Param::get_param(
        );

        self.fun.run(params);
    }
}    

// IsFun prevents collision
struct IsFun;

impl<Marker, F:'static> IntoSystem<(Marker,IsFun)> for F
where
    Marker: 'static,
    F: SystemFunction<Marker>
{
    type System = FunctionSystem<Marker, F>;

    fn into_system(this: Self) -> Self::System {
        FunctionSystem {
            fun: this,
            marker: Default::default()
        }
    }

}

type ParamArg<P> = <P as SystemParam>::Item;

pub trait SystemFunction<Marker> {
    type Param: SystemParam;

    fn run(&mut self, param: ParamArg<Self::Param>);
}

impl<F:'static> SystemFunction<fn()> for F
    where F:FnMut() -> ()
{
    type Param = ();

    fn run(&mut self, _param: ParamArg<()>) {
        (self)()
    }
}

impl<F:'static,P:SystemParam> SystemFunction<fn(P)> for F
    where F:FnMut(P) -> () +
            FnMut(ParamArg<P>) -> ()
{
    type Param = P;

    fn run(&mut self, param: ParamArg<P>) {
        (self)(param)
    }
}

impl<F:'static,P1:SystemParam,P2:SystemParam> SystemFunction<fn(P1,P2)> for F
    where F:FnMut(P1, P2) -> () +
            FnMut(ParamArg<P1>, ParamArg<P2>) -> ()
{
    type Param = (P1, P2);

    fn run(&mut self, param: ParamArg<(P1,P2)>) {
        let (p1, p2) = param;
        (self)(p1, p2)
    }
}
 
#[cfg(test)]
mod tests {
    use std::any::type_name;
    use std::marker::PhantomData;

    use super::IntoSystem;
    use super::System;
    use super::SystemParam;

    #[test]
    fn help() {
        build(test_null);
        build(test_arg1);
        build(test_arg2);
    }

    fn build<M>(fun: impl IntoSystem<M>) {
        let mut system = IntoSystem::into_system(fun);
        system.run();
    }

    fn test_null() {
       println!("test-null"); 
    }

    fn test_arg1(arg1: TestArg<String>) {
        println!("test-arg1 {}", arg1.name); 
    }

    fn test_arg2(arg1: TestArg<String>, arg2: TestArg<u32>) {
        println!("test-arg2 ({}, {})", arg1.name, arg2.name); 
    }

    #[derive(Debug)]
    struct TestArg<V> {
        name: String,
        marker: PhantomData<V>,
    }

    impl<V> SystemParam for TestArg<V> {
        type Item = TestArg<V>;

        fn get_param() -> Self::Item {
            Self {
                name: type_name::<V>().to_string(),
                marker: PhantomData,
            }
        }
    }
 }