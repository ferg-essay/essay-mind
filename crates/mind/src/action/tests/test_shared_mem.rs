use crate::action::shared_memory::SharedWriter;

#[test]
fn basic_shared_memory() {
    let writer = SharedWriter::<Test<i32>>::new();
    let reader = writer.reader();

    assert_eq!(*reader.read(0).unwrap().value(), 0);

    writer.write(0).unwrap().replace(10);
    assert_eq!(*reader.read(0).unwrap().value(), 0);
    assert_eq!(*reader.read(1).unwrap().value(), 10);

    writer.write(1).unwrap().replace(20);
    assert_eq!(*reader.read(0).unwrap().value(), 20);
    assert_eq!(*reader.read(1).unwrap().value(), 10);
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