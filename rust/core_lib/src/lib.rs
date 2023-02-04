use std::fmt;

type FiberFn<T> = dyn Fn(&FiberId,&T)->() + Send;

struct FiberBind<T>
{
    id: FiberId,
    callback: Box<FiberFn<T>>,
}

impl<T> FiberBind<T> {
    pub fn new(id: &FiberId, callback: Box<FiberFn<T>>) -> FiberBind<T>
    {
        Self {
            id: id.clone(),
            callback,
        }
    }

    fn send(&self, args: &T)
    {
        (self.callback)(&self.id, args)
    }
}

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

pub struct Fiber<T>
{
    pub id: FiberId,

    to: Vec<Box<FiberBind<T>>>,
}

impl<T> Fiber<T> {
    pub fn new(builder: &mut FiberBuilder, name: &str) -> Fiber<T> {
        Self {
            id: FiberId { 
                id: builder.fiber_id(),
                name: String::from(name),
            },

            to: Vec::new(),
        }
    }
    
    pub fn to(&mut self, callback: Box<FiberFn<T>>)
    {
        let bind = Box::new(FiberBind::new(&self.id, callback));

        self.to.push(bind)
    }
    
    pub fn send(&self, args: &T) {
        for to in &self.to {
            to.send(args)
        }
    }
}

pub struct FiberBuilder {
     fiber_id: i32,
}

impl FiberBuilder {
    
    pub fn new() -> FiberBuilder {
        println!("fiber builder");
        Self {
            fiber_id : 0
        }
    }

    fn fiber_id(&mut self) -> i32 {
        let x = self.fiber_id + 1;
        self.fiber_id = x;

        x
    }
}

pub struct MindNode {
    pub name : String
}

impl MindNode {
}

pub fn test_core() -> () { 
    println!("test_cores");
}
