#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionId(usize);

impl ActionId {
    pub fn new(id: usize) -> Self {
        ActionId(id)
    }

    pub fn i(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Turn {
    Left,
    Right,
}

impl Turn {
    pub fn id(&self) -> ActionId {
        match self {
            Turn::Left => ActionId::new(0),
            Turn::Right => ActionId::new(1),
        }
    }
}