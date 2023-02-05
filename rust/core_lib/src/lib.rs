use std::fmt;

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

pub struct FiberIdBuilder {
    fiber_id: i32,
}

impl FiberIdBuilder {
   
   pub fn new() -> FiberIdBuilder {
       println!("fiber builder");
       Self {
           fiber_id : 0
       }
   }

   pub fn fiber<T>(&mut self, name : &str) -> FiberBuilder<T>
   {
        FiberBuilder {
            id: FiberId { id: self.fiber_id(), name: String::from(name) },

            to: Vec::new(),
        }
   }

   fn fiber_id(&mut self) -> i32 {
       let x = self.fiber_id + 1;
       self.fiber_id = x;

       x
   }
}

pub struct FiberBind<T>
{
    callback: Box<FiberFn<T>>,
}

impl<T> FiberBind<T> {
    pub fn new(cb : Box<FiberFn<T>>) -> FiberBind<T>
    {
        Self { callback : cb }
    }

    fn send(&self, id: &FiberId, args: &T)
    {
        (self.callback)(id, args)
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

    to: Vec<FiberBind<T>>,
}

impl<T> Fiber<T> {
    /*
    pub fn builder(id_builder: &mut FiberIdBuilder, name : &str) -> FiberBuilder<T>
    {
        FiberBuilder {
            id: FiberId { id: id_builder.fiber_id(), name: String::from(name) },

            to: Vec::new(),
        }
    }
    */
    /*
    pub fn new(builder: &mut FiberIdBuilder, name: &str) -> Fiber<T> {
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
    */
    
    pub fn send(&self, args: &T) {
        for to in &self.to {
            to.send(&self.id, args)
        }
    }
}

pub struct FiberBuilder<T>
{
    id: FiberId,

    to: Vec<FiberBind<T>>,
}

impl<T> FiberBuilder<T> {
    /*
    pub fn new(builder: &mut FiberIdBuilder, name: &str) -> FiberBuilder<T> {
        Self {
            id: FiberId { 
                id: builder.fiber_id(),
                name: String::from(name),
            },

            to: Vec::new(),
        }
    }
    */

    pub fn to_bind(&mut self, bind: FiberBind<T>)
    {
        self.to.push(bind);
    }
    
    pub fn to(&mut self, callback: Box<FiberFn<T>>) -> &mut FiberBuilder<T>
    {
        //let bind = FiberBind::new(self.id.clone(), callback);
        let bind = FiberBind { callback: callback };

        self.to.push(bind);

        self
    }

    pub fn build(&mut self) -> Fiber<T>
    {
        let mut fiber_vec : Vec<FiberBind<T>> = Vec::new();

        for v in self.to.drain(..) {
            fiber_vec.push(v);
        }

        Fiber {
            id: self.id.clone(),
            to: fiber_vec,
        }
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
