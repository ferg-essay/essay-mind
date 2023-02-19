use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
mod builder_test;

struct AddInner {
    values: Vec<String>,
}

impl AddInner {
}
#[derive(Clone)]
struct AddItem {
    ptr: Rc<RefCell<AddInner>>,
}

impl AddItem {
    fn new() -> Self {
        AddItem {
            ptr: Rc::new(RefCell::new(AddInner {
                values: Vec::new()
            }))
        }
    }

    fn add(&mut self, value: String) {
        self.ptr.borrow_mut().values.push(value);
    }

    fn peek(&self) -> String {
        (self.ptr.borrow().values).join(", ")
    }

    fn take(&mut self) -> String {
        let msg = self.peek();

        self.ptr.borrow_mut().values.drain(..);

        msg
    }
}
