use std::{marker::PhantomData, ops::{Deref, DerefMut}};

use crate::{world::prelude::World, system::prelude::Param};

#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell};

    use crate::{app::App, world::prelude::{Res, ResMut}};

    #[test]
    fn base_resource() {
        /*
        let mut app = App::new();
        app.add_resource(TestA(2));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));
        let ptr = values.clone();

        app.add_system(move |r: Res<TestA>| {
            ptr.borrow_mut().push(format!("{:?}", r.get()));
        });
        
        assert_eq!(take(&values), "");
        app.update();
        assert_eq!(take(&values), "TestA(2)");
        app.update();
        assert_eq!(take(&values), "TestA(2)");
        */
    }

    fn take(values: &Rc<RefCell<Vec<String>>>) -> String {
        let v : Vec::<String> = values.borrow_mut().drain(..).collect();

        v.join(", ")
    }

    #[test]
    fn mut_resource() {
        /*
        let mut app = App::new();
        app.add_resource(TestA(1));

        let values = Rc::new(RefCell::new(Vec::<String>::new()));
        let ptr = values.clone();

        app.add_system(move |mut r: ResMut<TestA>| {
            // r.get_mut().0 += 1;
            r.0 += 1;
            ptr.borrow_mut().push(format!("{:?}", r.get()));
        });
        assert_eq!(take(&values), "");
        app.update();
        assert_eq!(take(&values), "TestA(2)");
        assert_eq!(app.resource::<TestA>(), &TestA(2));
        app.update();
        assert_eq!(take(&values), "TestA(3)");
        assert_eq!(app.get_resource::<TestA>().expect(""), &TestA(3));
        */
    }

    #[test]
    fn multi_resource() {
        let mut app = App::new();
        app.insert_resource(Vec::<String>::new());
        app.insert_resource(TestA(1));

        //app.add_system(test_multi_resource_a);

        app.update();
        assert_eq!(app.resource::<Vec<String>>().join(", "), "TestA(1)");
        app.resource_mut::<Vec<String>>().drain(..);
    }

    fn test_multi_resource_a(mut out: ResMut<Vec<String>>, res: Res<TestA>) {
        out.get_mut().push(format!("{:?}", res.get()));
    }

    #[derive(PartialEq, Debug)]
    struct TestA(u32);
    
}

//fn get<'w>(world: &'w World) -> &'w T;
