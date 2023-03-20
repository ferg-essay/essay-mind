use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;


#[test]
fn fun_empty() {
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

#[test]
fn fun_resource() {
    let mut app = App::new();

    //app.add_system(test_resource);
    app.update();
}

fn test_system() {
    println!("hello");
}

fn test_resource(res: Res<String>) {
}

fn take(ptr: &Rc<RefCell<Vec<String>>>) -> String {
    ptr.borrow_mut().drain(..).collect::<Vec<String>>().join(", ")
}
