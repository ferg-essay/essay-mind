mod builder;
pub mod fiber;
use fiber::*;
use builder::*;
use std::{fmt, cell::RefCell, rc::Rc, error::Error};

/*
pub type FiberFn<T> = dyn Fn(&FiberId,&T)->() + Send;

pub struct FiberId {
    pub id: i32,
    pub name: String,
}

impl FiberId {
    pub fn new(id: i32, name: &str) -> FiberId {
        Self {
            id,
            name: String::from(name),
        }
    }
}

impl Clone for FiberId {
    fn clone(&self) -> FiberId {
        FiberId { id: self.id, name: self.name.clone(), }
    }
}

impl fmt::Display for FiberId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FiberId[{},{}]", self.id, self.name)
    }
}
*/
/*
pub struct FiberToBind<T>
{
    callback: Box<FiberFn<T>>,
}

impl<T> FiberToBind<T> {
    pub fn new(cb : Box<FiberFn<T>>) -> FiberToBind<T>
    {
        Self { callback : cb }
    }

    fn send(&self, id: &FiberId, args: &T)
    {
        (self.callback)(id, args)
    }
}

pub struct Fiber<T>
{
    pub id: FiberId,

    to: Vec<Box<FiberFn<T>>>,
}

impl<T> Fiber<T> {
    pub fn send(&self, args: &T) {
        for to in &self.to {
            to(&self.id, args)
        }
    }
}

impl<T> fmt::Display for Fiber<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fiber[{},{}]", self.id.id, self.id.name)
    }
}

pub struct FiberBuilder<T>
{
    parent: Rc<RefCell<TickerBuilderData>>,

    name: String,

    is_built: bool,

    //to: Vec<FiberToBind<T>>,
    to: Vec<Box<FiberFn<T>>>,
}

impl<T> FiberBuilder<T> {
    fn new(parent: &Rc<RefCell<TickerBuilderData>>, name: &str)->FiberBuilder<T> {
        assert!(! parent.borrow().is_built);

        Self {
            parent: Rc::clone(&parent),
            name: String::from(name),
            is_built: false,
            to: Vec::new(),
        }
    }
    /*
    pub fn to_bind(&mut self, bind: FiberToBind<T>) {
        self.to.push(bind);
    }
    */
    
    pub fn to(&mut self, callback: Box<FiberFn<T>>) -> &mut FiberBuilder<T> {
        //let bind = FiberToBind { callback: callback };

        self.to.push(callback);

        self
    }

    pub fn build(&mut self) -> Fiber<T> {
        assert!(! self.parent.borrow().is_built);
        assert!(! self.is_built);
        self.is_built = true;

        let mut fiber_vec : Vec<Box<FiberFn<T>>> = Vec::new();

        for v in self.to.drain(..) {
            fiber_vec.push(v);
        }

        Fiber {
            id: FiberId { 
                id: self.parent.borrow_mut().fiber_id(),
                name: self.name.clone(),
            },
            to: fiber_vec,
        }
    }
}
*/
pub struct Ticker {
    pub name : String
}

impl Ticker {
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticker[{}]", self.name)
    }
}

pub struct TickerSystem {
}

impl TickerSystem {
}
/* 
struct TickerBuilderData {
    is_built : bool,

    fiber_id : i32,
}

impl TickerBuilderData {
    fn fiber_id(&mut self) -> i32 {
        self.fiber_id += 1;
        self.fiber_id
    }

    fn build(&mut self) {
        assert!(! self.is_built);
        self.is_built = true;
    }
}
*/
pub struct TickerSystemBuilder {
    data : Rc<RefCell<TickerBuilderData>>,
}

impl TickerSystemBuilder {
    pub fn new() -> TickerSystemBuilder {
        Self {
            data: Rc::new(RefCell::new(TickerBuilderData { 
                is_built: false,
                fiber_id: 0,
            }))
        }
    }

    pub fn fiber<T>(&mut self, name: &str) -> FiberBuilder<T> {
        assert!(! self.data.borrow().is_built);

        FiberBuilder::new(&self.data, name)
    }

    pub fn build(&mut self) -> TickerSystem {
        self.data.borrow_mut().build();

        TickerSystem {
        }
    }
}