use crate::action::shared_memory::SharedMemory;

#[test]
fn basic_shared_memory() {
    let memory = SharedMemory::<Test<i32>,2>::new();

    assert_eq!(*memory.read(0).unwrap().value(), 0);

    memory.write(0).unwrap().replace(10);
    assert_eq!(*memory.read(0).unwrap().value(), 0);
    assert_eq!(*memory.read(1).unwrap().value(), 10);

    memory.write(1).unwrap().replace(20);
    assert_eq!(*memory.read(0).unwrap().value(), 20);
    assert_eq!(*memory.read(1).unwrap().value(), 10);
}

#[derive(Default,Debug)]
struct Test<T:Default> {
    value: T
}

impl<T:Default> Test<T> {
    fn value(&self) -> &T {
        &self.value
    }

    fn replace(&mut self, value: T) {
        self.value = value;
    }
}