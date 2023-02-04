pub struct FiberKey {
    pub id : i32,
    pub name : String,
}

pub struct FiberBuilder {
     _fiber_id : i32,
}

impl FiberBuilder {
    
    pub fn new() -> FiberBuilder {
        println!("fiber builder");
        Self {
            _fiber_id : 0
        }
    }

    pub fn fiber_id(&mut self) -> i32 {
        let x = self._fiber_id + 1;
        self._fiber_id = x;

        x
    }

    pub fn fiber_key(&mut self, s:&str) -> FiberKey {
        FiberKey { 
            id : self.fiber_id(),
            name : String::from(s),
         }
    }
}

pub struct MindNode {
    pub name : String
}

impl MindNode {
    pub fn send(&self, key : &str, value : f32, p : f32) {
        println!("send {} ({}, {}, {})", self.name, key, value, p);
    }
}

pub fn test_core() -> () { 
    println!("test_cores");
}
