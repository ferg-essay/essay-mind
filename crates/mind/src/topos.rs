use std::fmt;

#[derive(Clone)]
pub enum Topos {
    Nil,
    Unit(f32),
}

impl fmt::Debug for Topos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Topos::Unit(value) => { write!(f, "Unit({})", value) }
            Topos::Nil => { write!(f, "Nil") }
        }
    }
}
/*
impl Clone for Topos {
    fn clone(&self) -> Self {
        match self {
            Self::Unit(Nil => Self::Nil,
            Self::Nil => Self::Nil,
        }
    }
}
*/