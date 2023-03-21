use super::table::{Table};

pub struct World<'w> {
    entities: Table<'w>,
    resources: Table<'w>,
}

impl<'e> World<'e> {
    pub fn new() -> Self {
        Self {
            entities: Table::new(),
            resources: Table::new(),
        }
    }

    pub fn add_entity<T:'static>(&mut self, value: T) {
        self.entities.push(value);
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn eval<T:'static,F>(&mut self, fun: &mut F)
        where F: FnMut(&mut T)
    {
        self.entities.eval(fun);
    }

    pub fn add_resource<T:'static>(&mut self, value: T) {
        self.resources.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    fn spawn() {
        let mut world = World::new();
        assert_eq!(world.len(), 0);

        world.add_entity(TestA(1));
        assert_eq!(world.len(), 1);

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        world.add_entity(TestB(10000));
        assert_eq!(world.len(), 2);

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10000)");

        world.add_entity(TestB(100));
        assert_eq!(world.len(), 3);

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10000),TestB(100)");

        let mut values = Vec::<String>::new();
        world.eval(&mut|t: &mut TestB| t.0 += 1);
        world.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10001),TestB(101)");
    }

    #[derive(Debug)]
    struct TestA(u32);

    #[derive(Debug)]
    struct TestB(u16);
}