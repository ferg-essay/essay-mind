
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StriatumId(usize);

impl StriatumId {
    pub fn new(id: usize) -> Self {
        StriatumId(id)
    }

    pub fn i(&self) -> usize {
        self.0
    }
}

pub trait StriatumSnr {
    fn attend(&mut self, id: ActionId, value: f32);
}
