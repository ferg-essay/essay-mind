use std::{any::{TypeId, type_name}, borrow::Cow};

use ticker_macros::{SystemParam, Component};

struct App {
    systems: Vec<Box<dyn System>>,
}

impl Default for App {
    fn default() -> Self {
        Self { systems: Default::default() }
    }
}

trait IntoSystemAppConfig<Marker>: Sized {
}

pub trait System: 'static {
    fn type_id(&self) -> TypeId;
    
    fn run(&mut self);
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ComponentId(usize);

impl ComponentId {
    pub const fn new(index: usize) -> ComponentId {
        ComponentId(index)
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

struct ComponentDescriptor {
    name: Cow<'static, str>,
    type_id: Option<TypeId>,
}

impl std::fmt::Debug for ComponentDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentDescriptor")
            .field("name", &self.name)
            .field("type_id", &self.type_id)
            .finish()
    }
}

impl ComponentDescriptor {
    pub fn new<T: Component>() -> Self {
        Self {
            name: Cow::Borrowed(type_name::<T>()),
            type_id: Some(TypeId::of::<T>()),
        }
    }

    #[inline]
    pub fn type_id(&self) -> Option<TypeId> {
        self.type_id
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

pub trait Component: 'static {

}

impl App {
    fn new() -> App {
        Self {
            ..Default::default()
        }
    }

    fn add_system(&mut self, param: impl System) -> &mut Self {
        let mut param = Box::new(param);
        self.systems.push(param);
        self
    }

    fn component<C:Component>(&mut self, param: C) -> &mut Self {
        self
    }

    fn tick(&mut self) {
        for system in &mut self.systems {
            system.run();
        }
    }
}

impl<Func:'static> System for Func 
    where Func: FnMut()
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<Func>()
    }

    fn run(&mut self) {
        self()
    }
}

trait SystemParam: Sized {
    type Item: SystemParam;

    fn get_param(

    ) -> Self::Item;
}

trait SystemParamFunction<Marker>: 'static {
    type Param: SystemParam;

    fn run(
        &mut self, 
        param: SystemParamItem<Self::Param>
    );

    fn lazy(&self) -> &Self {
        self
    }
}

trait EvalQuery {
    type Item<'q>;
}

impl<T: Component> EvalQuery for &T {
    type Item<'q> = &'q T;
}

struct Eval<'q, T: EvalQuery> {
    item: &'q T
}

impl<'q, T: EvalQuery> SystemParam for Eval<'q, T> {
    type Item = Eval<'q, T>;

    fn get_param(

    ) -> Self::Item {
        todo!();
    }
}

//impl SystemParam for () {

//}

struct SystemParamItem<P> {
    param: P,
}

struct Dummy {

}

impl SystemParam for Dummy {
    type Item = Dummy;

    fn get_param(

    ) -> Self::Item {
        todo!()
    }
}

impl<Func:'static> SystemParamFunction<()> for Func 
    where Func: FnMut() +
        FnMut()
{
    type Param = Dummy;

    fn run(&mut self, _param: SystemParamItem<Dummy>) {
        self()
    }
}

impl<Func:'static, P:SystemParam> SystemParamFunction<P> for Func 
    where Func: FnMut(P) +
        FnMut(SystemParamItem<P>)
{
    type Param = P;

    fn run(&mut self, param: SystemParamItem<P>) {
        self(param)
    }
}

#[test]
fn test_tick() {
    let mut app = App::new();
    app.add_system(hello.lazy());
    app.tick();
    app.tick();
}

#[test]
fn test_component() {
    let mut app = App::new();
    app.component(TestComponent {});
    app.add_system(hello);
    app.tick();
    app.tick();
}

fn hello() {
    println!("hello, world");
}

fn tick(test: &Test) {
    println!("test tick");
}

struct Test;

#[derive(Component)]
struct TestComponent {

}