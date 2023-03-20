use crate::{
    schedule::Schedule, 
    system::{System, IntoSystem}, 
    env::Env
};

pub struct App {
    schedule: Schedule,
    env: Env<'static>,
}

impl App {
    pub fn new() -> Self {
        App {
            schedule: Schedule::new(),
            env: Env::new(),
        }
    }

    /*
    pub fn add_system<S:System+'static>(&mut self, system: S) -> &mut Self
    {
        self.schedule.push(Box::new(system));

        self
    }
     */
    pub fn add_system<M>(&mut self, into_system: impl IntoSystem<M>) -> &mut Self
    {
        self.schedule.push(Box::new(IntoSystem::into_system(into_system)));

        self
    }

    pub fn update(&mut self) -> &mut Self {
        self.schedule.update();
        self
    }
}

/*
pub trait IntoSystem<M> {
    fn to_system(&self) -> Box<dyn System>;
}
 */

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::prelude::*;

    #[test]
    fn app_system() {
        let mut app = App::new();
        let value = Vec::<String>::new();
        let value = Rc::new(RefCell::new(value));
        let ptr = Rc::clone(&value);

        //app.add_system(move || value.borrow_mut().push("update".to_string()));
        assert_eq!(take(&ptr), "");
        app.update();
        assert_eq!(take(&ptr), "update");
        app.update();
        app.update();
        assert_eq!(take(&ptr), "update, update");
    }

    fn take(ptr: &Rc<RefCell<Vec<String>>>) -> String {
        ptr.borrow_mut().drain(..).collect::<Vec<String>>().join(", ")
    }

    fn test_system() {
        println!("hello");
    }

}